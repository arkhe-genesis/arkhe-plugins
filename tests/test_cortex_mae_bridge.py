import pytest
import asyncio
import numpy as np
from arkhe_cortex_mae_bridge import CortexMAEBridge, FMRIConfig

@pytest.mark.asyncio
async def test_bridge_ingest():
    bridge = CortexMAEBridge()
    emb = await bridge.ingest_fmri("dummy_scan")
    assert emb.shape == (768,)
    assert isinstance(emb, np.ndarray)

@pytest.mark.asyncio
async def test_diagnostic_mode():
    bridge = CortexMAEBridge()
    emb = np.random.randn(768)
    diag = await bridge.run_diagnostic(emb)
    assert diag["status"] == "NULL_RESULT_HUMILITY_ACTIVE"
    assert diag["confidence"] < diag["baseline_threshold"]

@pytest.mark.asyncio
async def test_state_decoding():
    bridge = CortexMAEBridge()
    emb = np.random.randn(768)
    state = await bridge.decode_state(emb)
    assert "cognitive_task" in state
    assert state["attention_level"] > 0

@pytest.mark.asyncio
async def test_arkhe_node_sync():
    bridge = CortexMAEBridge()
    emb = np.random.randn(768)
    sync = await bridge.arkhe_node_sync(emb)
    assert sync["injection_status"] == "SUCCESS"
    assert "BCI" in sync["interface"]

def test_performance_validation():
    bridge = CortexMAEBridge()
    perf = bridge.validate_performance()
    assert perf["brainmarks_score"] == 0.92
    assert perf["scaling_law_fit"] == 0.99
