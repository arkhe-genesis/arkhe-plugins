//! Ações administrativas do NEXUS mapeadas para ActionClass.
//!
//! Esta enum mapeia as ações específicas do NEXUS para os tipos
//! genéricos de ActionClass do arkhe-governance.

use arkhe_governance::ActionClass;

/// Ações administrativas específicas do NEXUS/Safe Core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NexusAdminAction {
    /// Atualização do kernel do NEXUS.
    KernelUpdate,
    /// Modificação de políticas de segurança.
    SecurityPolicyChange,
    /// Alteração em Capsules (privilégios, isolamento).
    CapsuleModification,
    /// Atualização de regras de ComplianceReport.
    ComplianceRulesUpdate,
    /// Modificação de parâmetros Flock.
    FlockConfigUpdate,
    /// Operação no WormGraph (routing, anchoring).
    WormGraphOperation,
    /// Alteração no Hashtree de bundles.
    BundleHashtreeUpdate,
    /// Outra ação administrativa.
    Other,
}

impl NexusAdminAction {
    /// Mapeia para ActionClass genérico.
    pub fn to_generic(&self) -> ActionClass {
        match self {
            Self::KernelUpdate => ActionClass::Critical,
            Self::SecurityPolicyChange => ActionClass::Critical,
            Self::CapsuleModification => ActionClass::Operational,
            Self::ComplianceRulesUpdate => ActionClass::Operational,
            Self::FlockConfigUpdate => ActionClass::Operational,
            Self::WormGraphOperation => ActionClass::Critical,
            Self::BundleHashtreeUpdate => ActionClass::Operational,
            Self::Other => ActionClass::Other,
        }
    }

    /// Cria proposta de governança para esta ação.
    pub fn to_proposal(
        &self,
        description: String,
        proposer_did: String,
        delay_hours: u64,
        action_hash: [u8; 32],
    ) -> arkhe_governance::GovernanceAction {
        arkhe_governance::GovernanceAction::new(
            self.to_generic(),
            description,
            proposer_did,
            std::time::Duration::from_secs(delay_hours * 3600),
            action_hash,
        )
    }
}
