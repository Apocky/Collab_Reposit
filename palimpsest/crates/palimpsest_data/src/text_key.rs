//! Localization text keys. SPEC: canon §1.10.
//!
//! Text keys are not content IDs: grammar `^loc\.[a-z0-9_.]+$`, at most 160
//! bytes, no segment limit, no `id32`. The grammar is a hand-written `const fn`
//! byte scan, like the content-ID grammar in [`crate::ids`].

use crate::ids::ContentId;
use alloc::format;
use alloc::string::String;
use core::fmt;
use core::str::FromStr;

/// SPEC: canon §1.10 maximum length of a text key in bytes.
pub const TEXT_KEY_MAX_BYTES: usize = 160;
/// SPEC: canon §1.10 prefix every text key starts with.
pub const TEXT_KEY_PREFIX: &str = "loc.";
/// SPEC: canon §1.10 namespace whose IDs supply the `<genre_suffix>` of skin keys.
pub const GENRE_NAMESPACE: &str = "genre";

/// SPEC: canon §1.10. Why a string is not a text key. The first broken rule in
/// this order is reported: length, prefix, empty body, bytes left to right.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextKeyError {
    /// Longer than [`TEXT_KEY_MAX_BYTES`].
    TooLong {
        /// Length in bytes.
        len: usize,
    },
    /// Does not start with `loc.`.
    MissingPrefix,
    /// Nothing after `loc.`.
    EmptyBody,
    /// A byte outside `a..=z`, `0..=9`, `_` and `.` after the prefix.
    InvalidByte {
        /// Byte offset.
        index: usize,
        /// The offending byte.
        byte: u8,
    },
}

impl fmt::Display for TextKeyError {
    /// SPEC: canon §1.10 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TooLong { len } => {
                write!(f, "text key is {len} bytes, more than {TEXT_KEY_MAX_BYTES}")
            }
            Self::MissingPrefix => f.write_str("text key does not start with `loc.`"),
            Self::EmptyBody => f.write_str("text key has nothing after `loc.`"),
            Self::InvalidByte { index, byte } => write!(
                f,
                "text key has byte 0x{byte:02x} at offset {index}, outside [a-z0-9_.]"
            ),
        }
    }
}

impl core::error::Error for TextKeyError {}

/// SPEC: canon §1.10 grammar `^loc\.[a-z0-9_.]+$`, at most 160 bytes, no
/// segment limit. Applied literally: empty segments such as `loc.a..b` pass.
pub const fn validate_text_key(key: &str) -> Result<(), TextKeyError> {
    let b = key.as_bytes();
    if b.len() > TEXT_KEY_MAX_BYTES {
        return Err(TextKeyError::TooLong { len: b.len() });
    }
    if b.len() < 4 || b[0] != b'l' || b[1] != b'o' || b[2] != b'c' || b[3] != b'.' {
        return Err(TextKeyError::MissingPrefix);
    }
    if b.len() == 4 {
        return Err(TextKeyError::EmptyBody);
    }
    let mut i = 4;
    while i < b.len() {
        let c = b[i];
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'.') {
            return Err(TextKeyError::InvalidByte { index: i, byte: c });
        }
        i += 1;
    }
    Ok(())
}

/// SPEC: canon §1.10 `true` when `key` is a valid text key.
#[must_use]
pub const fn is_text_key(key: &str) -> bool {
    validate_text_key(key).is_ok()
}

/// SPEC: canon §1.10 a validated localization key. It deliberately has no
/// `id32`: text keys never appear in saves or packets.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextKey(String);

impl TextKey {
    /// SPEC: canon §1.10 validate and copy `key`.
    pub fn new(key: &str) -> Result<Self, TextKeyError> {
        validate_text_key(key)?;
        Ok(Self(String::from(key)))
    }

    /// SPEC: canon §1.10 `loc.<id>.name`.
    #[must_use]
    pub fn name(id: &ContentId) -> Self {
        Self::derived(format!("{TEXT_KEY_PREFIX}{id}.name"))
    }

    /// SPEC: canon §1.10 `loc.<id>.desc`.
    #[must_use]
    pub fn desc(id: &ContentId) -> Self {
        Self::derived(format!("{TEXT_KEY_PREFIX}{id}.desc"))
    }

    /// SPEC: canon §1.10 `loc.<id>.flavor`.
    #[must_use]
    pub fn flavor(id: &ContentId) -> Self {
        Self::derived(format!("{TEXT_KEY_PREFIX}{id}.flavor"))
    }

    /// SPEC: canon §1.10 genre-skin name `loc.<id>.skin.<genre_suffix>.name`,
    /// where `genre_suffix` is the part of `genre` after `genre.`. Returns
    /// `None` when `genre` is not in the `genre` namespace.
    #[must_use]
    pub fn skin_name(id: &ContentId, genre: &ContentId) -> Option<Self> {
        let suffix = genre
            .as_str()
            .strip_prefix(GENRE_NAMESPACE)?
            .strip_prefix('.')?;
        Some(Self::derived(format!(
            "{TEXT_KEY_PREFIX}{id}.skin.{suffix}.name"
        )))
    }

    /// SPEC: canon §1.10 the key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Wraps a key built from valid content IDs. Content IDs only contain
    /// `[a-z0-9_.]` and are at most 64 bytes, so the longest derived key
    /// (`loc.` + 64 + `.skin.` + 58 + `.name` = 137 bytes) is always valid.
    fn derived(key: String) -> Self {
        debug_assert!(is_text_key(&key), "invariant: derived text key is valid");
        Self(key)
    }
}

