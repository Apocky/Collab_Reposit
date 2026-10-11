//! SPEC: doc 02 §4.5-4.6 Fx3, capsules, rays, voxel DDA
//!
//! Vectors are `Fx` triples in meters (canon §1.8: right-handed, +Y up, -Z
//! forward). Wide products (`dot_wide`, `dist2`) are raw Q64.64 `i128` values
//! that are never narrowed (canon §1.4).

use core::ops::{Add, Index, Neg, Sub};

use crate::fx::{Fx, div_fx, fx_floor_block, isqrt_u128, mul_fx};
use crate::int::{U256, floor_div_i128, floor_div_i256, narrow_i64};
pub use crate::int::I256;

/// SPEC: canon §1.8, doc 02 §4.6 `EPS = Fx(256)` (2^-24 m²), the degeneracy threshold for
/// squared segment lengths and the capsule-ray quadratic.
pub const EPS: Fx = Fx::from_raw(256);

/// SPEC: canon §1.8, doc 02 §4.6. Ray direction components with `|raw| < DDA_DIR_EPS_RAW`
/// count as 0 in the voxel DDA, so `div_fx(Fx::ONE, d)` cannot overflow.
pub const DDA_DIR_EPS_RAW: i64 = 256;

/// SPEC: canon §1.4, doc 02 §4.5. A vector of three `Fx` (meters).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fx3 {
    /// East (+X).
    pub x: Fx,
    /// Up (+Y).
    pub y: Fx,
    /// South (+Z); forward is -Z.
    pub z: Fx,
}

/// SPEC: canon §1.8. Block coordinates: an `i32` triple. Block `(bx, by, bz)`
/// occupies `[bx, bx+1) x [by, by+1) x [bz, bz+1)`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IVec3 {
    /// Block x.
    pub x: i32,
    /// Block y.
    pub y: i32,
    /// Block z.
    pub z: i32,
}

impl IVec3 {
    /// SPEC: canon §1.8. Build a block coordinate.
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> IVec3 {
        IVec3 { x, y, z }
    }

    /// SPEC: canon §1.8. The block's minimum corner as an `Fx3`.
    #[inline]
    #[must_use]
    pub const fn min_corner(self) -> Fx3 {
        Fx3::from_ints(self.x, self.y, self.z)
    }
}

impl From<[i32; 3]> for IVec3 {
    #[inline]
    fn from(a: [i32; 3]) -> IVec3 {
        IVec3::new(a[0], a[1], a[2])
    }
}

impl From<IVec3> for [i32; 3] {
    #[inline]
    fn from(v: IVec3) -> [i32; 3] {
        [v.x, v.y, v.z]
    }
}

/// Checked i128 sum of three products; panics on overflow.
#[inline]
#[track_caller]
fn sum3_i128(a: i128, b: i128, c: i128) -> i128 {
    a.checked_add(b)
        .and_then(|s| s.checked_add(c))
        .expect("Fx3: i128 overflow in wide sum")
}

/// Checked square of an i128 difference; panics on overflow.
#[inline]
#[track_caller]
fn sq_i128(d: i128) -> i128 {
    d.checked_mul(d).expect("Fx3: i128 overflow in square")
}

impl Fx3 {
    /// SPEC: canon §1.4, doc 02 §4.5. The zero vector.
    pub const ZERO: Fx3 = Fx3::new(Fx::ZERO, Fx::ZERO, Fx::ZERO);

    /// SPEC: canon §1.4, doc 02 §4.5. Build from three `Fx`.
    #[inline]
    #[must_use]
    pub const fn new(x: Fx, y: Fx, z: Fx) -> Fx3 {
        Fx3 { x, y, z }
    }

    /// SPEC: canon §1.4 `Fx::from_int` per axis.
    #[inline]
    #[must_use]
    pub const fn from_ints(x: i32, y: i32, z: i32) -> Fx3 {
        Fx3::new(Fx::from_int(x), Fx::from_int(y), Fx::from_int(z))
    }

