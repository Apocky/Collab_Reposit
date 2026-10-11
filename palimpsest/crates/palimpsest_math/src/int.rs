//! SPEC: canon §1.3 integer primitives
//!
//! Every function here is total on its documented domain and panics (in every
//! Cargo profile) outside it. No function wraps silently.

/// Basis-point denominator: 10000 Bp = 100% = x1.0 (canon §1.2).
pub const BP_ONE: i32 = 10_000;

/// SPEC: canon §1.3 `floor_div`. Floor division `a.div_euclid(b)`.
///
/// Panics if `b <= 0`. Never overflows for `b > 0`.
#[inline]
#[must_use]
#[track_caller]
pub const fn floor_div(a: i64, b: i64) -> i64 {
    assert!(b > 0, "floor_div: divisor must be > 0");
    a.div_euclid(b)
}

/// SPEC: canon §1.3 `floor_div`, `i128` variant used for wide intermediates.
///
/// Panics if `b <= 0`.
#[inline]
#[must_use]
#[track_caller]
pub const fn floor_div_i128(a: i128, b: i128) -> i128 {
    assert!(b > 0, "floor_div_i128: divisor must be > 0");
    a.div_euclid(b)
}

/// SPEC: canon §1.3 `ceil_div(a, b) = -floor_div(-a, b)`, `b > 0`.
///
/// Computed without negating `a`, so it is also defined for `a = i64::MIN`
/// (where the literal formula would overflow); for every other `a` the result
/// is identical to the canon formula.
#[inline]
#[must_use]
#[track_caller]
pub const fn ceil_div(a: i64, b: i64) -> i64 {
    assert!(b > 0, "ceil_div: divisor must be > 0");
    let q = a.div_euclid(b);
    // r != 0 implies b >= 2, so q <= i64::MAX / 2 and q + 1 cannot overflow.
    if a.rem_euclid(b) != 0 { q + 1 } else { q }
}

/// SPEC: canon §1.3 `ceil_div`, `i128` variant.
#[inline]
#[must_use]
#[track_caller]
pub const fn ceil_div_i128(a: i128, b: i128) -> i128 {
    assert!(b > 0, "ceil_div_i128: divisor must be > 0");
    let q = a.div_euclid(b);
    if a.rem_euclid(b) != 0 { q + 1 } else { q }
}

/// SPEC: canon §1.3 `round_half_even(a, b)` for `b > 0`.
///
/// `q = floor_div(a, b)`, `r = a - b*q`; returns `q + 1` if `2r > b`, `q` if
/// `2r < b`, otherwise `q + (q & 1)`. The comparison `2r > b` is evaluated as
/// `r > b - r`, which is equivalent and cannot overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn round_half_even(a: i64, b: i64) -> i64 {
    assert!(b > 0, "round_half_even: divisor must be > 0");
    let q = a.div_euclid(b);
    let r = a.rem_euclid(b); // == a - b*q, in 0..b
    let rest = b - r; // in 1..=b
    // When r > 0 we have b >= 2, so |q| <= i64::MAX / 2 and q + 1 is safe.
    if r > rest {
        q + 1
    } else if r < rest {
        q
    } else {
        q + (q & 1)
    }
}

/// SPEC: canon §1.3 `round_half_even`, `i128` variant.
#[inline]
#[must_use]
#[track_caller]
pub const fn round_half_even_i128(a: i128, b: i128) -> i128 {
    assert!(b > 0, "round_half_even_i128: divisor must be > 0");
    let q = a.div_euclid(b);
    let r = a.rem_euclid(b);
    let rest = b - r;
    if r > rest {
        q + 1
    } else if r < rest {
        q
    } else {
        q + (q & 1)
    }
}

/// SPEC: canon §1.3. Checked narrowing `i128 -> i64`; panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn narrow_i64(v: i128) -> i64 {
    assert!(
        v >= i64::MIN as i128 && v <= i64::MAX as i128,
        "narrow_i64: value out of i64 range"
    );
    v as i64
}

/// SPEC: canon §1.3. Checked narrowing `i128 -> i32`; panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn narrow_i32(v: i128) -> i32 {
    assert!(
        v >= i32::MIN as i128 && v <= i32::MAX as i128,
        "narrow_i32: value out of i32 range"
    );
    v as i32
}

