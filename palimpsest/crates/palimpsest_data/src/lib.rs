//! palimpsest_data: content IDs, `id32`, the ID registry, text keys, hex `u64`
//! strings and JSON content checks. SPEC: canon §1.1, §1.6, §1.10, §19; doc 02
//! §3.2, §11.
//!
//! The core (`ids`, `registry`, `text_key`, `hex`) is `no_std` + `alloc`, with
//! no floats, no hash maps and no unsafe code (doc 02 §3.2). The `loader`
//! feature adds std, serde_json and the `content_check` module (doc 02
//! §3.3); unit tests always compile it.
#![cfg_attr(not(any(test, feature = "loader")), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub mod hex;
pub mod ids;
pub mod registry;
pub mod text_key;

#[cfg(any(test, feature = "loader"))]
pub mod content_check;

#[cfg(test)]
mod canon_tests;
#[cfg(test)]
mod test_support;

pub use hex::{HexU64Error, format_hex_u64, parse_hex_u64};
pub use ids::{
    ContentId, Id32, IdError, IdOrigin, ModSegmentError, is_content_id, mod_id_of, namespace_of,
    validate_content_id, validate_id_origin,
};
pub use registry::{Entry, ID32_COLLISION, IdRegistry, MOD_SEGMENT_INVALID, RegistryError};
pub use text_key::{TextKey, TextKeyError, is_text_key, validate_text_key};
