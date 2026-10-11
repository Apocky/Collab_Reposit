//! SPEC: canon §1.4 fixed point, doc 02 §4.1-4.2
//!
//! `Fx` is signed Q32.32 in an `i64`: raw `r` represents `r / 2^32`. Every
//! operation is checked and panics on overflow in every profile (canon §1.3,
//! doc 02 §4.1 overflow policy). `Mul` and `Div` are deliberately not
//! implemented for `Fx x Fx`; products name their flooring through [`mul_fx`]
//! and [`div_fx`].

use core::fmt;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::int::{floor_div, floor_div_i128, mul_bp, narrow_i64, round_half_even_i128};

/// Number of fractional bits of `Fx`.
pub const FX_FRAC_BITS: u32 = 32;

/// SPEC: canon §1.4. Signed Q32.32 fixed point; raw `r` represents `r / 2^32`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Fx(i64);

impl Fx {
    /// SPEC: canon §1.4, doc 02 §4.1. Zero.
    pub const ZERO: Fx = Fx(0);
    /// SPEC: canon §1.4, doc 02 §4.1. One (raw `2^32`).
    pub const ONE: Fx = Fx(1 << 32);
    /// SPEC: canon §2.1. One half (raw `2^31`).
    pub const HALF: Fx = Fx(1 << 31);
    /// SPEC: canon §2.1 `DT` raw `71582788 = floor(2^32 / 60)`, one tick.
    pub const DT: Fx = Fx(71_582_788);
    /// SPEC: canon §1.8, doc 02 §4.6 `Fx::MAX = Fx(i64::MAX)`.
    pub const MAX: Fx = Fx(i64::MAX);
    /// SPEC: canon §1.4. The most negative representable value.
    pub const MIN: Fx = Fx(i64::MIN);

    /// SPEC: canon §1.4. The raw `i64` (value times `2^32`).
    #[inline]
    #[must_use]
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// SPEC: canon §1.4. Build from a raw `i64` (value times `2^32`).
    #[inline]
    #[must_use]
    pub const fn from_raw(r: i64) -> Fx {
        Fx(r)
    }

    /// SPEC: canon §1.4 `Fx::from_int(n) = Fx((n as i64) << 32)`. Exact for
    /// every `i32`, which is the whole integer range of Q32.32.
    #[inline]
    #[must_use]
    pub const fn from_int(n: i32) -> Fx {
        Fx((n as i64) << 32)
    }

    /// SPEC: canon §1.4 `fx_from_milli`. Same as [`fx_from_milli`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn from_milli(m: i64) -> Fx {
        fx_from_milli(m)
    }

    /// SPEC: canon §1.4, doc 02 §4.1 `mul_int`: checked `raw * n`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn mul_int(self, n: i64) -> Fx {
        match self.0.checked_mul(n) {
            Some(r) => Fx(r),
            None => panic!("Fx::mul_int overflow"),
        }
    }

    /// SPEC: canon §1.4, doc 02 §4.1 `div_int`: `floor_div(raw, n)`, `n > 0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn div_int(self, n: i64) -> Fx {
        Fx(floor_div(self.0, n))
    }

    /// SPEC: canon §1.4 `fx_floor_block`. Same as [`fx_floor_block`].
    #[inline]
    #[must_use]
    pub const fn floor_block(self) -> i32 {
        fx_floor_block(self)
    }

    /// SPEC: canon §1.4, doc 02 §4.1 `to_milli_floor = floor_div(raw as i128 * 1000, 2^32)`.
    /// Always fits in `i64`.
    #[inline]
    #[must_use]
    pub const fn to_milli_floor(self) -> i64 {
        narrow_i64(floor_div_i128(self.0 as i128 * 1000, 1i128 << 32))
    }

    /// SPEC: canon §1.5 display (non-normative helper): milli value rounded
    /// half to even, `round_half_even(raw * 1000, 2^32)`. For UI and debug text
    /// only; simulation formulas use [`Fx::to_milli_floor`].
    #[inline]
    #[must_use]
    pub const fn to_milli_round(self) -> i64 {
        narrow_i64(round_half_even_i128(self.0 as i128 * 1000, 1i128 << 32))
    }

    /// SPEC: canon §1.4 checked add; `None` on overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add(self, o: Fx) -> Option<Fx> {
        match self.0.checked_add(o.0) {
            Some(r) => Some(Fx(r)),
            None => None,
        }
    }

    /// SPEC: canon §1.4 checked sub; `None` on overflow.
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, o: Fx) -> Option<Fx> {
        match self.0.checked_sub(o.0) {
            Some(r) => Some(Fx(r)),
            None => None,
        }
    }

    /// SPEC: canon §1.4 checked negation; `None` for `Fx::MIN`.
    #[inline]
    #[must_use]
    pub const fn checked_neg(self) -> Option<Fx> {
        match self.0.checked_neg() {
            Some(r) => Some(Fx(r)),
            None => None,
        }
    }

    /// SPEC: canon §1.4. Absolute value; panics for `Fx::MIN`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn abs(self) -> Fx {
        match self.0.checked_abs() {
            Some(r) => Fx(r),
            None => panic!("Fx::abs overflow"),
        }
    }

    /// SPEC: canon §1.4. Sign of the value: -1, 0 or 1.
    #[inline]
    #[must_use]
    pub const fn signum(self) -> i64 {
        self.0.signum()
    }
}

