use std::sync::Arc;
use std::fs;
use serde_json::Value;

use crate::config_loader::{AgentConfigFile, MemoryConfig, TrustConfig};

#[derive(Debug)]
pub enum OrchestratorError {
    InvalidTask(String),
}

pub struct SphincsSigner {}

impl SphincsSigner {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct EventBus {}

pub struct MultiAgentOrchestrator {
    pub event_bus: Option<Arc<EventBus>>,
    pub signer: Arc<SphincsSigner>,
    pub memory_config: MemoryConfig,
    pub trust_config: TrustConfig,
}

impl MultiAgentOrchestrator {
    pub fn new(event_bus: Option<Arc<EventBus>>, signer: Arc<SphincsSigner>) -> Self {
        Self {
            event_bus,
            signer,
            memory_config: MemoryConfig {
                short_term_capacity: 0,
                long_term_enabled: false,
                vector_db: String::new(),
            },
            trust_config: TrustConfig {
                require_memory_proof: false,
                require_spex: false,
                post_quantum_signature: false,
            },
        }
    }

    pub async fn from_config_files(
        config_path: &str,
        manifest_path: &str,
        event_bus: Option<Arc<EventBus>>,
        signer: Arc<SphincsSigner>,
    ) -> Result<Self, OrchestratorError> {
        // 1. Carregar config.yaml
        let agent_config = AgentConfigFile::from_yaml(config_path)
            .map_err(|e| OrchestratorError::InvalidTask(format!("Config load error: {}", e)))?;

        // 2. Carregar manifest.json
        let manifest_content = fs::read_to_string(manifest_path)
            .map_err(|e| OrchestratorError::InvalidTask(e.to_string()))?;
        let _manifest: Value = serde_json::from_str(&manifest_content)
            .map_err(|e| OrchestratorError::InvalidTask(e.to_string()))?;

        // 3. Configurar memória e ferramentas a partir do config
        let memory_cfg = agent_config.agent.memory;
        let trust_cfg = agent_config.agent.trust;

        // 4. Inicializar orquestrador com esses parâmetros
        let mut orchestrator = Self::new(event_bus, signer);
        orchestrator.memory_config = memory_cfg;
        orchestrator.trust_config = trust_cfg;
        // ... registrar agentes com base nas roles do config, etc.

        println!(
            "Orquestrador carregado com agente {} v{} operando com role '{}'",
            agent_config.agent.id, agent_config.agent.version, agent_config.agent.role
        );
        println!(
            "Planejamento: estratégia {}, máximo de passos {}, consenso {}",
            agent_config.agent.planning.strategy,
            agent_config.agent.planning.max_steps,
            agent_config.agent.planning.consensus_mode
        );
        println!("Confiança: memory_proof={}, spex={}, pqc={}",
            orchestrator.trust_config.require_memory_proof,
            orchestrator.trust_config.require_spex,
            orchestrator.trust_config.post_quantum_signature
        );

        Ok(orchestrator)
    }

    pub async fn new_with_config(
        config_path: &str,
        manifest_path: &str,
    ) -> Result<Self, OrchestratorError> {
        let signer = Arc::new(SphincsSigner::new());
        let event_bus = None; // EventBus opcional para este helper
        Self::from_config_files(config_path, manifest_path, event_bus, signer).await
    }
}