    /// SPEC: canon §1.4 `fx_from_milli` per axis (content values in mm).
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn from_milli(x: i64, y: i64, z: i64) -> Fx3 {
        Fx3::new(Fx::from_milli(x), Fx::from_milli(y), Fx::from_milli(z))
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `add`: checked per-axis add.
    #[inline]
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.5 API; the operator trait is implemented too
    pub fn add(self, o: Fx3) -> Fx3 {
        Fx3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `sub`: checked per-axis sub.
    #[inline]
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.5 API; the operator trait is implemented too
    pub fn sub(self, o: Fx3) -> Fx3 {
        Fx3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `neg`: checked per-axis negation.
    #[inline]
    #[must_use]
    #[track_caller]
    #[allow(clippy::should_implement_trait)] // doc 02 §4.5 API; the operator trait is implemented too
    pub fn neg(self) -> Fx3 {
        Fx3::new(-self.x, -self.y, -self.z)
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `scale`: `mul_fx` per axis.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn scale(self, s: Fx) -> Fx3 {
        Fx3::new(mul_fx(self.x, s), mul_fx(self.y, s), mul_fx(self.z, s))
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `dot_wide`: `Σ a.raw * b.raw` in `i128`, raw Q64.64,
    /// never narrowed. Panics only if the sum exceeds `i128`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot_wide(self, o: Fx3) -> i128 {
        sum3_i128(
            i128::from(self.x.raw()) * i128::from(o.x.raw()),
            i128::from(self.y.raw()) * i128::from(o.y.raw()),
            i128::from(self.z.raw()) * i128::from(o.z.raw()),
        )
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `dot = Fx((dot_wide >> 32) as i64)`, floor, checked narrowing.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, o: Fx3) -> Fx {
        Fx::from_raw(narrow_i64(self.dot_wide(o) >> 32))
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `cross`: `mul_fx` terms, x then y then z.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn cross(self, o: Fx3) -> Fx3 {
        let x = mul_fx(self.y, o.z) - mul_fx(self.z, o.y);
        let y = mul_fx(self.z, o.x) - mul_fx(self.x, o.z);
        let z = mul_fx(self.x, o.y) - mul_fx(self.y, o.x);
        Fx3::new(x, y, z)
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `length = Fx(isqrt_u128(dot_wide as u128))`, floor.
    /// `dot_wide` is Q64.64, so its integer square root is Q32.32 directly.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length(self) -> Fx {
        let w = self.dot_wide(self);
        // w >= 0 as a sum of squares.
        let r = isqrt_u128(w.unsigned_abs());
        Fx::from_raw(i64::try_from(r).expect("Fx3::length: result out of range"))
    }

    /// SPEC: canon §1.4, doc 02 §4.5 `try_normalize`: `None` if `length.raw < 256`, else
    /// `div_fx` of each axis by the length.
    #[must_use]
    #[track_caller]
    pub fn try_normalize(self) -> Option<Fx3> {
        let l = self.length();
        if l.raw() < 256 {
            return None;
        }
        Some(Fx3::new(
            div_fx(self.x, l),
            div_fx(self.y, l),
            div_fx(self.z, l),
        ))
    }

    /// SPEC: canon §1.4 `fx_floor_block` per axis: the block containing the point.
    #[inline]
    #[must_use]
    pub const fn floor_block(self) -> IVec3 {
        IVec3::new(
            fx_floor_block(self.x),
            fx_floor_block(self.y),
            fx_floor_block(self.z),
        )
    }
}

impl Add for Fx3 {
    type Output = Fx3;
    #[inline]
    #[track_caller]
    fn add(self, o: Fx3) -> Fx3 {
        Fx3::add(self, o)
    }
}

impl Sub for Fx3 {
    type Output = Fx3;
    #[inline]
    #[track_caller]
    fn sub(self, o: Fx3) -> Fx3 {
        Fx3::sub(self, o)
    }
}

impl Neg for Fx3 {
    type Output = Fx3;
    #[inline]
    #[track_caller]
    fn neg(self) -> Fx3 {
        Fx3::neg(self)
    }
}

impl Index<usize> for Fx3 {
    type Output = Fx;
    /// SPEC: canon §1.8, doc 02 §4.6. `0 = x`, `1 = y`, `2 = z`; panics otherwise.
    #[inline]
    #[track_caller]
    fn index(&self, i: usize) -> &Fx {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Fx3 index out of range"),
        }
    }
}

/// SPEC: canon §1.4 `dist2(p, q) = Σ (p_i.raw - q_i.raw)²` over the three axes
/// in `i128`. The result is raw Q64.64; compare it against `(r.raw as i128)²`
/// and never narrow it. Panics only if the sum exceeds `i128` (axis
/// differences beyond about 2^62.7 raw).
#[inline]
#[must_use]
#[track_caller]
pub fn dist2(p: Fx3, q: Fx3) -> i128 {
    let dx = i128::from(p.x.raw()) - i128::from(q.x.raw());
    let dy = i128::from(p.y.raw()) - i128::from(q.y.raw());
    let dz = i128::from(p.z.raw()) - i128::from(q.z.raw());
    sum3_i128(sq_i128(dx), sq_i128(dy), sq_i128(dz))
}

/// SPEC: canon §1.4, doc 02 §4.5 `dist(p, q) = Fx(isqrt_u128(dist2(p, q) as u128))`, floor.
#[inline]
#[must_use]
#[track_caller]
pub fn dist(p: Fx3, q: Fx3) -> Fx {
    let r = isqrt_u128(dist2(p, q).unsigned_abs());
    Fx::from_raw(i64::try_from(r).expect("dist: result out of range"))
}

/// SPEC: canon §1.4. `true` when `dist2(p, q) <= r²`, compared in `i128`.
#[inline]
#[must_use]
#[track_caller]
pub fn within(p: Fx3, q: Fx3, r: Fx) -> bool {
    let rr = i128::from(r.raw());
    dist2(p, q) <= rr * rr
}

/// SPEC: canon §1.8, doc 02 §4.6 `clamp01(x) = x.clamp(Fx::ZERO, Fx::ONE)`.
#[inline]
#[must_use]
pub fn clamp01(x: Fx) -> Fx {
    x.clamp(Fx::ZERO, Fx::ONE)
}

/// SPEC: canon §1.8, doc 02 §4.6 `nonneg(t) = Some(t) if t >= 0 else None`.
#[inline]
#[must_use]
pub fn nonneg(t: Fx) -> Option<Fx> {
    if t >= Fx::ZERO { Some(t) } else { None }
}

/// SPEC: canon §1.8, doc 02 §4.6 `min_nonneg(a, b)`: the smaller of the `Some` values.
#[inline]
#[must_use]
pub fn min_nonneg(a: Option<Fx>, b: Option<Fx>) -> Option<Fx> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        (None, y) => y,
    }
}

/// SPEC: canon §1.8, doc 02 §4.6. A capsule: the segment `a`-`b` with radius `r`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Capsule {
    /// First segment endpoint.
    pub a: Fx3,
    /// Second segment endpoint.
    pub b: Fx3,
    /// Radius in meters.
    pub r: Fx,
}

/// SPEC: canon §1.8, doc 02 §4.6 and canon §1.8, §4.1. The vertical movement capsule: the
/// segment from `origin + (0, radius, 0)` to `origin + (0, height - radius, 0)`
/// with radius `radius`. `origin` is the bottom-center of the capsule.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VCapsule {
    /// Bottom-center of the capsule (the entity origin, canon §1.8).
    pub origin: Fx3,
    /// Radius in meters.
    pub radius: Fx,
    /// Total height in meters (bottom of the lower cap to top of the upper cap).
    pub height: Fx,
}

impl VCapsule {
    /// SPEC: canon §1.8, doc 02 §4.6. Segment endpoints `(bottom, top)`:
    /// `origin + (0, r, 0)` and `origin + (0, height - r, 0)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn segment(&self) -> (Fx3, Fx3) {
        let o = self.origin;
        (
            Fx3::new(o.x, o.y + self.radius, o.z),
            Fx3::new(o.x, o.y + self.height - self.radius, o.z),
        )
    }

    /// SPEC: canon §1.8, doc 02 §4.6. The general capsule with the same segment and radius.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_capsule(&self) -> Capsule {
        let (a, b) = self.segment();
        Capsule {
            a,
            b,
            r: self.radius,
        }
    }

    /// SPEC: canon §1.8, doc 02 §4.6 vertical capsule against a block. For the box
    /// `[lo, hi]` of block `blk` and the vertical segment at `(x, z)` spanning
    /// `[y0, y1]`: `dx = max(0, lo.x - x, x - hi.x)`, `dz` likewise,
    /// `dy = max(0, lo.y - y1, y0 - hi.y)`; overlap iff
    /// `dx² + dy² + dz² < r²` with `i128` raw squares (strict).
    #[must_use]
    #[track_caller]
    pub fn overlaps_block(&self, blk: IVec3) -> bool {
        let (s0, s1) = self.segment();
        let lo = blk.min_corner();
        // hi = lo + 1 on each axis, computed in i128 so block i32::MAX is safe.
        let one = i128::from(Fx::ONE.raw());
        let gap = |lo_raw: i128, a_raw: i128, b_raw: i128| -> i128 {
            // max(0, lo - b, a - hi) for an interval [a, b] against [lo, lo + 1].
            let hi_raw = lo_raw + one;
            (lo_raw - b_raw).max(a_raw - hi_raw).max(0)
        };
        let x = i128::from(s0.x.raw());
        let z = i128::from(s0.z.raw());
        let dx = gap(i128::from(lo.x.raw()), x, x);
        let dz = gap(i128::from(lo.z.raw()), z, z);
        let dy = gap(
            i128::from(lo.y.raw()),
            i128::from(s0.y.raw()),
            i128::from(s1.y.raw()),
        );
        let rr = i128::from(self.radius.raw());
        sum3_i128(sq_i128(dx), sq_i128(dy), sq_i128(dz)) < rr * rr
    }
}

/// SPEC: canon §1.4, doc 02 §4.6 `div128(n, d)`: the ratio of two values of
/// equal scale as `Fx`, `floor_div(n << 32, d)` with checked narrowing to
/// `i64`. Requires `d != 0`; negates both operands when `d < 0`. The shift is
/// checked too (a plain `<<` would drop bits silently), so it panics when
/// `|n| >= 2^95` instead of wrapping.
#[must_use]
#[track_caller]
pub fn div128(n: i128, d: i128) -> Fx {
    let (n, d) = if d < 0 {
        (n.checked_neg().expect("div128: overflow"), d.checked_neg().expect("div128: overflow"))
    } else {
        (n, d)
    };
    let shifted = n.checked_mul(1i128 << 32).expect("div128: n << 32 overflows i128");
    Fx::from_raw(i64::try_from(floor_div_i128(shifted, d)).expect("invariant: div128 range"))
}

