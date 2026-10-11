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

/// SPEC: canon §1.3 `floor_div`, `i32` variant (block and chunk coordinates,
/// canon §1.8: `cx = floor_div(bx, 32)`). Panics if `b <= 0`.
#[inline]
#[must_use]
#[track_caller]
pub const fn floor_div_i32(a: i32, b: i32) -> i32 {
    assert!(b > 0, "floor_div_i32: divisor must be > 0");
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

/// Unsigned 256-bit magnitude used by the wide division routines.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct U256 {
    pub(crate) hi: u128,
    pub(crate) lo: u128,
}

impl U256 {
    pub(crate) const ZERO: U256 = U256 { hi: 0, lo: 0 };

    /// `self << 1`; the caller guarantees the top bit is clear.
    #[inline]
    pub(crate) const fn shl1(self) -> U256 {
        U256 {
            hi: (self.hi << 1) | (self.lo >> 127),
            lo: self.lo << 1,
        }
    }

    /// `self - o`; the caller guarantees `self >= o`.
    #[inline]
    pub(crate) const fn sub(self, o: U256) -> U256 {
        let (lo, borrow) = self.lo.overflowing_sub(o.lo);
        U256 {
            hi: self.hi - o.hi - borrow as u128,
            lo,
        }
    }

    /// Bit `i` (0 = least significant) of the value.
    #[inline]
    pub(crate) const fn bit(self, i: u32) -> u128 {
        if i >= 128 {
            (self.hi >> (i - 128)) & 1
        } else {
            (self.lo >> i) & 1
        }
    }

    /// Exact product of two `u128`.
    pub(crate) const fn mul_u128(a: u128, b: u128) -> U256 {
        const M: u128 = (1u128 << 64) - 1;
        let (a0, a1, b0, b1) = (a & M, a >> 64, b & M, b >> 64);
        let p00 = a0 * b0;
        let p01 = a0 * b1;
        let p10 = a1 * b0;
        let p11 = a1 * b1;
        let (mid, carry_mid) = p01.overflowing_add(p10);
        let (lo, carry_lo) = p00.overflowing_add(mid << 64);
        let hi = p11 + (mid >> 64) + ((carry_mid as u128) << 64) + carry_lo as u128;
        U256 { hi, lo }
    }

    /// `floor(num * 2^32 / den)` for `num < den` (and `den < 2^255`), by 32
    /// steps of restoring binary long division. The result is below `2^32`.
    pub(crate) fn frac_q32(num: U256, den: U256) -> u64 {
        debug_assert!(num < den);
        let mut rem = num;
        let mut q: u64 = 0;
        let mut i = 0;
        while i < 32 {
            rem = rem.shl1(); // rem < den < 2^255, so no bit is lost
            q <<= 1;
            if rem >= den {
                rem = rem.sub(den);
                q |= 1;
            }
            i += 1;
        }
        q
    }
}

/// SPEC: canon §1.3 (wide intermediates), doc 02 §4.6. Two's-complement
/// 256-bit integer: value = `hi * 2^128 + lo`. Used for exact products of two
/// `i128` Q64.64 values (Q128.128). Every operation is exact or panics.
///
/// Ordering (`Ord`) is numeric: the derived lexicographic order on the signed
/// `hi` word and then the unsigned `lo` word is exactly two's-complement order.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct I256 {
    /// High 128 bits, signed.
    pub hi: i128,
    /// Low 128 bits.
    pub lo: u128,
}

impl From<i128> for I256 {
    #[inline]
    fn from(v: i128) -> I256 {
        I256::from_i128(v)
    }
}

impl I256 {
    /// SPEC: canon §1.3, doc 02 §4.6. Zero.
    pub const ZERO: I256 = I256 { hi: 0, lo: 0 };

    /// SPEC: canon §1.3, doc 02 §4.6. Sign-extend an `i128`.
    #[inline]
    #[must_use]
    pub const fn from_i128(v: i128) -> I256 {
        I256 {
            hi: if v < 0 { -1 } else { 0 },
            lo: v as u128, // two's-complement bit pattern
        }
    }

    /// SPEC: canon §1.3, doc 02 §4.6. Narrow to `i128`; `None` if out of range.
    #[inline]
    #[must_use]
    pub const fn to_i128(self) -> Option<i128> {
        let top = 1u128 << 127;
        if (self.hi == 0 && self.lo < top) || (self.hi == -1 && self.lo >= top) {
            Some(self.lo as i128)
        } else {
            None
        }
    }

