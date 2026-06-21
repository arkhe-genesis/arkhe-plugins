//! src/main.rs
//! Exemplo de inicialização do CCA 2.0 com todos os componentes.

use std::sync::Arc;
use cathedral_cse::{
    CCAgentV2, CCAConfig,
    trinity_core::TrinityCore,
    llm::LlmClient,
};
use cathedral_cse::tools::{ToolContext, SessionManager};

// Implementação concreta do LlmClient (exemplo)
struct OpenAiClient { /* ... */ }

impl OpenAiClient {
    pub fn new(_url: &str, _api_key: Option<&str>, _model: &str) -> Self {
        Self { }
    }
}

#[async_trait::async_trait]
impl LlmClient for OpenAiClient {
    async fn chat_completion(&self, _messages: &[cathedral_cse::agent::AgentMessage], _tools: Option<serde_json::Value>) -> Result<String, String> {
        Ok("Resposta simulada".to_string())
    }

    async fn chat_completion_stream(
        &self,
        _messages: &[cathedral_cse::agent::AgentMessage],
        _tools: Option<serde_json::Value>,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String, String>> + Send + Unpin>, String> {
        unimplemented!()
    }

    fn clone_arc(&self) -> Arc<dyn LlmClient + Send + Sync> {
        Arc::new(Self {})
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let llm_client = Arc::new(OpenAiClient::new("http://localhost:11434/v1/chat/completions", None, "llama3"));
    let trinity = Arc::new(TrinityCore::new());
    let session_manager = Arc::new(SessionManager::new(100));

    let config = CCAConfig::default();
    let agent = CCAgentV2::new(config, llm_client, trinity, session_manager.clone()).await;

    let session_id = "test-session";
    session_manager.create_session(session_id, Arc::new(ToolContext::new("./workspace".into()))).await;

    let response = agent.process("Cria uma função em Rust que calcula o factorial", session_id).await.unwrap();
    println!("Resposta:\n{}", response);

    Ok(())
}
