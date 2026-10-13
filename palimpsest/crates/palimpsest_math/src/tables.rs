//! SPEC: canon §13.8 power tables P, T, H, S
//!
//! Every table is `i64` Bp. Rows 1 to 40 of `P`, `T` and `H` are committed
//! literal constants copied from the canon §13.8 table and are never
//! regenerated at runtime. Rows 41 to 300 follow the canon recurrences and are
//! evaluated at compile time into constant arrays. Every lookup uses the power
//! depth cap `d_pow = min(d, 300)`.

use crate::int::mul_bp;

/// SPEC: canon §13.8 power depth cap: every table uses `d_pow = min(d, 300)`.
pub const D_POW_CAP: u32 = 300;

/// SPEC: canon §13.8. Number of committed literal rows (1 to 40).
pub const LITERAL_ROWS: usize = 40;

/// SPEC: canon §13.8 recurrence multiplier of `P` beyond row 40 (Bp).
pub const P_GROWTH_BP: i32 = 10_600;

/// SPEC: canon §13.8 recurrence multiplier of `T` beyond row 40 (Bp).
pub const T_GROWTH_BP: i32 = 10_400;

/// SPEC: canon §13.8 table `P(d)`, rows 1 to 40 (index `d - 1`), committed literals.
pub const P_ROWS: [i64; LITERAL_ROWS] = [
    10000, 10900, 11881, 12950, 14115, 15386, 16771, 18280, 19925, 21718, //  1..=10
    23673, 25804, 28126, 30658, 33417, 36424, 39703, 43276, 47171, 51416, // 11..=20
    56044, 61088, 66586, 72578, 79110, 83856, 88887, 94220, 99873, 105865, // 21..=30
    112216, 118948, 126084, 133649, 141667, 150167, 159177, 168727, 178850, 189581, // 31..=40
];

/// SPEC: canon §13.8 table `T(d)`, rows 1 to 40 (index `d - 1`), committed literals.
pub const T_ROWS: [i64; LITERAL_ROWS] = [
    10000, 10550, 11130, 11742, 12388, 13069, 13788, 14546, 15346, 16190, //  1..=10
    17081, 18020, 19012, 20057, 21160, 22324, 23552, 24848, 26214, 27656, // 11..=20
    29177, 30782, 32475, 34261, 36145, 37590, 39093, 40656, 42282, 43973, // 21..=30
    45731, 47560, 49462, 51440, 53497, 55636, 57861, 60175, 62582, 65085, // 31..=40
];

/// SPEC: canon §13.8 table `H(d)`, rows 1 to 40 (index `d - 1`), committed literals.
pub const H_ROWS: [i64; LITERAL_ROWS] = [
    10000, 11608, 13425, 15475, 17784, 20386, 23311, 26597, 30286, 34423, //  1..=10
    39060, 44253, 50064, 56564, 63826, 71937, 80994, 91095, 102361, 114914, // 11..=20
    128901, 144473, 161803, 181082, 202521, 216767, 231995, 248269, 265662, 284247, // 21..=30
    304105, 325322, 347991, 372212, 398084, 425723, 455246, 486777, 520453, 556420, // 31..=40
];

/// Length of the full tables: index `d` for `d` in `0..=300` (index 0 unused, 0).
const FULL: usize = D_POW_CAP as usize + 1;

/// Rows 1..=40 from `lit`, then `row(d) = mul_bp(row(d-1), growth)` up to 300.
const fn build_table(lit: &[i64; LITERAL_ROWS], growth: i32) -> [i64; FULL] {
    let mut out = [0i64; FULL];
    let mut d = 1;
    while d <= LITERAL_ROWS {
        out[d] = lit[d - 1];
        d += 1;
    }
    while d < FULL {
        out[d] = mul_bp(out[d - 1], growth);
        d += 1;
    }
    out
}

/// SPEC: canon §13.8 `H(d)` multiplier in Bp:
/// `10000 + 650 × (min(d, 25) - 1) + 250 × max(0, d - 25)`, for `d >= 1`
/// (callers pass `d_pow`).
#[must_use]
#[track_caller]
pub const fn h_mult_bp(d: u32) -> i32 {
    assert!(d >= 1, "h_mult_bp: d must be >= 1");
    let d = d as i64;
    let lo = if d < 25 { d } else { 25 };
    let hi = if d > 25 { d - 25 } else { 0 };
    let m = 10_000 + 650 * (lo - 1) + 250 * hi;
    assert!(m <= i32::MAX as i64, "h_mult_bp: multiplier out of range");
    m as i32
}

