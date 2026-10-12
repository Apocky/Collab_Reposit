//! palimpsest_sim: deterministic simulation core. SPEC: canon §2, §4 to §17; doc 03.
//!
//! Doc 02 §1 rule 2 and §6.2: the simulation is integer-only, single-threaded
//! and `no_std` with `alloc` in every non-test build. `no_std` removes clocks,
//! threads, files, the environment and `HashMap` by construction; the sim's
//! collections are `alloc`'s `Vec`, `String`, `BTreeMap` and `BTreeSet`. Unit
//! tests link std (proptest, `catch_unwind`, the canon table checks).
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod contradiction;
pub mod damage;
pub mod entity;
pub mod genre;
pub mod loot;
pub mod movement;
pub mod progression;
pub mod sim;
pub mod stats;
pub mod status;
pub mod time;
