//! SPEC: canon §4.2 jump, fall, dodge, speed
//!
//! Movement constants, direction classification (forward, strafe,
//! backpedal), move-speed composition, jump height and launch speed with the
//! semi-implicit Euler apex simulation of canon §2.1, safe-fall height and
//! fall damage, and dodge timing. Physics quantities are `Fx`; content speeds
//! stay in mm/s and convert with `fx_from_milli`.

use palimpsest_math::geom::{Fx3, semi_implicit_euler};
use palimpsest_math::int::{floor_div, mul_bp, narrow_i32};
use palimpsest_math::{Fx, cos_bam, fx_from_milli, mul_fx, sqrt_fx};

/// SPEC: canon §4.2 walk speed, mm/s.
pub const WALK_MM_S: i64 = 4300;
/// SPEC: canon §4.2 sprint speed, mm/s.
pub const SPRINT_MM_S: i64 = 6500;
/// SPEC: canon §4.2 crouch-walk speed, mm/s.
pub const CROUCH_MM_S: i64 = 2000;
/// SPEC: canon §4.2 swim speed, mm/s.
pub const SWIM_MM_S: i64 = 2400;
/// SPEC: canon §4.2 climb speed (ladders, vines, rope), mm/s.
pub const CLIMB_MM_S: i64 = 2400;
/// SPEC: canon §4.2 backpedal multiplier on walk.
pub const BACKPEDAL_BP: i32 = 8000;
/// SPEC: canon §4.2 strafe multiplier on walk.
pub const STRAFE_BP: i32 = 9500;
/// SPEC: canon §4.2 ground acceleration, mm/s².
pub const GROUND_ACCEL_MM_S2: i64 = 40_000;
/// SPEC: canon §4.2 air acceleration, mm/s².
pub const AIR_ACCEL_MM_S2: i64 = 8000;
/// SPEC: canon §4.2. Backpedal applies above this angle (67.5°).
pub const BACKPEDAL_ANGLE_BAM: u16 = 12_288;
/// SPEC: canon §4.2. Strafe applies above this angle (22.5°).
pub const STRAFE_ANGLE_BAM: u16 = 4096;
/// SPEC: canon §4 `GRAVITY_BASE`, mm/s².
pub const GRAVITY_BASE_MM_S2: i64 = 20_000;
/// SPEC: canon §4 `TERMINAL_VELOCITY`, mm/s.
pub const TERMINAL_VELOCITY_MM_S: i64 = 50_000;
/// SPEC: canon §4.2 lean: the eye shifts 350 mm sideways.
pub const LEAN_OFFSET_MM: i64 = 350;
/// SPEC: canon §4.2 lean: over 9 ticks.
pub const LEAN_TICKS: u32 = 9;

/// SPEC: canon §4.2 movement direction class.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MoveClass {
    /// Angle to facing at most 22.5°.
    Forward,
    /// Above 22.5°, at most 67.5°.
    Strafe,
    /// Above 67.5°.
    Backpedal,
}

impl MoveClass {
    /// SPEC: canon §4.2. Speed multiplier on the gait speed: 10000, 9500 or 8000.
    #[must_use]
    pub const fn bp(self) -> i32 {
        match self {
            MoveClass::Forward => 10_000,
            MoveClass::Strafe => STRAFE_BP,
            MoveClass::Backpedal => BACKPEDAL_BP,
        }
    }
}

/// SPEC: canon §4.2. Class from the angle between input direction and facing
/// (`ang_diff`, `0..=32768`): backpedal above 12288, strafe above 4096.
#[must_use]
pub const fn move_class_from_angle(diff: u16) -> MoveClass {
    if diff > BACKPEDAL_ANGLE_BAM {
        MoveClass::Backpedal
    } else if diff > STRAFE_ANGLE_BAM {
        MoveClass::Strafe
    } else {
        MoveClass::Forward
    }
}

