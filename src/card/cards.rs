//! Compatibility constants generated from the built-in [`super::sets`] records.
//!
//! Card records are the sole authored source of IDs. This module preserves the
//! existing Rust API without maintaining a second registry by hand.

include!(concat!(env!("OUT_DIR"), "/card_definition_ids.rs"));
