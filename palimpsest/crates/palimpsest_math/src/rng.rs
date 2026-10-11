//! SPEC: canon §3 xoshiro256++, SplitMix64, streams, keyed draws
//!
//! The generator is canon §3.1 verbatim (xoshiro256++ seeded through
//! SplitMix64). Sequential streams (canon §3.2) and keyed draws (canon §3.3)
//! derive seeds from the run seed, which every function here takes explicitly.

use crate::hash::fnv1a64;

/// SPEC: canon §3.1 SplitMix64 finalizer. Note: `mix64(0) == 0`.
#[inline]
#[must_use]
pub const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// SPEC: canon §3.1 SplitMix64 step: advance `state` by the golden gamma and
/// return `mix64(state)`.
#[inline]
pub const fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    mix64(*state)
}

/// SPEC: canon §3.1 xoshiro256++ generator. Its four state words are part of
/// every snapshot (doc 02 §4.7).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// SPEC: canon §3.1 `from_seed64`: the four state words are four
    /// consecutive `splitmix64` outputs starting from `seed`.
    #[must_use]
    pub const fn from_seed64(seed: u64) -> Self {
        let mut sm = seed;
        let a = splitmix64(&mut sm);
        let b = splitmix64(&mut sm);
        let c = splitmix64(&mut sm);
        let d = splitmix64(&mut sm);
        Rng { s: [a, b, c, d] }
    }

    /// SPEC: canon §2.5 and doc 02 §4.7. Rebuild a generator from its saved
    /// state words (the inverse of [`Rng::state`]).
    #[inline]
    #[must_use]
    pub const fn from_state(s: [u64; 4]) -> Self {
        Rng { s }
    }

    /// SPEC: canon §2.5 and doc 02 §4.7. The four state words, for saves and
    /// the state hash.
    #[inline]
    #[must_use]
    pub const fn state(&self) -> [u64; 4] {
        self.s
    }

    /// SPEC: canon §3.1 xoshiro256++ `next_u64`.
    #[inline]
    pub const fn next_u64(&mut self) -> u64 {
        let result = (self.s[0].wrapping_add(self.s[3]))
            .rotate_left(23)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// SPEC: canon §3.1 `range`: uniform integer in `[0, n)` by Lemire's
    /// method. May consume more than one `u64` on rejection. Panics if `n == 0`.
    #[track_caller]
    pub fn range(&mut self, n: u64) -> u64 {
        assert!(n > 0, "Rng::range: n must be > 0");
        let mut m = u128::from(self.next_u64()) * u128::from(n);
        let mut l = m as u64; // low 64 bits (intentional truncation)
        if l < n {
            let t = n.wrapping_neg() % n;
            while l < t {
                m = u128::from(self.next_u64()) * u128::from(n);
                l = m as u64;
            }
        }
        (m >> 64) as u64 // high 64 bits, always < n
    }

    /// SPEC: canon §3.1 `range_incl`: uniform integer in `lo..=hi`, one
    /// `range(hi - lo + 1)` call. Panics unless `lo <= hi` and
    /// `hi - lo < u64::MAX` (the canon's assertion).
    ///
    /// The span and the sum are computed in `i128`, so results equal the
    /// canon reference wherever it does not overflow `i64` (`hi - lo + 1 <=
    /// i64::MAX`), and are also correct for wider spans.
    #[track_caller]
    pub fn range_incl(&mut self, lo: i64, hi: i64) -> i64 {
        assert!(
            lo <= hi && (i128::from(hi) - i128::from(lo)) < i128::from(u64::MAX),
            "Rng::range_incl: need lo <= hi and hi - lo < u64::MAX"
        );
        let span = u64::try_from(i128::from(hi) - i128::from(lo) + 1).expect("span fits u64");
        let v = i128::from(lo) + i128::from(self.range(span));
        i64::try_from(v).expect("range_incl: result within lo..=hi")
    }

    /// SPEC: canon §3.1 `roll_bp`: calls `range(10000)` exactly once
    /// regardless of `chance_bp`; true when the draw is below `chance_bp`.
    #[inline]
    #[track_caller]
    pub fn roll_bp(&mut self, chance_bp: i32) -> bool {
        // range(10000) < 10000 always fits i32.
        (self.range(10_000) as i32) < chance_bp
    }

    /// SPEC: canon §3.1 `weighted`: index chosen with probability proportional
    /// to `weights`; calls `range` exactly once. Panics if the total is 0.
    #[track_caller]
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let total: u64 = weights.iter().map(|&w| u64::from(w)).sum();
        assert!(total > 0, "Rng::weighted: total weight must be > 0");
        let mut r = self.range(total);
        for (i, &w) in weights.iter().enumerate() {
            if r < u64::from(w) {
                return i;
            }
            r -= u64::from(w);
        }
        unreachable!("Rng::weighted: r < total")
    }

    /// SPEC: canon §3.1 `shuffle`: Fisher-Yates from the last index down; for
    /// `i` from `len-1` down to 1, swap `v[i]` with `v[range(i+1)]`.
    #[track_caller]
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.range((i + 1) as u64) as usize; // j <= i < len
            v.swap(i, j);
        }
    }

    /// SPEC: canon §3.1 `bell3`: sum of three uniform draws in `[0, n)`, drawn
    /// left to right. Panics if `n == 0` or the sum overflows.
    #[track_caller]
    pub fn bell3(&mut self, n: u64) -> u64 {
        self.range(n) + self.range(n) + self.range(n)
    }
}