/// True when the angle between `(right, fwd)` and straight ahead exceeds the
/// angle whose Q2.30 cosine is `cos_q30` (`0 < angle < 90°`): exactly
/// `fwd < cos × |v|`, compared on squares in `i128` without rounding `|v|`.
fn angle_exceeds(fwd: i64, right: i64, cos_q30: i32) -> bool {
    if fwd <= 0 {
        return true;
    }
    let c = i128::from(cos_q30);
    (i128::from(fwd * fwd) << 60) < c * c * i128::from(right * right + fwd * fwd)
}

/// SPEC: canon §4.2 class from command axes (`fwd_in = -move_z`,
/// `right_in = move_x`, doc 03 §5.2): backpedal when the input direction is
/// more than 12288 Bam from facing, strafe above 4096 Bam, else forward;
/// `None` without input. The test is exact against `cos_bam` of the
/// threshold (`fwd < cos × |v|` on squares), unlike doc 03's floored `isqrt`
/// form, which the canon overrides (see reported spec issues).
#[must_use]
pub fn move_class_from_input(move_x: i8, move_z: i8) -> Option<MoveClass> {
    let fwd = -i64::from(move_z);
    let right = i64::from(move_x);
    if fwd == 0 && right == 0 {
        return None;
    }
    Some(if angle_exceeds(fwd, right, cos_bam(BACKPEDAL_ANGLE_BAM)) {
        MoveClass::Backpedal
    } else if angle_exceeds(fwd, right, cos_bam(STRAFE_ANGLE_BAM)) {
        MoveClass::Strafe
    } else {
        MoveClass::Forward
    })
}

/// SPEC: canon §4.2 movement modes of the speed table.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Gait {
    /// 4300 mm/s.
    Walk,
    /// 6500 mm/s, forward only.
    Sprint,
    /// 2000 mm/s.
    Crouch,
    /// 2400 mm/s.
    Swim,
    /// 2400 mm/s.
    Climb,
}

/// SPEC: canon §4.2 base speed of a gait and class, mm/s. Backpedal and
/// strafe multiply the walk (or crouch) speed; sprint needs the forward class
/// and falls back to walk otherwise; swim and climb ignore the class.
#[must_use]
pub const fn base_speed_mm_s(gait: Gait, class: MoveClass) -> i64 {
    match gait {
        Gait::Sprint if matches!(class, MoveClass::Forward) => SPRINT_MM_S,
        Gait::Walk | Gait::Sprint => mul_bp(WALK_MM_S, class.bp()),
        Gait::Crouch => mul_bp(CROUCH_MM_S, class.bp()),
        Gait::Swim => SWIM_MM_S,
        Gait::Climb => CLIMB_MM_S,
    }
}

/// SPEC: canon §4.2. One active status move multiplier, keyed by status ID.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusMultiplier<'a> {
    /// Status content ID, for example `st.slowed`.
    pub id: &'a str,
    /// Move speed multiplier in Bp.
    pub bp: i32,
}

