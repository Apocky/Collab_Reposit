//! JSON content checks. SPEC: canon §1.1, §1.6; doc 02 §11.1.
//!
//! Walks a parsed `serde_json::Value` and reports every violation with its
//! RFC 6901 JSON Pointer path (the path form `jsonschema` reports too):
//!
//! - any number with a fraction or exponent part (canon §1.6);
//! - any integer outside `-(2^53 - 1)..=2^53 - 1` (doc 02 §11.1, canon gap G6);
//! - any field declared as a hex `u64` that is not a `"0x"`-prefixed string of
//!   1 to 16 hex digits (canon §1.6);
//! - for a content file, the `{ "schema": …, "records": [ … ] }` envelope and
//!   the content-ID grammar of every record's `"id"` (canon §1.1).
//!
//! Walk order is deterministic: depth first, object keys in ascending byte
//! order, array elements in index order.
//!
//! Number classification without floats: serde_json (without its
//! `arbitrary_precision` feature) stores every number token with a `.`, `e`
//! or `E` as an `f64`, and also the integer token `-0` and integers beyond the
//! `i64`/`u64` range. Such numbers are told apart through their text form, so
//! this module never names a float type. `-0` therefore reports as
//! [`ViolationKind::NonInteger`] (its value would be `-0.0`).

use crate::hex::{HexU64Error, parse_hex_u64};
use crate::ids::{ContentId, IdError};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Write as _};
use serde_json::{Number, Value};

/// SPEC: canon §1.6; doc 02 §11.1 (canon gap G6): largest integer magnitude allowed in
/// content, `2^53 - 1`; larger values must be hex strings (canon §1.6).
pub const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// SPEC: canon §1.6 and doc 02 §11.1 the kind of a content violation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViolationKind {
    /// A number with a fraction or exponent part (canon §1.6), or `-0`.
    NonInteger,
    /// An integer outside `-(2^53 - 1)..=2^53 - 1` (doc 02 §11.1).
    IntegerOutOfRange,
    /// A field declared as a hex `u64` holds a non-string value.
    HexU64NotString,
    /// A field declared as a hex `u64` holds a malformed string.
    HexU64Malformed(HexU64Error),
    /// The file is not `{ "schema": string, "records": [object, …] }` or a
    /// record has no string `"id"`.
    Envelope(EnvelopeError),
    /// A record's `"id"` breaks the content-ID grammar (canon §1.1).
    InvalidId(IdError),
}

/// SPEC: canon §1.1; doc 02 §11.1: how a content file's envelope is malformed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnvelopeError {
    /// The top-level value is not an object.
    NotAnObject,
    /// `"schema"` is missing or not a string.
    SchemaNotString,
    /// `"records"` is missing or not an array.
    RecordsNotArray,
    /// A record is not an object.
    RecordNotObject,
    /// A record's `"id"` is missing or not a string.
    IdNotString,
}

impl fmt::Display for EnvelopeError {
    /// SPEC: canon §1.1; doc 02 §11.1 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotAnObject => "content file is not a JSON object",
            Self::SchemaNotString => "content file has no string \"schema\"",
            Self::RecordsNotArray => "content file has no array \"records\"",
            Self::RecordNotObject => "record is not a JSON object",
            Self::IdNotString => "record has no string \"id\"",
        })
    }
}

/// SPEC: canon §1.6 one violation: where (RFC 6901 JSON Pointer, `""` for the
/// root) and what.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Violation {
    /// JSON Pointer of the offending value, e.g. `/records/0/seed`.
    pub path: String,
    /// What is wrong with it.
    pub kind: ViolationKind,
}

impl fmt::Display for ViolationKind {
    /// SPEC: canon §1.6 human-readable reason.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonInteger => f.write_str("number has a fraction or exponent part (canon §1.6)"),
            Self::IntegerOutOfRange => {
                f.write_str("integer outside ±(2^53 - 1); write it as a hex string")
            }
            Self::HexU64NotString => f.write_str("u64 field must be a \"0x\"-prefixed hex string"),
            Self::HexU64Malformed(e) => write!(f, "{e}"),
            Self::Envelope(e) => write!(f, "{e}"),
            Self::InvalidId(e) => write!(f, "{e}"),
        }
    }
}

impl fmt::Display for Violation {
    /// SPEC: canon §1.6 `"<path>": <reason>`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\": {}", self.path, self.kind)
    }
}

