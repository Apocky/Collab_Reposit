//! The `id32` registry: content IDs and aliases, collision-free. SPEC: canon
//! §1.1; doc 02 §11.1 and §13.1.
//!
//! Every content ID and every alias's old ID owns one `id32`. A second string
//! with an `id32` already present, an identical string included, is rejected
//! with `id32_collision`. An alias's old ID resolves to its new ID's `id32`,
//! both by name (content files that still say the old ID) and by `id32`
//! (saves written before the rename).

use crate::ids::{ContentId, Id32};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;
use palimpsest_math::hash::fnv1a32;

/// SPEC: canon §1.1 error code for an `id32` that is already taken.
pub const ID32_COLLISION: &str = "id32_collision";
/// SPEC: canon §1.1 error code for an alias whose new ID is not registered
/// content. The canon names no code for this case; this one is provisional.
pub const ALIAS_TARGET_UNKNOWN: &str = "alias_target_unknown";

/// SPEC: canon §1.1 why a registration was refused. The registry is unchanged
/// after any error.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegistryError {
    /// `incoming` has the same `id32` as the already registered `existing`
    /// (which may be the same string).
    Id32Collision {
        /// The shared `id32`.
        id32: Id32,
        /// The content ID or alias old ID registered first.
        existing: ContentId,
        /// The content ID or alias old ID being registered.
        incoming: ContentId,
    },
    /// The alias `old -> new` names a `new` that is not a registered content
    /// ID (unknown, or itself an alias's old ID).
    AliasTargetUnknown {
        /// The alias's old ID.
        old: ContentId,
        /// The alias's new ID.
        new: ContentId,
    },
}

impl RegistryError {
    /// SPEC: canon §1.1 machine-readable error code (`id32_collision` for every
    /// collision, base or mod).
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Id32Collision { .. } => ID32_COLLISION,
            Self::AliasTargetUnknown { .. } => ALIAS_TARGET_UNKNOWN,
        }
    }
}

impl fmt::Display for RegistryError {
    /// SPEC: canon §1.1 `code: detail`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id32Collision {
                id32,
                existing,
                incoming,
            } if existing == incoming => {
                write!(
                    f,
                    "{ID32_COLLISION}: `{incoming}` ({id32}) is already registered"
                )
            }
            Self::Id32Collision {
                id32,
                existing,
                incoming,
            } => write!(
                f,
                "{ID32_COLLISION}: `{incoming}` and `{existing}` share id32 {id32}"
            ),
            Self::AliasTargetUnknown { old, new } => write!(
                f,
                "{ALIAS_TARGET_UNKNOWN}: alias `{old}` names `{new}`, which is not registered content"
            ),
        }
    }
}

impl core::error::Error for RegistryError {}

/// SPEC: canon §1.1 what an `id32` stands for.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Entry {
    /// A content ID.
    Content(ContentId),
    /// A renamed ID: `old` resolves to the content ID whose `id32` is `target`.
    Alias {
        /// The retired ID.
        old: ContentId,
        /// The `id32` of the new content ID.
        target: Id32,
    },
}

impl Entry {
    /// SPEC: canon §1.1 the string that owns this entry's `id32`.
    #[must_use]
    pub fn id(&self) -> &ContentId {
        match self {
            Self::Content(id) | Self::Alias { old: id, .. } => id,
        }
    }
}

/// SPEC: canon §1.1 and doc 02 §11.1 ID registry, keyed by `id32` in a
/// `BTreeMap` so iteration is in ascending `id32` order on every platform.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdRegistry {
    entries: BTreeMap<Id32, Entry>,
    content_len: usize,
    alias_len: usize,
}