/// SPEC: canon §4.2 final move speed: base speed, then §6.4 row 11, then each
/// active status multiplier in ascending status ID byte order, one `mul_bp`
/// each. Panics if `statuses` is not strictly ascending by ID.
#[must_use]
pub fn final_move_speed_mm_s(
    base_mm_s: i64,
    row11_bp: i32,
    statuses: &[StatusMultiplier<'_>],
) -> i64 {
    assert!(
        statuses
            .windows(2)
            .all(|w| w[0].id.as_bytes() < w[1].id.as_bytes()),
        "invariant: status multipliers sorted by ascending status ID"
    );
    statuses
        .iter()
        .fold(mul_bp(base_mm_s, row11_bp), |v, s| mul_bp(v, s.bp))
}

/// SPEC: canon §5.9 `air_accel_bp` multiplies air acceleration (mm/s²).
#[must_use]
pub const fn air_accel_mm_s2(air_accel_bp: i32) -> i64 {
    mul_bp(AIR_ACCEL_MM_S2, air_accel_bp)
}

/// SPEC: canon §5.9 `ground_friction_bp` multiplies ground deceleration (mm/s²).
#[must_use]
pub const fn ground_decel_mm_s2(ground_friction_bp: i32) -> i64 {
    mul_bp(GROUND_ACCEL_MM_S2, ground_friction_bp)
}

// ---------------------------------------------------------------- jump

/// SPEC: canon §4.2 `JUMP_H = 1100 + 30 × min(vrv, 5) + 150 × max(0, vrv - 5)` mm.
#[must_use]
pub const fn jump_height_mm(vrv: u8) -> i64 {
    let v = vrv as i64;
    1100 + 30 * if v < 5 { v } else { 5 } + 150 * if v > 5 { v - 5 } else { 0 }
}

/// SPEC: canon §4.2. Air jumps exist at `vrv(e) >= 9`.
pub const AIR_JUMP_MIN_VRV: u8 = 9;
/// SPEC: canon §4.2. Air jump height is `mul_bp(JUMP_H, 6000)`.
pub const AIR_JUMP_BP: i32 = 6000;

/// SPEC: canon §4.2. Air jump height in mm, `None` below VERVE 9.
#[must_use]
pub const fn air_jump_height_mm(vrv: u8) -> Option<i64> {
    if vrv >= AIR_JUMP_MIN_VRV {
        Some(mul_bp(jump_height_mm(vrv), AIR_JUMP_BP))
    } else {
        None
    }
}

/// SPEC: canon §4, §5.2. Local gravity `mul_bp(GRAVITY_BASE, gravity_bp)` as `Fx` m/s².
#[must_use]
pub const fn local_gravity(gravity_bp: i32) -> Fx {
    fx_from_milli(mul_bp(GRAVITY_BASE_MM_S2, gravity_bp))
}

/// SPEC: canon §4.2 launch speed `v0 = sqrt_fx(2 × g × JUMP_H) + g × DT / 2`,
/// evaluated left to right: `mul_fx(2g, h)`, then `mul_fx(g, DT)` floored by 2.
#[must_use]
pub fn launch_speed(g: Fx, jump_h: Fx) -> Fx {
    sqrt_fx(mul_fx(g.mul_int(2), jump_h)) + mul_fx(g, Fx::DT).div_int(2)
}

/// SPEC: canon §4.2 ground jump launch speed at `vrv` under `gravity_bp`.
#[must_use]
pub fn jump_launch_speed(vrv: u8, gravity_bp: i32) -> Fx {
    launch_speed(
        local_gravity(gravity_bp),
        fx_from_milli(jump_height_mm(vrv)),
    )
}

/// SPEC: canon §4.2 air jump launch speed (`None` below VERVE 9).
#[must_use]
pub fn air_jump_launch_speed(vrv: u8, gravity_bp: i32) -> Option<Fx> {
    air_jump_height_mm(vrv).map(|h| launch_speed(local_gravity(gravity_bp), fx_from_milli(h)))
}

/// SPEC: canon §2.1, §4.2. Highest point of a jump and the tick it is reached.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Apex {
    /// Height above the launch point.
    pub height: Fx,
    /// Tick (1-based after launch) of the first tick at that height.
    pub tick: u32,
}

/// SPEC: canon §2.1 semi-implicit Euler (`v += a × DT`, then `p += v × DT`,
/// each `mul_fx`) from height 0 with upward speed `v0` under gravity `g > 0`,
/// until the velocity stops pointing up. Panics if `g <= 0`.
#[must_use]
pub fn simulate_apex(g: Fx, v0: Fx) -> Apex {
    assert!(g > Fx::ZERO, "invariant: gravity must be positive");
    let a = Fx3::new(Fx::ZERO, -g, Fx::ZERO);
    let (mut p, mut v) = (Fx3::ZERO, Fx3::new(Fx::ZERO, v0, Fx::ZERO));
    let mut best = Apex {
        height: Fx::ZERO,
        tick: 0,
    };
    let mut tick = 0u32;
    while v.y > Fx::ZERO {
        (p, v) = semi_implicit_euler(p, v, a);
        tick += 1;
        if p.y > best.height {
            best = Apex { height: p.y, tick };
        }
    }
    best
}

