//! Binário de treino

use cathedral_33t::CathedralConfig;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("🏛️ Cathedral 33T Training");

    let config = CathedralConfig::default();
    info!("Training config: {:?}", config.training);

    Ok(())
}