/// SPEC: canon §3.2 `stream_seed(run_seed, name, folio_index) =
/// mix64(run_seed ^ mix64(fnv1a64(name) ^ mix64(folio_index as u64)))`.
/// Run-scoped streams use `folio_index = 0`; Folio indices start at 1.
#[inline]
#[must_use]
pub const fn stream_seed(run_seed: u64, name: &str, folio_index: u32) -> u64 {
    mix64(run_seed ^ mix64(fnv1a64(name.as_bytes()) ^ mix64(folio_index as u64)))
}

/// SPEC: canon §3.2. A sequential stream generator:
/// `Rng::from_seed64(stream_seed(run_seed, name, folio_index))`.
#[inline]
#[must_use]
pub const fn stream_rng(run_seed: u64, name: &str, folio_index: u32) -> Rng {
    Rng::from_seed64(stream_seed(run_seed, name, folio_index))
}

/// SPEC: canon §3.3. The seed of a keyed generator:
/// `mix64(stream_seed(run_seed, stream, folio as u64) ^ mix64(actor ^ mix64(tick ^ ((seq as u64) << 48))))`.
#[inline]
#[must_use]
pub const fn keyed_seed(
    run_seed: u64,
    stream: &str,
    folio: u32,
    actor: u64,
    tick: u64,
    seq: u32,
) -> u64 {
    mix64(stream_seed(run_seed, stream, folio) ^ mix64(actor ^ mix64(tick ^ ((seq as u64) << 48))))
}

/// SPEC: canon §3.3 `keyed_rng(stream, folio, actor, tick, seq)`, with
/// `run_seed` as an explicit first parameter (doc 02 §4.7).
///
/// `actor` is the acting entity's `EntityId` (or the killed entity's id or
/// container uid for loot), `tick` the input tick of the action, `seq` a
/// per-actor action counter that resets to 0 each tick (or the receiving
/// player's slot for loot). The result depends only on these arguments, never
/// on how many other draws happened.
#[inline]
#[must_use]
pub const fn keyed_rng(
    run_seed: u64,
    stream: &str,
    folio: u32,
    actor: u64,
    tick: u64,
    seq: u32,
) -> Rng {
    Rng::from_seed64(keyed_seed(run_seed, stream, folio, actor, tick, seq))
}

/// SPEC: canon §3.2, §3.3, §3.5 and §3.7 stream names.
pub mod streams {
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: genre candidates at a Folio exit.
    pub const LATTICE: &str = "rng.lattice";
    /// SPEC: canon §3.2 sequential, run scope, owner doc 10: run-start appearance shuffle.
    pub const APPEARANCE: &str = "rng.appearance";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: layout, rooms, blocks, dressing.
    pub const WORLDGEN: &str = "rng.worldgen";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 13: initial population and waves.
    pub const SPAWN: &str = "rng.spawn";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 11: placed containers, shops, floor loot.
    pub const LOOT_PLACE: &str = "rng.loot_place";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: ambient and narrative events.
    pub const EVENT: &str = "rng.event";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 13: idle and wander behavior.
    pub const AI_IDLE: &str = "rng.ai_idle";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: fire ignition and spread.
    pub const FIRE: &str = "rng.fire";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: random block ticks.
    pub const BLOCKTICK: &str = "rng.blocktick";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: debris and explosion variation.
    pub const PHYSICS: &str = "rng.physics";