    /// SPEC: canon §1.3, doc 02 §4.6 `I256::mul(a, b)`: the exact product of
    /// two `i128` (schoolbook on 64-bit limbs of `|a|`, `|b|`, then the sign).
    /// Never overflows: `|a * b| <= 2^254`.
    #[must_use]
    pub const fn mul(a: i128, b: i128) -> I256 {
        let m = U256::mul_u128(a.unsigned_abs(), b.unsigned_abs());
        let v = I256 {
            hi: m.hi as i128, // < 2^126, so the bit pattern is non-negative
            lo: m.lo,
        };
        if (a < 0) != (b < 0) { v.wrapping_neg() } else { v }
    }

    #[inline]
    const fn wrapping_neg(self) -> I256 {
        let lo = (!self.lo).wrapping_add(1);
        let hi = (!self.hi).wrapping_add(if lo == 0 { 1 } else { 0 });
        I256 { hi, lo }
    }

    /// SPEC: canon §1.3, doc 02 §4.6. Checked addition; panics on overflow.
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.6 API names the inherent method
    pub const fn add(self, o: I256) -> I256 {
        let (lo, carry) = self.lo.overflowing_add(o.lo);
        let hi = self.hi.wrapping_add(o.hi).wrapping_add(carry as i128);
        let (sa, sb, sr) = (self.hi < 0, o.hi < 0, hi < 0);
        assert!(!(sa == sb && sr != sa), "I256 add overflow");
        I256 { hi, lo }
    }

    /// SPEC: canon §1.3, doc 02 §4.6. Checked subtraction; panics on overflow.
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.6 API names the inherent method
    pub const fn sub(self, o: I256) -> I256 {
        let (lo, borrow) = self.lo.overflowing_sub(o.lo);
        let hi = self.hi.wrapping_sub(o.hi).wrapping_sub(borrow as i128);
        let (sa, sb, sr) = (self.hi < 0, o.hi < 0, hi < 0);
        assert!(!(sa != sb && sr != sa), "I256 sub overflow");
        I256 { hi, lo }
    }

    /// SPEC: canon §1.3, doc 02 §4.6. Checked negation; panics for the minimum value.
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.6 API names the inherent method
    pub const fn neg(self) -> I256 {
        I256::ZERO.sub(self)
    }

    /// SPEC: canon §1.3, doc 02 §4.6 `shl32`: multiply by `2^32`; panics if
    /// any bit would be shifted out (the value must lie in `[-2^223, 2^223)`).
    #[must_use]
    #[track_caller]
    pub const fn shl32(self) -> I256 {
        assert!(
            self.hi >= -(1i128 << 95) && self.hi < (1i128 << 95),
            "I256 shl32 overflow"
        );
        I256 {
            hi: (self.hi << 32) | (self.lo >> 96) as i128,
            lo: self.lo << 32,
        }
    }

    /// SPEC: canon §1.4 (the arithmetic shift is floor), doc 02 §4.6.
    /// Arithmetic shift right by `k` in `1..=127` bits: `floor(self / 2^k)`.
    #[must_use]
    #[track_caller]
    pub const fn sar(self, k: u32) -> I256 {
        assert!(k >= 1 && k <= 127, "I256 sar: shift must be 1..=127");
        I256 {
            hi: self.hi >> k,
            lo: (self.lo >> k) | ((self.hi as u128) << (128 - k)),
        }
    }

    /// SPEC: canon §1.3, doc 02 §4.6. `true` when the value is below zero.
    #[inline]
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.hi < 0
    }

    /// Magnitude as an unsigned 256-bit value (exact, including the minimum).
    pub(crate) const fn unsigned_abs(self) -> U256 {
        let v = if self.is_negative() { self.wrapping_neg() } else { self };
        U256 {
            hi: v.hi as u128,
            lo: v.lo,
        }
    }
}