/// SPEC: canon §13.8 `H(d) = mul_bp(P(d), h_mult_bp(d))` evaluated by formula
/// (no literal rows), for `d` in `1..=300`.
const fn h_formula_uncapped(p: &[i64; FULL], d: usize) -> i64 {
    mul_bp(p[d], h_mult_bp(d as u32))
}

const fn build_h(p: &[i64; FULL]) -> [i64; FULL] {
    let mut out = [0i64; FULL];
    let mut d = 1;
    while d <= LITERAL_ROWS {
        out[d] = H_ROWS[d - 1];
        d += 1;
    }
    while d < FULL {
        out[d] = h_formula_uncapped(p, d);
        d += 1;
    }
    out
}

/// SPEC: canon §13.8. `P(d)` for `d` in `0..=300` (index 0 is unused and 0).
pub const P_TABLE: [i64; FULL] = build_table(&P_ROWS, P_GROWTH_BP);
/// SPEC: canon §13.8. `T(d)` for `d` in `0..=300` (index 0 is unused and 0).
pub const T_TABLE: [i64; FULL] = build_table(&T_ROWS, T_GROWTH_BP);
/// SPEC: canon §13.8. `H(d)` for `d` in `0..=300` (index 0 is unused and 0).
pub const H_TABLE: [i64; FULL] = build_h(&P_TABLE);

/// SPEC: canon §13.8 power depth cap `d_pow = min(d, 300)`.
#[inline]
#[must_use]
pub const fn d_pow(d: u32) -> u32 {
    if d < D_POW_CAP { d } else { D_POW_CAP }
}

#[inline]
#[track_caller]
const fn idx(d: u32) -> usize {
    assert!(d >= 1, "power tables: d must be >= 1");
    d_pow(d) as usize
}

/// SPEC: canon §13.8 `P(d)` in Bp: committed literal rows 1 to 40, then
/// `P(d) = mul_bp(P(d-1), 10600)`, with `d_pow = min(d, 300)`. Panics if `d == 0`.
/// Used for Errata weapon and source damage (§10.4), enemy armor rating,
/// Lucre amounts and item values, and the break threshold of terrain
/// destructible blocks at `d_current` (doc 03).
#[inline]
#[must_use]
#[track_caller]
pub const fn power_p(d: u32) -> i64 {
    P_TABLE[idx(d)]
}

/// SPEC: canon §13.8 `T(d)` in Bp: committed literal rows 1 to 40, then
/// `T(d) = mul_bp(T(d-1), 10400)`, with `d_pow = min(d, 300)`. Panics if `d == 0`.
/// Used for enemy damage, Errata gear armor rating, shield block values and
/// item ward pools (§13.2 `slot.ward`), and the HP of Errata-placed blocks at
/// the placing Folio's depth (doc 07).
#[inline]
#[must_use]
#[track_caller]
pub const fn power_t(d: u32) -> i64 {
    T_TABLE[idx(d)]
}

/// SPEC: canon §13.8 `H(d) = mul_bp(P(d), 10000 + 650 × (min(d, 25) - 1) + 250 × max(0, d - 25))`
/// in Bp, with `d_pow = min(d, 300)`. Rows 1 to 40 come from the committed
/// literals (which equal the formula). Panics if `d == 0`. Used for enemy
/// HP: `template_hp × H(d) × rank HP × party-size HP × diff_hp_bp`
/// (`palimpsest_sim::progression::enemy_max_hp_mhp`).
#[inline]
#[must_use]
#[track_caller]
pub const fn power_h(d: u32) -> i64 {
    H_TABLE[idx(d)]
}

/// SPEC: canon §13.8 `H(d)` evaluated by formula from `P` (never from the
/// literal `H` rows), with `d_pow = min(d, 300)`. Equal to [`power_h`].
#[inline]
#[must_use]
#[track_caller]
pub const fn power_h_formula(d: u32) -> i64 {
    h_formula_uncapped(&P_TABLE, idx(d))
}

