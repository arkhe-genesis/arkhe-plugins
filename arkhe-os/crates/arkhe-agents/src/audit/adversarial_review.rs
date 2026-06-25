//! Agente de revisão adversária — tenta refutar cada PR antes do merge.

// Stub definition for types
pub struct Arc<T>(T);
pub struct InferenceEngine;
impl InferenceEngine {
    pub async fn generate(&self, _prompt: &str, _temp: f32, _max_tokens: usize) -> Result<String, String> {
        Ok("VERIFIED: safe".to_string())
    }
}
pub struct Escalator;
impl Escalator {
    pub async fn escalate(&self, _pr: &str, _resp: &str) {}
}
pub enum ReviewVerdict {
    Verified,
    Rejected(String),
}
pub struct ArkheError(String);

pub struct AdversarialReviewer {
    llm: Arc<InferenceEngine>,
    escalator: Escalator,
}

impl AdversarialReviewer {
    pub async fn review_pr(
        &self,
        pr_description: &str,
        diff: &str,
    ) -> Result<ReviewVerdict, ArkheError> {
        let prompt = format!(
            r#"
            You are an adversarial reviewer. Your job is to DISPROVE the safety of this PR.

            PR: {}
            Diff:
            {}

            Find at least one concrete attack scenario that exploits this change.
            If you cannot find one, state 'VERIFIED: safe'.
            If you find one, state 'REJECTED: [attack scenario]'.
            "#,
            pr_description, diff
        );

        let response = self.llm.0.generate(&prompt, 0.3, 4096).await.map_err(|e| ArkheError(e))?;

        if response.contains("REJECTED") {
            self.escalator.escalate(pr_description, &response).await;
            Ok(ReviewVerdict::Rejected(response))
        } else {
            Ok(ReviewVerdict::Verified)
        }
    }
}