/// SPEC: canon §1.3 `floor_div` on 256-bit values, doc 02 §4.6
/// `floor_div_i256`: `floor(n / d)` for `d > 0`, by restoring binary long
/// division on magnitudes (256 iterations): `q = |n| / d` truncated; if
/// `n < 0`, `q = -q - (1 if remainder != 0 else 0)`. Panics if `d <= 0` or
/// the quotient does not fit `i128`.
#[must_use]
#[track_caller]
pub fn floor_div_i256(n: I256, d: I256) -> i128 {
    assert!(!d.is_negative() && d != I256::ZERO, "floor_div_i256: divisor must be > 0");
    let mn = n.unsigned_abs();
    let md = d.unsigned_abs();
    let mut rem = U256::ZERO;
    let mut q = U256::ZERO;
    let mut i = 256u32;
    while i > 0 {
        i -= 1;
        rem = rem.shl1(); // rem < md <= 2^255 - 1, so no bit is lost
        rem.lo |= mn.bit(i);
        if rem >= md {
            rem = rem.sub(md);
            if i >= 128 {
                q.hi |= 1u128 << (i - 128);
            } else {
                q.lo |= 1u128 << i;
            }
        }
    }
    assert!(q.hi == 0, "floor_div_i256: quotient out of i128 range");
    let top = 1u128 << 127;
    if !n.is_negative() {
        assert!(q.lo < top, "floor_div_i256: quotient out of i128 range");
        q.lo as i128
    } else {
        let bump = u128::from(rem != U256::ZERO);
        let mag = q.lo.checked_add(bump).expect("floor_div_i256: quotient out of i128 range");
        assert!(mag <= top, "floor_div_i256: quotient out of i128 range");
        if mag == top { i128::MIN } else { -(mag as i128) }
    }
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
        assert_eq!(floor_div_i32(-33, 32), -2);
        assert_eq!(floor_div_i32(i32::MIN, 32), i32::MIN / 32);
        // Local coordinate lx = bx - 32*cx is in 0..32 (canon §1.8).
        assert_eq!(-33 - 32 * floor_div_i32(-33, 32), 31);
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

    fn i256_value(v: I256) -> (i128, u128) {
        (v.hi, v.lo)
    }

    #[test]
    fn i256_basics() {
        assert_eq!(I256::from(-1i128), I256 { hi: -1, lo: u128::MAX });
        assert_eq!(I256::from(5i128).to_i128(), Some(5));
        assert_eq!(I256::from(i128::MIN).to_i128(), Some(i128::MIN));
        assert_eq!(I256::from(i128::MAX).to_i128(), Some(i128::MAX));
        assert_eq!(I256 { hi: 1, lo: 0 }.to_i128(), None);
        assert_eq!(I256 { hi: -1, lo: 0 }.to_i128(), None);
        // Doc 02 §4.6: I256::mul(3 << 64, 1 << 64) is 3 * 2^128.
        assert_eq!(i256_value(I256::mul(3 << 64, 1 << 64)), (3, 0));
        assert_eq!(i256_value(I256::mul(-(3 << 64), 1 << 64)), (-3, 0));
        // Extremes: MIN * MIN = 2^254, MIN * MAX = -(2^254 - 2^127).
        assert_eq!(i256_value(I256::mul(i128::MIN, i128::MIN)), (1 << 126, 0));
        assert_eq!(I256::mul(i128::MIN, i128::MAX), I256 { hi: -(1 << 126), lo: 1 << 127 });
        assert_eq!(I256::mul(i128::MAX, i128::MAX), I256 { hi: (1 << 126) - 1, lo: 1 });
        assert_eq!(I256::mul(-7, 6).to_i128(), Some(-42));
        assert_eq!(I256::mul(0, i128::MIN), I256::ZERO);
        assert!(I256::from(-3i128) < I256::from(2i128));
        assert!(I256 { hi: 1, lo: 0 } > I256::from(i128::MAX));
        assert!(I256 { hi: -1, lo: 0 } < I256::from(i128::MIN));
        assert!(I256::mul(-1, 1).is_negative());
        assert_eq!(I256::from(1i128).shl32().to_i128(), Some(1 << 32));
        assert_eq!(I256::from(-1i128).shl32().to_i128(), Some(-(1 << 32)));
        assert_eq!(i256_value(I256::from(1i128 << 100).shl32()), (1 << 4, 0));
        assert_eq!(I256::from(-5i128).sar(1).to_i128(), Some(-3));
        assert_eq!(I256 { hi: 1, lo: 0 }.sar(1), I256 { hi: 0, lo: 1 << 127 });
        assert_eq!(I256 { hi: -2, lo: 0 }.sar(1), I256 { hi: -1, lo: 0 });
        assert_eq!(I256 { hi: -1, lo: 0 }.sar(127), I256 { hi: -1, lo: u128::MAX - 1 });
    }

    #[test]
    #[should_panic(expected = "I256 add overflow")]
    fn i256_add_overflow_panics() {
        let max = I256 { hi: i128::MAX, lo: u128::MAX };
        let _ = max.add(I256::from(1i128));
    }

    #[test]
    #[should_panic(expected = "I256 sub overflow")]
    fn i256_neg_min_panics() {
        let min = I256 { hi: i128::MIN, lo: 0 };
        let _ = min.neg();
    }

    #[test]
    #[should_panic(expected = "I256 shl32 overflow")]
    fn i256_shl32_overflow_panics() {
        let _ = I256 { hi: 1 << 95, lo: 0 }.shl32();
    }

    #[test]
    fn floor_div_i256_examples() {
        assert_eq!(floor_div_i256(I256::from(7i128), I256::from(2i128)), 3);
        assert_eq!(floor_div_i256(I256::from(-7i128), I256::from(2i128)), -4);
        assert_eq!(floor_div_i256(I256::from(-8i128), I256::from(2i128)), -4);
        assert_eq!(floor_div_i256(I256::mul(3 << 64, 1 << 64).shl32(), I256::mul(4 << 64, 1 << 64)), 3_221_225_472);
        assert_eq!(floor_div_i256(I256::mul(i128::MIN, 2), I256::from(2i128)), i128::MIN);
        assert_eq!(floor_div_i256(I256::mul(i128::MAX, 4), I256::from(4i128)), i128::MAX);
        assert_eq!(floor_div_i256(I256::mul(i128::MIN, i128::MAX), I256::from(i128::MAX)), i128::MIN);
        assert_eq!(floor_div_i256(I256::mul(i128::MAX, i128::MAX), I256::from(i128::MAX)), i128::MAX);
        assert_eq!(floor_div_i256(I256::mul(i128::MIN, i128::MAX).add(I256::from(1i128)), I256::from(i128::MAX)), i128::MIN);
    }

    #[test]
    #[should_panic(expected = "quotient out of i128 range")]
    fn floor_div_i256_range_panics() {
        let _ = floor_div_i256(I256 { hi: 1, lo: 0 }, I256::from(1i128));
    }

    #[test]
    #[should_panic(expected = "divisor must be > 0")]
    fn floor_div_i256_zero_panics() {
        let _ = floor_div_i256(I256::from(1i128), I256::ZERO);
    }

    proptest! {
        #[test]
        fn i256_mul_matches_i128(a in -(1i128 << 62)..(1i128 << 62), b in -(1i128 << 62)..(1i128 << 62)) {
            prop_assert_eq!(I256::mul(a, b).to_i128(), Some(a * b));
        }

        #[test]
        fn i256_add_sub_round_trip(
            a in -(1i128 << 126)..(1i128 << 126), b in -(1i128 << 126)..(1i128 << 126),
            c in -(1i128 << 126)..(1i128 << 126), d in -(1i128 << 126)..(1i128 << 126),
        ) {
            let x = I256::mul(a, b);
            let y = I256::mul(c, d);
            prop_assert_eq!(x.add(y).sub(y), x);
            prop_assert_eq!(x.sub(y).add(y), x);
            prop_assert_eq!(x.add(y), y.add(x));
            prop_assert_eq!(x.neg().neg(), x);
            prop_assert_eq!(x.cmp(&y), x.sub(y).cmp(&I256::ZERO));
        }

        #[test]
        fn i256_mul_sign_and_symmetry(a in any::<i128>(), b in any::<i128>()) {
            prop_assert_eq!(I256::mul(a, b), I256::mul(b, a));
            if a != i128::MIN {
                prop_assert_eq!(I256::mul(-a, b), I256::mul(a, b).neg());
            }
        }

        #[test]
        fn i256_mul_distributes(a in any::<i64>(), b in any::<i128>(), c in any::<i64>()) {
            // a*b + c*b == (a + c)*b, all exact.
            let (a, c) = (i128::from(a), i128::from(c));
            prop_assert_eq!(I256::mul(a, b).add(I256::mul(c, b)), I256::mul(a + c, b));
        }

        #[test]
        fn i256_sar_is_floor(v in any::<i128>(), k in 1u32..=127) {
            prop_assert_eq!(I256::from(v).sar(k).to_i128(), Some(v >> k));
        }

        #[test]
        fn floor_div_i256_matches_i128(n in any::<i128>(), d in 1i128..=i128::MAX) {
            prop_assert_eq!(floor_div_i256(I256::from(n), I256::from(d)), floor_div_i128(n, d));
        }

        #[test]
        fn floor_div_i256_wide(a in any::<i128>(), b in 1i128..=i128::MAX, r in any::<u64>()) {
            // (a*b + r') / b == a + floor(r'/b) for 0 <= r' < b.
            let r = i128::from(r) % b;
            let n = I256::mul(a, b).add(I256::from(r));
            prop_assert_eq!(floor_div_i256(n, I256::from(b)), a);
        }

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