/// SPEC: canon §1.4, doc 02 §4.6 `div256(n, d) = Fx(floor_div_i256(n.shl32(), d))`,
/// checked; negates both operands when `d < 0`. Panics if `d == 0` or the
/// quotient does not fit `Fx`.
#[must_use]
#[track_caller]
pub fn div256(n: I256, d: I256) -> Fx {
    let (n, d) = if d.is_negative() { (n.neg(), d.neg()) } else { (n, d) };
    Fx::from_raw(i64::try_from(floor_div_i256(n.shl32(), d)).expect("invariant: div256 range"))
}

/// `clamp01(floor(n * 2^32 / d))` for `d != 0`, exact and total: it equals
/// `clamp01(div256(n, d))` whenever `div256` does not panic, and also covers
/// ratios whose unclamped value does not fit `Fx`.
#[track_caller]
fn ratio01(n: I256, d: I256) -> Fx {
    assert!(d != I256::ZERO, "ratio01: zero divisor");
    let (n, d) = if d.is_negative() { (n.neg(), d.neg()) } else { (n, d) };
    if n.is_negative() || n == I256::ZERO {
        Fx::ZERO // floor(n * 2^32 / d) <= 0
    } else if n >= d {
        Fx::ONE // floor(n * 2^32 / d) >= 2^32
    } else {
        // 0 < n < d < 2^255: the 32-bit fraction fits.
        let q = U256::frac_q32(n.unsigned_abs(), d.unsigned_abs());
        Fx::from_raw(i64::try_from(q).expect("fraction below 2^32"))
    }
}

/// SPEC: canon §1.8, doc 02 §4.6 closest points between segments `p1`-`q1`
/// and `p2`-`q2`: the one implementation of the closest-point problem.
///
/// Returns `(s, t, c1, c2)` with `s, t` in `0..=1`, `c1 = p1 + (q1 - p1)·s`
/// and `c2 = p2 + (q2 - p2)·t`. All dot products are exact `i128` Q64.64
/// (`dot_wide`) and there is no epsilon: a segment is degenerate only when its
/// squared length is exactly 0.
///
/// This follows the doc 02 §4.6 code step by step and gives the same `(s, t)`
/// wherever that code completes. The doc's `clamp01(div128(..))`,
/// `clamp01(div256(..))` and `t = div128(((b * s.raw()) >> 32) + f, e)` are
/// evaluated in 256-bit arithmetic with the clamp decided before narrowing, so
/// nearly parallel or very short segments (whose unclamped ratios exceed the
/// `Fx` range and would hit the doc's "invariant" panics) still return the
/// clamped result.
#[must_use]
#[track_caller]
pub fn closest_seg_seg(p1: Fx3, q1: Fx3, p2: Fx3, q2: Fx3) -> (Fx, Fx, Fx3, Fx3) {
    let (d1, d2, r) = (q1.sub(p1), q2.sub(p2), p1.sub(p2));
    let (a, e, f) = (d1.dot_wide(d1), d2.dot_wide(d2), d2.dot_wide(r)); // i128, Q64.64
    let w = I256::from_i128;
    let (s, t) = if a == 0 && e == 0 {
        (Fx::ZERO, Fx::ZERO)
    } else if a == 0 {
        (Fx::ZERO, ratio01(w(f), w(e)))
    } else {
        let c = d1.dot_wide(r);
        if e == 0 {
            (ratio01(w(c).neg(), w(a)), Fx::ZERO)
        } else {
            let b = d1.dot_wide(d2);
            let denom = I256::mul(a, e).sub(I256::mul(b, b)); // >= 0 (Cauchy-Schwarz)
            let mut s = if denom != I256::ZERO {
                ratio01(I256::mul(b, f).sub(I256::mul(c, e)), denom)
            } else {
                Fx::ZERO
            };
            // tn = ((b * s.raw) >> 32) + f, exact (the shift is floor).
            let tn = I256::mul(b, i128::from(s.raw())).sar(32).add(w(f));
            // t = floor(tn * 2^32 / e) with e > 0, so t < 0 iff tn < 0 and
            // t > 1 iff tn * 2^32 >= (2^32 + 1) * e.
            let t;
            if tn.is_negative() {
                t = Fx::ZERO;
                s = ratio01(w(c).neg(), w(a));
            } else if tn.shl32() >= I256::mul(e, (1i128 << 32) + 1) {
                t = Fx::ONE;
                s = ratio01(w(b).sub(w(c)), w(a));
            } else {
                // 0 <= tn * 2^32 < (2^32 + 1) * e, so t is in 0..=2^32.
                t = Fx::from_raw(
                    i64::try_from(floor_div_i256(tn.shl32(), w(e))).expect("t in 0..=2^32"),
                );
            }
            (s, t)
        }
    };
    (s, t, p1.add(d1.scale(s)), p2.add(d2.scale(t)))
}

/// SPEC: canon §1.8, doc 02 §4.6 capsule overlap:
/// `dist2(cp1, cp2) <= (c1.r + c2.r).raw()²` in `i128`, with `cp1`, `cp2`
/// from [`closest_seg_seg`].
#[must_use]
#[track_caller]
pub fn capsule_overlap(c1: &Capsule, c2: &Capsule) -> bool {
    let (_, _, cp1, cp2) = closest_seg_seg(c1.a, c1.b, c2.a, c2.b);
    within(cp1, cp2, c1.r + c2.r)
}

/// SPEC: canon §1.8, doc 02 §4.6 ray against sphere. `rd` must be unit length. Returns the
/// entry distance `t >= 0`, or `None` on a miss, a tangent ray, or when the
/// entry point lies behind the origin.
#[must_use]
#[track_caller]
pub fn ray_sphere(ro: Fx3, rd: Fx3, cen: Fx3, r: Fx) -> Option<Fx> {
    let oc = ro.sub(cen);
    let b = rd.dot(oc);
    let bb = i128::from(b.raw()) * i128::from(b.raw());
    let c = oc.dot(oc) - mul_fx(r, r);
    let h = bb
        .checked_sub(i128::from(c.raw()) << 32)
        .expect("ray_sphere: overflow");
    if h <= 0 {
        return None;
    }
    let sq = i64::try_from(isqrt_u128(h.unsigned_abs())).expect("ray_sphere: sqrt out of range");
    nonneg(-b - Fx::from_raw(sq))
}

/// SPEC: canon §1.8, doc 02 §4.6 ray against capsule (Quilez form). `rd` must be unit
/// length (from [`Fx3::try_normalize`]). Returns the entry distance `t >= 0`
/// or `None`.
#[must_use]
#[track_caller]
pub fn ray_capsule(ro: Fx3, rd: Fx3, pa: Fx3, pb: Fx3, r: Fx) -> Option<Fx> {
    let (ba, oa) = (pb.sub(pa), ro.sub(pa));
    let (baba, bard, baoa, rdoa, oaoa) =
        (ba.dot(ba), ba.dot(rd), ba.dot(oa), rd.dot(oa), oa.dot(oa));
    let a = baba - mul_fx(bard, bard);
    let b = mul_fx(baba, rdoa) - mul_fx(baoa, bard);
    let c = mul_fx(baba, oaoa) - mul_fx(baoa, baoa) - mul_fx(mul_fx(r, r), baba);
    if a > EPS {
        let bb = i128::from(b.raw()) * i128::from(b.raw());
        let ac = i128::from(a.raw()) * i128::from(c.raw());
        let h = bb.checked_sub(ac).expect("ray_capsule: overflow"); // Q64.64
        if h < 0 {
            return None;
        }
        let sq =
            i64::try_from(isqrt_u128(h.unsigned_abs())).expect("ray_capsule: sqrt out of range");
        let t = div_fx(-b - Fx::from_raw(sq), a);
        let y = baoa + mul_fx(t, bard);
        if y > Fx::ZERO && y < baba {
            return nonneg(t); // cylinder body
        }
        return ray_sphere(ro, rd, if y <= Fx::ZERO { pa } else { pb }, r); // nearer cap
    }
    min_nonneg(ray_sphere(ro, rd, pa, r), ray_sphere(ro, rd, pb, r)) // ray parallel to axis
}

