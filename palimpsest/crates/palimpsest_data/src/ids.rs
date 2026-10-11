//! Content IDs and their compact `Id32` form. SPEC: canon §1.1, §19.
//!
//! The content-ID grammar `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$` (at most 64
//! bytes, ASCII only) is implemented by hand as a `const fn` byte scan, so IDs
//! used in `const` items are checked at compile time and no regex engine is
//! needed.

use alloc::string::String;
use core::borrow::Borrow;
use core::fmt;
use core::str::FromStr;
use palimpsest_math::hash::fnv1a32;

/// SPEC: canon §1.1 maximum length of a content ID in bytes.
pub const CONTENT_ID_MAX_BYTES: usize = 64;
/// SPEC: canon §1.1 minimum segment count: the namespace plus one more segment.
pub const CONTENT_ID_MIN_SEGMENTS: usize = 2;
/// SPEC: canon §1.1 maximum segment count: the namespace plus five more segments.
pub const CONTENT_ID_MAX_SEGMENTS: usize = 6;
/// SPEC: canon §1.10 and §19: `loc` is the text-key namespace, so no content ID
/// may use it (text keys never get an `id32`).
pub const TEXT_KEY_NAMESPACE: &str = "loc";

/// SPEC: canon §1.1. Why a string is not a content ID. When several rules are
/// broken, the first in this order is reported: emptiness, length, bytes from
/// left to right (including empty segments), segment count, reserved namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdError {
    /// The string is empty.
    Empty,
    /// Longer than [`CONTENT_ID_MAX_BYTES`].
    TooLong {
        /// Length in bytes.
        len: usize,
    },
    /// The first byte is not `a..=z`.
    InvalidFirstByte {
        /// The offending byte.
        byte: u8,
    },
    /// A byte outside `a..=z`, `0..=9`, `_` and `.` (this covers upper case and
    /// every non-ASCII byte).
    InvalidByte {
        /// Byte offset.
        index: usize,
        /// The offending byte.
        byte: u8,
    },
    /// Two adjacent dots, or a trailing dot.
    EmptySegment {
        /// Zero-based index of the empty segment.
        segment: usize,
    },
    /// Only a namespace, no further segment.
    TooFewSegments,
    /// More than [`CONTENT_ID_MAX_SEGMENTS`] segments.
    TooManySegments {
        /// Number of segments found.
        count: usize,
    },
    /// The namespace is `loc`, which canon §19 reserves for text keys.
    TextKeyNamespace,
}

impl fmt::Display for IdError {
    /// SPEC: canon §1.1 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Empty => f.write_str("content id is empty"),
            Self::TooLong { len } => write!(
                f,
                "content id is {len} bytes, more than {CONTENT_ID_MAX_BYTES}"
            ),
            Self::InvalidFirstByte { byte } => {
                write!(f, "content id starts with byte 0x{byte:02x}, not a..=z")
            }
            Self::InvalidByte { index, byte } => write!(
                f,
                "content id has byte 0x{byte:02x} at offset {index}, outside [a-z0-9_.]"
            ),
            Self::EmptySegment { segment } => {
                write!(f, "content id segment {segment} is empty")
            }
            Self::TooFewSegments => f.write_str("content id has no segment after the namespace"),
            Self::TooManySegments { count } => write!(
                f,
                "content id has {count} segments, more than {CONTENT_ID_MAX_SEGMENTS}"
            ),
            Self::TextKeyNamespace => {
                f.write_str("namespace `loc` is reserved for text keys (canon §1.10)")
            }
        }
    }
}

impl core::error::Error for IdError {}

/// SPEC: canon §1.1 grammar `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$`, at most
/// 64 bytes, ASCII only, and canon §19: the namespace is not `loc`.
pub const fn validate_content_id(id: &str) -> Result<(), IdError> {
    let b = id.as_bytes();
    if b.is_empty() {
        return Err(IdError::Empty);
    }
    if b.len() > CONTENT_ID_MAX_BYTES {
        return Err(IdError::TooLong { len: b.len() });
    }
    if !b[0].is_ascii_lowercase() {
        return Err(IdError::InvalidFirstByte { byte: b[0] });
    }
    let mut segments = 1;
    let mut seg_len = 0;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'.' {
            if seg_len == 0 {
                return Err(IdError::EmptySegment {
                    segment: segments - 1,
                });
            }
            segments += 1;
            seg_len = 0;
        } else if c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' {
            seg_len += 1;
        } else {
            return Err(IdError::InvalidByte { index: i, byte: c });
        }
        i += 1;
    }
    if seg_len == 0 {
        return Err(IdError::EmptySegment {
            segment: segments - 1,
        });
    }
    if segments < CONTENT_ID_MIN_SEGMENTS {
        return Err(IdError::TooFewSegments);
    }
    if segments > CONTENT_ID_MAX_SEGMENTS {
        return Err(IdError::TooManySegments { count: segments });
    }
    // A valid ID has at least two segments, so namespace `loc` means prefix `loc.`.
    if b[0] == b'l' && b[1] == b'o' && b[2] == b'c' && b[3] == b'.' {
        return Err(IdError::TextKeyNamespace);
    }
    Ok(())
}

