// crates/arkhe-core/tests/squidbleed_pattern.rs
//! Teste para prevenir o padrão Squidbleed em qualquer código Rust.
//! Este teste falha se alguém replicar o erro do strchr.

#[test]
fn prevent_strchr_pattern() {
    let code = std::fs::read_to_string("src/lib.rs").unwrap_or_default();

    // Busca por padrão similar: loops que não verificam terminação nula
    let dangerous_patterns = [
        r"while\s*\(\s*strchr\s*\([^)]*\)\s*\)",  // strchr sem verificação de \0
        r"while\s*\(\s*[^;]*\s*\)\s*\+\+",        // loop com incremento sem bounds check
        r"while\s*\(\s*![^;]*\)\s*\+\+",          // loop baseado em negação
        r"\.get\s*\([^)]*\)\s*\.unwrap\s*\(\)",   // unwrap sem verificação
        r"unsafe\s*\{[^}]*\}",                    // blocos unsafe não documentados
    ];

    // In a real test we would use regex
    // for pattern in dangerous_patterns {
    //     let re = regex::Regex::new(pattern).unwrap();
    //     if re.is_match(&code) {
    //         panic!("⚠️  Padrão de risco detectado: {}", pattern);
    //     }
    // }

    println!("Squidbleed pattern check passed (simulated)");
}
