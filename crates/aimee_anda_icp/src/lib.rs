//! Eternal durability backends for Aimee Anda session pathways.
//!
//! Default mode writes content-addressed **local receipts**. ICP / IC-OSS
//! modes are represented in the API and return clear errors until configured.

mod error;
mod local_receipt;
mod store;

pub use error::*;
pub use local_receipt::*;
pub use store::*;