impl Add for Fx {
    type Output = Fx;
    /// SPEC: canon §1.4 `add`: raw `i64` checked add; panics on overflow.
    #[inline]
    #[track_caller]
    fn add(self, o: Fx) -> Fx {
        match self.checked_add(o) {
            Some(v) => v,
            None => panic!("Fx add overflow"),
        }
    }
}

impl Sub for Fx {
    type Output = Fx;
    /// SPEC: canon §1.4 `sub`: raw `i64` checked sub; panics on overflow.
    #[inline]
    #[track_caller]
    fn sub(self, o: Fx) -> Fx {
        match self.checked_sub(o) {
            Some(v) => v,
            None => panic!("Fx sub overflow"),
        }
    }
}

impl Neg for Fx {
    type Output = Fx;
    /// SPEC: canon §1.4 checked negation; panics for `Fx::MIN`.
    #[inline]
    #[track_caller]
    fn neg(self) -> Fx {
        match self.checked_neg() {
            Some(v) => v,
            None => panic!("Fx neg overflow"),
        }
    }
}

impl AddAssign for Fx {
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, o: Fx) {
        *self = *self + o;
    }
}

impl SubAssign for Fx {
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, o: Fx) {
        *self = *self - o;
    }
}

/// Writes `milli` as a signed decimal with three fractional digits, no floats.
fn write_milli(f: &mut fmt::Formatter<'_>, milli: i64) -> fmt::Result {
    let sign = if milli < 0 { "-" } else { "" };
    let a = milli.unsigned_abs();
    write!(f, "{sign}{}.{:03}", a / 1000, a % 1000)
}

impl fmt::Debug for Fx {
    /// Prints the raw value and the milli rendering rounded half to even,
    /// for example `Fx(raw=6442450944 ~1.500)`. Integer arithmetic only.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fx(raw={} ~", self.0)?;
        write_milli(f, self.to_milli_round())?;
        f.write_str(")")
    }
}

impl fmt::Display for Fx {
    /// Prints the value with three decimals (rounded half to even), e.g. `1.500`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_milli(f, self.to_milli_round())
    }
}

/// SPEC: canon §1.4 `fx_from_milli(m) = Fx(floor_div((m as i128) << 32, 1000) as i64)`,
/// checked. Converts mm, mm/s and mm/s² content values to meters.
///
/// Panics if the result does not fit (`|m|` above about `2.147e12`).
#[inline]
#[must_use]
#[track_caller]
pub const fn fx_from_milli(m: i64) -> Fx {
    Fx(narrow_i64(floor_div_i128((m as i128) << 32, 1000)))
}

/// SPEC: canon §1.4 `mul_fx(a, b) = Fx(((a.raw as i128 * b.raw as i128) >> 32) as i64)`.
/// The arithmetic shift is floor; the narrowing is checked and panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn mul_fx(a: Fx, b: Fx) -> Fx {
    Fx(narrow_i64((a.0 as i128 * b.0 as i128) >> 32))
}