/// SPEC: canon §1.1 `true` when `id` is a valid content ID.
#[must_use]
pub const fn is_content_id(id: &str) -> bool {
    validate_content_id(id).is_ok()
}

/// SPEC: canon §1.1 "the first segment is the namespace". Returns `None` when
/// `id` is not a valid content ID.
#[must_use]
pub fn namespace_of(id: &str) -> Option<&str> {
    if is_content_id(id) {
        id.split('.').next()
    } else {
        None
    }
}

/// SPEC: canon §1.1. Mod content IDs use `x_<mod_id>` as their second
/// segment, for example `item.x_frostpack.ice_blade`; base content never uses
/// a second segment that starts with `x_`.
pub const MOD_SEGMENT_PREFIX: &str = "x_";

/// SPEC: canon §1.1. Who owns a content ID: the base game or one mod.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdOrigin<'a> {
    /// Base content: the second segment never starts with `x_`.
    Base,
    /// Content of the mod `mod_id`: the second segment is `x_<mod_id>`.
    Mod(&'a str),
}

/// SPEC: canon §1.1. Why a valid content ID breaks the mod namespace rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModSegmentError {
    /// Base content whose second segment starts with `x_`.
    BaseUsesModSegment,
    /// Mod content whose second segment is not `x_<mod_id>` of its mod.
    NotOwnModSegment,
}

impl fmt::Display for ModSegmentError {
    /// SPEC: canon §1.1 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BaseUsesModSegment => {
                f.write_str("base content id has a second segment starting with `x_`")
            }
            Self::NotOwnModSegment => {
                f.write_str("mod content id's second segment is not `x_<mod_id>` of its mod")
            }
        }
    }
}

impl core::error::Error for ModSegmentError {}

/// SPEC: canon §1.1. The second segment of a valid content ID (the segment
/// after the namespace); `None` when `id` is not a valid content ID.
#[must_use]
pub fn second_segment(id: &str) -> Option<&str> {
    if is_content_id(id) {
        id.split('.').nth(1)
    } else {
        None
    }
}

/// SPEC: canon §1.1. The `<mod_id>` of a mod content ID whose second segment
/// is `x_<mod_id>`; `None` for base-shaped IDs and for non-IDs.
/// GAP: `x_` alone (an empty `<mod_id>`) counts as a mod segment with an
/// empty mod ID, which matches no mod; doc 02 §13.1 owns the `mod_id` grammar.
#[must_use]
pub fn mod_id_of(id: &str) -> Option<&str> {
    second_segment(id)?.strip_prefix(MOD_SEGMENT_PREFIX)
}

/// SPEC: canon §1.1. Check the mod namespace rule for a valid content ID:
/// base content never uses an `x_` second segment, and a mod's content uses
/// exactly `x_<mod_id>`. Grammar errors are [`validate_content_id`]'s job;
/// a string that is not a content ID passes this check unchanged.
pub fn validate_id_origin(id: &str, origin: IdOrigin<'_>) -> Result<(), ModSegmentError> {
    if !is_content_id(id) {
        return Ok(());
    }
    match (origin, mod_id_of(id)) {
        (IdOrigin::Base, Some(_)) => Err(ModSegmentError::BaseUsesModSegment),
        (IdOrigin::Base, None) => Ok(()),
        (IdOrigin::Mod(m), Some(found)) if found == m && !m.is_empty() => Ok(()),
        (IdOrigin::Mod(_), _) => Err(ModSegmentError::NotOwnModSegment),
    }
}

/// SPEC: canon §1.1 compact content identifier `id32 = fnv1a32(utf8(id))`,
/// used in saves and packets. Doc 02 §3.4 gives it a public `u32` field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id32(pub u32);

