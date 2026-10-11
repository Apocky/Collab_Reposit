//! SPEC: canon §3.6 Crockford seed display and decode
//!
//! A run seed is a `u64`. Its display form is 13 characters of Crockford
//! Base32 in groups 4-4-5 (`028T-5CY4-TQKFF`), most-significant group first,
//! where the first character carries only the top 4 bits. Daily and weekly
//! seeds are FNV-1a 64 hashes of fixed prefixes plus a UTC date or ISO week.

use core::fmt;

use crate::hash::{fnv1a64, fnv1a64_extend};
use crate::int::floor_div;

/// SPEC: canon §3.6 Crockford Base32 alphabet (no I, L, O, U).
pub const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// SPEC: canon §3.6 daily seed prefix.
pub const DAILY_PREFIX: &str = "PALIMPSEST/DAILY/";

/// SPEC: canon §3.6 weekly seed prefix.
pub const WEEKLY_PREFIX: &str = "PALIMPSEST/WEEKLY/";

/// SPEC: canon §3.6. Number of significant characters in a seed.
pub const SEED_CHARS: usize = 13;

/// SPEC: canon §3.6. The 15-byte display form of a seed (`XXXX-XXXX-XXXXX`).
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SeedText([u8; 15]);

impl SeedText {
    /// SPEC: canon §3.6. The display text as `&str` (always ASCII).
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("SeedText is ASCII")
    }
}

impl fmt::Display for SeedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for SeedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SeedText({})", self.as_str())
    }
}

/// SPEC: canon §3.6 display: 13 Crockford Base32 characters, most-significant
/// first; the first character is the top 4 bits (`0` to `F`), each following
/// character 5 bits; grouped 4-4-5 with hyphens.
#[must_use]
pub fn seed_display(seed: u64) -> SeedText {
    let mut chars = [0u8; SEED_CHARS];
    chars[0] = CROCKFORD[usize::try_from(seed >> 60).expect("4 bits")];
    for (i, c) in chars.iter_mut().enumerate().skip(1) {
        let shift = 60 - 5 * i; // 55, 50, ..., 0
        *c = CROCKFORD[usize::try_from((seed >> shift) & 31).expect("5 bits")];
    }
    let mut out = [b'-'; 15];
    out[0..4].copy_from_slice(&chars[0..4]);
    out[5..9].copy_from_slice(&chars[4..8]);
    out[10..15].copy_from_slice(&chars[8..13]);
    SeedText(out)
}

/// SPEC: canon §3.6. Why a seed string was rejected.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SeedParseError {
    /// After stripping hyphens and spaces the input is not exactly 13 characters.
    WrongLength {
        /// Number of characters found after stripping.
        found: usize,
    },
    /// The input contains `U` (or `u`), which canon §3.6 rejects.
    ContainsU {
        /// Index among the 13 stripped characters.
        index: usize,
    },
    /// A character outside the Crockford alphabet and its aliases.
    InvalidChar {
        /// Index among the 13 stripped characters.
        index: usize,
        /// The offending character as given.
        ch: char,
    },
    /// The first character is above `F` (it may carry only 4 bits).
    FirstCharAboveF,
}

impl fmt::Display for SeedParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeedParseError::WrongLength { found } => {
                write!(f, "seed must have 13 characters, found {found}")
            }
            SeedParseError::ContainsU { index } => {
                write!(f, "seed character {index} is U, which is not allowed")
            }
            SeedParseError::InvalidChar { index, ch } => {
                write!(f, "seed character {index} ({ch:?}) is not Crockford Base32")
            }
            SeedParseError::FirstCharAboveF => f.write_str("seed first character must be 0-F"),
        }
    }
}

/// Value of one normalized (uppercased, aliased) character, or `None`.
const fn crockford_value(c: u8) -> Option<u64> {
    let mut i = 0;
    while i < 32 {
        if CROCKFORD[i] == c {
            return Some(i as u64);
        }
        i += 1;
    }
    None
}

