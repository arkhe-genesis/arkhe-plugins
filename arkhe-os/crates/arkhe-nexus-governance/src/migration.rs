//! Guia de migração do Safe Core para GovernanceGuard.

/// Documentação de migração para desenvolvedores do NEXUS.
///
/// # Passo 1: Substituir SafeCoreGuard
///
/// Antes:
/// ```rust,ignore
/// pub struct SafeCoreGuard {
///     // stubs existentes
/// }
///
/// impl SafeCoreGuard {
///     pub fn execute(&self, action: AdminAction) {
///         action.run(); // ❌ Sem verificação de governança
///     }
/// }
/// ```
///
/// Depois:
/// ```rust,ignore
/// use arkhe_nexus_governance::NexusGovernanceAdapter;
///
/// pub struct SafeCoreGuard {
///     governance: NexusGovernanceAdapter,
/// }
///
/// impl SafeCoreGuard {
///     pub fn execute(&self, proposal: GovernanceAction, action: AdminActionFn) {
///         self.governance.execute_admin_action(proposal, action)?;
///     }
/// }
/// ```
///
/// # Passo 2: Atualizar chamadores
///
/// Antes:
/// ```rust,ignore
/// guard.execute(AdminAction::UpdateKernel { ... });
/// ```
///
/// Depois:
/// ```rust,ignore
/// let proposal = NexusAdminAction::KernelUpdate.to_proposal(
///     "Atualizar kernel para v2.0".into(),
///     "did:arkhe:admin".into(),
///     48, // delay hours
///     [0u8; 32], // action hash
/// );
///
/// guard.execute_admin_action(proposal, |p| {
///     // Ação real do NEXUS
///     kernel.update()?;
///     Ok(())
/// })?;
/// ```
///
/// # Passo 3: Adicionar ao Cargo.toml do NEXUS
///
/// ```toml
/// [dependencies]
/// arkhe-nexus-governance = { path = "../arkhe-nexus-governance" }
/// ```
pub struct MigrationGuide;

/// Lista de stubs do NEXUS que precisam ser substituídos.
pub const STUBS_TO_REPLACE: &[&str] = &[
    "SafeCoreGuard::execute",
    "SafeCoreGuard::update_kernel",
    "SafeCoreGuard::modify_capsule",
    "SafeCoreGuard::update_compliance",
    "NEXUS::admin_action",
    "NEXUS::privileged_operation",
];

/// Verifica se um módulo ainda contém stubs não migrados.
pub fn check_migration_status(code: &str) -> Vec<&'static str> {
    STUBS_TO_REPLACE
        .iter()
        .filter(|stub| code.contains(**stub))
        .copied()
        .collect()
}