impl Id32 {
    /// SPEC: canon §1.1 `id32` of a validated content ID.
    #[must_use]
    pub fn of(id: &ContentId) -> Self {
        Self(fnv1a32(id.as_str().as_bytes()))
    }

    /// SPEC: canon §1.1 validate `id` and return its `id32`; text keys and
    /// other non-IDs get an error, never an `id32` (canon §1.10).
    pub const fn try_of(id: &str) -> Result<Self, IdError> {
        match validate_content_id(id) {
            Ok(()) => Ok(Self(fnv1a32(id.as_bytes()))),
            Err(e) => Err(e),
        }
    }

    /// SPEC: canon §1.1 `id32` of an ID literal, for `const` items. Panics (at
    /// compile time in a `const`) when `id` is not a valid content ID.
    #[must_use]
    pub const fn of_static(id: &'static str) -> Self {
        match Self::try_of(id) {
            Ok(v) => v,
            Err(_) => panic!("invalid content id literal (canon §1.1)"),
        }
    }

    /// SPEC: canon §1.1 the raw `u32` written to saves and packets.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Id32 {
    /// SPEC: canon §1.1 hexadecimal display, e.g. `0x99b54427`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

/// SPEC: canon §1.1 a string known to satisfy the content-ID grammar. Its
/// ordering is byte order, the order doc 02 §11.2 hashes records in.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentId(String);

impl ContentId {
    /// SPEC: canon §1.1 validate and copy `id`.
    pub fn new(id: &str) -> Result<Self, IdError> {
        validate_content_id(id)?;
        Ok(Self(String::from(id)))
    }

    /// SPEC: canon §1.1 the ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// SPEC: canon §1.1 the namespace, i.e. the first segment.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.0.split('.').next().unwrap_or(&self.0)
    }

    /// SPEC: canon §1.1 the dot-separated segments, namespace first (2 to 6).
    pub fn segments(&self) -> core::str::Split<'_, char> {
        self.0.split('.')
    }

    /// SPEC: canon §1.1 `id32 = fnv1a32(utf8(id))`.
    #[must_use]
    pub fn id32(&self) -> Id32 {
        Id32::of(self)
    }
}

impl TryFrom<String> for ContentId {
    type Error = IdError;
    /// SPEC: canon §1.1 validate an owned string without copying it.
    fn try_from(id: String) -> Result<Self, IdError> {
        validate_content_id(&id)?;
        Ok(Self(id))
    }
}

impl TryFrom<&str> for ContentId {
    type Error = IdError;
    /// SPEC: canon §1.1 same as [`ContentId::new`].
    fn try_from(id: &str) -> Result<Self, IdError> {
        Self::new(id)
    }
}

impl FromStr for ContentId {
    type Err = IdError;
    /// SPEC: canon §1.1 same as [`ContentId::new`].
    fn from_str(id: &str) -> Result<Self, IdError> {
        Self::new(id)
    }
}

impl AsRef<str> for ContentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ContentId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentId {
    /// SPEC: canon §1.1 the ID text, unchanged.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::content_id_strategy;
    use alloc::format;
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use proptest::prelude::*;

    /// SPEC: canon §1.1 reference implementation written differently (split
    /// based) for differential testing of the byte scanner.
    fn reference_is_content_id(s: &str) -> bool {
        if s.is_empty() || s.len() > 64 || !s.is_ascii() {
            return false;
        }
        let segs: Vec<&str> = s.split('.').collect();
        let seg_ok = |seg: &str| {
            !seg.is_empty()
                && seg
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        };
        (2..=6).contains(&segs.len())
            && segs.iter().all(|seg| seg_ok(seg))
            && segs[0].starts_with(|c: char| c.is_ascii_lowercase())
            && segs[0] != "loc"
    }

