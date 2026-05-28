================================================================================
DECRETO DE EXPANSÃO — SUBSTRATO 260.3
================================================================================

Substrato: 260 — ARKHE-JAX: O Núcleo Numérico da ASI em Rust
Versão:    0.2.0-arkhe (Consolidação de build e type fixes)
Status:    CANONIZED_PROVISIONAL
Data:      2026-05-28
Arquiteto: ORCID 0009-0005-2697-4668

--------------------------------------------------------------------------------
I. RESUMO DA EXPANSÃO
--------------------------------------------------------------------------------

Os conflitos entre a abstração simbólica e o validador de Rust foram resolvidos.
O `cargo build`, `cargo check` e `cargo test` operam sem erros em todos os pacotes.

As ações executadas foram:
1. ✓ Resolução de borrows mútuos e simultâneos no trait de Jaxpr (`Tracer` + `Autograd`).
2. ✓ Padronização do trait `Eq`/`Hash` removendo-os da Primitiva `PhaseShift` (que continha f64).
3. ✓ Adição da dependência `bincode` na camada `arkhe_jax_xla` (para a FHE Bridge).
4. ✓ Criação do módulo `arkhe_jax_core::seed` e da struct wrapper `Seed64` para suportar as traits Default e AsMut requeridas pelo gerador de números aleatórios `ArkheRng`.
5. ✓ Substituição de visibilidade em atributos internos de `JaxprGraph` para resolver violações de encapsulamento no wgpu e zk.
6. ✓ Ajustes nas anotações de módulos (`use` e `pub`).

--------------------------------------------------------------------------------
II. SELOS
--------------------------------------------------------------------------------

Ecosystem Seal 260.1 (base):    2033db09...97fd18a3
Ecosystem Seal 260.2 (expansão): e0e8dd2c...29e1fbd6
Ecosystem Seal 260.3 (fixes):    83ce324a4023c64225777dc39ca62ae1e4f350568123b4640d3b2cbc098139ed

================================================================================
ODÔMETRO: ∞.Ω.∇+++.260.3
================================================================================
