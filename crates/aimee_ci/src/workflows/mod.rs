//! Workflow definitions for CI/CD

mod autofix;
mod bounty;
mod ci;
mod coderabbit;
mod devcontainer;
mod labels;
mod release_drafter;
mod release_publish;
mod stale;

pub use autofix::*;
pub use bounty::*;
pub use ci::*;
pub use coderabbit::*;
pub use devcontainer::*;
pub use labels::*;
pub use release_drafter::*;
pub use release_publish::*;
pub use stale::*;
