// scripts/audit_legacy.rs
//! IA-Assisted Legacy Analysis (Análise de Legado com IA)

fn main() {
    let age = 5;
    let code = "/* old code */";

    let prompt = format!(
        r#"
        This code was written over {} years ago.
        Analyze it for vulnerabilities that would be missed by modern linters:
        1. Historical C/C++ idiosyncrasies (if any)
        2. Assumptions about standard library behavior that may have changed
        3. Patterns that were safe in 1997 but are unsafe in 2026
        4. Implicit type conversions that could overflow

        Code:
        {}
        "#,
        age, code
    );

    println!("Prompting IA for legacy code analysis:");
    println!("{}", prompt);
}