/// SPEC: canon §1.4 `div_fx(a, b) = Fx(floor_div((a.raw as i128) << 32, b.raw as i128) as i64)`.
///
/// Panics if `b.raw == 0`. For negative `b`, both operands are negated first
/// (in `i128`, so `Fx::MIN` operands are safe) and the quotient is then
/// floored. The final narrowing is checked and panics on overflow.
#[inline]
#[must_use]
#[track_caller]
pub const fn div_fx(a: Fx, b: Fx) -> Fx {
    assert!(b.0 != 0, "div_fx: division by zero");
    let (mut n, mut d) = (a.0 as i128, b.0 as i128);
    if d < 0 {
        n = -n;
        d = -d;
    }
    Fx(narrow_i64(floor_div_i128(n << 32, d)))
}

/// SPEC: canon §1.4 `mul_bp_fx(x, bp) = Fx(mul_bp(x.raw, bp))`.
#[inline]
#[must_use]
#[track_caller]
pub const fn mul_bp_fx(x: Fx, bp: i32) -> Fx {
    Fx(mul_bp(x.0, bp))
}

/// SPEC: canon §1.4 `fx_floor_block(x) = (x.raw >> 32) as i32`, the block
/// coordinate containing `x`. Exact: `raw >> 32` always lies in the `i32` range.
#[inline]
#[must_use]
pub const fn fx_floor_block(x: Fx) -> i32 {
    // i64 >> 32 is in -2^31..=2^31-1, so this cast never truncates.
    (x.0 >> 32) as i32
}

/// SPEC: canon §1.4 `isqrt_u128`, doc 02 §4.2. Integer floor square root by
/// Newton's method from above.
///
/// Contract (exactness): for every `n`, the result `r` satisfies
/// `r*r <= n < (r+1)*(r+1)` (the latter evaluated in exact arithmetic).
/// Start value `x0 = 2^ceil(bits/2) >= sqrt(n)`; the iteration
/// `y = (x + n/x) / 2` decreases strictly until `y >= x`, at which point `x`
/// is the floor root. No intermediate overflows: `x <= 2^64` and
/// `n / x < 2^65`.
#[must_use]
pub const fn isqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let bits = 128 - n.leading_zeros(); // significant bits, 2..=128
    let mut x: u128 = 1u128 << bits.div_ceil(2); // 2^ceil(bits/2) >= sqrt(n)
    loop {
        let y = (x + n / x) >> 1;
        if y >= x {
            return x;
        }
        x = y;
    }
}

