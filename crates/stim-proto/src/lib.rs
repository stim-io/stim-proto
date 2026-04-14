//! Shared protocol contracts for `stim.io` participants.
//!
//! The cold-start baseline intentionally keeps this crate minimal.
//! Real shared contract types land in the next execution step.

/// Current crate version exposed for downstream bootstrap checks.
pub const STIM_PROTO_VERSION: &str = env!("CARGO_PKG_VERSION");
