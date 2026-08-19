mod agent;
mod agent_definition;
mod context_engine;
mod conversation;
mod database;
mod omega_repo;
mod fs_snap;
mod fuzzy_search;
mod provider;
mod skill;
mod validation;

mod proto_generated {
    tonic::include_proto!("omega.v1");
}

// Only expose omega_repo container
pub use omega_repo::*;
