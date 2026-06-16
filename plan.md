1. **Completar multi_agent.rs:** Criar o arquivo `cathedral-arkhe-v28.3/orchestrator/src/multi_agent.rs` contendo a lógica completa (em Rust) para o `MultiAgentOrchestrator` carregar as configurações de `config_loader.rs` (que vai instanciar a estrutura do orchestrator, aplicar o `planning.strategy`, `trust.require_memory_proof`, etc. aos agentes padrão).
2. **Criar Dockerfile.llm:** Criar o `cathedral-arkhe-v28.3/runtime/Dockerfile.llm` que define um servidor de inferência (baseado num vLLM compatível com a arquitetura definida).
3. **Adicionar arquivos de exemplo do modelo:** Criar `cathedral-arkhe-v28.3/core/model/chat_template.jinja` e `cathedral-arkhe-v28.3/core/model/generation_config.json`.
4. **Criar init.sh:** Criar o script `cathedral-arkhe-v28.3/init.sh` que garante a execução da stack, dando permissões e rodando o `docker-compose`.
5. **Completar passos pré-commit e submeter a PR.**
