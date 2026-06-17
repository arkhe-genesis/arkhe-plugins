pub mod attestation;
pub mod identity_attestation;
pub mod mcp;
pub mod voice;

pub mod geometry {
    pub mod causal_inner_product;
    pub mod concept_directions;
    pub mod embedding_bridge;
    pub mod service;
    pub mod steering_vectors;
    pub mod subspace_operations;
}

pub mod integration {
    pub mod hpe_simulation_adapter;
}

pub mod simulation {
    pub mod runner;
    pub mod tool_simulator;
    pub mod trajectory_store;
}

pub mod config_loader;
pub mod multi_agent;
pub mod stubs;
