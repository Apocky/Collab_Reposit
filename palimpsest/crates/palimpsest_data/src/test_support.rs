//! Shared test helpers: content-ID strategies and real fnv1a32 collision
//! pairs. SPEC: canon §1.1. Compiled only for tests.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use palimpsest_math::hash::fnv1a32;
use palimpsest_math::rng::mix64;
use proptest::prelude::*;

/// SPEC: canon §1.1 strings generated from the canon regex itself
/// (`^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$`), bounded so that they always fit in
/// 64 bytes (10 + 5 × 10 = 60), minus the reserved `loc` namespace.
pub(crate) fn content_id_strategy() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-z][a-z0-9_]{0,9}(\\.[a-z0-9_]{1,9}){1,5}")
        .expect("regex")
        .prop_filter("loc is reserved for text keys", |s| !s.starts_with("loc."))
}

/// SPEC: canon §1.1 the `n`-th candidate ID of the collision search:
/// `item.c` followed by the 16 lower-case hex digits of `mix64(n)`. Mixing the
/// counter matters: FNV-1a spreads a changed final byte poorly, so sequential
/// suffixes need far more candidates before two hashes meet.
pub(crate) fn collision_candidate(n: u64) -> String {
    format!("item.c{:016x}", mix64(n))
}

/// SPEC: canon §1.1 brute-force birthday search for the first pair of valid
/// content IDs with equal `fnv1a32`. Deterministic: it always returns the same
/// pair, found after 89,119 candidates (a few tens of milliseconds).
pub(crate) fn first_brute_force_collision() -> (String, String, u32) {
    let mut seen: BTreeMap<u32, u64> = BTreeMap::new();
    let mut n = 0u64;
    loop {
        let id = collision_candidate(n);
        let h = fnv1a32(id.as_bytes());
        if let Some(prev) = seen.insert(h, n) {
            return (collision_candidate(prev), id, h);
        }
        n += 1;
    }
}

/// SPEC: canon §1.1 four disjoint pairs of valid content IDs with equal
/// `fnv1a32`, the first four found by the search above (computed
/// independently with a Python script and re-checked by a test).
pub(crate) const COLLISION_PAIRS: [(&str, &str, u32); 4] = [
    ("item.c93fb1c3b7c754cf3", "item.cd16e77dd95ebbaed", 0x1dc7_edb8),
    ("item.c0640759fbcdbd638", "item.cdc57e8cb2128c8ca", 0x5d0c_a0c8),
    ("item.c8a651be5c267ee9e", "item.ca63bf2042bd07c1d", 0xc8ee_4dcc),
    ("item.c2cda799b408e5506", "item.c56ceaf992bc17b30", 0x82a7_fae9),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::validate_content_id;

    /// SPEC: canon §1.1 the hard-coded pairs really collide and are valid IDs.
    #[test]
    fn collision_pairs_collide() {
        for (a, b, h) in COLLISION_PAIRS {
            assert_ne!(a, b);
            assert_eq!(validate_content_id(a), Ok(()));
            assert_eq!(validate_content_id(b), Ok(()));
            assert_eq!(fnv1a32(a.as_bytes()), h, "{a}");
            assert_eq!(fnv1a32(b.as_bytes()), h, "{b}");
        }
    }

    /// SPEC: canon §1.1 the brute-force search finds the first hard-coded pair.
    #[test]
    fn brute_force_finds_first_pair() {
        let (a, b, h) = first_brute_force_collision();
        assert_eq!((a.as_str(), b.as_str(), h), COLLISION_PAIRS[0]);
    }
}
