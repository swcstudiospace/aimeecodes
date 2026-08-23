mod agent;
mod agent_definition;
mod aimee_repo;
mod context_engine;
mod conversation;
mod database;
mod fs_snap;
mod fuzzy_search;
mod provider;
mod skill;
mod validation;

mod proto_generated {
    tonic::include_proto!("aimee.v1");
}

// Only expose aimee_repo container
pub use aimee_repo::*;