    /// SPEC: canon §1.1 mod namespace: `item.x_frostpack.ice_blade` is mod
    /// `frostpack`'s; base content never uses an `x_` second segment.
    #[test]
    fn mod_segment_rule() {
        let modded = "item.x_frostpack.ice_blade";
        assert!(is_content_id(modded));
        assert_eq!(second_segment(modded), Some("x_frostpack"));
        assert_eq!(mod_id_of(modded), Some("frostpack"));
        assert_eq!(
            validate_id_origin(modded, IdOrigin::Mod("frostpack")),
            Ok(())
        );
        assert_eq!(
            validate_id_origin(modded, IdOrigin::Base),
            Err(ModSegmentError::BaseUsesModSegment)
        );
        assert_eq!(
            validate_id_origin(modded, IdOrigin::Mod("other")),
            Err(ModSegmentError::NotOwnModSegment)
        );
        // A mod record without its x_ segment is refused; base content passes.
        assert_eq!(
            validate_id_origin("item.ice_blade", IdOrigin::Mod("frostpack")),
            Err(ModSegmentError::NotOwnModSegment)
        );
        assert_eq!(validate_id_origin("item.ice_blade", IdOrigin::Base), Ok(()));
        assert_eq!(mod_id_of("item.ice_blade"), None);
        // `x_` only counts in the second segment; `xx_` and a later `x_` do not.
        assert_eq!(mod_id_of("item.xx_frost.a"), None);
        assert_eq!(mod_id_of("item.blade.x_frost"), None);
        assert_eq!(
            validate_id_origin("item.blade.x_frost", IdOrigin::Base),
            Ok(())
        );
        // An empty mod ID matches no mod.
        assert_eq!(
            validate_id_origin("item.x_.a", IdOrigin::Mod("")),
            Err(ModSegmentError::NotOwnModSegment)
        );
        // Non-IDs are the grammar check's business.
        assert_eq!(second_segment("Genre.X"), None);
        assert_eq!(validate_id_origin("Genre.X", IdOrigin::Base), Ok(()));
    }

    /// SPEC: canon §1.1 accepted examples from the canon and §19 namespaces.
    #[test]
    fn content_id_accepts_canon_examples() {
        for id in [
            "genre.cyberpunk",
            "arch.melee.blade_long",
            "crate.sim",
            "rng.loot",
            "st.blessed",
            "item.x_frostpack.ice_blade",
            "a.b",
            "a.0",
            "a1_.b.c.d.e.f",
            "q._",
        ] {
            assert_eq!(validate_content_id(id), Ok(()), "{id}");
        }
    }

    /// SPEC: canon §1.1 each grammar rule, with the exact error reported.
    #[test]
    fn content_id_rejects_each_rule() {
        let cases: [(&str, IdError); 14] = [
            ("", IdError::Empty),
            ("Genre.Cyberpunk", IdError::InvalidFirstByte { byte: b'G' }),
            (
                "genre.Cyberpunk",
                IdError::InvalidByte {
                    index: 6,
                    byte: b'C',
                },
            ),
            ("genre..x", IdError::EmptySegment { segment: 1 }),
            ("genre.x.", IdError::EmptySegment { segment: 2 }),
            (".genre.x", IdError::InvalidFirstByte { byte: b'.' }),
            ("1genre.x", IdError::InvalidFirstByte { byte: b'1' }),
            ("_genre.x", IdError::InvalidFirstByte { byte: b'_' }),
            ("genre", IdError::TooFewSegments),
            ("a.b.c.d.e.f.g", IdError::TooManySegments { count: 7 }),
            (
                "genre.cyber-punk",
                IdError::InvalidByte {
                    index: 11,
                    byte: b'-',
                },
            ),
            (
                "genre.caf\u{e9}",
                IdError::InvalidByte {
                    index: 9,
                    byte: 0xc3,
                },
            ),
            (
                "genre.x y",
                IdError::InvalidByte {
                    index: 7,
                    byte: b' ',
                },
            ),
            ("loc.item.sword", IdError::TextKeyNamespace),
        ];
        for (id, err) in cases {
            assert_eq!(validate_content_id(id), Err(err), "{id:?}");
            assert!(ContentId::new(id).is_err());
            assert_eq!(Id32::try_of(id), Err(err));
        }
    }

    /// SPEC: canon §1.1 64-byte limit is inclusive.
    #[test]
    fn content_id_length_limit() {
        let ok = format!("a.{}", "b".repeat(62));
        assert_eq!(ok.len(), 64);
        assert_eq!(validate_content_id(&ok), Ok(()));
        let long = format!("a.{}", "b".repeat(63));
        assert_eq!(
            validate_content_id(&long),
            Err(IdError::TooLong { len: 65 })
        );
        // Length is checked before the bytes.
        let long_bad = "A".repeat(65);
        assert_eq!(
            validate_content_id(&long_bad),
            Err(IdError::TooLong { len: 65 })
        );
    }

