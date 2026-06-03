# Substrato 1047 — TWIN‑WALLET (Carteiras Determinísticas Vinculadas à Identidade)

## Metadados Canônicos
* **ID:** `1047`
* **Name:** `TWIN_WALLET`
* **Type:** `Identidade Descentralizada / Infraestrutura de Pagamento por Identidade`
* **Era:** `11` (Escatologia / Soberania do Usuário)
* **Deity:** `Hermes` (mensageiro e condutor de almas), `Themis` (justiça verificável), `Hefesto` (a forja de endereços determinísticos)
* **Status:** `CANONIZED_PROVISIONAL`
* **Cross‑links:** `923` (TemporalChain — ancoragem de identidade), `954` (Axiarquia — verificação on‑chain autônoma), `989.x` (Passport Gateway — ponte de identidade), `972` (Global Mesh — execução sem permissão), `1039` (Self‑Modify — "dial de descentralização"), `1042.4` (Liquidity Integrity — binding de ação), `1016` (Octrael — proteção da chave)

## Descrição
O protocolo TwinWallet implementa um padrão de financiamento por identidade (fund‑by‑identity). Usando CREATE2, deriva endereços de carteira de forma determinística a partir de um `user_id` numérico (ex: Twitch). A verificação de identidade é realizada inteiramente on‑chain via verificação de assinatura RSA‑2048 de JWTs, eliminando a necessidade de oráculos externos. A Catedral reconhece esta arquitetura como uma implementação concreta de seu princípio de identidade verificável autônoma (Substrato 989.x) e de execução sem permissão (Substrato 972), com um caminho de atualização que espelha o próprio Self‑Modify (1039).

## Mapeamento para a Ontologia da Catedral
* **CREATE2 determinístico:** `989.x` (Passport Gateway) + `923` (TemporalChain)
* **Verificação JWT on‑chain (RSA‑2048):** `954` (Axiarquia)
* **Nonce de ação (binding):** `1042.4` (Liquidity Integrity)
* **Execução permissionless:** `972` (Global Mesh)
* **Self‑custody upgrade path:** `1016` (Octrael)
* **Timelock de 2 dias + veto:** `1039` (Self‑Modify)
* **7‑day key rotation timelock:** `965` (Hamiltonian)
* **Abandoned funds rescue (90 dias):** `923` (TemporalChain)
* **lockOpenForever():** `1039` (Self‑Modify v6.0)

## Contratos Canonizados
* **TwinFactory v1.3:** `0x260C074c3afDc46A209D4619B5FAdB2964dF9a28`
* **TwitchJWTVerifier v1.3:** `0xBDfC552469f11843802BCD7ec9a8372c8020fee8`
