use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::agent::AgentMessage;

pub struct ToolContext {
    pub workspace: String,
}

impl ToolContext {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }
}

pub struct SessionData {
    pub history: Vec<AgentMessage>,
    pub tool_context: Arc<ToolContext>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, SessionData>>>,
    max_history: usize,
}

impl SessionManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            max_history,
        }
    }

    pub async fn create_session(&self, id: &str, tool_context: Arc<ToolContext>) {
        let mut sessions = self.sessions.lock().await;
        sessions.insert(
            id.to_string(),
            SessionData {
                history: Vec::new(),
                tool_context,
                created_at: chrono::Utc::now(),
                status: "active".to_string(),
            },
        );
    }

    pub async fn get_session(&self, id: &str) -> Option<SessionData> {
        let sessions = self.sessions.lock().await;
        sessions.get(id).map(|s| SessionData {
            history: s.history.clone(),
            tool_context: s.tool_context.clone(),
            created_at: s.created_at,
            status: s.status.clone(),
        })
    }

    pub async fn append_message(&self, id: &str, message: AgentMessage) {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(id) {
            session.history.push(message);
            if session.history.len() > self.max_history {
                session.history.drain(0..session.history.len() - self.max_history);
            }
        }
    }
}
