use std::sync::Arc;
use tracing::{info, error};

use orchestrator::attestation::{AttestationManager, CathedralComputeProvider, AttestationProvider, AttestationVerifier};
use orchestrator::identity_attestation::{IdentityAttestationProvider, IdentityAttestation};
use orchestrator::voice::VoiceCore;
use orchestrator::mcp::server::start_mcp_server;

struct DummyIdentityProvider {}

impl IdentityAttestationProvider for DummyIdentityProvider {
    fn attest_identity(
        &self,
        _force_refresh: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<IdentityAttestation, String>> + Send + '_>> {
        Box::pin(async {
            Ok(IdentityAttestation {
                confidence: 0.9,
                identity_verified: true,
                timestamp: chrono::Utc::now().timestamp(),
            })
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let attestation_manager = Arc::new(AttestationManager::new());
    let identity_provider: Arc<dyn IdentityAttestationProvider + Send + Sync> = Arc::new(DummyIdentityProvider {});
    let voice_core = Arc::new(VoiceCore::new());

    // 1. Configuração do MCP
    let mcp_enabled = std::env::var("ENABLE_MCP_SERVER")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    if mcp_enabled {
        let mcp_port = std::env::var("MCP_PORT")
            .unwrap_or_else(|_| "3032".to_string())
            .parse::<u16>()
            .unwrap_or(3032);

        let mcp_token = std::env::var("MCP_AUTH_TOKEN").ok();

        // 2. Cria execution provider (CathedralComputeProvider)
        let execution_provider: Arc<dyn AttestationProvider + Send + Sync> =
            Arc::new(CathedralComputeProvider::new());

        // 3. Verificador (pode ser opcional)
        let architect_verifier: Option<Arc<dyn AttestationVerifier + Send + Sync>> = None;

        // 4. Inicia o servidor
        let attestation_manager_clone = attestation_manager.clone();
        let identity_provider_clone = identity_provider.clone();
        let execution_provider_clone = execution_provider.clone();
        let voice_core_clone = Some(voice_core.clone());

        tokio::spawn(async move {
            if let Err(e) = start_mcp_server(
                attestation_manager_clone,
                identity_provider_clone,
                execution_provider_clone,
                architect_verifier,
                voice_core_clone,
                mcp_port,
                mcp_token,
            )
            .await
            {
                error!("❌ MCP Server falhou: {}", e);
            }
        });

        info!("🧠 MCP Server iniciado na porta {}", mcp_port);
    }

    tokio::signal::ctrl_c().await?;

    Ok(())
}