/// SPEC: canon §1.8, doc 02 §4.6. A ray with origin and direction (unit length for the
/// capsule and sphere tests; any length for the voxel DDA, where `t` is then
/// in units of the direction's length).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ray {
    /// Origin.
    pub origin: Fx3,
    /// Direction.
    pub dir: Fx3,
}

impl Ray {
    /// SPEC: canon §1.8, doc 02 §4.6. Build a ray.
    #[inline]
    #[must_use]
    pub const fn new(origin: Fx3, dir: Fx3) -> Ray {
        Ray { origin, dir }
    }

    /// SPEC: canon §1.8, doc 02 §4.6. The point `origin + dir·t` (`mul_fx` per axis).
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn at(&self, t: Fx) -> Fx3 {
        self.origin.add(self.dir.scale(t))
    }

    /// SPEC: canon §1.8, doc 02 §4.6 [`ray_capsule`] against `c`; `dir` must be unit length.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn hit_capsule(&self, c: &Capsule) -> Option<Fx> {
        ray_capsule(self.origin, self.dir, c.a, c.b, c.r)
    }

    /// SPEC: canon §1.8, doc 02 §4.6 [`ray_sphere`]; `dir` must be unit length.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn hit_sphere(&self, center: Fx3, r: Fx) -> Option<Fx> {
        ray_sphere(self.origin, self.dir, center, r)
    }

    /// SPEC: canon §1.8, doc 02 §4.6 voxel traversal from this ray; see [`VoxelDda`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn voxels(&self, t_max: Fx, max_steps: u32) -> VoxelDda {
        VoxelDda::new(self.origin, self.dir, t_max, max_steps)
    }
}

/// SPEC: canon §1.8, doc 02 §4.6. The face of a block, named by its outward normal.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Face {
    /// The +X face.
    PosX,
    /// The -X face.
    NegX,
    /// The +Y face (top).
    PosY,
    /// The -Y face (bottom).
    NegY,
    /// The +Z face.
    PosZ,
    /// The -Z face.
    NegZ,
}

impl Face {
    /// SPEC: canon §1.8, doc 02 §4.6 `Face::entered(axis, step)`: the face of the new block
    /// a ray crosses when stepping along `axis` (0 = x, 1 = y, 2 = z) by
    /// `step` (+1 enters through the negative face, -1 through the positive).
    /// Panics for `axis > 2` or `step == 0`.
    #[must_use]
    #[track_caller]
    pub fn entered(axis: usize, step: i32) -> Face {
        assert!(step != 0, "Face::entered: step must be nonzero");
        match (axis, step > 0) {
            (0, true) => Face::NegX,
            (0, false) => Face::PosX,
            (1, true) => Face::NegY,
            (1, false) => Face::PosY,
            (2, true) => Face::NegZ,
            (2, false) => Face::PosZ,
            _ => panic!("Face::entered: axis out of range"),
        }
    }

    /// SPEC: canon §1.8. The outward unit normal of the face in block units.
    #[must_use]
    pub const fn normal(self) -> IVec3 {
        match self {
            Face::PosX => IVec3::new(1, 0, 0),
            Face::NegX => IVec3::new(-1, 0, 0),
            Face::PosY => IVec3::new(0, 1, 0),
            Face::NegY => IVec3::new(0, -1, 0),
            Face::PosZ => IVec3::new(0, 0, 1),
            Face::NegZ => IVec3::new(0, 0, -1),
        }
    }
}

/// SPEC: canon §1.8, doc 02 §4.6. One block visited by the voxel DDA.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DdaStep {
    /// The block visited.
    pub cell: IVec3,
    /// Ray parameter at which the ray enters the block (0 for the start block).
    pub t: Fx,
    /// The face of `cell` the ray entered through (`None` for the start block).
    pub face: Option<Face>,
}

/// SPEC: canon §1.8, doc 02 §4.6 voxel traversal (Amanatides-Woo DDA) as an iterator.
///
/// Yields every block the ray `o + d·t` crosses, in order, starting with the
/// block containing `o` at `t = 0`. Each later block is yielded with the `t`
/// at which the ray enters it and the face it enters through. Axis ties
/// resolve x, then y, then z. Direction components with `|raw| < 256` count
/// as 0. Iteration ends when the next crossing lies beyond `t_max` (a
/// crossing exactly at `t_max` is still yielded), after `max_steps` steps
/// beyond the start block (so at most `max_steps + 1` items), when the ray
/// does not move, or when the next block coordinate would leave the `i32`
/// range. Callers stop early (for example on a solid block) by breaking.
#[derive(Clone, Debug)]
pub struct VoxelDda {
    cell: [i32; 3],
    step: [i32; 3],
    t_delta: [Fx; 3],
    t_next: [Fx; 3],
    t_max: Fx,
    steps_left: u32,
    started: bool,
    done: bool,
}

impl VoxelDda {
    /// SPEC: canon §1.8, doc 02 §4.6. Start a traversal from `o` along `d` up to `t_max`,
    /// taking at most `max_steps` steps beyond the start block.
    #[must_use]
    #[track_caller]
    pub fn new(o: Fx3, d: Fx3, t_max: Fx, max_steps: u32) -> VoxelDda {
        let cell = [o.x.floor_block(), o.y.floor_block(), o.z.floor_block()];
        let mut step = [0i32; 3];
        let mut t_delta = [Fx::MAX; 3];
        let mut t_next = [Fx::MAX; 3];
        for i in 0..3 {
            let (oi, di) = (o[i], d[i]);
            // Fractional part of the origin within its block, in [0, 2^32).
            let frac = Fx::from_raw(oi.raw() & 0xFFFF_FFFF);
            if di.raw() >= DDA_DIR_EPS_RAW {
                step[i] = 1;
                t_delta[i] = div_fx(Fx::ONE, di);
                // Fx::from_int(cell + 1) - oi == ONE - frac, without overflow at the i32 edge.
                t_next[i] = div_fx(Fx::ONE - frac, di);
            } else if di.raw() <= -DDA_DIR_EPS_RAW {
                step[i] = -1;
                t_delta[i] = div_fx(Fx::ONE, -di);
                // oi - Fx::from_int(cell) == frac.
                t_next[i] = div_fx(frac, -di);
            }
        }
        VoxelDda {
            cell,
            step,
            t_delta,
            t_next,
            t_max,
            steps_left: max_steps,
            started: false,
            done: false,
        }
    }
}

impl Iterator for VoxelDda {
    type Item = DdaStep;