/// SPEC: canon §3.6 decode: strip hyphens and spaces, uppercase, map `O -> 0`
/// and `I`, `L -> 1`, then require exactly 13 characters; reject a first
/// character above `F` and any `U`. Any other character outside the alphabet
/// is rejected as well.
///
/// Check order: length, then per character (U, then alphabet), then the
/// first-character range.
pub fn decode_seed(text: &str) -> Result<u64, SeedParseError> {
    let mut buf = ['\0'; SEED_CHARS];
    let mut n = 0usize;
    for ch in text.chars() {
        if ch == '-' || ch == ' ' {
            continue;
        }
        if n < SEED_CHARS {
            buf[n] = ch;
        }
        n += 1;
    }
    if n != SEED_CHARS {
        return Err(SeedParseError::WrongLength { found: n });
    }
    let mut v: u64 = 0;
    for (index, &ch) in buf.iter().enumerate() {
        let up = ch.to_ascii_uppercase();
        let norm = match up {
            'O' => '0',
            'I' | 'L' => '1',
            c => c,
        };
        if norm == 'U' {
            return Err(SeedParseError::ContainsU { index });
        }
        let val = if norm.is_ascii() {
            crockford_value(u8::try_from(norm).expect("ascii"))
        } else {
            None
        };
        let Some(val) = val else {
            return Err(SeedParseError::InvalidChar { index, ch });
        };
        if index == 0 {
            if val > 15 {
                return Err(SeedParseError::FirstCharAboveF);
            }
            v = val;
        } else {
            v = (v << 5) | val;
        }
    }
    Ok(v)
}

/// SPEC: canon §3.6. Why a daily date or weekly ISO-week string was rejected.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SeedDateError {
    /// Not of the form `YYYY-MM-DD` (daily) or `YYYY-Www` (weekly).
    BadFormat,
    /// Year outside `0001..=9999`.
    YearOutOfRange,
    /// Month outside `01..=12`.
    MonthOutOfRange,
    /// Day outside the month.
    DayOutOfRange,
    /// ISO week outside `01..=weeks_in_year` (52 or 53).
    WeekOutOfRange,
}

impl fmt::Display for SeedDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SeedDateError::BadFormat => "expected YYYY-MM-DD or YYYY-Www",
            SeedDateError::YearOutOfRange => "year must be 0001-9999",
            SeedDateError::MonthOutOfRange => "month must be 01-12",
            SeedDateError::DayOutOfRange => "day is outside the month",
            SeedDateError::WeekOutOfRange => "ISO week is outside the year",
        })
    }
}

/// SPEC: canon §3.6 (proleptic Gregorian calendar). Leap-year rule.
#[must_use]
pub const fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

/// SPEC: canon §3.6. Days in month `m` (1..=12) of year `y`; 0 for an invalid month.
#[must_use]
pub const fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Days since 1970-01-01 of a valid proleptic Gregorian date (Hinnant's
/// `days_from_civil`, integer only).
fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    let y = i64::from(y) - i64::from(m <= 2);
    let era = floor_div(y, 400);
    let yoe = y - era * 400; // 0..=399
    let m = i64::from(m);
    let mp = if m > 2 { m - 3 } else { m + 9 }; // March = 0
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1; // 0..=365
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // 0..=146096
    era * 146_097 + doe - 719_468
}

/// ISO weekday, Monday = 1 .. Sunday = 7. 1970-01-01 was a Thursday.
fn iso_weekday(y: i32, m: u32, d: u32) -> u32 {
    let w = (days_from_civil(y, m, d) + 3).rem_euclid(7) + 1;
    u32::try_from(w).expect("weekday 1..=7")
}

/// SPEC: canon §3.6 ISO week. Number of ISO weeks (52 or 53) in ISO year `y`:
/// 53 when January 1 is a Thursday, or a Wednesday in a leap year.
#[must_use]
pub fn iso_weeks_in_year(y: i32) -> u32 {
    let jan1 = iso_weekday(y, 1, 1);
    if jan1 == 4 || (jan1 == 3 && is_leap_year(y)) {
        53
    } else {
        52
    }
}

fn check_ymd(y: i32, m: u32, d: u32) -> Result<(), SeedDateError> {
    if !(1..=9999).contains(&y) {
        return Err(SeedDateError::YearOutOfRange);
    }
    if !(1..=12).contains(&m) {
        return Err(SeedDateError::MonthOutOfRange);
    }
    if d < 1 || d > days_in_month(y, m) {
        return Err(SeedDateError::DayOutOfRange);
    }
    Ok(())
}

