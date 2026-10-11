//! palimpsest_math: integer primitives, Fx, trig, CORDIC, RNG, hashes. SPEC: canon §1, §3; doc 02 §4.
//!
//! `no_std` (doc 02 §3.2), no floats, no hash maps, no unsafe code. Every
//! operation is deterministic and checked: overflow panics in every profile
//! (canon §1.3).
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cordic;
pub mod fx;
pub mod geom;
pub mod hash;
pub mod int;
pub mod rng;
pub mod seed;
pub mod tables;
pub mod trig;

mod cordic_table;
mod trig_table;

pub use cordic::atan2_bam;
pub use fx::{Fx, div_fx, fx_floor_block, fx_from_milli, isqrt_u128, mul_bp_fx, mul_fx, sqrt_fx};
pub use geom::{Fx3, IVec3, dist2};
pub use hash::{blake3_256, fnv1a32, fnv1a64};
pub use int::{ceil_div, floor_div, mul_bp, round_half_even};
pub use rng::{Rng, keyed_rng, mix64, splitmix64, stream_seed};
pub use trig::{Bam, ang_diff, cos_bam, fwd, sin_bam};