/// SPEC: canon §1.3 `mul_bp(x, bp) = floor_div(x as i128 * bp as i128, 10000)`,
/// narrowed back to `i64` with a checked conversion that panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn mul_bp(x: i64, bp: i32) -> i64 {
    narrow_i64(floor_div_i128(x as i128 * bp as i128, BP_ONE as i128))
}

/// SPEC: canon §1.3 `mul_bp` for an `i32` input, narrowed back to `i32`
/// (the input width) with a checked conversion that panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn mul_bp_i32(x: i32, bp: i32) -> i32 {
    narrow_i32(floor_div_i128(x as i128 * bp as i128, BP_ONE as i128))
}

/// SPEC: canon §1.3 `mul_bp` with an `i64` multiplier (power tables are `i64`
/// Bp, canon §1.2 and §13.8). Same flooring and checked narrowing.
#[inline]
#[must_use]
#[track_caller]
pub const fn mul_bp_wide(x: i64, bp: i64) -> i64 {
    narrow_i64(floor_div_i128(x as i128 * bp as i128, BP_ONE as i128))
}

/// SPEC: canon §1.3 chained Bp multipliers: applied left to right, flooring
/// after each step, in exactly the listed order (never reordered).
#[must_use]
#[track_caller]
pub fn mul_bp_chain(x: i64, bps: &[i32]) -> i64 {
    let mut acc = x;
    for &bp in bps {
        acc = mul_bp(acc, bp);
    }
    acc
}

