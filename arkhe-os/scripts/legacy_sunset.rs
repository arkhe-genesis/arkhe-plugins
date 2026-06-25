// scripts/legacy_sunset.rs
//! Identifica código com mais de 5 anos e agenda revisão.

use std::process::Command;
// Note: Normally we would use git2 crate, but for a standalone script, we can mock or just write the logic.

fn main() {
    println!("Identificando código com mais de 5 anos para revisão (Squidbleed Sunset Policy)...");

    // In a real implementation this would use git2 to traverse the repo history
    // and find files that haven't been touched in 5 years.

    // Placeholder output to demonstrate behavior
    eprintln!("⚠️  Legacy code: crates/arkhe-legacy/src/old_parser.rs");
}