    fn next(&mut self) -> Option<DdaStep> {
        if self.done {
            return None;
        }
        if !self.started {
            self.started = true;
            return Some(DdaStep {
                cell: IVec3::from(self.cell),
                t: Fx::ZERO,
                face: None,
            });
        }
        if self.steps_left == 0 {
            self.done = true;
            return None;
        }
        let mut ax = 0;
        if self.t_next[1] < self.t_next[ax] {
            ax = 1;
        }
        if self.t_next[2] < self.t_next[ax] {
            ax = 2;
        }
        let t = self.t_next[ax];
        if t > self.t_max || self.step[ax] == 0 {
            self.done = true;
            return None;
        }
        let Some(c) = self.cell[ax].checked_add(self.step[ax]) else {
            self.done = true;
            return None;
        };
        self.cell[ax] = c;
        // A crossing time beyond the Fx range is never reached: keep it at MAX.
        self.t_next[ax] = self.t_next[ax]
            .checked_add(self.t_delta[ax])
            .unwrap_or(Fx::MAX);
        self.steps_left -= 1;
        Some(DdaStep {
            cell: IVec3::from(self.cell),
            t,
            face: Some(Face::entered(ax, self.step[ax])),
        })
    }
}

/// SPEC: canon §1.8, doc 02 §4.6 `dda(o, d, t_max, visit)`: callback form of
/// [`VoxelDda`]. Calls `visit(cell, t, face)` for every block the ray crosses
/// up to `t_max` (the start block first, with `t = 0` and no face) and stops
/// early when `visit` returns `false`. Bounded by `t_max` and by the `i32`
/// block range; use [`VoxelDda::new`] directly for an explicit step bound.
#[track_caller]
pub fn dda(o: Fx3, d: Fx3, t_max: Fx, mut visit: impl FnMut(IVec3, Fx, Option<Face>) -> bool) {
    for s in VoxelDda::new(o, d, t_max, u32::MAX) {
        if !visit(s.cell, s.t, s.face) {
            return;
        }
    }
}