/// SPEC: canon §3.6 ISO week. The ISO 8601 `(iso_year, week)` of a UTC date
/// with year `0001..=9999`. For these dates the ISO year also lies in
/// `0001..=9999`.
pub fn iso_week(y: i32, m: u32, d: u32) -> Result<(i32, u32), SeedDateError> {
    check_ymd(y, m, d)?;
    let ordinal = days_from_civil(y, m, d) - days_from_civil(y, 1, 1) + 1;
    let wd = i64::from(iso_weekday(y, m, d));
    let w = (ordinal - wd + 10) / 7; // ordinal - wd + 10 >= 4, so this is floor
    if w < 1 {
        let py = y - 1;
        return Ok((py, iso_weeks_in_year(py)));
    }
    let w = u32::try_from(w).expect("week fits u32");
    if w > iso_weeks_in_year(y) {
        return Ok((y + 1, 1));
    }
    Ok((y, w))
}

fn digits(b: &[u8]) -> Option<u32> {
    let mut v = 0u32;
    for &c in b {
        if !c.is_ascii_digit() {
            return None;
        }
        v = v * 10 + u32::from(c - b'0');
    }
    Some(v)
}

/// SPEC: canon §3.6. Parse a strict `YYYY-MM-DD` date (calendar-validated).
pub fn parse_date(date: &str) -> Result<(i32, u32, u32), SeedDateError> {
    let b = date.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return Err(SeedDateError::BadFormat);
    }
    let (Some(y), Some(m), Some(d)) = (digits(&b[0..4]), digits(&b[5..7]), digits(&b[8..10]))
    else {
        return Err(SeedDateError::BadFormat);
    };
    let y = i32::try_from(y).expect("4 digits fit i32");
    check_ymd(y, m, d)?;
    Ok((y, m, d))
}

/// SPEC: canon §3.6. Parse a strict ISO week `YYYY-Www` (validated against the
/// number of ISO weeks in that year).
pub fn parse_iso_week(week: &str) -> Result<(i32, u32), SeedDateError> {
    let b = week.as_bytes();
    if b.len() != 8 || b[4] != b'-' || b[5] != b'W' {
        return Err(SeedDateError::BadFormat);
    }
    let (Some(y), Some(w)) = (digits(&b[0..4]), digits(&b[6..8])) else {
        return Err(SeedDateError::BadFormat);
    };
    let y = i32::try_from(y).expect("4 digits fit i32");
    if !(1..=9999).contains(&y) {
        return Err(SeedDateError::YearOutOfRange);
    }
    if w < 1 || w > iso_weeks_in_year(y) {
        return Err(SeedDateError::WeekOutOfRange);
    }
    Ok((y, w))
}

/// Write `v` as `width` zero-padded decimal digits into `out`.
fn put_digits(out: &mut [u8], mut v: u32) {
    for c in out.iter_mut().rev() {
        *c = b'0' + u8::try_from(v % 10).expect("digit");
        v /= 10;
    }
}

/// SPEC: canon §3.6 daily seed: `fnv1a64("PALIMPSEST/DAILY/" + "YYYY-MM-DD")`,
/// date in UTC. The date string must be a valid `YYYY-MM-DD` date.
pub fn daily_seed(date: &str) -> Result<u64, SeedDateError> {
    parse_date(date)?;
    Ok(fnv1a64_extend(
        fnv1a64(DAILY_PREFIX.as_bytes()),
        date.as_bytes(),
    ))
}

/// SPEC: canon §3.6 daily seed from a UTC calendar date.
pub fn daily_seed_ymd(y: i32, m: u32, d: u32) -> Result<u64, SeedDateError> {
    check_ymd(y, m, d)?;
    let mut s = *b"0000-00-00";
    put_digits(&mut s[0..4], y.unsigned_abs());
    put_digits(&mut s[5..7], m);
    put_digits(&mut s[8..10], d);
    Ok(fnv1a64_extend(fnv1a64(DAILY_PREFIX.as_bytes()), &s))
}

/// SPEC: canon §3.6 weekly seed: `fnv1a64("PALIMPSEST/WEEKLY/" + "YYYY-Www")`,
/// using the ISO week. The string must be a valid ISO week.
pub fn weekly_seed(iso_week_text: &str) -> Result<u64, SeedDateError> {
    parse_iso_week(iso_week_text)?;
    Ok(fnv1a64_extend(
        fnv1a64(WEEKLY_PREFIX.as_bytes()),
        iso_week_text.as_bytes(),
    ))
}