// ---------------------------------------------------------------- fall

/// SPEC: canon §4.2 `SAFE_FALL = 4000 mm × (10000 + (vrv(e) - 5) × 400) Bp`, in mm.
#[must_use]
pub const fn safe_fall_mm(vrv: u8) -> i64 {
    mul_bp(4000, 10_000 + (vrv as i32 - 5) * 400)
}

/// SPEC: canon §4.2 `fall_height = max(0, peak - land)` along the current
/// gravity axis: `peak` is the extreme origin coordinate against gravity, so
/// under flipped gravity the fall is `land - peak`.
#[must_use]
pub fn fall_height(peak: Fx, land: Fx, gravity_flipped: bool) -> Fx {
    let h = if gravity_flipped {
        land - peak
    } else {
        peak - land
    };
    if h > Fx::ZERO { h } else { Fx::ZERO }
}

/// SPEC: canon §4.2. Fall damage is 800 Bp (8%) of Max HP per meter of excess.
pub const FALL_DAMAGE_BP_PER_M: i64 = 800;

/// SPEC: canon §4.2 fall damage before multipliers: if
/// `excess = fall_height - SAFE_FALL > 0`, `mul_bp(max_hp_mhp,
/// ((excess.raw as i128 × 800) >> 32) as i32)` mHP (checked narrowing), else 0.
/// It is a `dmg.blunt` hit entering the pipeline at step 4, skipping armor and
/// the chip floor, never critting.
#[must_use]
pub fn fall_damage_mhp(fall_height: Fx, vrv: u8, max_hp_mhp: i64) -> i64 {
    let excess = fall_height - fx_from_milli(safe_fall_mm(vrv));
    if excess <= Fx::ZERO {
        return 0;
    }
    let bp = narrow_i32((i128::from(excess.raw()) * i128::from(FALL_DAMAGE_BP_PER_M)) >> 32);
    mul_bp(max_hp_mhp, bp)
}

/// SPEC: canon §5.9 `fall_dmg_bp`, then §4.2 "Acrobatics (doc 04) multiplies
/// it afterwards": `later_bps` (Acrobatics and other doc 04 factors, in their
/// owning order) chain after the genre factor, one `mul_bp` each.
#[must_use]
pub fn apply_fall_multipliers(damage_mhp: i64, fall_dmg_bp: i32, later_bps: &[i32]) -> i64 {
    later_bps
        .iter()
        .fold(mul_bp(damage_mhp, fall_dmg_bp), |v, &bp| mul_bp(v, bp))
}

// ---------------------------------------------------------------- dodge

/// SPEC: canon §4.2 dodge burst length, mm.
pub const DODGE_DISTANCE_MM: i64 = 3000;
/// SPEC: canon §4.2 dodge duration, ticks.
pub const DODGE_TICKS: u32 = 12;
/// SPEC: canon §4.2, §1.5 dodge cost: 20 stamina = 20,000 milli.
pub const DODGE_COST_MILLI: i64 = 20_000;
/// SPEC: canon §4.2. A new dodge starts at least 18 ticks after the previous one started.
pub const DODGE_COOLDOWN_TICKS: u64 = 18;
/// SPEC: canon §4.2 derived: 3000 mm over 12 ticks (0.2 s) is 15000 mm/s.
pub const DODGE_SPEED_MM_S: i64 = DODGE_DISTANCE_MM * 60 / DODGE_TICKS as i64;

/// SPEC: canon §4.2 invulnerability ticks
/// `min(12, 8 + floor_div(max(0, AGI - 40), 15))`.
#[must_use]
pub const fn dodge_invuln_ticks(agi: i32) -> u32 {
    let over = if agi > 40 { agi as i64 - 40 } else { 0 };
    let t = 8 + floor_div(over, 15);
    if t > DODGE_TICKS as i64 {
        DODGE_TICKS
    } else {
        t as u32
    }
}