/// SPEC: canon §2.1 semi-implicit Euler for one tick: `v += a × DT`, then
/// `p += v × DT`, each product with `mul_fx` per axis. Returns `(p', v')`.
#[must_use]
#[track_caller]
pub fn semi_implicit_euler(p: Fx3, v: Fx3, a: Fx3) -> (Fx3, Fx3) {
    let v2 = v.add(a.scale(Fx::DT));
    let p2 = p.add(v2.scale(Fx::DT));
    (p2, v2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::fx_from_milli;
    use proptest::prelude::*;

    fn m(v: i64) -> Fx {
        fx_from_milli(v)
    }

    #[test]
    fn fx3_basic_ops() {
        let a = Fx3::from_ints(1, 2, 3);
        let b = Fx3::from_ints(4, -5, 6);
        assert_eq!(a.add(b), Fx3::from_ints(5, -3, 9));
        assert_eq!(a + b, Fx3::from_ints(5, -3, 9));
        assert_eq!(a.sub(b), Fx3::from_ints(-3, 7, -3));
        assert_eq!(a - b, Fx3::from_ints(-3, 7, -3));
        assert_eq!(a.neg(), Fx3::from_ints(-1, -2, -3));
        assert_eq!(-a, Fx3::from_ints(-1, -2, -3));
        assert_eq!(a.scale(Fx::HALF), Fx3::new(Fx::HALF, Fx::ONE, m(1500)));
        assert_eq!(a.dot(b), Fx::from_int(4 - 10 + 18));
        assert_eq!(a.dot_wide(b), 12i128 << 64);
        assert_eq!(
            a.cross(b),
            Fx3::from_ints(2 * 6 - 3 * -5, 3 * 4 - 6, -5 - 2 * 4)
        );
        assert_eq!(a[0], a.x);
        assert_eq!(a[1], a.y);
        assert_eq!(a[2], a.z);
        assert_eq!(
            Fx3::from_milli(2500, -1, 0).floor_block(),
            IVec3::new(2, -1, 0)
        );
    }

    #[test]
    #[should_panic(expected = "index out of range")]
    fn fx3_index_panics() {
        let _ = Fx3::ZERO[3];
    }

    #[test]
    fn length_examples() {
        // Doc 02 §4.5: length((3 m, 4 m, 0)) = exactly 5 m.
        let v = Fx3::from_ints(3, 4, 0);
        assert_eq!(v.dot_wide(v), 25i128 << 64);
        assert_eq!(v.length(), Fx::from_int(5));
        assert_eq!(Fx3::ZERO.length(), Fx::ZERO);
        assert_eq!(Fx3::ZERO.try_normalize(), None);
        assert_eq!(
            Fx3::new(Fx::from_raw(255), Fx::ZERO, Fx::ZERO).try_normalize(),
            None
        );
        let n = v.try_normalize().unwrap();
        assert_eq!(
            n,
            Fx3::new(
                div_fx(Fx::from_int(3), Fx::from_int(5)),
                div_fx(Fx::from_int(4), Fx::from_int(5)),
                Fx::ZERO
            )
        );
        assert_eq!(
            Fx3::from_ints(0, 0, -7).try_normalize(),
            Some(Fx3::from_ints(0, 0, -1))
        );
    }

    #[test]
    fn dist2_examples() {
        let p = Fx3::from_ints(1, 2, 3);
        let q = Fx3::from_ints(4, 6, 3);
        assert_eq!(dist2(p, q), 25i128 << 64);
        assert_eq!(dist(p, q), Fx::from_int(5));
        assert!(within(p, q, Fx::from_int(5)));
        assert!(!within(p, q, Fx::from_raw(Fx::from_int(5).raw() - 1)));
        // Extreme but representable coordinates do not overflow.
        let far = dist2(Fx3::new(Fx::MAX, Fx::ZERO, Fx::ZERO), Fx3::ZERO);
        assert_eq!(far, i128::from(i64::MAX) * i128::from(i64::MAX));
        let far2 = dist2(Fx3::new(Fx::MAX, Fx::MIN, Fx::ZERO), Fx3::ZERO);
        assert_eq!(
            far2,
            i128::from(i64::MAX) * i128::from(i64::MAX) + (1i128 << 126)
        );
    }

    #[test]
    #[should_panic(expected = "i128 overflow")]
    fn dist2_overflow_panics() {
        let a = Fx3::new(Fx::MAX, Fx::MAX, Fx::MAX);
        let b = Fx3::new(Fx::MIN, Fx::MIN, Fx::MIN);
        let _ = dist2(a, b);
    }

    #[test]
    fn closest_seg_seg_example_1() {
        // Doc 02 §4.6 example 1.
        let (s, t, c1, c2) = closest_seg_seg(
            Fx3::from_ints(0, 0, 0),
            Fx3::from_ints(2, 0, 0),
            Fx3::from_ints(1, -1, 1),
            Fx3::from_ints(1, 1, 1),
        );
        assert_eq!(s.raw(), 2_147_483_648);
        assert_eq!(t.raw(), 2_147_483_648);
        assert_eq!(c1, Fx3::from_ints(1, 0, 0));
        assert_eq!(c2, Fx3::from_ints(1, 0, 1));
        assert_eq!(dist2(c1, c2), 1i128 << 64);
    }

    #[test]
    fn closest_seg_seg_degenerate_and_parallel() {
        // Two points.
        let (s, t, c1, c2) = closest_seg_seg(
            Fx3::ZERO,
            Fx3::ZERO,
            Fx3::from_ints(1, 0, 0),
            Fx3::from_ints(1, 0, 0),
        );
        assert_eq!((s, t), (Fx::ZERO, Fx::ZERO));
        assert_eq!((c1, c2), (Fx3::ZERO, Fx3::from_ints(1, 0, 0)));
        // Point against segment.
        let (_, t, _, c2) = closest_seg_seg(
            Fx3::from_ints(1, 5, 0),
            Fx3::from_ints(1, 5, 0),
            Fx3::ZERO,
            Fx3::from_ints(4, 0, 0),
        );
        assert_eq!(t, div_fx(Fx::ONE, Fx::from_int(4)));
        assert_eq!(c2, Fx3::from_ints(1, 0, 0));
        // Segment against point.
        let (s, _, c1, _) = closest_seg_seg(
            Fx3::ZERO,
            Fx3::from_ints(0, 4, 0),
            Fx3::from_ints(3, 3, 0),
            Fx3::from_ints(3, 3, 0),
        );
        assert_eq!(s, div_fx(Fx::from_int(3), Fx::from_int(4)));
        assert_eq!(c1, Fx3::from_ints(0, 3, 0));
        // Parallel segments: denom = 0 so s0 = 0; tnom = f = -2 < 0 gives t = 0
        // and s = clamp01(-c / a) = 2 / 4.
        let (s, t, c1, c2) = closest_seg_seg(
            Fx3::ZERO,
            Fx3::from_ints(2, 0, 0),
            Fx3::from_ints(1, 1, 0),
            Fx3::from_ints(3, 1, 0),
        );
        assert_eq!(s, Fx::HALF);
        assert_eq!(t, Fx::ZERO);
        assert_eq!(c1, Fx3::from_ints(1, 0, 0));
        assert_eq!(c2, Fx3::from_ints(1, 1, 0));
        // Disjoint crossing segments where s clamps to 1.
        let (s, _, c1, _) = closest_seg_seg(
            Fx3::ZERO,
            Fx3::from_ints(1, 0, 0),
            Fx3::from_ints(5, -1, 1),
            Fx3::from_ints(5, 1, 1),
        );
        assert_eq!(s, Fx::ONE);
        assert_eq!(c1, Fx3::from_ints(1, 0, 0));
    }

    #[test]
    fn closest_seg_seg_long_segments_do_not_overflow() {
        // 600 m segments: num is about 2^100 raw, so the doc 02 expression
        // `num << 32` would exceed i128 here.
        let (s, t, _, _) = closest_seg_seg(
            Fx3::from_ints(-300, 0, 0),
            Fx3::from_ints(300, 0, 0),
            Fx3::from_ints(30, -300, 150),
            Fx3::from_ints(30, 300, 150),
        );
        assert_eq!(s, div_fx(Fx::from_int(330), Fx::from_int(600)));
        assert_eq!(t, Fx::HALF);
    }

    #[test]
    fn capsule_overlap_examples() {
        let c1 = Capsule {
            a: Fx3::from_ints(0, 0, 0),
            b: Fx3::from_ints(2, 0, 0),
            r: Fx::HALF,
        };
        let c2 = Capsule {
            a: Fx3::from_ints(1, -1, 1),
            b: Fx3::from_ints(1, 1, 1),
            r: Fx::HALF,
        };
        assert!(capsule_overlap(&c1, &c2)); // distance 1 = 0.5 + 0.5 (touching counts)
        let c3 = Capsule { r: m(499), ..c2 };
        assert!(!capsule_overlap(&c1, &c3));
    }

    #[test]
    fn vcapsule_block_example() {
        // Doc 02 §4.6: origin (2.5, 1.0, 0.5), r 0.3, height 1.8 against block (3, 1, 0).
        let mut c = VCapsule {
            origin: Fx3::from_milli(2500, 1000, 500),
            radius: m(300),
            height: m(1800),
        };
        let (s0, s1) = c.segment();
        assert_eq!(s0.y, m(1000) + m(300));
        assert_eq!(s1.y, m(1000) + m(1800) - m(300));
        assert!(!c.overlaps_block(IVec3::new(3, 1, 0)));
        c.origin.x = m(2750);
        assert!(c.overlaps_block(IVec3::new(3, 1, 0)));
        // The block the capsule stands in overlaps; a block far above does not.
        assert!(c.overlaps_block(IVec3::new(2, 1, 0)));
        assert!(!c.overlaps_block(IVec3::new(2, 5, 0)));
        // Block directly below the feet: dy = lo.y... gap is r exactly, strict test fails.
        let stand = VCapsule {
            origin: Fx3::from_milli(500, 1000, 500),
            radius: m(300),
            height: m(1800),
        };
        assert!(!stand.overlaps_block(IVec3::new(0, 0, 0)));
        let cap = stand.to_capsule();
        assert_eq!(cap.r, m(300));
        assert_eq!((cap.a, cap.b), stand.segment());
    }

    #[test]
    fn ray_capsule_worked_examples() {
        // Doc 02 §4.6: ray along +X against capsule (5, 0.3, 0)-(5, 1.5, 0), r 0.3.
        let pa = Fx3::from_milli(5000, 300, 0);
        let pb = Fx3::from_milli(5000, 1500, 0);
        let r = m(300);
        let rd = Fx3::from_ints(1, 0, 0);
        let body = ray_capsule(Fx3::from_milli(0, 1000, 0), rd, pa, pb, r);
        assert_eq!(body.map(Fx::raw), Some(20_186_346_293));
        let cap = ray_capsule(Fx3::from_milli(0, 1700, 0), rd, pa, pb, r);
        assert_eq!(cap.map(Fx::raw), Some(20_514_452_597));
        let miss = ray_capsule(Fx3::from_milli(0, 2000, 0), rd, pa, pb, r);
        assert_eq!(miss, None);
        // Same through the Ray / Capsule types.
        let ray = Ray::new(Fx3::from_milli(0, 1000, 0), rd);
        assert_eq!(
            ray.hit_capsule(&Capsule { a: pa, b: pb, r }).map(Fx::raw),
            Some(20_186_346_293)
        );
        // Ray pointing away misses.
        assert_eq!(
            ray_capsule(Fx3::from_milli(0, 1000, 0), rd.neg(), pa, pb, r),
            None
        );
    }

    #[test]
    fn ray_capsule_parallel_to_axis() {
        // Ray straight down the capsule axis (a = 0) hits the top cap at
        // 10 - 1.5 - 0.3 m. With r = m(300) = 1288490188 raw, r² floors to
        // 386547056 raw and isqrt(386547056 << 32) = 1288490187, so the exact
        // result is 8.5 m - 1288490187 raw (one raw above 8.5 m - r).
        let pa = Fx3::from_milli(0, 300, 0);
        let pb = Fx3::from_milli(0, 1500, 0);
        let t = ray_capsule(
            Fx3::from_ints(0, 10, 0),
            Fx3::from_ints(0, -1, 0),
            pa,
            pb,
            m(300),
        );
        assert_eq!(t.map(Fx::raw), Some(35_218_731_829));
        assert_eq!(
            t,
            Some(Fx::from_int(10) - m(1500) - m(300) + Fx::from_raw(1))
        );
    }

    #[test]
    fn ray_sphere_examples() {
        let t = ray_sphere(
            Fx3::ZERO,
            Fx3::from_ints(0, 0, -1),
            Fx3::from_ints(0, 0, -10),
            Fx::from_int(2),
        );
        assert_eq!(t, Some(Fx::from_int(8)));
        // Tangent counts as a miss (h <= 0).
        let t = ray_sphere(
            Fx3::from_ints(2, 0, 0),
            Fx3::from_ints(0, 0, -1),
            Fx3::from_ints(0, 0, -10),
            Fx::from_int(2),
        );
        assert_eq!(t, None);
        // Behind the origin.
        let t = ray_sphere(
            Fx3::ZERO,
            Fx3::from_ints(0, 0, 1),
            Fx3::from_ints(0, 0, -10),
            Fx::from_int(2),
        );
        assert_eq!(t, None);
        let ray = Ray::new(Fx3::ZERO, Fx3::from_ints(1, 0, 0));
        assert_eq!(
            ray.hit_sphere(Fx3::from_ints(5, 0, 0), Fx::ONE),
            Some(Fx::from_int(4))
        );
        assert_eq!(ray.at(Fx::from_int(3)), Fx3::from_ints(3, 0, 0));
        assert_eq!(min_nonneg(Some(Fx::ONE), Some(Fx::ZERO)), Some(Fx::ZERO));
        assert_eq!(min_nonneg(None, Some(Fx::ONE)), Some(Fx::ONE));
        assert_eq!(min_nonneg(Some(Fx::ONE), None), Some(Fx::ONE));
        assert_eq!(min_nonneg(None, None), None);
        assert_eq!(nonneg(Fx::from_raw(-1)), None);
        assert_eq!(clamp01(Fx::from_int(3)), Fx::ONE);
    }

    #[test]
    fn dda_worked_example() {
        // Doc 02 §4.6: origin (0.5, 1.5, 0.5), direction (0.6, 0, -0.8), t_max 4 m,
        // block (2, 1, -3) solid.
        let o = Fx3::from_milli(500, 1500, 500);
        let d = Fx3::from_milli(600, 0, -800);
        let solid = IVec3::new(2, 1, -3);
        let mut seen = Vec::new();
        for s in VoxelDda::new(o, d, Fx::from_int(4), 64) {
            seen.push(s);
            if s.cell == solid {
                break;
            }
        }
        let cells: Vec<IVec3> = seen.iter().map(|s| s.cell).collect();
        assert_eq!(
            cells,
            [
                IVec3::new(0, 1, 0),
                IVec3::new(0, 1, -1),
                IVec3::new(1, 1, -1),
                IVec3::new(1, 1, -2),
                IVec3::new(2, 1, -2),
                IVec3::new(2, 1, -3),
            ]
        );
        assert_eq!(seen[0].t, Fx::ZERO);
        assert_eq!(seen[0].face, None);
        assert_eq!(seen[1].t.raw(), 2_684_354_559);
        assert_eq!(seen[1].face, Some(Face::PosZ));
        assert_eq!(seen[2].t.to_milli_round(), 833);
        assert_eq!(seen[2].face, Some(Face::NegX));
        assert_eq!(seen[3].t.to_milli_round(), 1875);
        assert_eq!(seen[3].face, Some(Face::PosZ));
        assert_eq!(seen[4].t.to_milli_round(), 2500);
        assert_eq!(seen[4].face, Some(Face::NegX));
        assert_eq!(seen[5].t.raw(), 13_421_772_797);
        assert_eq!(seen[5].face, Some(Face::PosZ));
    }

    #[test]
    fn dda_bounds() {
        let o = Fx3::from_milli(500, 500, 500);
        // Stops at t_max: along +X with t_max 2.5 m, crossings at 0.5, 1.5, 2.5 (inclusive).
        let v: Vec<DdaStep> = VoxelDda::new(o, Fx3::from_ints(1, 0, 0), m(2500), 100).collect();
        assert_eq!(v.len(), 4);
        assert_eq!(v[3].cell, IVec3::new(3, 0, 0));
        assert_eq!(v[3].t, m(2500));
        // max_steps bounds the number of steps beyond the start block.
        let v: Vec<DdaStep> = VoxelDda::new(o, Fx3::from_ints(1, 0, 0), Fx::MAX, 5).collect();
        assert_eq!(v.len(), 6);
        assert_eq!(v[5].cell, IVec3::new(5, 0, 0));
        let v: Vec<DdaStep> = VoxelDda::new(o, Fx3::from_ints(1, 0, 0), Fx::MAX, 0).collect();
        assert_eq!(v.len(), 1);
        // A zero direction yields only the start block, even with t_max = MAX.
        let v: Vec<DdaStep> = VoxelDda::new(o, Fx3::ZERO, Fx::MAX, 100).collect();
        assert_eq!(v.len(), 1);
        // Tiny components count as zero.
        let tiny = Fx3::new(Fx::from_raw(255), Fx::from_raw(-255), Fx::ONE);
        let v: Vec<DdaStep> = VoxelDda::new(o, tiny, Fx::from_int(3), 100).collect();
        assert!(v.iter().all(|s| s.cell.x == 0 && s.cell.y == 0));
        assert_eq!(v.len(), 4);
        // Negative directions and negative coordinates.
        let v: Vec<DdaStep> = Ray::new(Fx3::from_milli(-500, -500, -500), Fx3::from_ints(0, -1, 0))
            .voxels(Fx::from_int(2), 10)
            .collect();
        assert_eq!(v.iter().map(|s| s.cell.y).collect::<Vec<_>>(), [-1, -2, -3]);
        assert_eq!(v[1].face, Some(Face::PosY));
        assert_eq!(v[1].t, Fx::HALF);
    }

    #[test]
    fn dda_tie_order_x_then_y_then_z() {
        // Exact diagonal from a block center: every crossing is a three-way tie.
        let o = Fx3::from_milli(500, 500, 500);
        let v: Vec<DdaStep> =
            VoxelDda::new(o, Fx3::from_ints(1, 1, 1), Fx::from_int(1), 10).collect();
        let cells: Vec<IVec3> = v.iter().map(|s| s.cell).collect();
        assert_eq!(
            cells,
            [
                IVec3::new(0, 0, 0),
                IVec3::new(1, 0, 0),
                IVec3::new(1, 1, 0),
                IVec3::new(1, 1, 1)
            ]
        );
        assert!(v[1..].iter().all(|s| s.t == Fx::HALF));
    }

    #[test]
    fn dda_at_i32_edge_terminates() {
        let o = Fx3::new(Fx::from_raw(i64::MAX - (1 << 31)), Fx::ZERO, Fx::ZERO);
        let v: Vec<DdaStep> = VoxelDda::new(o, Fx3::from_ints(1, 0, 0), Fx::MAX, 10).collect();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].cell.x, i32::MAX);
    }

    #[test]
    fn dda_callback_matches_iterator() {
        let o = Fx3::from_milli(500, 1500, 500);
        let d = Fx3::from_milli(600, 0, -800);
        let solid = IVec3::new(2, 1, -3);
        let mut seen = Vec::new();
        dda(o, d, Fx::from_int(4), |cell, t, face| {
            seen.push(DdaStep { cell, t, face });
            cell != solid
        });
        let mut want = Vec::new();
        for s in VoxelDda::new(o, d, Fx::from_int(4), 1000) {
            want.push(s);
            if s.cell == solid {
                break;
            }
        }
        assert_eq!(seen, want);
        assert_eq!(seen.len(), 6);
        // Without a solid block the traversal runs to t_max.
        let mut n = 0;
        dda(o, d, Fx::from_int(4), |_, t, _| {
            assert!(t <= Fx::from_int(4));
            n += 1;
            true
        });
        assert_eq!(n, VoxelDda::new(o, d, Fx::from_int(4), 1000).count());
    }

    #[test]
    fn semi_implicit_euler_steps() {
        // Constant velocity 6 m/s along x for 60 ticks: each tick adds mul_fx(6, DT).
        let v = Fx3::from_ints(6, 0, 0);
        let mut p = Fx3::ZERO;
        let mut vv = v;
        for _ in 0..60 {
            (p, vv) = semi_implicit_euler(p, vv, Fx3::ZERO);
        }
        assert_eq!(vv, v);
        assert_eq!(p.x, mul_fx(Fx::from_int(6), Fx::DT).mul_int(60));
        assert_eq!(p.x.raw(), 25_769_803_680); // 60 * floor(6 * 71582788)
        // Velocity is updated before position (semi-implicit).
        let g = Fx3::new(Fx::ZERO, Fx::from_int(-20), Fx::ZERO);
        let (p1, v1) = semi_implicit_euler(Fx3::ZERO, Fx3::ZERO, g);
        assert_eq!(v1.y, mul_fx(Fx::from_int(-20), Fx::DT));
        assert_eq!(p1.y, mul_fx(v1.y, Fx::DT));
        // Jump launch with v0 = sqrt(2 g h) for h = 1.25 m, g = 20 m/s²
        // (doc 02 §4.2): the discrete apex lies just below 1.25 m.
        let mut p = Fx3::ZERO;
        let mut v = Fx3::new(Fx::ZERO, sqrt_fx_for_test(50), Fx::ZERO);
        let mut apex = Fx::ZERO;
        for _ in 0..120 {
            (p, v) = semi_implicit_euler(p, v, g);
            apex = apex.max(p.y);
        }
        assert!(apex > m(1100) && apex < m(1250), "apex {apex:?}");
    }

    fn sqrt_fx_for_test(n: i32) -> Fx {
        crate::fx::sqrt_fx(Fx::from_int(n))
    }

    #[test]
    fn face_helpers() {
        assert_eq!(Face::entered(0, 1), Face::NegX);
        assert_eq!(Face::entered(0, -1), Face::PosX);
        assert_eq!(Face::entered(1, 1), Face::NegY);
        assert_eq!(Face::entered(1, -1), Face::PosY);
        assert_eq!(Face::entered(2, 1), Face::NegZ);
        assert_eq!(Face::entered(2, -1), Face::PosZ);
        assert_eq!(Face::PosY.normal(), IVec3::new(0, 1, 0));
        assert_eq!(Face::NegZ.normal(), IVec3::new(0, 0, -1));
        assert_eq!(<[i32; 3]>::from(IVec3::new(1, 2, 3)), [1, 2, 3]);
    }

    #[test]
    fn wide_division_examples() {
        // Doc 02 §4.6 examples.
        assert_eq!(div128(2 << 64, 4 << 64).raw(), 2_147_483_648);
        assert_eq!(div128(-1, 3).raw(), -1_431_655_766);
        assert_eq!(div128(1, -3).raw(), -1_431_655_766);
        assert_eq!(div256(I256::mul(3 << 64, 1 << 64), I256::mul(4 << 64, 1 << 64)).raw(), 3_221_225_472);
        assert_eq!(div256(I256::mul(3 << 64, 1 << 64), I256::mul(-(4 << 64), 1 << 64)).raw(), -3_221_225_472);
        for (n, d) in [(1i128, 2i128), (1, 3), (2, 3), (5, 7), (123_456_789, 987_654_321)] {
            let q = U256::frac_q32(I256::from(n).unsigned_abs(), I256::from(d).unsigned_abs());
            assert_eq!(i128::from(q), (n << 32) / d);
            assert_eq!(ratio01(I256::from(n), I256::from(d)), div128(n, d));
        }
        assert_eq!(ratio01(I256::from(-5i128), I256::from(3i128)), Fx::ZERO);
        assert_eq!(ratio01(I256::from(5i128), I256::from(3i128)), Fx::ONE);
        assert_eq!(ratio01(I256::from(3i128), I256::from(3i128)), Fx::ONE);
        assert_eq!(ratio01(I256::from(1i128), I256::from(-3i128)), Fx::ZERO);
        assert_eq!(ratio01(I256::from(-1i128), I256::from(-3i128)), div128(1, 3));
    }

    #[test]
    #[should_panic(expected = "invariant: div128 range")]
    fn div128_range_panics() {
        let _ = div128(1 << 64, 1);
    }

    #[test]
    #[should_panic(expected = "n << 32 overflows")]
    fn div128_shift_overflow_panics() {
        let _ = div128(1 << 100, 1 << 100);
    }

    fn small() -> impl Strategy<Value = i64> {
        -(1i64 << 40)..(1i64 << 40)
    }

    fn v3() -> impl Strategy<Value = Fx3> {
        (small(), small(), small())
            .prop_map(|(x, y, z)| Fx3::new(Fx::from_raw(x), Fx::from_raw(y), Fx::from_raw(z)))
    }

    proptest! {
        #[test]
        fn dist2_symmetric_and_nonnegative(p in v3(), q in v3()) {
            let d = dist2(p, q);
            prop_assert!(d >= 0);
            prop_assert_eq!(d, dist2(q, p));
            prop_assert_eq!(d, p.sub(q).dot_wide(p.sub(q)));
            prop_assert_eq!(dist2(p, p), 0);
        }

        #[test]
        fn add_sub_round_trip(p in v3(), q in v3()) {
            prop_assert_eq!(p.add(q).sub(q), p);
            prop_assert_eq!(p.add(q), q.add(p));
        }

        #[test]
        fn dot_symmetric(p in v3(), q in v3()) {
            prop_assert_eq!(p.dot_wide(q), q.dot_wide(p));
        }

        #[test]
        fn length_is_floor_sqrt(p in v3()) {
            let l = i128::from(p.length().raw());
            let w = p.dot_wide(p);
            prop_assert!(l * l <= w && (l + 1) * (l + 1) > w);
        }

        #[test]
        fn closest_params_in_unit_range(p1 in v3(), q1 in v3(), p2 in v3(), q2 in v3()) {
            let (s, t, _, _) = closest_seg_seg(p1, q1, p2, q2);
            prop_assert!(Fx::ZERO <= s && s <= Fx::ONE);
            prop_assert!(Fx::ZERO <= t && t <= Fx::ONE);
        }

        #[test]
        fn dda_cells_are_adjacent_and_times_monotone(
            ox in -100_000i64..100_000, oy in -100_000i64..100_000, oz in -100_000i64..100_000,
            dx in -1000i64..1000, dy in -1000i64..1000, dz in -1000i64..1000,
        ) {
            let o = Fx3::from_milli(ox, oy, oz);
            let d = Fx3::from_milli(dx, dy, dz);
            let t_max = Fx::from_int(20);
            let v: Vec<DdaStep> = VoxelDda::new(o, d, t_max, 200).collect();
            prop_assert_eq!(v[0].cell, o.floor_block());
            for w in v.windows(2) {
                let (a, b) = (w[0], w[1]);
                let manhattan = (a.cell.x - b.cell.x).abs() + (a.cell.y - b.cell.y).abs() + (a.cell.z - b.cell.z).abs();
                prop_assert_eq!(manhattan, 1);
                prop_assert!(a.t <= b.t);
                prop_assert!(b.t <= t_max);
                let face = b.face.unwrap();
                let n = face.normal();
                // The entered face's normal points back toward the previous cell.
                prop_assert_eq!(IVec3::new(b.cell.x + n.x, b.cell.y + n.y, b.cell.z + n.z), a.cell);
            }
        }

        #[test]
        fn dda_deterministic(ox in -10_000i64..10_000, dz in -1000i64..1000, dx in -1000i64..1000) {
            let o = Fx3::from_milli(ox, 0, 0);
            let d = Fx3::from_milli(dx, 0, dz);
            let a: Vec<DdaStep> = VoxelDda::new(o, d, Fx::from_int(10), 50).collect();
            let b: Vec<DdaStep> = VoxelDda::new(o, d, Fx::from_int(10), 50).collect();
            prop_assert_eq!(a, b);
        }
    }
}