    /// SPEC: canon §3.3 keyed, owner doc 05: crits, spread, recoil, Contradiction failure, procs, Death's Door.
    pub const COMBAT: &str = "rng.combat";
    /// SPEC: canon §3.3 keyed, owner doc 05: declared "X Bp chance to apply" effects and confusion.
    pub const STATUS: &str = "rng.status";
    /// SPEC: canon §3.3 keyed, owner doc 06: wild surges, spell variance, cast interruption.
    pub const MAGIC: &str = "rng.magic";
    /// SPEC: canon §3.3 keyed, owner doc 09: detection, pickpocket, lockpicking, traps, secrets.
    pub const STEALTH: &str = "rng.stealth";
    /// SPEC: canon §3.3 keyed, owner doc 10: appraisal error, curse sensing.
    pub const IDENT: &str = "rng.ident";
    /// SPEC: canon §3.3 keyed, owner doc 07: crafting quality.
    pub const CRAFT: &str = "rng.craft";
    /// SPEC: canon §3.3 keyed, owner doc 08: brewing variance, unknown liquids.
    pub const ALCHEMY: &str = "rng.alchemy";
    /// SPEC: canon §3.3 keyed, owner doc 11: kill and container drops, Reliquary, Fortune Engine reels.
    pub const LOOT: &str = "rng.loot";
    /// SPEC: canon §3.3 keyed, owner doc 13: utility AI tie-breaks.
    pub const AI: &str = "rng.ai";
    /// SPEC: canon §3.3 keyed, owner doc 04: talent offers and level-up choices.
    pub const PROGRESS: &str = "rng.progress";

    /// SPEC: canon §3.2. Every sequential stream name, in canon table order.
    pub const SEQUENTIAL: [&str; 10] = [
        LATTICE, APPEARANCE, WORLDGEN, SPAWN, LOOT_PLACE, EVENT, AI_IDLE, FIRE, BLOCKTICK, PHYSICS,
    ];
    /// SPEC: canon §3.3. Every keyed stream name, in canon table order.
    pub const KEYED: [&str; 10] = [
        COMBAT, STATUS, MAGIC, STEALTH, IDENT, CRAFT, ALCHEMY, LOOT, AI, PROGRESS,
    ];