/// SPEC: canon §1.6 why JSON text was refused.
#[derive(Debug)]
pub enum CheckError {
    /// Not well-formed JSON.
    Parse(serde_json::Error),
    /// Well-formed, but with content violations (in walk order).
    Violations(Vec<Violation>),
}

impl fmt::Display for CheckError {
    /// SPEC: canon §1.6 first-line summary.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "invalid JSON: {e}"),
            Self::Violations(v) => write!(f, "{} content violation(s)", v.len()),
        }
    }
}

impl core::error::Error for CheckError {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PatternSeg {
    Any,
    Literal(String),
}

/// SPEC: canon §1.6 error in a hex-field path pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternError {
    /// A non-empty pattern must start with `/`.
    MissingLeadingSlash,
    /// `~` not followed by `0` or `1` (RFC 6901 escapes).
    BadEscape,
}

/// SPEC: canon §1.6 the set of paths whose values are `u64` and must be hex
/// strings. Patterns are JSON Pointers in which a segment `*` matches any one
/// key or array index, e.g. `/records/*/seed`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HexU64Fields {
    patterns: Vec<Vec<PatternSeg>>,
}

impl HexU64Fields {
    /// SPEC: canon §1.6 no hex fields.
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// SPEC: canon §1.6 parse JSON-Pointer patterns (`*` = any one segment).
    pub fn new<'a, I>(patterns: I) -> Result<Self, PatternError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut out = Vec::new();
        for p in patterns {
            out.push(parse_pattern(p)?);
        }
        Ok(Self { patterns: out })
    }

    fn matches(&self, path: &[Seg<'_>]) -> bool {
        self.patterns.iter().any(|pat| {
            pat.len() == path.len() && pat.iter().zip(path).all(|(p, s)| seg_matches(p, s))
        })
    }
}