    /// SPEC: canon §1.1 namespace extraction.
    #[test]
    fn namespace_extraction() {
        assert_eq!(namespace_of("genre.cyberpunk"), Some("genre"));
        assert_eq!(namespace_of("arch.melee.blade_long"), Some("arch"));
        assert_eq!(namespace_of("Genre.Cyberpunk"), None);
        assert_eq!(namespace_of("genre"), None);
        let id = ContentId::new("arch.melee.blade_long").expect("valid");
        assert_eq!(id.namespace(), "arch");
        assert_eq!(
            id.segments().collect::<Vec<_>>(),
            ["arch", "melee", "blade_long"]
        );
    }

    /// SPEC: canon §1.1 and §21 test 4: `fnv1a32("genre.cyberpunk") = 0x99b54427`;
    /// doc 02 Exports: `fnv1a32("crate.sim") = 0xdc634881`.
    #[test]
    fn id32_vectors() {
        const CYBERPUNK: Id32 = Id32::of_static("genre.cyberpunk");
        assert_eq!(CYBERPUNK, Id32(0x99b5_4427));
        assert_eq!(CYBERPUNK.raw(), 0x99b5_4427);
        assert_eq!(CYBERPUNK.to_string(), "0x99b54427");
        let sim = ContentId::new("crate.sim").expect("valid");
        assert_eq!(sim.id32(), Id32(0xdc63_4881));
        assert_eq!(Id32::try_of("crate.sim"), Ok(Id32(0xdc63_4881)));
    }

    /// SPEC: canon §1.1 `Id32::of_static` refuses a bad literal.
    #[test]
    #[should_panic(expected = "invalid content id literal")]
    fn id32_of_static_panics_on_bad_literal() {
        let _ = Id32::of_static("Genre.Cyberpunk");
    }

    proptest! {
        /// SPEC: canon §1.1 the byte scanner agrees with a split-based reference.
        #[test]
        fn content_id_matches_reference(
            s in prop_oneof!["[a-z0-9_.]{0,70}", "[a-zA-Z0-9_. \u{e9}-]{0,70}"]
        ) {
            prop_assert_eq!(is_content_id(&s), reference_is_content_id(&s));
        }

        /// SPEC: canon §1.1 every string the canon regex generates is accepted
        /// when it fits in 64 bytes, and its namespace is its first segment.
        #[test]
        fn regex_generated_ids_are_valid(s in content_id_strategy()) {
            prop_assume!(s.len() <= 64);
            prop_assert_eq!(validate_content_id(&s), Ok(()));
            let id = ContentId::new(&s).expect("valid");
            prop_assert_eq!(id.namespace(), s.split('.').next().unwrap_or(""));
            prop_assert_eq!(id.id32(), Id32(fnv1a32(s.as_bytes())));
            let round: ContentId = id.to_string().parse().expect("round trip");
            prop_assert_eq!(round, id);
        }

        /// Review (canon §1.1 (canon-1.11)): `ns.x_<m>.rest` belongs to mod
        /// `m` alone and never to base content; an ID whose second segment
        /// does not start with `x_` is base-shaped and no mod's record.
        #[test]
        fn mod_segment_property(
            ns in "[a-z][a-z0-9_]{0,6}", m in "[a-z0-9][a-z0-9_]{0,8}",
            other in "[a-z0-9][a-z0-9_]{0,8}", seg in "[a-z0-9_]{1,8}",
        ) {
            let modded = format!("{ns}.x_{m}.{seg}");
            prop_assert!(is_content_id(&modded));
            prop_assert_eq!(mod_id_of(&modded), Some(m.as_str()));
            prop_assert_eq!(validate_id_origin(&modded, IdOrigin::Mod(&m)), Ok(()));
            prop_assert_eq!(
                validate_id_origin(&modded, IdOrigin::Base),
                Err(ModSegmentError::BaseUsesModSegment)
            );
            if other != m {
                prop_assert_eq!(
                    validate_id_origin(&modded, IdOrigin::Mod(&other)),
                    Err(ModSegmentError::NotOwnModSegment)
                );
            }
            let plain = format!("{ns}.{seg}");
            prop_assert_eq!(second_segment(&plain), Some(seg.as_str()));
            if seg.starts_with(MOD_SEGMENT_PREFIX) {
                prop_assert_eq!(mod_id_of(&plain), seg.strip_prefix(MOD_SEGMENT_PREFIX));
            } else {
                prop_assert_eq!(mod_id_of(&plain), None);
                prop_assert_eq!(validate_id_origin(&plain, IdOrigin::Base), Ok(()));
                prop_assert_eq!(
                    validate_id_origin(&plain, IdOrigin::Mod(&m)),
                    Err(ModSegmentError::NotOwnModSegment)
                );
            }
        }
    }
}
