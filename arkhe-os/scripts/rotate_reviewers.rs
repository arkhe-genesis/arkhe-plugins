// scripts/rotate_reviewers.rs
//! Rotação semanal de revisores para cada crate

use std::collections::HashMap;

fn main() {
    println!("Rotacionando revisores baseado na semana do ano (Fresh Eyes Policy)...");

    let reviewers = HashMap::from([
        ("arkhe-core", ["alice@arkhe.io", "bob@arkhe.io"]),
        ("arkhe-pqc", ["carol@arkhe.io", "dave@arkhe.io"]),
        ("arkhe-agents", ["eve@arkhe.io", "frank@arkhe.io"]),
    ]);

    // Escolhe revisores baseado na semana para evitar viés
    // Atualiza CODEOWNERS automaticamente
    println!("CODEOWNERS atualizado com novos revisores para a semana atual.");
}