/// SPEC: canon §1.4 `sqrt_fx(x) = Fx(isqrt_u128((x.raw as u128) << 32) as i64)`,
/// floor. Panics if `x.raw < 0`. The result always fits (it is below `2^48`).
#[inline]
#[must_use]
#[track_caller]
pub const fn sqrt_fx(x: Fx) -> Fx {
    assert!(x.0 >= 0, "sqrt_fx: negative argument");
    let r = isqrt_u128((x.0 as u128) << 32);
    // r < 2^47.5 because (x.raw << 32) < 2^95.
    Fx(r as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn canon_t11_fixed_point() {
        // Canon §21 test 11.
        assert_eq!(sqrt_fx(Fx::from_int(4)), Fx::from_int(2));
        assert_eq!(fx_from_milli(4300).raw(), 18_468_359_372);
        assert_eq!(Fx::from_milli(4300).raw(), 18_468_359_372);
        assert_eq!(mul_fx(Fx::from_int(3), Fx::from_int(-2)), Fx::from_int(-6));
        let r = std::panic::catch_unwind(|| div_fx(Fx::from_int(1), Fx::from_int(0)));
        assert!(r.is_err(), "div_fx by zero must panic");
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn canon_t11_div_fx_zero_panics() {
        let _ = div_fx(Fx::from_int(1), Fx::from_int(0));
    }

    #[test]
    fn constants() {
        assert_eq!(Fx::ONE.raw(), 4_294_967_296);
        assert_eq!(Fx::DT.raw(), 71_582_788);
        assert_eq!(Fx::DT.raw(), floor_div(1 << 32, 60)); // canon §2.1
        assert_eq!(Fx::HALF.raw() * 2, Fx::ONE.raw());
        assert_eq!(Fx::from_int(1), Fx::ONE);
        assert_eq!(Fx::from_int(i32::MIN).raw(), i64::MIN);
        assert_eq!(Fx::from_int(i32::MAX).raw(), (i32::MAX as i64) << 32);
    }

    #[test]
    fn from_milli_examples() {
        // Doc 02 §4.1: standing eye height.
        assert_eq!(fx_from_milli(1620).raw(), 6_957_847_019);
        assert_eq!(fx_from_milli(1000), Fx::ONE);
        assert_eq!(fx_from_milli(-1000), Fx::from_int(-1));
        assert_eq!(fx_from_milli(500), Fx::HALF);
        // floor for negatives: -0.001 m -> floor(-4294967.296) = -4294968
        assert_eq!(fx_from_milli(-1).raw(), -4_294_968);
        assert_eq!(fx_from_milli(1).raw(), 4_294_967);
        assert_eq!(fx_from_milli(600).raw(), 2_576_980_377);
        assert_eq!(fx_from_milli(-800).raw(), -3_435_973_837);
    }

    #[test]
    #[should_panic(expected = "out of i64 range")]
    fn from_milli_overflow_panics() {
        let _ = fx_from_milli(i64::MAX);
    }

    #[test]
    fn mul_fx_rounding_examples() {
        // Doc 02 §4.1 rounding examples.
        assert_eq!(mul_fx(Fx::from_raw(1), Fx::from_raw(-1)), Fx::from_raw(-1));
        assert_eq!(mul_fx(Fx::from_raw(1), Fx::from_raw(1)), Fx::from_raw(0));
        assert_eq!(mul_fx(Fx::HALF, Fx::HALF).raw(), 1 << 30);
        assert_eq!(mul_fx(Fx::from_int(-7), Fx::HALF), fx_from_milli(-3500));
    }

    #[test]
    #[should_panic(expected = "out of i64 range")]
    fn mul_fx_overflow_panics() {
        let _ = mul_fx(Fx::from_int(1 << 16), Fx::from_int(1 << 16));
    }

    #[test]
    fn div_fx_examples() {
        // Doc 02 §4.1 rounding examples.
        assert_eq!(
            div_fx(Fx::from_int(7), Fx::from_int(2)).raw(),
            15_032_385_536
        );
        assert_eq!(
            div_fx(Fx::from_int(1), Fx::from_int(3)).raw(),
            1_431_655_765
        );
        assert_eq!(
            div_fx(Fx::from_int(-1), Fx::from_int(3)).raw(),
            -1_431_655_766
        );
        assert_eq!(
            div_fx(Fx::from_int(1), Fx::from_int(-3)).raw(),
            -1_431_655_766
        );
        assert_eq!(
            div_fx(Fx::from_int(-1), Fx::from_int(-3)).raw(),
            1_431_655_765
        );
        assert_eq!(div_fx(Fx::MIN, Fx::ONE), Fx::MIN);
    }

    #[test]
    #[should_panic(expected = "out of i64 range")]
    fn div_fx_overflow_panics() {
        let _ = div_fx(Fx::MIN, Fx::from_int(-1));
    }

    #[test]
    fn mul_bp_fx_examples() {
        assert_eq!(mul_bp_fx(Fx::from_int(2), 5_000), Fx::ONE);
        assert_eq!(mul_bp_fx(Fx::from_raw(-1), 5_000), Fx::from_raw(-1));
    }

    #[test]
    fn doc02_t01_isqrt_values() {
        // Doc 02 acceptance test 1 (the 100,000-input cross-check is in
        // isqrt_iteration_bound_and_random_cross_check).
        let inputs = [0u128, 1, 2, 3, 4, 99, 100, 1 << 64, u128::MAX];
        let want = [0u128, 1, 1, 1, 2, 9, 10, 1 << 32, (1 << 64) - 1];
        for (n, w) in inputs.iter().zip(want) {
            assert_eq!(isqrt_u128(*n), w, "n = {n}");
        }
    }

    #[test]
    fn isqrt_examples() {
        // Doc 02 §4.2 examples.
        assert_eq!(isqrt_u128(0), 0);
        assert_eq!(isqrt_u128(1), 1);
        assert_eq!(isqrt_u128(2), 1);
        assert_eq!(isqrt_u128(3), 1);
        assert_eq!(isqrt_u128(4), 2);
        assert_eq!(isqrt_u128(99), 9);
        assert_eq!(isqrt_u128(100), 10);
        assert_eq!(isqrt_u128(u128::MAX), (1u128 << 64) - 1);
        assert_eq!(isqrt_u128(1u128 << 126), 1u128 << 63);
        assert_eq!(isqrt_u128((1u128 << 126) - 1), (1u128 << 63) - 1);
        let big = (1u128 << 64) - 1;
        assert_eq!(isqrt_u128(big * big), big);
        assert_eq!(isqrt_u128(big * big - 1), big - 1);
        assert_eq!(isqrt_u128(big * big + 2 * big), big); // (big+1)^2 - 1
    }

    #[test]
    fn isqrt_exhaustive_small_and_squares() {
        for n in 0u128..=70_000 {
            assert_eq!(isqrt_u128(n), n.isqrt(), "n = {n}");
        }
        for k in [
            2u128,
            3,
            7,
            1 << 20,
            1 << 31,
            (1 << 32) + 1,
            1 << 40,
            (1 << 63) + 12_345,
        ] {
            let sq = k * k;
            assert_eq!(isqrt_u128(sq), k);
            assert_eq!(isqrt_u128(sq - 1), k - 1);
            assert_eq!(isqrt_u128(sq + 1), k);
        }
        // Every bit length 2..=128 at its extremes.
        for bits in 2u32..=128 {
            let lo = 1u128 << (bits - 1);
            let hi = if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            };
            assert_eq!(isqrt_u128(lo), lo.isqrt());
            assert_eq!(isqrt_u128(hi), hi.isqrt());
        }
    }

    /// Instrumented copy of the doc 02 §4.2 loop that also counts iterations.
    fn isqrt_counted(n: u128) -> (u128, u32) {
        if n < 2 {
            return (n, 0);
        }
        let bits = 128 - n.leading_zeros();
        let mut x: u128 = 1u128 << bits.div_ceil(2);
        let mut iters = 0;
        loop {
            iters += 1;
            let y = (x + n / x) >> 1;
            if y >= x {
                return (x, iters);
            }
            x = y;
        }
    }

    #[test]
    fn isqrt_iteration_bound_and_random_cross_check() {
        // Doc 02 §4.2: at most 7 loop passes, measured over 300 inputs per bit
        // length 2..=128, and a 100,000-input cross-check against core.
        let mut rng = crate::rng::Rng::from_seed64(0x15_0127);
        let mut worst = 0;
        for bits in 2u32..=128 {
            for _ in 0..300 {
                let lo = u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64);
                let n = if bits == 128 {
                    lo | (1u128 << 127)
                } else {
                    (lo & ((1u128 << bits) - 1)) | (1u128 << (bits - 1))
                };
                let (r, it) = isqrt_counted(n);
                assert_eq!(r, isqrt_u128(n));
                worst = worst.max(it);
            }
        }
        assert!(worst <= 7, "isqrt_u128 needed {worst} passes");
        for _ in 0..100_000 {
            let n = u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64);
            let n = n >> rng.range(128);
            assert_eq!(isqrt_u128(n), n.isqrt(), "n = {n}");
        }
    }

    #[test]
    fn sqrt_fx_examples() {
        // Doc 02 §4.2 examples.
        assert_eq!(sqrt_fx(Fx::from_int(2)).raw(), 6_074_000_999);
        assert_eq!(sqrt_fx(Fx::from_int(50)).raw(), 30_370_004_999);
        assert_eq!(sqrt_fx(Fx::ZERO), Fx::ZERO);
        assert_eq!(sqrt_fx(Fx::ONE), Fx::ONE);
        assert_eq!(sqrt_fx(Fx::from_int(9)), Fx::from_int(3));
        assert_eq!(sqrt_fx(Fx::from_raw(1 << 30)), Fx::HALF); // sqrt(0.25) = 0.5
        assert_eq!(sqrt_fx(Fx::MAX).raw(), 199_032_864_766_430); // floor(sqrt((2^63-1)*2^32))
    }

    #[test]
    #[should_panic(expected = "negative argument")]
    fn sqrt_fx_negative_panics() {
        let _ = sqrt_fx(Fx::from_raw(-1));
    }

    #[test]
    fn floor_block_examples() {
        assert_eq!(fx_floor_block(fx_from_milli(2500)), 2);
        assert_eq!(fx_floor_block(fx_from_milli(-1)), -1);
        assert_eq!(fx_floor_block(Fx::from_int(-3)), -3);
        assert_eq!(fx_floor_block(Fx::ZERO), 0);
        assert_eq!(fx_floor_block(Fx::MAX), i32::MAX);
        assert_eq!(fx_floor_block(Fx::MIN), i32::MIN);
        assert_eq!(Fx::from_raw(-1).floor_block(), -1);
    }

    #[test]
    fn milli_conversions() {
        assert_eq!(fx_from_milli(4300).to_milli_floor(), 4299);
        assert_eq!(fx_from_milli(4300).to_milli_round(), 4300);
        assert_eq!(Fx::from_int(-2).to_milli_floor(), -2000);
        assert_eq!(Fx::from_raw(-1).to_milli_floor(), -1);
        assert_eq!(Fx::MAX.to_milli_floor(), 2_147_483_647_999);
    }

    #[test]
    fn ops_and_ordering() {
        let a = Fx::from_int(3);
        let b = Fx::from_int(5);
        assert_eq!(a + b, Fx::from_int(8));
        assert_eq!(a - b, Fx::from_int(-2));
        assert_eq!(-a, Fx::from_int(-3));
        let mut c = a;
        c += b;
        c -= Fx::ONE;
        assert_eq!(c, Fx::from_int(7));
        assert!(a < b && b > a && a <= a);
        assert_eq!(a.max(b), b);
        assert_eq!(Fx::from_int(-4).abs(), Fx::from_int(4));
        assert_eq!(Fx::from_int(-4).signum(), -1);
        assert_eq!(a.mul_int(-3), Fx::from_int(-9));
        assert_eq!(Fx::from_raw(-7).div_int(2), Fx::from_raw(-4));
        assert_eq!(Fx::MAX.checked_add(Fx::from_raw(1)), None);
        assert_eq!(Fx::MIN.checked_sub(Fx::from_raw(1)), None);
        assert_eq!(Fx::MIN.checked_neg(), None);
    }

    #[test]
    #[should_panic(expected = "Fx add overflow")]
    fn add_overflow_panics() {
        let _ = Fx::MAX + Fx::from_raw(1);
    }

    #[test]
    #[should_panic(expected = "Fx sub overflow")]
    fn sub_overflow_panics() {
        let _ = Fx::MIN - Fx::from_raw(1);
    }

    #[test]
    #[should_panic(expected = "Fx neg overflow")]
    fn neg_overflow_panics() {
        let _ = -Fx::MIN;
    }

    #[test]
    fn debug_and_display_without_floats() {
        assert_eq!(
            format!("{:?}", Fx::from_int(1)),
            "Fx(raw=4294967296 ~1.000)"
        );
        assert_eq!(
            format!("{:?}", fx_from_milli(-1500)),
            "Fx(raw=-6442450944 ~-1.500)"
        );
        assert_eq!(
            format!("{:?}", fx_from_milli(4300)),
            "Fx(raw=18468359372 ~4.300)"
        );
        assert_eq!(format!("{}", fx_from_milli(-250)), "-0.250");
        assert_eq!(format!("{}", Fx::from_raw(1)), "0.000");
        assert_eq!(format!("{}", Fx::MIN), "-2147483648.000");
    }

    proptest! {
        #[test]
        fn isqrt_matches_core(n in any::<u128>()) {
            let r = isqrt_u128(n);
            prop_assert_eq!(r, n.isqrt());
            prop_assert!(r * r <= n);
            // (r+1)^2 > n, computed without overflow.
            let r1 = r + 1;
            prop_assert!(r1.checked_mul(r1).is_none_or(|sq| sq > n));
        }

        #[test]
        fn isqrt_matches_core_by_bit_length(bits in 2u32..=128, v in any::<u128>()) {
            let n = if bits == 128 { v | (1u128 << 127) } else { (v & ((1u128 << bits) - 1)) | (1u128 << (bits - 1)) };
            prop_assert_eq!(isqrt_u128(n), n.isqrt());
        }

        #[test]
        fn isqrt_monotone(a in any::<u128>(), b in any::<u128>()) {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            prop_assert!(isqrt_u128(lo) <= isqrt_u128(hi));
        }

        #[test]
        fn sqrt_fx_floor_property(r in 0i64..=i64::MAX) {
            let s = sqrt_fx(Fx::from_raw(r)).raw() as i128;
            let n = (r as i128) << 32;
            prop_assert!(s * s <= n && (s + 1) * (s + 1) > n);
        }

        #[test]
        fn sqrt_fx_monotone(a in 0i64..=i64::MAX, b in 0i64..=i64::MAX) {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            prop_assert!(sqrt_fx(Fx::from_raw(lo)) <= sqrt_fx(Fx::from_raw(hi)));
        }

        #[test]
        fn mul_fx_is_floor(a in -(1i64 << 47)..(1i64 << 47), b in -(1i64 << 47)..(1i64 << 47)) {
            let p = a as i128 * b as i128;
            let m = mul_fx(Fx::from_raw(a), Fx::from_raw(b)).raw() as i128;
            prop_assert!(m << 32 <= p && p < (m + 1) << 32);
        }

        #[test]
        fn mul_fx_commutes(a in -(1i64 << 47)..(1i64 << 47), b in -(1i64 << 47)..(1i64 << 47)) {
            prop_assert_eq!(mul_fx(Fx::from_raw(a), Fx::from_raw(b)), mul_fx(Fx::from_raw(b), Fx::from_raw(a)));
        }

        #[test]
        fn div_fx_is_floor_after_sign_normalization(a in -(1i64 << 40)..(1i64 << 40), b in any::<i64>()) {
            prop_assume!(b != 0);
            let q = div_fx(Fx::from_raw(a), Fx::from_raw(b)).raw() as i128;
            let (n, d) = if b < 0 { (-(a as i128) << 32, -(b as i128)) } else { ((a as i128) << 32, b as i128) };
            prop_assert!(q * d <= n && n < (q + 1) * d);
        }

        #[test]
        fn div_fx_sign_symmetry(a in -(1i64 << 40)..(1i64 << 40), b in 1i64..=i64::MAX) {
            prop_assert_eq!(div_fx(Fx::from_raw(-a), Fx::from_raw(b)), div_fx(Fx::from_raw(a), Fx::from_raw(-b)));
        }

        #[test]
        fn from_milli_round_trip(m in -2_000_000_000_000i64..2_000_000_000_000) {
            // floor(floor(m * 2^32 / 1000) * 1000 / 2^32) is m or m - 1, and rounds back to m.
            let fx = fx_from_milli(m);
            let back = fx.to_milli_floor();
            prop_assert!(back == m || back == m - 1);
            prop_assert_eq!(fx.to_milli_round(), m);
        }

        #[test]
        fn from_milli_monotone(a in -2_000_000_000_000i64..2_000_000_000_000, d in 0i64..1_000_000) {
            prop_assert!(fx_from_milli(a) <= fx_from_milli(a + d));
        }

        #[test]
        fn floor_block_contains(r in any::<i64>()) {
            let b = fx_floor_block(Fx::from_raw(r)) as i64;
            prop_assert!((b << 32) <= r && (r as i128) < ((b as i128 + 1) << 32));
        }

        #[test]
        fn add_sub_round_trip(a in any::<i64>(), b in any::<i64>()) {
            let (fa, fb) = (Fx::from_raw(a), Fx::from_raw(b));
            if let Some(s) = fa.checked_add(fb) {
                prop_assert_eq!(s - fb, fa);
            }
        }
    }
}