/// SPEC: canon §13.8 `S(lvl) = 10000 + 300 × (lvl - 1)` in Bp, with the
/// power depth cap applied to `lvl` (`min(lvl, 300)`, "every table uses
/// d_pow"). Panics if `lvl == 0`.
#[inline]
#[must_use]
#[track_caller]
pub const fn power_s(lvl: u32) -> i64 {
    let l = idx(lvl) as i64;
    10_000 + 300 * (l - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Minimal exact unsigned big integer for the generator cross-check
    /// (little-endian base-2^32 limbs).
    #[derive(Clone)]
    struct Big(std::vec::Vec<u32>);

    impl Big {
        fn from_u64(v: u64) -> Big {
            Big(std::vec![(v & 0xFFFF_FFFF) as u32, (v >> 32) as u32])
        }
        fn mul_small(&mut self, m: u32) {
            let mut carry: u64 = 0;
            for limb in self.0.iter_mut() {
                let x = u64::from(*limb) * u64::from(m) + carry;
                *limb = (x & 0xFFFF_FFFF) as u32;
                carry = x >> 32;
            }
            if carry > 0 {
                self.0.push(carry as u32);
            }
        }
        /// Floor division by a small divisor.
        fn div_small(&mut self, d: u32) {
            let mut rem: u64 = 0;
            for limb in self.0.iter_mut().rev() {
                let x = (rem << 32) | u64::from(*limb);
                *limb = (x / u64::from(d)) as u32;
                rem = x % u64::from(d);
            }
        }
        fn to_i64(&self) -> i64 {
            assert!(self.0.iter().skip(2).all(|&l| l == 0), "Big does not fit");
            i64::try_from(u64::from(self.0[0]) | (u64::from(self.0[1]) << 32)).unwrap()
        }
    }

    /// Canon §13.8 rows 1..=25: `floor_div(10000 × g^(d-1), 10000^(d-1))` in
    /// exact integer arithmetic. Dividing by 10000 `d-1` times equals one
    /// division by `10000^(d-1)` because nested floors by positive integers compose.
    fn closed_form(growth: u32, d: u32) -> i64 {
        let mut x = Big::from_u64(10_000);
        for _ in 1..d {
            x.mul_small(growth);
        }
        for _ in 1..d {
            x.div_small(10_000);
        }
        x.to_i64()
    }

    /// Exact generator for rows 1..=n: closed form for 1..=25, recurrence beyond.
    fn generate(growth_closed: u32, growth_rec: i32, n: u32) -> std::vec::Vec<i64> {
        let mut v = std::vec::Vec::new();
        for d in 1..=n {
            let x = if d <= 25 {
                closed_form(growth_closed, d)
            } else {
                mul_bp(v[(d - 2) as usize], growth_rec)
            };
            v.push(x);
        }
        v
    }

    #[test]
    fn canon_t21_power_tables() {
        // Canon §21 test 21.
        assert_eq!(power_p(25), 79_110);
        assert_eq!(power_t(25), 36_145);
        assert_eq!(power_h(25), 202_521);
        assert_eq!(power_s(25), 17_200);
        assert_eq!(power_p(41), 200_955);
        // Every committed row 1 to 40 matches §13.8 (generator in exact integers).
        let p = generate(10_900, P_GROWTH_BP, 40);
        let t = generate(10_550, T_GROWTH_BP, 40);
        for d in 1..=40u32 {
            let i = (d - 1) as usize;
            assert_eq!(P_ROWS[i], p[i], "P({d})");
            assert_eq!(T_ROWS[i], t[i], "T({d})");
            let h = mul_bp(p[i], h_mult_bp(d));
            assert_eq!(H_ROWS[i], h, "H({d})");
            assert_eq!(power_p(d), P_ROWS[i]);
            assert_eq!(power_t(d), T_ROWS[i]);
            assert_eq!(power_h(d), H_ROWS[i]);
            assert_eq!(power_h_formula(d), H_ROWS[i]);
        }
    }

    /// SPEC: canon-1.12 §21 test 50. Second Edition depth (§13.8).
    #[test]
    fn canon_t50_second_edition_depth() {
        // Second Edition Folio n uses depth 25 + 2n (DECISIONS 106).
        let se_depth = |n: u32| 25 + 2 * n;
        assert_eq!(se_depth(1), 27);
        assert_eq!(se_depth(5), 35);
        assert_eq!(se_depth(15), 55);
        assert_eq!(power_p(se_depth(1)), P_ROWS[26]);
        assert_eq!(power_p(se_depth(5)), P_ROWS[34]);
    }

    #[test]
    fn canon_table_spot_rows() {
        // Spot rows copied from the canon §13.8 table.
        assert_eq!(
            (power_p(1), power_t(1), power_h(1)),
            (10_000, 10_000, 10_000)
        );
        assert_eq!(
            (power_p(2), power_t(2), power_h(2)),
            (10_900, 10_550, 11_608)
        );
        assert_eq!(
            (power_p(10), power_t(10), power_h(10)),
            (21_718, 16_190, 34_423)
        );
        assert_eq!(
            (power_p(26), power_t(26), power_h(26)),
            (83_856, 37_590, 216_767)
        );
        assert_eq!(
            (power_p(40), power_t(40), power_h(40)),
            (189_581, 65_085, 556_420)
        );
    }

    #[test]
    fn recurrence_beyond_40() {
        assert_eq!(power_p(41), mul_bp(189_581, 10_600));
        assert_eq!(power_t(41), mul_bp(65_085, 10_400));
        for d in 41..=300u32 {
            assert_eq!(power_p(d), mul_bp(power_p(d - 1), P_GROWTH_BP), "P({d})");
            assert_eq!(power_t(d), mul_bp(power_t(d - 1), T_GROWTH_BP), "T({d})");
            assert_eq!(power_h(d), mul_bp(power_p(d), h_mult_bp(d)), "H({d})");
        }
        // Exact values at the cap, from an independent integer computation.
        assert_eq!(power_p(300), 719_946_187_289);
        assert_eq!(power_t(300), 1_746_060_986);
        assert_eq!(power_h(300), 6_792_692_277_071);
        // Recurrence rows 26..=40 also agree with the literals (canon: rows 26 to 40 follow the recurrence).
        for d in 26..=40u32 {
            assert_eq!(
                P_ROWS[(d - 1) as usize],
                mul_bp(P_ROWS[(d - 2) as usize], 10_600)
            );
            assert_eq!(
                T_ROWS[(d - 1) as usize],
                mul_bp(T_ROWS[(d - 2) as usize], 10_400)
            );
        }
    }

    #[test]
    fn depth_cap() {
        assert_eq!(d_pow(1), 1);
        assert_eq!(d_pow(300), 300);
        assert_eq!(d_pow(301), 300);
        assert_eq!(d_pow(u32::MAX), 300);
        assert_eq!(power_p(301), power_p(300));
        assert_eq!(power_t(1000), power_t(300));
        assert_eq!(power_h(u32::MAX), power_h(300));
        assert_eq!(power_s(300), 10_000 + 300 * 299);
        assert_eq!(power_s(301), power_s(300));
    }

    #[test]
    fn h_multiplier() {
        assert_eq!(h_mult_bp(1), 10_000);
        assert_eq!(h_mult_bp(25), 25_600);
        assert_eq!(h_mult_bp(26), 25_850);
        assert_eq!(h_mult_bp(300), 94_350);
    }

    #[test]
    fn s_table() {
        assert_eq!(power_s(1), 10_000);
        assert_eq!(power_s(2), 10_300);
        assert_eq!(power_s(25), 17_200);
    }

    #[test]
    #[should_panic(expected = "d must be >= 1")]
    fn depth_zero_panics() {
        let _ = power_p(0);
    }

    #[test]
    #[should_panic(expected = "d must be >= 1")]
    fn level_zero_panics() {
        let _ = power_s(0);
    }

    proptest! {
        #[test]
        fn tables_monotone(d in 1u32..300) {
            prop_assert!(power_p(d) < power_p(d + 1));
            prop_assert!(power_t(d) < power_t(d + 1));
            prop_assert!(power_h(d) < power_h(d + 1));
            prop_assert!(power_s(d) < power_s(d + 1));
            // P grows at least as fast as T, and H >= P.
            prop_assert!(power_p(d) >= power_t(d));
            prop_assert!(power_h(d) >= power_p(d));
        }

        #[test]
        fn capped_lookups_never_panic(d in 1u32..=u32::MAX) {
            prop_assert_eq!(power_p(d), power_p(d_pow(d)));
            prop_assert_eq!(power_h(d), power_h_formula(d));
        }
    }
}
