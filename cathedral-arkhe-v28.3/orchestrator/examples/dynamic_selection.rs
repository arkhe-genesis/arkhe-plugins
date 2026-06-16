//! Cathedral ARKHE v28.3 — Dynamic Oracle Selection Example
//! Demonstra como o orquestrador seleciona dinamicamente entre Oracle-Instant
//! e Oracle-Thinking com base na complexidade da tarefa.
//!
//! Selo: CATHEDRAL-ARKHE-v28.3-DYNAMIC-SELECTION-2026-06-16

use std::sync::Arc;
use std::collections::HashMap;

// Tipos simulados para o exemplo (em produção estariam nos módulos reais)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningMode {
    Instant,
    Thinking,
}

#[derive(Debug)]
pub struct AgentConfig {
    pub id: String,
    pub reasoning_mode: ReasoningMode,
    // ... outros campos
}

pub struct MultiAgentOrchestrator {
    agents: HashMap<String, AgentConfig>,
}

impl MultiAgentOrchestrator {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register_agent(&mut self, config: AgentConfig) {
        self.agents.insert(config.id.clone(), config);
    }

    /// Analisa a complexidade da tarefa para selecionar o perfil adequado.
    pub fn analyze_task_complexity(&self, task: &str) -> ReasoningMode {
        // Heurística simples de complexidade
        let word_count = task.split_whitespace().count();
        let has_complex_keywords = task.contains("analise") ||
                                   task.contains("compare") ||
                                   task.contains("explique detalhadamente") ||
                                   task.contains("trade-offs");

        if word_count > 30 || has_complex_keywords {
            ReasoningMode::Thinking
        } else {
            ReasoningMode::Instant
        }
    }

    /// Roteia a tarefa para o Oracle adequado.
    pub async fn route_task(&self, task: &str) -> String {
        let mode = self.analyze_task_complexity(task);

        let target_agent_id = match mode {
            ReasoningMode::Instant => "cathedral-oracle-instant-v28.3",
            ReasoningMode::Thinking => "cathedral-oracle-thinking-v28.3",
        };

        if let Some(agent) = self.agents.get(target_agent_id) {
            println!("🔄 Roteando tarefa para: {} (Modo: {:?})", target_agent_id, mode);
            // Em produção, aqui chamaríamos agent.run(task).await
            format!("Tarefa executada por {:?}", agent.reasoning_mode)
        } else {
            println!("⚠️ Oracle profile not found, falling back to default");
            "Tarefa executada pelo Oracle Default".to_string()
        }
    }
}

#[tokio::main]
async fn main() {
    let mut orchestrator = MultiAgentOrchestrator::new();

    // Registrar os dois perfis de Oracle
    orchestrator.register_agent(AgentConfig {
        id: "cathedral-oracle-instant-v28.3".to_string(),
        reasoning_mode: ReasoningMode::Instant,
    });

    orchestrator.register_agent(AgentConfig {
        id: "cathedral-oracle-thinking-v28.3".to_string(),
        reasoning_mode: ReasoningMode::Thinking,
    });

    // Testar com uma tarefa simples
    let simple_task = "Qual é o rendimento atual do hub DeFi?";
    println!("Tarefa: {}", simple_task);
    orchestrator.route_task(simple_task).await;

    println!("\n-------------------\n");

    // Testar com uma tarefa complexa
    let complex_task = "Analise as tendências do mercado DeFi nos últimos 30 dias e compare os trade-offs entre fornecer liquidez no hub A vs hub B considerando os riscos de impermanent loss.";
    println!("Tarefa: {}", complex_task);
    orchestrator.route_task(complex_task).await;
}