impl FromStr for TextKey {
    type Err = TextKeyError;
    /// SPEC: canon §1.10 same as [`TextKey::new`].
    fn from_str(key: &str) -> Result<Self, TextKeyError> {
        Self::new(key)
    }
}

impl AsRef<str> for TextKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TextKey {
    /// SPEC: canon §1.10 the key text, unchanged.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Id32;
    use crate::test_support::content_id_strategy;
    use alloc::string::ToString;
    use proptest::prelude::*;

    /// SPEC: canon §1.10 reference implementation for differential testing.
    fn reference_is_text_key(s: &str) -> bool {
        s.len() <= 160
            && s.strip_prefix("loc.").is_some_and(|rest| {
                !rest.is_empty()
                    && rest.chars().all(|c| {
                        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.'
                    })
            })
    }

    /// SPEC: canon §1.10 text-key grammar, applied literally.
    #[test]
    fn text_key_grammar() {
        for key in [
            "loc.arch.melee.blade_long.skin.wuxia.name",
            "loc.arch.melee.blade_long.skin.post_apocalypse.flavor",
            "loc.x",
            "loc.a..b",
            "loc..",
            "loc.0",
        ] {
            assert_eq!(validate_text_key(key), Ok(()), "{key}");
            assert_eq!(
                TextKey::new(key).map(|k| k.to_string()),
                Ok(key.to_string())
            );
        }
        let cases: [(&str, TextKeyError); 7] = [
            ("", TextKeyError::MissingPrefix),
            ("loc", TextKeyError::MissingPrefix),
            ("Loc.x", TextKeyError::MissingPrefix),
            ("genre.cyberpunk", TextKeyError::MissingPrefix),
            ("loc.", TextKeyError::EmptyBody),
            (
                "loc.Name",
                TextKeyError::InvalidByte {
                    index: 4,
                    byte: b'N',
                },
            ),
            (
                "loc.a-b",
                TextKeyError::InvalidByte {
                    index: 5,
                    byte: b'-',
                },
            ),
        ];
        for (key, err) in cases {
            assert_eq!(validate_text_key(key), Err(err), "{key:?}");
            assert_eq!(key.parse::<TextKey>(), Err(err));
        }
        let max = format!("loc.{}", "a".repeat(156));
        assert_eq!(validate_text_key(&max), Ok(()));
        let long = format!("loc.{}", "a".repeat(157));
        assert_eq!(
            validate_text_key(&long),
            Err(TextKeyError::TooLong { len: 161 })
        );
    }

    /// SPEC: canon §1.10 derived keys, including the canon's skin example.
    #[test]
    fn text_key_derivation() {
        let id = ContentId::new("arch.melee.blade_long").expect("valid");
        let wuxia = ContentId::new("genre.wuxia").expect("valid");
        assert_eq!(
            TextKey::name(&id).as_str(),
            "loc.arch.melee.blade_long.name"
        );
        assert_eq!(
            TextKey::desc(&id).as_str(),
            "loc.arch.melee.blade_long.desc"
        );
        assert_eq!(
            TextKey::flavor(&id).as_str(),
            "loc.arch.melee.blade_long.flavor"
        );
        assert_eq!(
            TextKey::skin_name(&id, &wuxia).map(|k| k.to_string()),
            Some("loc.arch.melee.blade_long.skin.wuxia.name".to_string())
        );
        let not_genre = ContentId::new("genres.wuxia").expect("valid");
        assert_eq!(TextKey::skin_name(&id, &not_genre), None);
        assert_eq!(TextKey::skin_name(&id, &id), None);
    }

    /// SPEC: canon §1.10 the longest possible derived key is 137 bytes.
    #[test]
    fn longest_derived_key_fits() {
        let id = ContentId::new(&format!("a.{}", "b".repeat(62))).expect("valid");
        let genre = ContentId::new(&format!("genre.{}", "c".repeat(58))).expect("valid");
        let key = TextKey::skin_name(&id, &genre).expect("genre");
        assert_eq!(key.as_str().len(), 137);
        assert_eq!(validate_text_key(key.as_str()), Ok(()));
    }

    proptest! {
        /// SPEC: canon §1.10 the text-key scanner agrees with a reference.
        #[test]
        fn text_key_matches_reference(
            s in prop_oneof!["loc\\.[a-z0-9_.]{0,170}", "(loc\\.)?[a-zA-Z0-9_. -]{0,170}"]
        ) {
            prop_assert_eq!(is_text_key(&s), reference_is_text_key(&s));
        }

        /// SPEC: canon §1.10 every derived key of every valid ID is a valid
        /// text key within 160 bytes, and never a content ID (no id32).
        #[test]
        fn derived_text_keys_are_valid(s in content_id_strategy(), g in "[a-z0-9_]{1,40}") {
            let id = ContentId::new(&s).expect("valid");
            let genre = ContentId::new(&format!("genre.{g}")).expect("valid genre");
            let keys = [
                TextKey::name(&id),
                TextKey::desc(&id),
                TextKey::flavor(&id),
                TextKey::skin_name(&id, &genre).expect("genre"),
            ];
            for k in keys {
                prop_assert!(validate_text_key(k.as_str()).is_ok());
                prop_assert!(Id32::try_of(k.as_str()).is_err());
            }
        }
    }
}