fn parse_pattern(p: &str) -> Result<Vec<PatternSeg>, PatternError> {
    if p.is_empty() {
        return Ok(Vec::new());
    }
    let rest = p
        .strip_prefix('/')
        .ok_or(PatternError::MissingLeadingSlash)?;
    rest.split('/')
        .map(|raw| {
            if raw == "*" {
                return Ok(PatternSeg::Any);
            }
            let mut lit = String::new();
            let mut chars = raw.chars();
            while let Some(c) = chars.next() {
                if c != '~' {
                    lit.push(c);
                    continue;
                }
                match chars.next() {
                    Some('0') => lit.push('~'),
                    Some('1') => lit.push('/'),
                    _ => return Err(PatternError::BadEscape),
                }
            }
            Ok(PatternSeg::Literal(lit))
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Seg<'a> {
    Key(&'a str),
    Index(usize),
}

fn seg_matches(p: &PatternSeg, s: &Seg<'_>) -> bool {
    match (p, s) {
        (PatternSeg::Any, _) => true,
        (PatternSeg::Literal(l), Seg::Key(k)) => l == k,
        (PatternSeg::Literal(l), Seg::Index(i)) => {
            // Canonical decimal only: "0", or no leading zero.
            (l == "0" || !l.starts_with('0')) && l.parse::<usize>().ok() == Some(*i)
        }
    }
}

fn pointer(path: &[Seg<'_>]) -> String {
    let mut s = String::new();
    for seg in path {
        s.push('/');
        match seg {
            Seg::Key(k) => {
                for c in k.chars() {
                    match c {
                        '~' => s.push_str("~0"),
                        '/' => s.push_str("~1"),
                        _ => s.push(c),
                    }
                }
            }
            Seg::Index(i) => {
                let _ = write!(s, "{i}");
            }
        }
    }
    s
}

/// SPEC: canon §1.6 and doc 02 §11.1 classify one JSON number: `None` when it
/// is an integer within `±(2^53 - 1)`.
#[must_use]
pub fn number_violation(n: &Number) -> Option<ViolationKind> {
    if let Some(i) = n.as_i64() {
        return (i.unsigned_abs() > MAX_SAFE_INTEGER).then_some(ViolationKind::IntegerOutOfRange);
    }
    if n.as_u64().is_some() {
        return Some(ViolationKind::IntegerOutOfRange);
    }
    // Neither i64 nor u64: a fraction or exponent token, `-0`, or (only with
    // serde_json's `arbitrary_precision`) an integer beyond 64 bits.
    let text = n.to_string();
    if text.bytes().any(|c| matches!(c, b'.' | b'e' | b'E')) {
        Some(ViolationKind::NonInteger)
    } else {
        Some(ViolationKind::IntegerOutOfRange)
    }
}

fn check_node(v: &Value, path: &[Seg<'_>], hex: &HexU64Fields, out: &mut Vec<Violation>) {
    let mut push = |kind| {
        out.push(Violation {
            path: pointer(path),
            kind,
        });
    };
    if let Value::Number(n) = v
        && let Some(kind) = number_violation(n)
    {
        push(kind);
    }
    if hex.matches(path) {
        match v {
            Value::String(s) => {
                if let Err(e) = parse_hex_u64(s) {
                    push(ViolationKind::HexU64Malformed(e));
                }
            }
            _ => push(ViolationKind::HexU64NotString),
        }
    }
}

/// SPEC: canon §1.6 and doc 02 §11.1 step 1: every number and every declared
/// hex `u64` field in `value`, in deterministic walk order. Empty means clean.
#[must_use]
pub fn check_value(value: &Value, hex: &HexU64Fields) -> Vec<Violation> {
    let mut out = Vec::new();
    let mut path: Vec<Seg<'_>> = Vec::new();
    // (parent path length, segment to append, value): an explicit stack, so
    // depth is bounded by memory rather than the call stack.
    let mut stack: Vec<(usize, Option<Seg<'_>>, &Value)> = Vec::new();
    stack.push((0, None, value));
    while let Some((depth, seg, v)) = stack.pop() {
        path.truncate(depth);
        if let Some(s) = seg {
            path.push(s);
        }
        check_node(v, &path, hex, &mut out);
        match v {
            Value::Array(items) => {
                for (i, item) in items.iter().enumerate().rev() {
                    stack.push((path.len(), Some(Seg::Index(i)), item));
                }
            }
            Value::Object(map) => {
                let mut members: Vec<(&String, &Value)> = map.iter().collect();
                members.sort_by(|a, b| a.0.cmp(b.0));
                for (k, item) in members.into_iter().rev() {
                    stack.push((path.len(), Some(Seg::Key(k)), item));
                }
            }
            _ => {}
        }
    }
    out
}

/// SPEC: canon §1.6 parse JSON text and run [`check_value`] on it.
pub fn parse_and_check(text: &str, hex: &HexU64Fields) -> Result<Value, CheckError> {
    let value: Value = serde_json::from_str(text).map_err(CheckError::Parse)?;
    let violations = check_value(&value, hex);
    if violations.is_empty() {
        Ok(value)
    } else {
        Err(CheckError::Violations(violations))
    }
}

/// SPEC: canon §1.1, §1.6 and doc 02 §11.1: check a content file
/// `{ "schema": "<schema id>", "records": [ … ] }`: [`check_value`], then the
/// envelope and each record's `"id"` grammar. Returns the record IDs in record
/// order, or every violation (value violations first, then envelope and ID
/// violations in record order). `id32` collisions are the registry's job
/// ([`crate::registry::IdRegistry`]).
pub fn check_content_file(
    value: &Value,
    hex: &HexU64Fields,
) -> Result<Vec<ContentId>, Vec<Violation>> {
    let mut out = check_value(value, hex);
    let mut ids = Vec::new();
    let mut env = |path: &str, kind| {
        out.push(Violation {
            path: path.to_string(),
            kind,
        });
    };
    let Value::Object(map) = value else {
        env("", ViolationKind::Envelope(EnvelopeError::NotAnObject));
        return Err(out);
    };
    if !matches!(map.get("schema"), Some(Value::String(_))) {
        env(
            "/schema",
            ViolationKind::Envelope(EnvelopeError::SchemaNotString),
        );
    }
    let Some(Value::Array(records)) = map.get("records") else {
        env(
            "/records",
            ViolationKind::Envelope(EnvelopeError::RecordsNotArray),
        );
        return Err(out);
    };
    for (i, rec) in records.iter().enumerate() {
        let id_path = alloc::format!("/records/{i}/id");
        match rec {
            Value::Object(r) => match r.get("id") {
                Some(Value::String(s)) => match ContentId::new(s) {
                    Ok(id) => ids.push(id),
                    Err(e) => env(&id_path, ViolationKind::InvalidId(e)),
                },
                _ => env(
                    &id_path,
                    ViolationKind::Envelope(EnvelopeError::IdNotString),
                ),
            },
            _ => env(
                &alloc::format!("/records/{i}"),
                ViolationKind::Envelope(EnvelopeError::RecordNotObject),
            ),
        }
    }
    if out.is_empty() { Ok(ids) } else { Err(out) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use alloc::vec;
    use proptest::prelude::*;
    use serde_json::json;

    fn v(path: &str, kind: ViolationKind) -> Violation {
        Violation {
            path: path.to_string(),
            kind,
        }
    }

    /// Parses test input from text, so no float literal appears in test code.
    fn parse(text: &str) -> Value {
        serde_json::from_str(text).expect("test JSON")
    }

    fn check_text(text: &str, hex: &HexU64Fields) -> Vec<Violation> {
        match parse_and_check(text, hex) {
            Ok(_) => Vec::new(),
            Err(CheckError::Violations(vs)) => vs,
            Err(CheckError::Parse(e)) => panic!("unexpected parse error {e}"),
        }
    }

    /// SPEC: doc 02 acceptance test 19: rejects `1.5`, `1e3`, `-0.0` and
    /// `9007199254740992`; accepts `9007199254740991` and
    /// `"0x00000000000000FF"` in a `u64` field.
    #[test]
    fn doc02_t19_validator_vectors() {
        let none = HexU64Fields::none();
        for (tok, kind) in [
            ("1.5", ViolationKind::NonInteger),
            ("1e3", ViolationKind::NonInteger),
            ("-0.0", ViolationKind::NonInteger),
            ("9007199254740992", ViolationKind::IntegerOutOfRange),
            ("-9007199254740992", ViolationKind::IntegerOutOfRange),
            ("18446744073709551615", ViolationKind::IntegerOutOfRange),
        ] {
            let text = format!("{{\"x\":{tok}}}");
            assert_eq!(check_text(&text, &none), vec![v("/x", kind)], "{tok}");
        }
        for tok in ["9007199254740991", "-9007199254740991", "0", "-1"] {
            let text = format!("{{\"x\":{tok}}}");
            assert_eq!(check_text(&text, &none), vec![], "{tok}");
        }
        let hex = HexU64Fields::new(["/seed"]).expect("pattern");
        assert_eq!(check_text(r#"{"seed":"0x00000000000000FF"}"#, &hex), vec![]);
    }

    /// SPEC: canon §1.6 every spelling of a fraction or exponent is rejected,
    /// including integral values such as `1.0` and `2E0`.
    #[test]
    fn rejects_every_fraction_and_exponent_form() {
        let none = HexU64Fields::none();
        for tok in [
            "1.0", "0.5", "-2.25", "2E0", "2e+0", "1E-3", "5e-324", "1.5e300", "-0e0",
        ] {
            let text = format!("[{tok}]");
            assert_eq!(
                check_text(&text, &none),
                vec![v("/0", ViolationKind::NonInteger)],
                "{tok}"
            );
        }
        // `-0` is an integer token, but serde_json stores it as -0.0.
        assert_eq!(
            check_text("[-0]", &none),
            vec![v("/0", ViolationKind::NonInteger)]
        );
        // A huge integer token overflows u64 and becomes an f64 in serde_json.
        assert_eq!(check_text("[123456789012345678901234]", &none).len(), 1);
    }

    /// SPEC: canon §1.6 hex u64 fields: string required, `0x` prefix, 1 to 16
    /// hex digits; patterns with `*` match any key or index.
    #[test]
    fn hex_u64_fields() {
        let hex = HexU64Fields::new(["/records/*/seed", "/records/*/hashes/*"]).expect("pattern");
        let doc = json!({
            "records": [
                { "seed": "0x0123456789ABCDEF", "hashes": ["0xff", "0x1"] },
                { "seed": 81985529216486895_u64, "hashes": ["ff", "0x", "0x1G", 3] },
                { "seed": null, "other": 7 },
                { "seed": "0x10000000000000000" }
            ]
        });
        assert_eq!(
            check_value(&doc, &hex),
            vec![
                v(
                    "/records/1/hashes/0",
                    ViolationKind::HexU64Malformed(HexU64Error::MissingPrefix)
                ),
                v(
                    "/records/1/hashes/1",
                    ViolationKind::HexU64Malformed(HexU64Error::NoDigits)
                ),
                v(
                    "/records/1/hashes/2",
                    ViolationKind::HexU64Malformed(HexU64Error::InvalidDigit {
                        index: 3,
                        byte: b'G'
                    })
                ),
                v("/records/1/hashes/3", ViolationKind::HexU64NotString),
                v("/records/1/seed", ViolationKind::IntegerOutOfRange),
                v("/records/1/seed", ViolationKind::HexU64NotString),
                v("/records/2/seed", ViolationKind::HexU64NotString),
                v(
                    "/records/3/seed",
                    ViolationKind::HexU64Malformed(HexU64Error::TooManyDigits { count: 17 })
                ),
            ]
        );
        // The same field outside a declared path is unconstrained.
        assert_eq!(check_value(&json!({ "seed": "anything" }), &hex), vec![]);
    }

    /// SPEC: canon §1.6 paths are RFC 6901 pointers with `~0` and `~1` escapes,
    /// and patterns use the same escapes; a literal index matches canonically.
    #[test]
    fn pointer_escaping_and_patterns() {
        let doc = parse(r#"{ "a/b": { "c~d": [1, 2.5] }, "": 0.5 }"#);
        assert_eq!(
            check_value(&doc, &HexU64Fields::none()),
            vec![
                v("/", ViolationKind::NonInteger),
                v("/a~1b/c~0d/1", ViolationKind::NonInteger)
            ]
        );
        let hex = HexU64Fields::new(["/a~1b/c~0d/0"]).expect("pattern");
        assert_eq!(
            check_value(&doc, &hex),
            vec![
                v("/", ViolationKind::NonInteger),
                v("/a~1b/c~0d/0", ViolationKind::HexU64NotString),
                v("/a~1b/c~0d/1", ViolationKind::NonInteger),
            ]
        );
        assert_eq!(
            HexU64Fields::new(["seed"]),
            Err(PatternError::MissingLeadingSlash)
        );
        assert_eq!(HexU64Fields::new(["/a~2"]), Err(PatternError::BadEscape));
        assert_eq!(HexU64Fields::new(["/a~"]), Err(PatternError::BadEscape));
        let leading_zero = HexU64Fields::new(["/00"]).expect("pattern");
        assert_eq!(check_value(&json!(["x"]), &leading_zero), vec![]);
        let root = HexU64Fields::new([""]).expect("pattern");
        assert_eq!(check_value(&json!("0x1"), &root), vec![]);
        assert_eq!(
            check_value(&json!(1), &root),
            vec![v("", ViolationKind::HexU64NotString)]
        );
    }

    /// SPEC: canon §1.1 and doc 02 §11.1 content-file envelope and record IDs.
    #[test]
    fn content_file_envelope_and_ids() {
        let none = HexU64Fields::none();
        let good = json!({ "schema": "crate", "records": [
            { "id": "crate.sim", "layer": 3 }, { "id": "crate.math", "layer": 0 }
        ]});
        assert_eq!(
            check_content_file(&good, &none),
            Ok(vec![
                ContentId::new("crate.sim").expect("valid"),
                ContentId::new("crate.math").expect("valid"),
            ])
        );
        let bad = parse(
            r#"{ "records": [
                { "id": "Genre.Cyberpunk" }, { "id": 5 }, 7, { "id": "genre..x", "w": 0.5 }
            ]}"#,
        );
        assert_eq!(
            check_content_file(&bad, &none),
            Err(vec![
                v("/records/3/w", ViolationKind::NonInteger),
                v(
                    "/schema",
                    ViolationKind::Envelope(EnvelopeError::SchemaNotString)
                ),
                v(
                    "/records/0/id",
                    ViolationKind::InvalidId(IdError::InvalidFirstByte { byte: b'G' })
                ),
                v(
                    "/records/1/id",
                    ViolationKind::Envelope(EnvelopeError::IdNotString)
                ),
                v(
                    "/records/2",
                    ViolationKind::Envelope(EnvelopeError::RecordNotObject)
                ),
                v(
                    "/records/3/id",
                    ViolationKind::InvalidId(IdError::EmptySegment { segment: 1 })
                ),
            ])
        );
        assert_eq!(
            check_content_file(&json!([]), &none),
            Err(vec![v(
                "",
                ViolationKind::Envelope(EnvelopeError::NotAnObject)
            )])
        );
        assert_eq!(
            check_content_file(&json!({ "schema": "s" }), &none),
            Err(vec![v(
                "/records",
                ViolationKind::Envelope(EnvelopeError::RecordsNotArray)
            )])
        );
    }

    /// SPEC: canon §1.6 malformed JSON is a parse error, not a panic.
    #[test]
    fn parse_errors_are_reported() {
        let none = HexU64Fields::none();
        assert!(matches!(
            parse_and_check("{", &none),
            Err(CheckError::Parse(_))
        ));
        assert!(matches!(
            parse_and_check("[1e400]", &none),
            Err(CheckError::Parse(_))
        ));
        let ok = parse_and_check(r#"{"a":[1,2,{"b":-3}]}"#, &none).expect("clean");
        assert_eq!(ok, json!({ "a": [1, 2, { "b": -3 }] }));
        let err = parse_and_check("[0.1]", &none).expect_err("float");
        assert_eq!(err.to_string(), "1 content violation(s)");
    }

    /// SPEC: canon §1.6 a nesting depth far beyond any call stack is walked
    /// without recursion.
    #[test]
    fn deep_nesting_does_not_recurse() {
        let mut doc = parse("1.5");
        for _ in 0..100_000 {
            doc = Value::Array(vec![doc]);
        }
        let out = check_value(&doc, &HexU64Fields::none());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].path.len(), 2 * 100_000);
        // Drop iteratively too, so the test itself does not overflow.
        let mut cur = doc;
        while let Value::Array(mut items) = cur {
            cur = items.pop().unwrap_or(Value::Null);
        }
    }

    /// SPEC: canon §1.6 integer token strategy without any float: an optional
    /// sign and up to 19 digits.
    fn int_token() -> impl Strategy<Value = String> {
        "-?(0|[1-9][0-9]{0,18})"
    }

    /// SPEC: canon §1.6 non-integer token strategy built from text only.
    fn non_int_token() -> impl Strategy<Value = String> {
        prop_oneof![
            "-?(0|[1-9][0-9]{0,6})\\.[0-9]{1,6}",
            "-?(0|[1-9][0-9]{0,6})(\\.[0-9]{1,3})?[eE][+-]?[0-9]{1,2}",
        ]
    }

    proptest! {
        /// SPEC: canon §1.6 and doc 02 §11.1: an integer token is accepted
        /// exactly when its magnitude is at most 2^53 - 1 (the text is
        /// compared as a decimal string, so no float is involved).
        #[test]
        fn integers_accepted_iff_safe(tok in int_token()) {
            let digits = tok.trim_start_matches('-');
            let max = MAX_SAFE_INTEGER.to_string();
            let safe = digits.len() < max.len() || (digits.len() == max.len() && digits <= max.as_str());
            let negative_zero = tok == "-0";
            let out = check_text(&format!("[{tok}]"), &HexU64Fields::none());
            prop_assert_eq!(out.is_empty(), safe && !negative_zero);
        }

        /// SPEC: canon §21 test 34 as a property: any fraction or exponent
        /// token, placed at a random depth and key, is reported at its path.
        #[test]
        fn non_integers_rejected_at_their_path(
            tok in non_int_token(),
            keys in proptest::collection::vec("[a-z]{1,4}", 0..5),
        ) {
            let mut text = tok.clone();
            for k in keys.iter().rev() {
                text = format!("{{\"{k}\":{text},\"z_int\":1}}");
            }
            let expected = keys.iter().map(|k| format!("/{k}")).collect::<String>();
            let out = check_text(&text, &HexU64Fields::none());
            prop_assert_eq!(out, vec![v(&expected, ViolationKind::NonInteger)]);
        }

        /// SPEC: canon §1.6 the walk is deterministic: violations come out in
        /// ascending key order whatever order the text lists the keys in.
        #[test]
        fn walk_order_is_key_order(keys in proptest::collection::btree_set("[a-z]{1,6}", 1..8)) {
            let forward: Vec<String> = keys.iter().map(|k| format!("\"{k}\":0.5")).collect();
            let mut backward = forward.clone();
            backward.reverse();
            let a = check_text(&format!("{{{}}}", forward.join(",")), &HexU64Fields::none());
            let b = check_text(&format!("{{{}}}", backward.join(",")), &HexU64Fields::none());
            prop_assert_eq!(&a, &b);
            let paths: Vec<String> = keys.iter().map(|k| format!("/{k}")).collect();
            prop_assert_eq!(a.iter().map(|x| x.path.clone()).collect::<Vec<_>>(), paths);
        }
    }
}
