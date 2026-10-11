//! `u64` values as `"0x"`-prefixed hexadecimal strings. SPEC: canon §1.6.
//!
//! JSON tooling loses precision above 2^53, so seeds, hashes and other `u64`
//! values are written as strings. Accepted form: the lower-case prefix `0x`
//! followed by 1 to 16 hexadecimal digits of either case (doc 02 acceptance test 19
//! uses `"0x00000000000000FF"`). The writer emits 16 upper-case digits.

use alloc::format;
use alloc::string::String;
use core::fmt;

/// SPEC: canon §1.6 the prefix of every hex-encoded `u64`.
pub const HEX_U64_PREFIX: &str = "0x";
/// SPEC: canon §1.6 at most 16 digits, so every accepted string fits in `u64`.
pub const HEX_U64_MAX_DIGITS: usize = 16;

/// SPEC: canon §1.6 why a string is not a hex-encoded `u64`. The first broken
/// rule in this order is reported: prefix, digits left to right, digit count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HexU64Error {
    /// Does not start with the lower-case `0x`.
    MissingPrefix,
    /// Nothing after `0x`.
    NoDigits,
    /// A byte that is not `0..=9`, `a..=f` or `A..=F`.
    InvalidDigit {
        /// Byte offset in the whole string.
        index: usize,
        /// The offending byte.
        byte: u8,
    },
    /// More than [`HEX_U64_MAX_DIGITS`] digits.
    TooManyDigits {
        /// Number of digits found.
        count: usize,
    },
}

impl fmt::Display for HexU64Error {
    /// SPEC: canon §1.6 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MissingPrefix => f.write_str("u64 string does not start with `0x`"),
            Self::NoDigits => f.write_str("u64 string has no digits after `0x`"),
            Self::InvalidDigit { index, byte } => write!(
                f,
                "u64 string has byte 0x{byte:02x} at offset {index}, not a hex digit"
            ),
            Self::TooManyDigits { count } => write!(
                f,
                "u64 string has {count} digits, more than {HEX_U64_MAX_DIGITS}"
            ),
        }
    }
}

impl core::error::Error for HexU64Error {}

const fn hex_digit(c: u8) -> Option<u64> {
    match c {
        b'0'..=b'9' => Some((c - b'0') as u64),
        b'a'..=b'f' => Some((c - b'a' + 10) as u64),
        b'A'..=b'F' => Some((c - b'A' + 10) as u64),
        _ => None,
    }
}

/// SPEC: canon §1.6 decode a `"0x"`-prefixed hexadecimal `u64` string.
pub const fn parse_hex_u64(s: &str) -> Result<u64, HexU64Error> {
    let b = s.as_bytes();
    if b.len() < 2 || b[0] != b'0' || b[1] != b'x' {
        return Err(HexU64Error::MissingPrefix);
    }
    let mut i = 2;
    while i < b.len() {
        if hex_digit(b[i]).is_none() {
            return Err(HexU64Error::InvalidDigit {
                index: i,
                byte: b[i],
            });
        }
        i += 1;
    }
    let count = b.len() - 2;
    if count == 0 {
        return Err(HexU64Error::NoDigits);
    }
    if count > HEX_U64_MAX_DIGITS {
        return Err(HexU64Error::TooManyDigits { count });
    }
    let mut v: u64 = 0;
    let mut j = 2;
    while j < b.len() {
        // At most 16 digits, so the shift never drops a set bit.
        if let Some(d) = hex_digit(b[j]) {
            v = (v << 4) | d;
        }
        j += 1;
    }
    Ok(v)
}

/// SPEC: canon §1.6 encode a `u64` as `0x` plus 16 upper-case hex digits,
/// e.g. `0x0123456789ABCDEF`.
#[must_use]
pub fn format_hex_u64(v: u64) -> String {
    format!("{HEX_U64_PREFIX}{v:016X}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// SPEC: canon §1.6 and doc 02 acceptance test 19 vectors.
    #[test]
    fn parses_canon_vectors() {
        assert_eq!(parse_hex_u64("0x00000000000000FF"), Ok(255));
        assert_eq!(
            parse_hex_u64("0x0123456789ABCDEF"),
            Ok(0x0123_4567_89AB_CDEF)
        );
        assert_eq!(
            parse_hex_u64("0xdfec5b83d454f05e"),
            Ok(0xdfec_5b83_d454_f05e)
        );
        assert_eq!(parse_hex_u64("0xFFFFFFFFFFFFFFFF"), Ok(u64::MAX));
        assert_eq!(parse_hex_u64("0x0"), Ok(0));
        assert_eq!(parse_hex_u64("0xaBc"), Ok(0xabc));
    }

    /// SPEC: canon §1.6 each rejection, with the exact error.
    #[test]
    fn rejects_malformed() {
        let cases: [(&str, HexU64Error); 9] = [
            ("", HexU64Error::MissingPrefix),
            ("255", HexU64Error::MissingPrefix),
            ("0X00FF", HexU64Error::MissingPrefix),
            ("x00FF", HexU64Error::MissingPrefix),
            ("0x", HexU64Error::NoDigits),
            (
                "0x00_FF",
                HexU64Error::InvalidDigit {
                    index: 4,
                    byte: b'_',
                },
            ),
            (
                "0x 1",
                HexU64Error::InvalidDigit {
                    index: 2,
                    byte: b' ',
                },
            ),
            ("-0x1", HexU64Error::MissingPrefix),
            (
                "0x10000000000000000",
                HexU64Error::TooManyDigits { count: 17 },
            ),
        ];
        for (s, err) in cases {
            assert_eq!(parse_hex_u64(s), Err(err), "{s:?}");
        }
    }

    /// SPEC: canon §1.6 the writer's form.
    #[test]
    fn formats_sixteen_upper_digits() {
        assert_eq!(format_hex_u64(255), "0x00000000000000FF");
        assert_eq!(format_hex_u64(0x0123_4567_89AB_CDEF), "0x0123456789ABCDEF");
    }

    proptest! {
        /// SPEC: canon §1.6 round trip: parse(format(v)) == v for every u64,
        /// and lower-case digits decode to the same value.
        #[test]
        fn round_trip(v in any::<u64>()) {
            let s = format_hex_u64(v);
            prop_assert_eq!(s.len(), 18);
            prop_assert_eq!(parse_hex_u64(&s), Ok(v));
            prop_assert_eq!(parse_hex_u64(&s.to_lowercase()), Ok(v));
            prop_assert_eq!(parse_hex_u64(&format!("0x{v:x}")), Ok(v));
        }
    }
}
