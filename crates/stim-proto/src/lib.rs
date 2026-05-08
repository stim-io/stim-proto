//! Shared protocol contracts for `stim.io` participants.

mod content;
mod control_plane;
mod delivery;
mod discovery;
mod ids;
mod message;
mod message_facts;
mod reply;

pub use content::*;
pub use control_plane::*;
pub use delivery::*;
pub use discovery::*;
pub use ids::*;
pub use message::*;
pub use message_facts::*;
pub use reply::*;

/// Current crate version exposed for downstream bootstrap checks.
pub const STIM_PROTO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Current shared protocol version string for the first execution wave.
pub const CURRENT_PROTOCOL_VERSION: &str = "stim/0.1";