    /// SPEC: canon §3.5. The run-scoped commit-only offer stream of party
    /// slot `slot` (`rng.offer.p0` to `rng.offer.p3`). Panics for `slot > 3`.
    #[must_use]
    #[track_caller]
    pub const fn offer(slot: u8) -> &'static str {
        match slot {
            0 => "rng.offer.p0",
            1 => "rng.offer.p1",
            2 => "rng.offer.p2",
            3 => "rng.offer.p3",
            _ => panic!("streams::offer: party slot must be 0..=3"),
        }
    }

    /// SPEC: canon §3.7. `true` when `name` is a mod stream
    /// `rng.mod.<mod_id>.<name>` (two non-empty segments after `rng.mod.`).
    /// Does not validate the content-ID grammar of the segments.
    #[must_use]
    pub fn is_mod_stream(name: &str) -> bool {
        match name.strip_prefix("rng.mod.") {
            Some(rest) => match rest.split_once('.') {
                Some((m, n)) => !m.is_empty() && !n.is_empty(),
                None => false,
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;
    use super::*;
    use proptest::prelude::*;

    const RUN: u64 = 0x0123_4567_89AB_CDEF;

    #[test]
    fn canon_t01_xoshiro_vectors() {
        // Canon §21 test 1.
        let mut r = Rng::from_seed64(0);
        assert_eq!(r.next_u64(), 0x5317_5d61_490b_23df);
        assert_eq!(r.next_u64(), 0x61da_6f3d_c380_d507);
        assert_eq!(r.next_u64(), 0x5c0f_df91_ec9a_7bfc);
        assert_eq!(r.next_u64(), 0x02ee_bf8c_3bbe_5e1a);
    }

    #[test]
    fn canon_t02_second_seed() {
        // Canon §21 test 2.
        let mut r = Rng::from_seed64(0x0123_4567_89AB_CDEF);
        assert_eq!(r.next_u64(), 0xb2f2_a310_e96b_d1c5);
        assert_eq!(r.next_u64(), 0xb540_6246_5b95_0493);
        assert_eq!(r.next_u64(), 0x87ac_a4a9_6688_14b0);
        assert_eq!(r.next_u64(), 0xf13d_2e24_48a9_cffb);
    }

    #[test]
    fn canon_t03_lemire_range() {
        // Canon §21 test 3.
        let mut r = Rng::from_seed64(42);
        let v: [u64; 5] = core::array::from_fn(|_| r.range(10_000));
        assert_eq!(v, [8143, 3188, 9838, 7011, 7935]);
    }

    #[test]
    fn canon_t05_mix64() {
        // Canon §21 test 5.
        assert_eq!(mix64(0), 0);
        assert_eq!(mix64(1), 0x5692_161d_100b_05e5);
    }

    #[test]
    fn canon_t06_stream_seed() {
        // Canon §21 test 6.
        assert_eq!(stream_seed(RUN, "rng.loot", 3), 0x3050_a071_1590_9f1c);
        let mut r = stream_rng(RUN, streams::LOOT, 3);
        assert_eq!(r.next_u64(), 0x5bdc_091e_897b_818b);
        assert_eq!(r.next_u64(), 0x53a8_da29_d781_85d1);
        assert_eq!(r.next_u64(), 0x1fc0_2fe2_ca3d_6c88);
        assert_eq!(stream_seed(RUN, "rng.worldgen", 1), 0x276f_33cc_0100_c222);
    }

    #[test]
    fn canon_t07_keyed_draw() {
        // Canon §21 test 7.
        let actor = (3u64 << 40) | 5;
        assert_eq!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0),
            0x4dcd_b61b_ac36_2a8f
        );
        let mut r = keyed_rng(RUN, streams::COMBAT, 3, actor, 1000, 0);
        assert_eq!(r, Rng::from_seed64(0x4dcd_b61b_ac36_2a8f));
        assert_eq!(r.range(10_000), 8822);
        assert_eq!(r.range(10_000), 5948);
        assert_eq!(r.range(10_000), 1792);
    }

    /// One "entity" drawing its crit roll from `rng.combat` on tick 1000.
    fn crit_roll(actor: u64, seq: u32) -> u64 {
        keyed_rng(RUN, streams::COMBAT, 3, actor, 1000, seq).range(10_000)
    }

    #[test]
    fn canon_t08_keyed_isolation() {
        // Canon §21 test 8: the crit roll for (actor (3 << 40) | 5, tick 1000,
        // seq 0) is identical whether 0 or 50 other entities drew from
        // rng.combat earlier in the tick.
        let actor = (3u64 << 40) | 5;
        let alone = crit_roll(actor, 0);
        let mut others = [0u64; 50];
        for (i, o) in others.iter_mut().enumerate() {
            let other_actor = (3u64 << 40) | (100 + i as u64);
            // Each other entity draws several values first.
            let mut g = keyed_rng(RUN, streams::COMBAT, 3, other_actor, 1000, 0);
            for _ in 0..(i % 7) {
                let _ = g.next_u64();
            }
            *o = g.range(10_000);
        }
        let after_50 = crit_roll(actor, 0);
        assert_eq!(alone, after_50);
        assert_eq!(alone, 8822);
        // Different keys do give different generators.
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 1),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1001, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.loot", 3, actor, 1000, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 4, actor, 1000, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
    }

    #[test]
    fn splitmix_sequence() {
        let mut s = 0u64;
        assert_eq!(splitmix64(&mut s), mix64(0x9E37_79B9_7F4A_7C15));
        assert_eq!(s, 0x9E37_79B9_7F4A_7C15);
        // Published SplitMix64 vector for seed 0: first output 0xe220a8397b1dcdaf.
        let mut s = 0u64;
        assert_eq!(splitmix64(&mut s), 0xe220_a839_7b1d_cdaf);
    }

    #[test]
    fn state_round_trip() {
        let mut a = Rng::from_seed64(7);
        let _ = a.next_u64();
        let mut b = Rng::from_state(a.state());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a, b);
    }

    #[test]
    fn range_edge_cases() {
        let mut r = Rng::from_seed64(1);
        for _ in 0..100 {
            assert_eq!(r.range(1), 0);
        }
        let mut a = Rng::from_seed64(9);
        let mut b = Rng::from_seed64(9);
        // n = 2^64 - 1 is valid; the result is the high word of x * n.
        let x = b.next_u64();
        let expect = ((u128::from(x) * u128::from(u64::MAX)) >> 64) as u64;
        let l = (u128::from(x) * u128::from(u64::MAX)) as u64;
        if l >= 1 {
            assert_eq!(a.range(u64::MAX), expect);
        }
        // Power of two: no rejection ever (t = 0).
        let mut c = Rng::from_seed64(11);
        let mut d = Rng::from_seed64(11);
        assert_eq!(c.range(1 << 32), d.next_u64() >> 32);
    }

    #[test]
    #[should_panic(expected = "n must be > 0")]
    fn range_zero_panics() {
        let _ = Rng::from_seed64(0).range(0);
    }

    #[test]
    fn range_incl_examples() {
        let mut r = Rng::from_seed64(42);
        let mut s = Rng::from_seed64(42);
        assert_eq!(r.range_incl(-5, 5), -5 + s.range(11) as i64);
        assert_eq!(r.range_incl(3, 3), 3);
        // Spans wider than i64::MAX are handled (the canon reference would overflow there).
        let mut a = Rng::from_seed64(5);
        let mut b = Rng::from_seed64(5);
        let v = a.range_incl(i64::MIN, i64::MAX - 1);
        let expect = i128::from(i64::MIN) + i128::from(b.range(u64::MAX));
        assert_eq!(i128::from(v), expect);
    }

    #[test]
    #[should_panic(expected = "need lo <= hi")]
    fn range_incl_full_span_panics() {
        let _ = Rng::from_seed64(0).range_incl(i64::MIN, i64::MAX);
    }

    #[test]
    #[should_panic(expected = "need lo <= hi")]
    fn range_incl_inverted_panics() {
        let _ = Rng::from_seed64(0).range_incl(1, 0);
    }

    #[test]
    fn roll_bp_draws_once() {
        for chance in [-5, 0, 1, 5000, 9999, 10_000, 20_000] {
            let mut a = Rng::from_seed64(42);
            let mut b = Rng::from_seed64(42);
            let hit = a.roll_bp(chance);
            let draw = b.range(10_000) as i32;
            assert_eq!(hit, draw < chance);
            assert_eq!(a, b, "roll_bp must call range(10000) exactly once");
        }
        // Seed 42's first range(10000) is 8143 (canon §21 test 3).
        assert!(Rng::from_seed64(42).roll_bp(8144));
        assert!(!Rng::from_seed64(42).roll_bp(8143));
        assert!(!Rng::from_seed64(42).roll_bp(0));
        assert!(Rng::from_seed64(42).roll_bp(10_000));
    }

    #[test]
    fn weighted_examples() {
        // Seed 42 first range(10000) = 8143: with weights [7000, 2000, 1000] -> index 1.
        let mut r = Rng::from_seed64(42);
        assert_eq!(r.weighted(&[7000, 2000, 1000]), 1);
        // Zero weights are never chosen.
        let mut r = Rng::from_seed64(3);
        for _ in 0..1000 {
            assert_eq!(r.weighted(&[0, 5, 0]), 1);
        }
        // Draws exactly once.
        let mut a = Rng::from_seed64(8);
        let mut b = Rng::from_seed64(8);
        let _ = a.weighted(&[1, 2, 3]);
        let _ = b.range(6);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic(expected = "total weight must be > 0")]
    fn weighted_zero_total_panics() {
        let _ = Rng::from_seed64(0).weighted(&[0, 0]);
    }

    #[test]
    fn shuffle_reference() {
        // Reference: Fisher-Yates from the last index down.
        let mut a = Rng::from_seed64(42);
        let mut b = Rng::from_seed64(42);
        let mut v = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        a.shuffle(&mut v);
        let mut w = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in (1..w.len()).rev() {
            let j = b.range(i as u64 + 1) as usize;
            w.swap(i, j);
        }
        assert_eq!(v, w);
        let mut sorted = v;
        sorted.sort_unstable();
        assert_eq!(sorted, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // Empty and single-element slices draw nothing.
        let mut c = Rng::from_seed64(1);
        let before = c.clone();
        c.shuffle::<u8>(&mut []);
        c.shuffle(&mut [1u8]);
        assert_eq!(c, before);
    }

    #[test]
    fn bell3_reference() {
        let mut a = Rng::from_seed64(42);
        // 8143 + 3188 + 9838 from canon §21 test 3's draws.
        assert_eq!(a.bell3(10_000), 8143 + 3188 + 9838);
    }

    #[test]
    fn stream_names() {
        assert_eq!(streams::offer(2), "rng.offer.p2");
        assert_eq!(streams::SEQUENTIAL.len(), 10);
        assert_eq!(streams::KEYED.len(), 10);
        assert!(streams::is_mod_stream("rng.mod.mymod.sparks"));
        assert!(!streams::is_mod_stream("rng.mod.mymod"));
        assert!(!streams::is_mod_stream("rng.mod..x"));
        assert!(!streams::is_mod_stream("rng.combat"));
        // Folio streams for different Folios are distinct; run-scoped uses folio 0.
        assert_ne!(
            stream_seed(RUN, streams::WORLDGEN, 1),
            stream_seed(RUN, streams::WORLDGEN, 2)
        );
        assert_ne!(
            stream_seed(RUN, streams::APPEARANCE, 0),
            stream_seed(RUN, streams::WORLDGEN, 0)
        );
        // Every stream name is distinct under fnv1a64.
        let mut all: std::vec::Vec<u64> = streams::SEQUENTIAL
            .iter()
            .chain(streams::KEYED.iter())
            .map(|n| fnv1a64(n.as_bytes()))
            .collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 20);
    }

    #[test]
    #[should_panic(expected = "party slot must be 0..=3")]
    fn offer_slot_out_of_range_panics() {
        let _ = streams::offer(4);
    }

    proptest! {
        #[test]
        fn range_in_bounds(seed in any::<u64>(), n in 1u64..=u64::MAX) {
            let mut r = Rng::from_seed64(seed);
            prop_assert!(r.range(n) < n);
        }

        #[test]
        fn range_incl_in_bounds(seed in any::<u64>(), lo in any::<i64>(), w in 0u64..u64::MAX) {
            let hi = i64::try_from((i128::from(lo) + i128::from(w)).min(i128::from(i64::MAX))).unwrap();
            prop_assume!((i128::from(hi) - i128::from(lo)) < i128::from(u64::MAX));
            let mut r = Rng::from_seed64(seed);
            let v = r.range_incl(lo, hi);
            prop_assert!(lo <= v && v <= hi);
        }

        #[test]
        fn range_incl_matches_canon_reference(seed in any::<u64>(), lo in -(1i64 << 40)..(1i64 << 40), w in 0i64..(1i64 << 40)) {
            // Canon §3.1 reference body, valid where it cannot overflow.
            let hi = lo + w;
            let mut a = Rng::from_seed64(seed);
            let mut b = Rng::from_seed64(seed);
            prop_assert_eq!(a.range_incl(lo, hi), lo + b.range((hi - lo + 1) as u64) as i64);
        }

        #[test]
        fn determinism(seed in any::<u64>()) {
            let mut a = Rng::from_seed64(seed);
            let mut b = Rng::from_seed64(seed);
            for _ in 0..16 {
                prop_assert_eq!(a.next_u64(), b.next_u64());
            }
        }

        #[test]
        fn weighted_in_bounds_and_nonzero(seed in any::<u64>(), w in proptest::collection::vec(0u32..1000, 1..16)) {
            prop_assume!(w.iter().any(|&x| x > 0));
            let mut r = Rng::from_seed64(seed);
            let i = r.weighted(&w);
            prop_assert!(i < w.len());
            prop_assert!(w[i] > 0);
        }

        #[test]
        fn shuffle_is_permutation(seed in any::<u64>(), mut v in proptest::collection::vec(any::<u16>(), 0..64)) {
            let mut sorted = v.clone();
            sorted.sort_unstable();
            Rng::from_seed64(seed).shuffle(&mut v);
            v.sort_unstable();
            prop_assert_eq!(v, sorted);
        }

        #[test]
        fn bell3_in_bounds(seed in any::<u64>(), n in 1u64..(1u64 << 60)) {
            let mut r = Rng::from_seed64(seed);
            prop_assert!(r.bell3(n) <= 3 * (n - 1));
        }

        #[test]
        fn keyed_isolation_property(
            run in any::<u64>(), folio in 1u32..30, actor in any::<u64>(), tick in any::<u64>(), seq in 0u32..8,
            noise in proptest::collection::vec((any::<u64>(), 0u32..8, 0usize..5), 0..50),
        ) {
            let first = keyed_rng(run, streams::COMBAT, folio, actor, tick, seq).range(10_000);
            for (other, s, k) in noise {
                let mut g = keyed_rng(run, streams::COMBAT, folio, other, tick, s);
                for _ in 0..k {
                    let _ = g.next_u64();
                }
            }
            let again = keyed_rng(run, streams::COMBAT, folio, actor, tick, seq).range(10_000);
            prop_assert_eq!(first, again);
        }
    }
}
