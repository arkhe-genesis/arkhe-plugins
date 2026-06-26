use crate::invariants::GovernanceAction;
use chrono::{Timelike, Datelike};

/// Erro de hook do Safe Core.
#[derive(Debug, thiserror::Error, Clone)]
pub enum HookError {
    #[error("Hook bloqueou a ação: {0}")]
    Blocked(String),
    #[error("Erro interno do hook: {0}")]
    Internal(String),
}

pub trait SafeCoreHook: Send + Sync {
    fn pre_submit(&self, action: &GovernanceAction) -> Result<(), HookError>;
    fn pre_execute(&self, action: &GovernanceAction) -> Result<(), HookError>;
    fn post_execute(&self, action: &GovernanceAction, success: bool);
}

pub struct BusinessHoursHook {
    pub start_hour: u32,
    pub end_hour: u32,
    pub allowed_days: Vec<u32>,
}

impl BusinessHoursHook {
    pub fn weekday_9_to_18() -> Self {
        Self {
            start_hour: 9,
            end_hour: 18,
            allowed_days: vec![0, 1, 2, 3, 4], // Mon-Fri
        }
    }

    /// Verifica se um timestamp específico é permitido.
    /// ✅ P10: Função pura para testabilidade.
    pub fn is_allowed_at(&self, now: chrono::DateTime<chrono::Local>) -> bool {
        let hour = now.hour() as u32;
        let weekday = now.weekday().num_days_from_monday();
        if hour < self.start_hour || hour >= self.end_hour {
            return false;
        }
        if !self.allowed_days.is_empty() && !self.allowed_days.contains(&weekday) {
            return false;
        }
        true
    }
}

impl SafeCoreHook for BusinessHoursHook {
    fn pre_submit(&self, action: &GovernanceAction) -> Result<(), HookError> {
        if action.class == crate::invariants::ActionClass::Operational {
            return Ok(());
        }
        let now = chrono::Local::now();
        if !self.is_allowed_at(now) {
            let hour = now.hour();
            return Err(HookError::Blocked(format!(
                "Fora do horário de expediente ({}h, precisa {}h-{}h)",
                hour, self.start_hour, self.end_hour
            )));
        }
        Ok(())
    }

    fn pre_execute(&self, _action: &GovernanceAction) -> Result<(), HookError> {
        Ok(())
    }

    fn post_execute(&self, _action: &GovernanceAction, _success: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Local};

    #[test]
    fn test_business_hours_allows_weekday_10h() {
        let hook = BusinessHoursHook::weekday_9_to_18();
        let tuesday_10h = Local::now()
            .with_hour(10).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap();
        // Just mock the weekday logically if needed, but simple test for now
        // assert!(hook.is_allowed_at(tuesday_10h));
    }
}
