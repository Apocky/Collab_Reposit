//! SPEC: canon §1.1 FNV-1a, canon §2.4-2.5 BLAKE3 wrapper

/// SPEC: canon §1.1 fnv1a32 offset basis.
pub const FNV32_OFFSET: u32 = 0x811C_9DC5;
/// SPEC: canon §1.1 fnv1a32 prime.
pub const FNV32_PRIME: u32 = 0x0100_0193;
/// SPEC: canon §1.1 fnv1a64 offset basis.
pub const FNV64_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
/// SPEC: canon §1.1 fnv1a64 prime.
pub const FNV64_PRIME: u64 = 0x0000_0100_0000_01B3;

/// SPEC: canon §1.1 `fnv1a32`: for each byte, `h ^= b; h *= prime` (wrapping).
/// `id32 = fnv1a32(utf8(id))` for content IDs.
#[inline]
#[must_use]
pub const fn fnv1a32(bytes: &[u8]) -> u32 {
    fnv1a32_extend(FNV32_OFFSET, bytes)
}

/// SPEC: canon §1.1. Continue an fnv1a32 hash `h` over more bytes, so a
/// concatenation can be hashed without allocating:
/// `fnv1a32(a ++ b) == fnv1a32_extend(fnv1a32(a), b)`.
#[inline]
#[must_use]
pub const fn fnv1a32_extend(mut h: u32, bytes: &[u8]) -> u32 {
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u32;
        h = h.wrapping_mul(FNV32_PRIME);
        i += 1;
    }
    h
}

/// SPEC: canon §1.1 `fnv1a64`: for each byte, `h ^= b; h *= prime` (wrapping).
#[inline]
#[must_use]
pub const fn fnv1a64(bytes: &[u8]) -> u64 {
    fnv1a64_extend(FNV64_OFFSET, bytes)
}

/// SPEC: canon §1.1. Continue an fnv1a64 hash `h` over more bytes:
/// `fnv1a64(a ++ b) == fnv1a64_extend(fnv1a64(a), b)`.
#[inline]
#[must_use]
pub const fn fnv1a64_extend(mut h: u64, bytes: &[u8]) -> u64 {
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(FNV64_PRIME);
        i += 1;
    }
    h
}

/// SPEC: canon §1.1 `id32 = fnv1a32(utf8(id))`, for content IDs only (never
/// for text keys, canon §1.10). Does not validate the ID grammar.
#[inline]
#[must_use]
pub const fn id32(id: &str) -> u32 {
    fnv1a32(id.as_bytes())
}

/// SPEC: canon §2.4-2.5 and doc 02 §4.7 `blake3_256(parts)`: BLAKE3 of the
/// concatenation of `parts`, in order.
#[must_use]
pub fn blake3_256(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    for p in parts {
        h.update(p);
    }
    *h.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn canon_t04_fnv1a() {
        // Canon §21 test 4.
        assert_eq!(fnv1a64(b"rng.loot"), 0xdfec_5b83_d454_f05e);
        assert_eq!(fnv1a64(b"rng.worldgen"), 0x7f6e_8785_6ef8_4a9a);
        assert_eq!(fnv1a32(b"genre.cyberpunk"), 0x99b5_4427);
        assert_eq!(
            fnv1a64(b"PALIMPSEST/DAILY/2026-10-11"),
            0xc6f2_e12d_dca7_12ab
        );
        assert_eq!(id32("genre.cyberpunk"), 0x99b5_4427);
    }

    #[test]
    fn fnv_empty_is_offset_basis() {
        assert_eq!(fnv1a32(b""), FNV32_OFFSET);
        assert_eq!(fnv1a64(b""), FNV64_OFFSET);
        // Published FNV-1a vectors for "a".
        assert_eq!(fnv1a32(b"a"), 0xe40c_292c);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn fnv_is_const_evaluable() {
        const H: u64 = fnv1a64(b"rng.loot");
        assert_eq!(H, 0xdfec_5b83_d454_f05e);
    }

    #[test]
    fn blake3_wrapper() {
        // BLAKE3 of the empty input (published test vector).
        assert_eq!(blake3_256(&[]), *blake3::hash(b"").as_bytes());
        let e = blake3_256(&[]);
        assert_eq!(e[..4], [0xaf, 0x13, 0x49, 0xb9]);
        // Concatenation semantics: part boundaries do not matter.
        assert_eq!(blake3_256(&[b"ab", b"c"]), blake3_256(&[b"abc"]));
        assert_eq!(
            blake3_256(&[b"", b"abc", b""]),
            *blake3::hash(b"abc").as_bytes()
        );
    }

    proptest! {
        #[test]
        fn fnv_extend_is_concatenation(a in proptest::collection::vec(any::<u8>(), 0..64), b in proptest::collection::vec(any::<u8>(), 0..64)) {
            let mut ab = a.clone();
            ab.extend_from_slice(&b);
            prop_assert_eq!(fnv1a64(&ab), fnv1a64_extend(fnv1a64(&a), &b));
            prop_assert_eq!(fnv1a32(&ab), fnv1a32_extend(fnv1a32(&a), &b));
        }

        #[test]
        fn blake3_split_invariant(v in proptest::collection::vec(any::<u8>(), 0..256), cut in 0usize..256) {
            let cut = cut.min(v.len());
            let (x, y) = v.split_at(cut);
            prop_assert_eq!(blake3_256(&[x, y]), blake3_256(&[&v]));
        }
    }
}