/// SPEC: canon §1.3 "clamp happens last". Clamp `x` to `lo..=hi`.
///
/// Panics if `lo > hi` (a spec error, never a runtime condition).
#[inline]
#[must_use]
#[track_caller]
pub const fn clamp_i64(x: i64, lo: i64, hi: i64) -> i64 {
    assert!(lo <= hi, "clamp_i64: lo > hi");
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// SPEC: canon §1.3 "clamp happens last". Clamp `x` to `lo..=hi` (`i32`).
///
/// Panics if `lo > hi`.
#[inline]
#[must_use]
#[track_caller]
pub const fn clamp_i32(x: i32, lo: i32, hi: i32) -> i32 {
    assert!(lo <= hi, "clamp_i32: lo > hi");
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// SPEC: canon §1.3 "clamp happens last". Clamp `x` to `lo..=hi` (`i128`).
///
/// Panics if `lo > hi`.
#[inline]
#[must_use]
#[track_caller]
pub const fn clamp_i128(x: i128, lo: i128, hi: i128) -> i128 {
    assert!(lo <= hi, "clamp_i128: lo > hi");
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// SPEC: canon §1.2 Bp. Clamp a basis-point value to `0..=10000`
/// (a probability or share).
#[inline]
#[must_use]
pub const fn clamp_bp_unit(bp: i32) -> i32 {
    clamp_i32(bp, 0, BP_ONE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn floor_div_examples() {
        assert_eq!(floor_div(7, 2), 3);
        assert_eq!(floor_div(-7, 2), -4);
        assert_eq!(floor_div(-8, 2), -4);
        assert_eq!(floor_div(0, 5), 0);
        assert_eq!(floor_div(-1, 1000), -1);
        assert_eq!(floor_div(i64::MIN, 1), i64::MIN);
        assert_eq!(floor_div(i64::MIN, 2), i64::MIN / 2);
        assert_eq!(floor_div_i128(-7, 2), -4);
        // Chunk coordinate examples, canon §1.8: cx = floor_div(bx, 32).
        assert_eq!(floor_div(-1, 32), -1);
        assert_eq!(floor_div(31, 32), 0);
        assert_eq!(floor_div(32, 32), 1);
        assert_eq!(floor_div(-32, 32), -1);
        assert_eq!(floor_div(-33, 32), -2);
    }

    #[test]
    #[should_panic(expected = "divisor must be > 0")]
    fn floor_div_zero_panics() {
        let _ = floor_div(1, 0);
    }

    #[test]
    #[should_panic(expected = "divisor must be > 0")]
    fn floor_div_negative_panics() {
        let _ = floor_div(1, -3);
    }

    #[test]
    #[should_panic(expected = "divisor must be > 0")]
    fn ceil_div_zero_panics() {
        let _ = ceil_div(1, 0);
    }

    #[test]
    #[should_panic(expected = "divisor must be > 0")]
    fn round_half_even_zero_panics() {
        let _ = round_half_even(1, 0);
    }

    #[test]
    fn ceil_div_examples() {
        assert_eq!(ceil_div(7, 2), 4);
        assert_eq!(ceil_div(-7, 2), -3);
        assert_eq!(ceil_div(8, 2), 4);
        // Canon §1.5: current HP displays as ceil_div(x, 1000); 400 mHP shows 1 (canon §21 test 30).
        assert_eq!(ceil_div(400, 1000), 1);
        assert_eq!(ceil_div(1000, 1000), 1);
        assert_eq!(ceil_div(1001, 1000), 2);
        assert_eq!(ceil_div(0, 1000), 0);
        assert_eq!(ceil_div(i64::MIN, 1), i64::MIN);
        assert_eq!(ceil_div(i64::MIN, 3), -3_074_457_345_618_258_602);
        assert_eq!(ceil_div(i64::MAX, 2), 4_611_686_018_427_387_904);
        assert_eq!(ceil_div_i128(-7, 2), -3);
    }

    #[test]
    fn round_half_even_examples() {
        assert_eq!(round_half_even(5, 2), 2); // 2.5 -> 2
        assert_eq!(round_half_even(7, 2), 4); // 3.5 -> 4
        assert_eq!(round_half_even(-5, 2), -2); // -2.5 -> -2
        assert_eq!(round_half_even(-7, 2), -4); // -3.5 -> -4
        assert_eq!(round_half_even(-3, 2), -2); // -1.5 -> -2
        assert_eq!(round_half_even(1, 3), 0);
        assert_eq!(round_half_even(2, 3), 1);
        assert_eq!(round_half_even(-2, 3), -1);
        assert_eq!(round_half_even(10, 5), 2);
        assert_eq!(round_half_even(i64::MAX, i64::MAX), 1);
        assert_eq!(round_half_even(i64::MIN, i64::MAX), -1);
        assert_eq!(round_half_even(i64::MAX - 1, i64::MAX), 1);
        assert_eq!(round_half_even(i64::MIN, 1), i64::MIN);
        assert_eq!(round_half_even_i128(5, 2), 2);
        assert_eq!(round_half_even_i128(-7, 2), -4);
        // Bp percentage display, canon §1.5: 1234 Bp -> 12.3% (tenths: 123.4 -> 123).
        assert_eq!(round_half_even(1234, 10), 123);
        assert_eq!(round_half_even(1235, 10), 124);
        assert_eq!(round_half_even(1225, 10), 122);
    }

    #[test]
    fn mul_bp_examples() {
        assert_eq!(mul_bp(10_000, 10_900), 10_900);
        assert_eq!(mul_bp(-1, 5_000), -1); // floor(-0.5)
        assert_eq!(mul_bp(1, 5_000), 0);
        assert_eq!(mul_bp(85_000, 10_000), 85_000);
        assert_eq!(mul_bp(20_000, 4_445), 8_890); // canon §21 test 17 armor arithmetic
        assert_eq!(mul_bp(i64::MAX, 10_000), i64::MAX);
        assert_eq!(mul_bp(i64::MIN, 10_000), i64::MIN);
        assert_eq!(mul_bp(i64::MAX, -10_000), -i64::MAX);
        assert_eq!(mul_bp_i32(10_000, 13_000), 13_000);
        assert_eq!(mul_bp_i32(-3, 3_333), -1);
        assert_eq!(mul_bp_wide(79_110, 25_600), 202_521); // H(25), canon §13.8
        // Chain order matters: floor after each step.
        assert_eq!(mul_bp_chain(999, &[5_000, 5_000]), 249);
        assert_eq!(mul_bp(mul_bp(999, 5_000), 5_000), 249);
        assert_eq!(mul_bp(999, 2_500), 249);
        assert_eq!(mul_bp_chain(7, &[]), 7);
    }

    #[test]
    #[should_panic(expected = "out of i64 range")]
    fn mul_bp_overflow_panics() {
        let _ = mul_bp(i64::MAX, 10_001);
    }

    #[test]
    #[should_panic(expected = "out of i32 range")]
    fn mul_bp_i32_overflow_panics() {
        let _ = mul_bp_i32(i32::MAX, 20_000);
    }

    #[test]
    fn clamp_examples() {
        assert_eq!(clamp_i64(5, 0, 3), 3);
        assert_eq!(clamp_i64(-5, 0, 3), 0);
        assert_eq!(clamp_i64(2, 0, 3), 2);
        assert_eq!(clamp_i32(16_001, -16_000, 16_000), 16_000);
        assert_eq!(clamp_i128(-1, 0, 1), 0);
        assert_eq!(clamp_bp_unit(12_000), 10_000);
        assert_eq!(clamp_bp_unit(-1), 0);
        assert_eq!(clamp_bp_unit(4_321), 4_321);
    }

    #[test]
    #[should_panic(expected = "lo > hi")]
    fn clamp_bad_bounds_panics() {
        let _ = clamp_i64(0, 1, 0);
    }

    #[test]
    fn narrow_examples() {
        assert_eq!(narrow_i64(i64::MIN as i128), i64::MIN);
        assert_eq!(narrow_i32(i32::MAX as i128), i32::MAX);
    }

    #[test]
    #[should_panic(expected = "out of i64 range")]
    fn narrow_i64_panics() {
        let _ = narrow_i64(i64::MAX as i128 + 1);
    }

    proptest! {
        #[test]
        fn floor_div_matches_definition(a in any::<i64>(), b in 1i64..=i64::MAX) {
            let q = floor_div(a, b);
            let r = a as i128 - b as i128 * q as i128;
            prop_assert!((0..b as i128).contains(&r));
        }

        #[test]
        fn ceil_div_matches_canon_formula(a in (i64::MIN + 1)..=i64::MAX, b in 1i64..=i64::MAX) {
            prop_assert_eq!(ceil_div(a, b), -floor_div(-a, b));
        }

        #[test]
        fn ceil_div_bounds(a in any::<i64>(), b in 1i64..=i64::MAX) {
            let q = ceil_div(a, b) as i128;
            prop_assert!(q * b as i128 >= a as i128);
            prop_assert!((q - 1) * (b as i128) < a as i128);
        }

        #[test]
        fn round_half_even_is_nearest(a in any::<i64>(), b in 1i64..=i64::MAX) {
            let q = round_half_even(a, b) as i128;
            // |a - b q| <= b/2, i.e. 2|a - b q| <= b.
            let d = (a as i128 - b as i128 * q).abs();
            prop_assert!(2 * d <= b as i128);
            if 2 * d == b as i128 {
                prop_assert_eq!(q & 1, 0);
            }
            prop_assert_eq!(round_half_even_i128(a as i128, b as i128), q);
        }

        #[test]
        fn round_half_even_literal_formula(a in -1_000_000_000i64..1_000_000_000, b in 1i64..1_000_000) {
            // Canon §1.3 written literally, safe for these ranges.
            let q = floor_div(a, b);
            let r = a - b * q;
            let lit = if 2 * r > b { q + 1 } else if 2 * r < b { q } else { q + (q & 1) };
            prop_assert_eq!(round_half_even(a, b), lit);
        }

        #[test]
        fn mul_bp_is_floor(x in -(1i64 << 50)..(1i64 << 50), bp in -100_000i32..100_000) {
            let p = x as i128 * bp as i128;
            let m = mul_bp(x, bp) as i128;
            prop_assert!(m * 10_000 <= p && p < (m + 1) * 10_000);
        }

        #[test]
        fn mul_bp_monotone_in_x(x in -(1i64 << 40)..(1i64 << 40), d in 0i64..1_000_000, bp in 0i32..100_000) {
            prop_assert!(mul_bp(x, bp) <= mul_bp(x + d, bp));
        }

        #[test]
        fn clamp_in_range(x in any::<i64>(), a in any::<i64>(), b in any::<i64>()) {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let c = clamp_i64(x, lo, hi);
            prop_assert!(lo <= c && c <= hi);
            prop_assert_eq!(c, x.clamp(lo, hi));
        }
    }
}