/// SPEC: canon §3.6 weekly seed for the ISO week containing a UTC date.
pub fn weekly_seed_for_date(y: i32, m: u32, d: u32) -> Result<u64, SeedDateError> {
    let (iy, w) = iso_week(y, m, d)?;
    let mut s = *b"0000-W00";
    put_digits(&mut s[0..4], iy.unsigned_abs());
    put_digits(&mut s[6..8], w);
    Ok(fnv1a64_extend(fnv1a64(WEEKLY_PREFIX.as_bytes()), &s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn canon_t09_seed_display() {
        // Canon §21 test 9.
        let run = 0x0123_4567_89AB_CDEF;
        assert_eq!(seed_display(run).as_str(), "028T-5CY4-TQKFF");
        assert_eq!(decode_seed("028T-5CY4-TQKFF"), Ok(run));
        assert_eq!(decode_seed("028t 5cy4 tqkff"), Ok(run));
        assert_eq!(decode_seed("O28T5CY4TQKFF"), Ok(run));
        assert_eq!(
            decode_seed("G28T-5CY4-TQKFF"),
            Err(SeedParseError::FirstCharAboveF)
        );
    }

    #[test]
    fn display_extremes() {
        assert_eq!(seed_display(0).as_str(), "0000-0000-00000");
        assert_eq!(seed_display(u64::MAX).as_str(), "FZZZ-ZZZZ-ZZZZZ");
        assert_eq!(decode_seed("FZZZ-ZZZZ-ZZZZZ"), Ok(u64::MAX));
        assert_eq!(decode_seed("0000-0000-00001"), Ok(1));
        assert_eq!(std::format!("{}", seed_display(1)), "0000-0000-00001");
    }

    #[test]
    fn decode_aliases_and_rejections() {
        // I and L map to 1, o to 0.
        assert_eq!(decode_seed("000000000000I"), Ok(1));
        assert_eq!(decode_seed("000000000000l"), Ok(1));
        assert_eq!(decode_seed("o000-0000-0000o"), Ok(0));
        assert_eq!(
            decode_seed("0000-0000-0000U"),
            Err(SeedParseError::ContainsU { index: 12 })
        );
        assert_eq!(
            decode_seed("0000-0000-0000u"),
            Err(SeedParseError::ContainsU { index: 12 })
        );
        assert_eq!(
            decode_seed("0000-0000-0000"),
            Err(SeedParseError::WrongLength { found: 12 })
        );
        assert_eq!(
            decode_seed("0000-0000-000000"),
            Err(SeedParseError::WrongLength { found: 14 })
        );
        assert_eq!(
            decode_seed(""),
            Err(SeedParseError::WrongLength { found: 0 })
        );
        assert_eq!(
            decode_seed("0000-0000-0000!"),
            Err(SeedParseError::InvalidChar { index: 12, ch: '!' })
        );
        assert_eq!(
            decode_seed("0000-0000-0000é"),
            Err(SeedParseError::InvalidChar {
                index: 12, ch: 'é'
            })
        );
        // Tabs are not stripped (only hyphens and spaces are).
        assert_eq!(
            decode_seed("000\t0000-00000"),
            Err(SeedParseError::InvalidChar { index: 3, ch: '\t' })
        );
        assert_eq!(
            decode_seed("Z000-0000-00000"),
            Err(SeedParseError::FirstCharAboveF)
        );
        assert_eq!(
            decode_seed("  028T--5CY4 TQKFF  "),
            Ok(0x0123_4567_89AB_CDEF)
        );
    }

    #[test]
    fn daily_and_weekly_seeds() {
        // Canon §21 test 4 vector, via the daily seed rule.
        assert_eq!(daily_seed("2026-10-11"), Ok(0xc6f2_e12d_dca7_12ab));
        assert_eq!(daily_seed_ymd(2026, 10, 11), Ok(0xc6f2_e12d_dca7_12ab));
        assert_eq!(
            daily_seed("2026-10-11"),
            Ok(fnv1a64(b"PALIMPSEST/DAILY/2026-10-11"))
        );
        assert_eq!(
            weekly_seed("2026-W41"),
            Ok(fnv1a64(b"PALIMPSEST/WEEKLY/2026-W41"))
        );
        assert_eq!(weekly_seed("2026-W41"), Ok(0x2e7e_2bae_906b_8339));
        assert_eq!(
            weekly_seed_for_date(2026, 10, 11),
            Ok(0x2e7e_2bae_906b_8339)
        );
        // Date validation.
        assert_eq!(daily_seed("2026-02-29"), Err(SeedDateError::DayOutOfRange));
        assert!(daily_seed("2024-02-29").is_ok());
        assert_eq!(
            daily_seed("2026-13-01"),
            Err(SeedDateError::MonthOutOfRange)
        );
        assert_eq!(
            daily_seed("2026-00-01"),
            Err(SeedDateError::MonthOutOfRange)
        );
        assert_eq!(daily_seed("2026-1-11"), Err(SeedDateError::BadFormat));
        assert_eq!(daily_seed("2026/10/11"), Err(SeedDateError::BadFormat));
        assert_eq!(daily_seed("0000-01-01"), Err(SeedDateError::YearOutOfRange));
        assert_eq!(weekly_seed("2026-W54"), Err(SeedDateError::WeekOutOfRange));
        assert_eq!(weekly_seed("2025-W53"), Err(SeedDateError::WeekOutOfRange));
        assert!(weekly_seed("2026-W53").is_ok());
        assert_eq!(weekly_seed("2026-W00"), Err(SeedDateError::WeekOutOfRange));
        assert_eq!(weekly_seed("2026-w41"), Err(SeedDateError::BadFormat));
        assert_eq!(
            daily_seed_ymd(2026, 2, 29),
            Err(SeedDateError::DayOutOfRange)
        );
    }

    #[test]
    fn iso_week_reference_dates() {
        // Reference values cross-checked against Python's datetime.isocalendar().
        assert_eq!(iso_week(2026, 10, 11), Ok((2026, 41)));
        assert_eq!(iso_week(2026, 1, 1), Ok((2026, 1)));
        assert_eq!(iso_week(2021, 1, 3), Ok((2020, 53)));
        assert_eq!(iso_week(2020, 12, 31), Ok((2020, 53)));
        assert_eq!(iso_week(2026, 12, 31), Ok((2026, 53)));
        assert_eq!(iso_week(2027, 1, 1), Ok((2026, 53)));
        assert_eq!(iso_week(9999, 12, 31), Ok((9999, 52)));
        assert_eq!(iso_week(1, 1, 1), Ok((1, 1)));
        assert_eq!(iso_week(2004, 12, 31), Ok((2004, 53)));
        assert_eq!(iso_week(2008, 12, 29), Ok((2009, 1)));
        assert_eq!(iso_week(2015, 12, 31), Ok((2015, 53)));
        assert_eq!(iso_week(2016, 1, 3), Ok((2015, 53)));
        let long: std::vec::Vec<i32> = (2000..=2030)
            .filter(|&y| iso_weeks_in_year(y) == 53)
            .collect();
        assert_eq!(long, [2004, 2009, 2015, 2020, 2026]);
        assert_eq!(iso_weekday(1970, 1, 1), 4);
        assert_eq!(iso_weekday(2026, 10, 11), 7);
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    }

    proptest! {
        #[test]
        fn display_decode_round_trip(seed in any::<u64>()) {
            let t = seed_display(seed);
            prop_assert_eq!(t.as_str().len(), 15);
            prop_assert_eq!(decode_seed(t.as_str()), Ok(seed));
            prop_assert_eq!(decode_seed(&t.as_str().to_ascii_lowercase()), Ok(seed));
            let stripped: std::string::String = t.as_str().chars().filter(|&c| c != '-').collect();
            prop_assert_eq!(decode_seed(&stripped), Ok(seed));
        }

        #[test]
        fn decode_display_round_trip(s in "[0-9A-F][0-9A-HJKMNP-TV-Z]{12}") {
            let v = decode_seed(&s).unwrap();
            let shown: std::string::String = seed_display(v).as_str().chars().filter(|&c| c != '-').collect();
            prop_assert_eq!(shown, s);
        }

        #[test]
        fn daily_ymd_matches_string(y in 1i32..=9999, m in 1u32..=12, d in 1u32..=28) {
            let s = std::format!("{y:04}-{m:02}-{d:02}");
            prop_assert_eq!(daily_seed(&s), daily_seed_ymd(y, m, d));
        }

        #[test]
        fn iso_week_consistent(y in 2i32..=9998, m in 1u32..=12, d in 1u32..=28) {
            let (iy, w) = iso_week(y, m, d).unwrap();
            prop_assert!((iy - y).abs() <= 1);
            prop_assert!(w >= 1 && w <= iso_weeks_in_year(iy));
            let s = std::format!("{iy:04}-W{w:02}");
            prop_assert_eq!(weekly_seed(&s), weekly_seed_for_date(y, m, d));
        }
    }
}
