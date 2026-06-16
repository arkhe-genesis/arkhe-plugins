# Cathedral ARKHE v28.3 — Phase 7: Asynchronous RL Infrastructure (ASystem-like)
# Selo: CATHEDRAL-ARKHE-v28.3-ASYNC-RL-PHASE7-2026-06-16

A Fase 7 introduz a preparação para treinamento de agentes usando Reinforcement Learning Assíncrono, inspirado no ASystem (Ling & Ring 2.6). O objetivo é treinar agentes para planeamento e raciocínio de longo prazo em ambientes complexos.

## 1. Arquitetura de Rollout Assíncrono

O treinamento RL tradicional (como PPO) sofre com gargalos de sincronização quando os episódios têm durações variáveis (ex: geração de código vs consulta simples). O ASystem resolve isso desacoplando a geração (rollout) da otimização (update).

No Cathedral ARKHE, implementaremos isso com a seguinte arquitetura:

*   **Rollout Workers (CathedralAgent):** Múltiplas instâncias do agente exploram o ambiente assincronamente. Eles geram trajetórias (estado, ação, recompensa) e as enviam para um buffer global (Redis Streams ou memória compartilhada).
*   **Replay Buffer (Experience Memory):** Um banco de dados rápido que armazena as trajetórias geradas pelos Rollout Workers.
*   **Optimization Server:** Um servidor dedicado (com GPU) que consome lotes do Replay Buffer, calcula as atualizações de política (gradients) e publica os novos pesos do modelo.
*   **Weight Sync:** Os Rollout Workers atualizam seus pesos assincronamente (ex: a cada N episódios) a partir do Optimization Server.

## 2. Recompensas do Ambiente (Environment Rewards)

Para o "Native Agentic Training", precisamos de sinais de recompensa claros além da perda de linguagem cruzada.

*   **Recompensa de Sucesso na Execução (Code/Tools):** +1.0 se o código gerado executa sem erro e passa nos testes. -1.0 se houver erro de sintaxe.
*   **Recompensa de Consenso (Oracle):** +0.5 se o voto do agente alinhar com o consenso final da coalizão KPop.
*   **Penalidade de Redundância (Evo-CoT):** Pequena penalidade negativa por cada passo de raciocínio não essencial (para incentivar eficiência de token).
*   **Penalidade de Tempo/Custo:** Penalidade baseada no número de chamadas de API externas ou uso de tokens (incentiva soluções mais diretas).

## 3. Preparação da Infraestrutura

Para habilitar a Fase 7, as seguintes alterações estruturais são necessárias:

### 3.1 Redis Streams para Replay Buffer

Usaremos o Redis (já presente no docker-compose) para construir o buffer de trajetórias.

```yaml
# Em telemetry/rl-config.yaml (Novo Arquivo)
rl_infrastructure:
  replay_buffer:
    type: "redis_streams"
    url: "redis://redis:6379/1"
    stream_name: "cathedral:rl:trajectories"
    max_length: 100000
  optimization_server:
    batch_size: 128
    sync_interval_secs: 60
```

### 3.2 Atualização do Agent Loop para gerar Trajetórias

O `AgentResult` atual precisa ser estendido para capturar o "Reward" e a "Trajetória".

```rust
// Em orchestrator/src/agent_loop.rs (Preparação Futura)

pub struct TrajectoryStep {
    state_context: String, // Prompt + Memória
    action: String,        // Tool Call ou Reasoning Step
    reward: f32,           // Calculado pós-execução
}

pub struct AgentResult {
    pub final_answer: String,
    pub steps_taken: u32,
    pub tools_used: Vec<String>,
    pub latency_secs: f64,
    pub memory_consolidated: bool,
    pub reasoning_mode: ReasoningMode,

    // Novo para Fase 7
    pub trajectory: Vec<TrajectoryStep>,
    pub total_reward: f32,
}
```

### 3.3 Serviço de Otimização (Optimization Worker)

Um novo serviço docker, `rl-optimizer`, será adicionado. Ele lerá do Redis, construirá os tensores PPO (ou DPO/KTO dependendo do algoritmo), fará o backpropagation e salvará o modelo atualizado via LoRA adapters.

```yaml
# Adição ao docker-compose.yml (Preparação Futura)
  rl-optimizer:
    image: cathedral-rl-optimizer:latest
    environment:
      - REDIS_URL=redis://redis:6379
      - MODEL_WEIGHTS_DIR=/models
    volumes:
      - ./models:/models
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
```

## 4. Próximos Passos (Fase 7 Ativa)

1.  **Implementar o `TrajectoryRecorder`:** Um componente em Rust que pega a saída do ReActPlanner e a formata como tensores/JSON para o Redis.
2.  **Definir as Funções de Recompensa (Reward Functions):** Criar validadores específicos (ex: um linter embutido, verificador de asserts) que atribuem pontuações reais às ações do Coder/Oracle.
3.  **Construir o `rl-optimizer`:** Um script Python usando `trl` (Transformers Reinforcement Learning) da HuggingFace, configurado para ler do Redis e aplicar PPO ou DPO assincronamente.