/// SPEC: canon §4.2. Invulnerability starts on the dodge's first tick
/// (`dodge_tick = 1`) and never runs past its 12th tick.
#[must_use]
pub const fn dodge_invulnerable_on(dodge_tick: u32, agi: i32) -> bool {
    dodge_tick >= 1 && dodge_tick <= dodge_invuln_ticks(agi)
}

/// SPEC: canon §4.2. A dodge may start at `now` if none started before or the
/// last one started at least 18 ticks earlier.
#[must_use]
pub const fn dodge_can_start(now: u64, last_start: Option<u64>) -> bool {
    match last_start {
        None => true,
        Some(t) => now >= t && now - t >= DODGE_COOLDOWN_TICKS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genre::Genre;
    use crate::stats::move_speed_bp;
    use proptest::prelude::*;

    /// Micrometers (mm with three decimals), floored.
    fn um(x: Fx) -> i64 {
        ((i128::from(x.raw()) * 1_000_000) >> 32) as i64
    }

    /// SPEC: canon §4.2 JUMP_H column.
    #[test]
    fn jump_heights() {
        let expect = [
            (0, 1100),
            (5, 1250),
            (6, 1400),
            (7, 1550),
            (8, 1700),
            (9, 1850),
            (10, 2000),
        ];
        for (v, h) in expect {
            assert_eq!(jump_height_mm(v), h);
        }
        assert_eq!(jump_height_mm(3), 1190);
        assert_eq!(air_jump_height_mm(8), None);
        assert_eq!(air_jump_height_mm(9), Some(1110));
        assert_eq!(air_jump_height_mm(10), Some(1200));
    }

    /// SPEC: canon §21 test 24. With gravity 20000 mm/s² the simulated apex is
    /// at least 1,099 mm at VERVE 0 and within 1,999..=2,001 mm at VERVE 10;
    /// with 16000 mm/s² at VERVE 10 also within 1,999..=2,001 mm.
    #[test]
    fn canon_t24_jump_apex() {
        let g20 = local_gravity(10_000);
        let g16 = local_gravity(8000);
        assert_eq!(g20, Fx::from_int(20));
        assert_eq!(g16, Fx::from_int(16));
        let v0 = simulate_apex(g20, jump_launch_speed(0, 10_000));
        assert!(v0.height >= fx_from_milli(1099));
        let v10 = simulate_apex(g20, jump_launch_speed(10, 10_000));
        assert!(v10.height >= fx_from_milli(1999) && v10.height <= fx_from_milli(2001));
        let wx = simulate_apex(g16, jump_launch_speed(10, 8000));
        assert!(wx.height >= fx_from_milli(1999) && wx.height <= fx_from_milli(2001));
    }

    /// Cross-check against the doc 03 §5.7 table (launch raws, apex in mm
    /// with three decimals, apex tick).
    #[test]
    fn doc03_jump_table() {
        let cases = [
            (0u8, 10_000, 29_205_417_893i64, 1_099_972i64, 21u32),
            (5, 10_000, 31_085_832_879, 1_249_874, 22),
            (10, 10_000, 39_131_183_219, 1_999_922, 28),
            (10, 8000, 34_932_400_672, 2_000_000, 31),
        ];
        for (vrv, gbp, raw, apex_um, tick) in cases {
            let v0 = jump_launch_speed(vrv, gbp);
            assert_eq!(v0.raw(), raw, "v0 at VERVE {vrv}, gravity {gbp}");
            let a = simulate_apex(local_gravity(gbp), v0);
            assert!(
                (um(a.height) - apex_um).abs() <= 1,
                "apex {} vs {apex_um}",
                um(a.height)
            );
            // Doc 03 numbers the apex tick one higher than this crate's
            // count of integration steps (spec issue reported).
            assert_eq!(a.tick + 1, tick);
        }
        let air = air_jump_launch_speed(10, 8000).expect("VERVE 10 has an air jump");
        assert_eq!(air.raw(), 27_187_601_199);
        let a = simulate_apex(local_gravity(8000), air);
        assert!(
            (um(a.height) - 1_199_874).abs() <= 1,
            "air apex {}",
            um(a.height)
        );
        assert_eq!(air_jump_launch_speed(8, 8000), None);
    }

    /// SPEC: canon §4.2: every genre can mount a 1-block ledge at its own
    /// VERVE, and Wuxia clears 2-block walls.
    #[test]
    fn every_genre_mounts_a_ledge() {
        for g in Genre::ALL {
            let vrv = g.axiom(crate::genre::Axiom::Verve);
            let a = simulate_apex(
                local_gravity(g.gravity_bp()),
                jump_launch_speed(vrv, g.gravity_bp()),
            );
            assert!(a.height >= Fx::from_int(1), "{}", g.id());
        }
        // Wuxia: the apex is 2 m minus 16 raw units (spec issue reported), so
        // a 2-block wall is cleared through the 550 mm step-up, not by the apex.
        let wx = Genre::Wuxia;
        let a = simulate_apex(
            local_gravity(wx.gravity_bp()),
            jump_launch_speed(10, wx.gravity_bp()),
        );
        assert_eq!(a.height.raw(), (2i64 << 32) - 16);
        assert!(a.height + fx_from_milli(550) >= Fx::from_int(2));
    }

    /// SPEC: canon §4.2 safe fall and fall damage (doc 04 test 46: a 13,600-mHP
    /// fall hit at Max HP 85, i.e. 2 m of excess at VERVE 5).
    #[test]
    fn fall_damage() {
        assert_eq!(safe_fall_mm(5), 4000);
        assert_eq!(safe_fall_mm(0), 3200);
        assert_eq!(safe_fall_mm(10), 4800);
        assert_eq!(fall_damage_mhp(Fx::from_int(6), 5, 85_000), 13_600);
        assert_eq!(fall_damage_mhp(Fx::from_int(4), 5, 85_000), 0);
        assert_eq!(fall_damage_mhp(Fx::from_int(3), 5, 85_000), 0);
        assert_eq!(fall_damage_mhp(Fx::from_int(5), 5, 85_000), 6800); // 8% per meter
        // A partial meter: 0.5 m excess is 400 Bp.
        assert_eq!(fall_damage_mhp(fx_from_milli(4500), 5, 100_000), 4000);
        assert_eq!(
            fall_height(Fx::from_int(10), Fx::from_int(4), false),
            Fx::from_int(6)
        );
        assert_eq!(
            fall_height(Fx::from_int(4), Fx::from_int(10), false),
            Fx::ZERO
        );
        assert_eq!(
            fall_height(Fx::from_int(4), Fx::from_int(10), true),
            Fx::from_int(6)
        );
        // Genre factor, then Acrobatics: Wuxia 5000 then 8000.
        assert_eq!(apply_fall_multipliers(13_600, 5000, &[8000]), 5440);
        assert_eq!(
            apply_fall_multipliers(13_600, Genre::Redacted.row().fall_dmg_bp, &[]),
            20_400
        );
    }

    /// SPEC: canon §21 test 27. AGI 150 gives 12 invulnerability ticks.
    #[test]
    fn canon_t27_dodge() {
        assert_eq!(dodge_invuln_ticks(150), 12);
        assert_eq!(dodge_invuln_ticks(40), 8);
        assert_eq!(dodge_invuln_ticks(1), 8);
        assert_eq!(dodge_invuln_ticks(54), 8);
        assert_eq!(dodge_invuln_ticks(55), 9);
        assert_eq!(dodge_invuln_ticks(100), 12);
        assert_eq!(dodge_invuln_ticks(99), 11);
        assert!(!dodge_invulnerable_on(0, 40));
        assert!(dodge_invulnerable_on(1, 40));
        assert!(dodge_invulnerable_on(8, 40));
        assert!(!dodge_invulnerable_on(9, 40));
        assert!(dodge_invulnerable_on(12, 150));
        assert!(!dodge_invulnerable_on(13, 150));
        assert!(dodge_can_start(5, None));
        assert!(!dodge_can_start(117, Some(100)));
        assert!(dodge_can_start(118, Some(100)));
        assert!(!dodge_can_start(99, Some(100)));
        assert_eq!(DODGE_SPEED_MM_S, 15_000);
    }

    /// SPEC: canon §4.2 class thresholds and the doc 03 §5.2 example
    /// (`move_x = 90`, `move_z = -90` is strafe).
    #[test]
    fn direction_classes() {
        assert_eq!(move_class_from_angle(0), MoveClass::Forward);
        assert_eq!(move_class_from_angle(4096), MoveClass::Forward);
        assert_eq!(move_class_from_angle(4097), MoveClass::Strafe);
        assert_eq!(move_class_from_angle(12_288), MoveClass::Strafe);
        assert_eq!(move_class_from_angle(12_289), MoveClass::Backpedal);
        assert_eq!(move_class_from_angle(32_768), MoveClass::Backpedal);
        assert_eq!(move_class_from_input(90, -90), Some(MoveClass::Strafe));
        assert_eq!(move_class_from_input(0, -127), Some(MoveClass::Forward));
        assert_eq!(move_class_from_input(0, 127), Some(MoveClass::Backpedal));
        assert_eq!(move_class_from_input(0, 0), None);
        assert_eq!(move_class_from_input(-128, -128), Some(MoveClass::Strafe));
        // Cases where doc 03's floored isqrt disagrees with the canon angle rule:
        // (1, -1) is 45° (strafe; doc 03 says forward), (10, -4) is 68.2°
        // (backpedal; doc 03 says strafe).
        assert_eq!(move_class_from_input(1, -1), Some(MoveClass::Strafe));
        assert_eq!(move_class_from_input(10, -4), Some(MoveClass::Backpedal));
        assert_eq!(move_class_from_input(-1, -3), Some(MoveClass::Forward)); // 18.4°
        // Pure sideways input is 90° from facing, above 67.5°: backpedal (see spec issues).
        assert_eq!(move_class_from_input(127, 0), Some(MoveClass::Backpedal));
        assert_eq!(cos_bam(12_288), 410_903_207);
        assert_eq!(cos_bam(4096), 992_008_094);
    }

    /// SPEC: canon §4.2 speed composition, with the doc 03 §5.2 worked
    /// values: strafe at AGI 60 is 4289 mm/s, forward 4515, with `st.slowed` 3160.
    #[test]
    fn speed_composition() {
        let row11 = move_speed_bp(60);
        assert_eq!(row11, 10_500);
        assert_eq!(
            final_move_speed_mm_s(base_speed_mm_s(Gait::Walk, MoveClass::Strafe), row11, &[]),
            4289
        );
        let fwd = base_speed_mm_s(Gait::Walk, MoveClass::Forward);
        assert_eq!(final_move_speed_mm_s(fwd, row11, &[]), 4515);
        let slowed = [StatusMultiplier {
            id: "st.slowed",
            bp: 7000,
        }];
        assert_eq!(final_move_speed_mm_s(fwd, row11, &slowed), 3160);
        let two = [
            StatusMultiplier {
                id: "st.encumbered",
                bp: 8500,
            },
            StatusMultiplier {
                id: "st.hasted",
                bp: 12_500,
            },
        ];
        assert_eq!(
            final_move_speed_mm_s(4300, 10_000, &two),
            mul_bp(mul_bp(4300, 8500), 12_500)
        );
        assert_eq!(base_speed_mm_s(Gait::Walk, MoveClass::Backpedal), 3440);
        assert_eq!(base_speed_mm_s(Gait::Sprint, MoveClass::Forward), 6500);
        assert_eq!(base_speed_mm_s(Gait::Sprint, MoveClass::Strafe), 4085);
        assert_eq!(base_speed_mm_s(Gait::Crouch, MoveClass::Forward), 2000);
        assert_eq!(base_speed_mm_s(Gait::Swim, MoveClass::Backpedal), 2400);
        assert_eq!(base_speed_mm_s(Gait::Climb, MoveClass::Forward), 2400);
        assert_eq!(air_accel_mm_s2(Genre::Wuxia.row().air_accel_bp), 16_000);
        assert_eq!(
            ground_decel_mm_s2(Genre::HardboiledNoir.row().ground_friction_bp),
            34_000
        );
    }

    /// SPEC: canon §4.2: status multipliers must arrive in ascending ID order.
    #[test]
    #[should_panic(expected = "invariant")]
    fn unsorted_statuses_panic() {
        let bad = [
            StatusMultiplier {
                id: "st.slowed",
                bp: 7000,
            },
            StatusMultiplier {
                id: "st.hasted",
                bp: 12_500,
            },
        ];
        let _ = final_move_speed_mm_s(4300, 10_000, &bad);
    }

    proptest! {
        /// SPEC: canon §4.2: with the §2.1 integrator the apex equals JUMP_H
        /// within 1 mm for g from 12000 to 20000 mm/s², for every VERVE and
        /// for air jumps.
        #[test]
        fn apex_within_1mm(g_mm in 12_000i64..=20_000, vrv in 0u8..=10) {
            let g = fx_from_milli(g_mm);
            let h = jump_height_mm(vrv);
            let a = simulate_apex(g, launch_speed(g, fx_from_milli(h)));
            prop_assert!((a.height - fx_from_milli(h)).abs() <= fx_from_milli(1), "g {} vrv {} apex {}", g_mm, vrv, um(a.height));
            if let Some(ah) = air_jump_height_mm(vrv) {
                let b = simulate_apex(g, launch_speed(g, fx_from_milli(ah)));
                prop_assert!((b.height - fx_from_milli(ah)).abs() <= fx_from_milli(1));
            }
        }

        /// SPEC: canon §4.2: jump height and safe fall never fall as VERVE
        /// rises; fall damage never falls as the fall grows.
        #[test]
        fn monotone_in_verve_and_height(vrv in 0u8..10, mm in 0i64..64_000, max_hp in 1i64..10_000_000) {
            prop_assert!(jump_height_mm(vrv + 1) > jump_height_mm(vrv));
            prop_assert!(safe_fall_mm(vrv + 1) > safe_fall_mm(vrv));
            let a = fall_damage_mhp(fx_from_milli(mm), vrv, max_hp);
            let b = fall_damage_mhp(fx_from_milli(mm + 1000), vrv, max_hp);
            prop_assert!(b >= a && a >= 0);
            prop_assert!(fall_damage_mhp(fx_from_milli(mm), vrv + 1, max_hp) <= a);
        }

        /// SPEC: canon §4.2: invulnerability is 8..=12 ticks, non-decreasing in AGI.
        #[test]
        fn dodge_ticks_bounded(agi in 1i32..=150) {
            let t = dodge_invuln_ticks(agi);
            prop_assert!((8..=12).contains(&t));
            prop_assert!(dodge_invuln_ticks(agi + 1) >= t);
        }

        /// SPEC: canon §4.2: the input classification agrees with the angle
        /// classification (CORDIC `atan2_bam`, ±1 Bam) except within 2 Bam of a threshold.
        #[test]
        fn input_class_matches_angle(x in -127i8..=127, z in -127i8..=127) {
            prop_assume!(x != 0 || z != 0);
            let class = move_class_from_input(x, z).expect("nonzero input");
            let fwd = -i64::from(z);
            let r = i64::from(x);
            let yaw_in = palimpsest_math::atan2_bam(-r, fwd);
            let diff = palimpsest_math::ang_diff(yaw_in, 0);
            let near = |t: u16| diff.abs_diff(t) <= 2;
            if !near(BACKPEDAL_ANGLE_BAM) && !near(STRAFE_ANGLE_BAM) {
                prop_assert_eq!(class, move_class_from_angle(diff));
            }
        }
    }
}
