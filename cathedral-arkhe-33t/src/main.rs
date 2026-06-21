//! Cathedral 33T CLI

use cathedral_arkhe_33t::CathedralConfig;
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("🏛️ Cathedral ARKHE 33T v{}", cathedral_arkhe_33t::VERSION);
    info!("📐 Model: 33T parameters, 33B active");

    let config = CathedralConfig::default();

    info!("✅ Configuration loaded");
    info!("📊 Experts: {}", config.model.num_experts);
    info!("📊 Active experts per token: {}", config.model.top_k);
    info!("📊 Context length: {}", config.model.max_seq_len);
    info!("📊 MHC expansion: {}", config.model.mhc_expansion_rate);

    info!("🚀 Ready for training/inference");
    Ok(())
}
