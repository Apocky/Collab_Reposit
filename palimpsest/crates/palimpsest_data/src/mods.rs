//! Mod packages. SPEC: doc 02 §3.2, §13.
//!
//! Doc 02 §3.2 names this module: "`mods` (§13: mod folders, classification,
//! patches, `mod_blob`, `modset_hash`)". It is a placeholder until doc 21
//! WP-077 adds doc 02 §13.1's `mods/<mod_id>/` layout and `mod.json`, the
//! presentation-only versus simulation-affecting classification, the
//! `patches/*.json` append records on `"x-patchable"` arrays, `mod_blob` and
//! doc 02 §13.4's `modset_hash`. The `x_<mod_id>` ID segment rules that mod
//! records must follow already live in [`crate::ids`] and
//! [`crate::registry`]. No item is defined here yet (doc 02 §18 rule 4).
