#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod config;
pub mod tensor;
pub mod moe;
pub mod attention;
pub mod mhc;
pub mod optimizer;
pub mod routing;
pub mod speculative;
pub mod placement;
pub mod utils;

pub use config::CathedralConfig;
pub use tensor::{Tensor, Shape};
pub use moe::{MoELayer, Expert, HierarchicalRouter};
pub use attention::HybridAttention;
pub use mhc::ManifoldConstrainedHyperConnections;
pub use optimizer::MONALiteOptimizer;
pub use routing::AnticipatoryRouter;
pub use speculative::Eagle3Decoder;
pub use placement::{OccultPlacementOptimizer, HybridEP};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
