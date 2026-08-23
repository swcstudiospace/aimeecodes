//! Aimee Anda — eternal session pathways backed by KIP / Cognitive Nexus.
//!
//! This crate does **not** replace Aimee's agent runtime. It adds:
//! - append-only session pathway checkpoints for each agent output
//! - hash-chained conversation snapshots for chat-only rollbacks
//! - optional KIP / Cognitive Nexus integration
//! - hooks for eternal durability via the companion `aimee_anda_icp` crate

mod backends;
mod domain;
mod hook;
mod infra;
mod services;

pub use backends::*;
pub use domain::*;
pub use hook::*;
pub use infra::*;
pub use services::*;
