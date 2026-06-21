//! Binário de inferência

use cathedral_33t::CathedralConfig;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("🏛️ Cathedral 33T Inference");

    let config = CathedralConfig::default();
    info!("Inference config: {:?}", config.inference);

    Ok(())
}
