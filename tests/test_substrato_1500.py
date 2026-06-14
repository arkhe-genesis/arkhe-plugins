import pytest
import asyncio

from arkhe.substrates.sovereign_cognitive_loop_1500 import CathedralOrchestratorV14_0_0, GgufInferenceEngine, InferenceRouter, PrometheusRegistry

@pytest.mark.asyncio
async def test_substrato_1500_components():
    prom = PrometheusRegistry()
    prom.gauge("test_gauge", 1.0)
    prom.counter_inc("test_counter", 1.0)
    out = prom.render()
    assert "test_gauge" in out
    assert "test_counter" in out

    config = {
        "gguf": {"model_id": "test", "quant": "test", "n_ctx": 100},
        "prometheus": {"port": 9091}
    }

    orch = CathedralOrchestratorV14_0_0(config)

    assert orch.gguf is not None
    assert orch.prom is not None
    assert orch.router is not None