impl IdRegistry {
    /// SPEC: canon §1.1 an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// SPEC: canon §1.1 build-time check of base content and aliases. Content
    /// IDs are registered first, in ascending byte order, then aliases in
    /// ascending order of old ID, so the result and the error list do not
    /// depend on file or record order. Every error is reported, not just the
    /// first.
    pub fn build<C, A>(content: C, aliases: A) -> Result<Self, Vec<RegistryError>>
    where
        C: IntoIterator<Item = ContentId>,
        A: IntoIterator<Item = (ContentId, ContentId)>,
    {
        let mut content: Vec<ContentId> = content.into_iter().collect();
        content.sort();
        let mut aliases: Vec<(ContentId, ContentId)> = aliases.into_iter().collect();
        aliases.sort();
        let mut reg = Self::new();
        let mut errors = Vec::new();
        for id in content {
            if let Err(e) = reg.insert_content(id) {
                errors.push(e);
            }
        }
        for (old, new) in aliases {
            if let Err(e) = reg.insert_alias(old, &new) {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(reg)
        } else {
            Err(errors)
        }
    }

    /// SPEC: canon §1.1 register one content ID (base content, or a mod record
    /// at mod load). Fails with `id32_collision` if its `id32` is taken.
    pub fn insert_content(&mut self, id: ContentId) -> Result<Id32, RegistryError> {
        let id32 = id.id32();
        self.check_free(id32, &id)?;
        self.entries.insert(id32, Entry::Content(id));
        self.content_len += 1;
        Ok(id32)
    }

    /// SPEC: canon §1.1 register the alias `old -> new` (an entry of
    /// `data/aliases.json`). `old` takes part in the collision check; `new` must
    /// already be a registered content ID. Returns `new`'s `id32`, which `old`
    /// now resolves to.
    pub fn insert_alias(&mut self, old: ContentId, new: &ContentId) -> Result<Id32, RegistryError> {
        let target = match self.entries.get(&new.id32()) {
            Some(Entry::Content(id)) if id == new => new.id32(),
            _ => {
                return Err(RegistryError::AliasTargetUnknown {
                    old,
                    new: new.clone(),
                });
            }
        };
        let id32 = old.id32();
        self.check_free(id32, &old)?;
        self.entries.insert(id32, Entry::Alias { old, target });
        self.alias_len += 1;
        Ok(target)
    }

    fn check_free(&self, id32: Id32, incoming: &ContentId) -> Result<(), RegistryError> {
        match self.entries.get(&id32) {
            None => Ok(()),
            Some(entry) => Err(RegistryError::Id32Collision {
                id32,
                existing: entry.id().clone(),
                incoming: incoming.clone(),
            }),
        }
    }

    /// SPEC: canon §1.1 and doc 02 §3.4 `id_of`: the `id32` a content file's
    /// reference resolves to: the ID's own `id32`, or for an alias's old ID the
    /// new ID's `id32`. `None` for anything not registered, text keys included.
    #[must_use]
    pub fn id_of(&self, id: &str) -> Option<Id32> {
        let raw = Id32(fnv1a32(id.as_bytes()));
        match self.entries.get(&raw)? {
            Entry::Content(c) if c.as_str() == id => Some(raw),
            Entry::Alias { old, target } if old.as_str() == id => Some(*target),
            _ => None,
        }
    }

    /// SPEC: canon §1.1 the `id32` a stored `id32` resolves to at load: itself
    /// for content, the new ID's `id32` for an alias's old ID.
    #[must_use]
    pub fn canonical(&self, raw: Id32) -> Option<Id32> {
        match self.entries.get(&raw)? {
            Entry::Content(_) => Some(raw),
            Entry::Alias { target, .. } => Some(*target),
        }
    }

    /// SPEC: canon §1.1 the content ID a stored `id32` resolves to, following
    /// an alias to its new ID.
    #[must_use]
    pub fn content_id(&self, raw: Id32) -> Option<&ContentId> {
        match self.entries.get(&self.canonical(raw)?)? {
            Entry::Content(id) => Some(id),
            Entry::Alias { .. } => None,
        }
    }

    /// SPEC: canon §1.1 the raw entry registered under `raw`, without
    /// following aliases.
    #[must_use]
    pub fn entry(&self, raw: Id32) -> Option<&Entry> {
        self.entries.get(&raw)
    }

    /// SPEC: canon §1.1 number of content IDs.
    #[must_use]
    pub fn content_len(&self) -> usize {
        self.content_len
    }

    /// SPEC: canon §1.1 number of aliases.
    #[must_use]
    pub fn alias_len(&self) -> usize {
        self.alias_len
    }

    /// SPEC: canon §1.1 `true` when nothing is registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// SPEC: canon §1.1 every entry in ascending `id32` order.
    pub fn entries(&self) -> impl Iterator<Item = (Id32, &Entry)> {
        self.entries.iter().map(|(k, v)| (*k, v))
    }

    /// SPEC: canon §1.1 every content ID in ascending `id32` order.
    pub fn content_ids(&self) -> impl Iterator<Item = (Id32, &ContentId)> {
        self.entries().filter_map(|(k, e)| match e {
            Entry::Content(id) => Some((k, id)),
            Entry::Alias { .. } => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{COLLISION_PAIRS, content_id_strategy};
    use alloc::string::ToString;
    use alloc::vec;
    use proptest::prelude::*;

    fn cid(s: &str) -> ContentId {
        ContentId::new(s).expect("valid test id")
    }

    /// SPEC: canon §1.1 content registration and lookups.
    #[test]
    fn registers_and_resolves_content() {
        let mut reg = IdRegistry::new();
        assert!(reg.is_empty());
        let cp = reg.insert_content(cid("genre.cyberpunk")).expect("free");
        assert_eq!(cp, Id32(0x99b5_4427));
        assert_eq!(reg.id_of("genre.cyberpunk"), Some(cp));
        assert_eq!(reg.canonical(cp), Some(cp));
        assert_eq!(reg.content_id(cp), Some(&cid("genre.cyberpunk")));
        assert_eq!(reg.entry(cp), Some(&Entry::Content(cid("genre.cyberpunk"))));
        assert_eq!(reg.id_of("genre.wuxia"), None);
        assert_eq!(reg.id_of("Genre.Cyberpunk"), None);
        assert_eq!((reg.content_len(), reg.alias_len()), (1, 0));
    }

    /// SPEC: canon §1.1 two content IDs with equal fnv1a32 (a real collision
    /// found by brute force) are rejected with `id32_collision`, and the
    /// registry is unchanged.
    #[test]
    fn rejects_real_fnv_collision() {
        let (a, b, h) = COLLISION_PAIRS[0];
        let mut reg = IdRegistry::new();
        reg.insert_content(cid(a)).expect("free");
        let before = reg.clone();
        let err = reg.insert_content(cid(b)).expect_err("collision");
        assert_eq!(
            err,
            RegistryError::Id32Collision {
                id32: Id32(h),
                existing: cid(a),
                incoming: cid(b),
            }
        );
        assert_eq!(err.code(), "id32_collision");
        assert_eq!(reg, before);
        assert_eq!(reg.id_of(b), None);
        assert!(err.to_string().starts_with("id32_collision: "));
    }

    /// SPEC: doc 02 §13.1: an identical ID (a mod reusing a base ID) is an
    /// `id32_collision` too.
    #[test]
    fn rejects_identical_id() {
        let mut reg = IdRegistry::new();
        reg.insert_content(cid("item.sword")).expect("free");
        let err = reg.insert_content(cid("item.sword")).expect_err("dup");
        assert_eq!(err.code(), ID32_COLLISION);
        let expected = alloc::format!(
            "id32_collision: `item.sword` ({}) is already registered",
            Id32(fnv1a32(b"item.sword"))
        );
        assert_eq!(err.to_string(), expected);
    }

    /// SPEC: canon §1.1 an alias's old ID maps to the new ID's id32, by name
    /// and by stored id32.
    #[test]
    fn alias_maps_to_new_id32() {
        let mut reg = IdRegistry::new();
        let new = reg
            .insert_content(cid("arch.melee.blade_long"))
            .expect("free");
        let old = cid("arch.melee.longsword");
        let old_raw = old.id32();
        assert_eq!(
            reg.insert_alias(old, &cid("arch.melee.blade_long")),
            Ok(new)
        );
        assert_eq!(reg.id_of("arch.melee.longsword"), Some(new));
        assert_eq!(reg.canonical(old_raw), Some(new));
        assert_eq!(reg.content_id(old_raw), Some(&cid("arch.melee.blade_long")));
        assert!(matches!(reg.entry(old_raw), Some(Entry::Alias { target, .. }) if *target == new));
        assert_eq!((reg.content_len(), reg.alias_len()), (1, 1));
        assert_eq!(reg.content_ids().count(), 1);
    }

    /// SPEC: canon §1.1 an alias's old ID takes part in the collision check,
    /// against content and against other aliases.
    #[test]
    fn alias_old_id_takes_part_in_collision_check() {
        let (c1, c2, h) = COLLISION_PAIRS[1];
        let mut reg = IdRegistry::new();
        reg.insert_content(cid("item.target")).expect("free");
        reg.insert_content(cid(c1)).expect("free");
        let err = reg
            .insert_alias(cid(c2), &cid("item.target"))
            .expect_err("alias vs content");
        assert_eq!(
            err,
            RegistryError::Id32Collision {
                id32: Id32(h),
                existing: cid(c1),
                incoming: cid(c2),
            }
        );
        let (a1, a2, _) = COLLISION_PAIRS[2];
        reg.insert_alias(cid(a1), &cid("item.target"))
            .expect("free");
        let err = reg
            .insert_alias(cid(a2), &cid("item.target"))
            .expect_err("alias vs alias");
        assert_eq!(err.code(), ID32_COLLISION);
        // A mod content ID colliding with an alias's old ID is rejected too.
        let err = reg.insert_content(cid(a2)).expect_err("content vs alias");
        assert_eq!(err.code(), ID32_COLLISION);
        // An alias whose old ID is still live content collides with itself.
        let err = reg
            .insert_alias(cid("item.target"), &cid("item.target"))
            .expect_err("self alias");
        assert_eq!(err.code(), ID32_COLLISION);
    }

    /// SPEC: canon §1.1 an alias must name registered content, not another
    /// alias and not an unknown ID.
    #[test]
    fn alias_target_must_be_content() {
        let mut reg = IdRegistry::new();
        reg.insert_content(cid("item.c")).expect("free");
        reg.insert_alias(cid("item.b"), &cid("item.c"))
            .expect("free");
        let err = reg
            .insert_alias(cid("item.a"), &cid("item.b"))
            .expect_err("chain");
        assert_eq!(err.code(), ALIAS_TARGET_UNKNOWN);
        let err = reg
            .insert_alias(cid("item.z"), &cid("item.missing"))
            .expect_err("unknown");
        assert_eq!(
            err,
            RegistryError::AliasTargetUnknown {
                old: cid("item.z"),
                new: cid("item.missing"),
            }
        );
        // The target lookup compares strings, not just id32s.
        let (x, y, _) = COLLISION_PAIRS[3];
        reg.insert_content(cid(x)).expect("free");
        let err = reg
            .insert_alias(cid("item.w"), &cid(y))
            .expect_err("hash twin");
        assert_eq!(err.code(), ALIAS_TARGET_UNKNOWN);
    }

    /// SPEC: canon §1.1 build reports every collision among base content and
    /// aliases, independent of input order.
    #[test]
    fn build_reports_all_errors_deterministically() {
        let (a, b, _) = COLLISION_PAIRS[0];
        let (c, d, _) = COLLISION_PAIRS[1];
        let content = vec![cid(b), cid("item.ok"), cid(a), cid(c)];
        let aliases = vec![(cid(d), cid("item.ok")), (cid("item.old"), cid("item.ok"))];
        let e1 = IdRegistry::build(content.clone(), aliases.clone()).expect_err("errors");
        let mut rc = content;
        rc.reverse();
        let mut ra = aliases;
        ra.reverse();
        let e2 = IdRegistry::build(rc, ra).expect_err("errors");
        assert_eq!(e1, e2);
        assert_eq!(e1.len(), 2);
        assert!(e1.iter().all(|e| e.code() == ID32_COLLISION));
        let ok = IdRegistry::build([cid("item.ok")], [(cid("item.old"), cid("item.ok"))])
            .expect("clean");
        assert_eq!(ok.id_of("item.old"), ok.id_of("item.ok"));
    }

    proptest! {
        /// SPEC: canon §1.1 for any set of distinct valid IDs: build is
        /// order-independent, every ID resolves to its own fnv1a32, and every
        /// rejected ID collided with a registered one.
        #[test]
        fn build_is_order_independent(
            ids in proptest::collection::btree_set(content_id_strategy(), 0..40),
            seed in any::<u64>(),
        ) {
            let list: Vec<ContentId> = ids.iter().map(|s| cid(s)).collect();
            let mut shuffled = list.clone();
            palimpsest_math::Rng::from_seed64(seed).shuffle(&mut shuffled);
            let r1 = IdRegistry::build(list.clone(), []);
            let r2 = IdRegistry::build(shuffled, []);
            prop_assert_eq!(&r1, &r2);
            if let Ok(reg) = r1 {
                prop_assert_eq!(reg.content_len(), list.len());
                for id in &list {
                    prop_assert_eq!(reg.id_of(id.as_str()), Some(Id32(fnv1a32(id.as_str().as_bytes()))));
                }
                let order: Vec<Id32> = reg.entries().map(|(k, _)| k).collect();
                prop_assert!(order.windows(2).all(|w| w[0] < w[1]));
            }
        }

        /// SPEC: canon §1.1 a failed insert never changes the registry, and a
        /// successful alias resolves exactly like its target.
        #[test]
        fn failed_inserts_are_atomic(
            ids in proptest::collection::vec(content_id_strategy(), 1..20),
            old in content_id_strategy(),
        ) {
            let mut reg = IdRegistry::new();
            for s in &ids {
                let before = reg.clone();
                if reg.insert_content(cid(s)).is_err() {
                    prop_assert_eq!(&reg, &before);
                }
            }
            let target = cid(&ids[0]);
            let before = reg.clone();
            match reg.insert_alias(cid(&old), &target) {
                Ok(t) => {
                    prop_assert_eq!(Some(t), reg.id_of(target.as_str()));
                    prop_assert_eq!(reg.id_of(&old), Some(t));
                }
                Err(_) => prop_assert_eq!(&reg, &before),
            }
        }
    }
}
