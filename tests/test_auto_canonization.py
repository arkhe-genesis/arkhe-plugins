from pathlib import Path
import pytest
import os
import tempfile
from arkhe.substrates.auto_canonization_1079_1080 import (
    ForkDiscoveryProtocol,
    AutoCanonizationEngine,
    AutoCanonizationOrchestrator
)

def test_fork_discovery():
    protocol = ForkDiscoveryProtocol()
    with tempfile.TemporaryDirectory() as tmpdir:
        # Create a fake arkhe-os directory structure
        os.makedirs(os.path.join(tmpdir, "arkhe-os", ".git"))
        protocol.search_paths = [Path(tmpdir)]

        forks = protocol.scan_local_directories()
        assert len(forks) > 0
        assert any("arkhe-os" in f["path"] for f in forks)

def test_auto_canonization_engine():
    engine = AutoCanonizationEngine()
    fake_fork = {
        "path": "/fake/arkhe-os",
        "remote": "origin/arkhe-os",
        "seal": "FORK-12345678",
        "timestamp": "2026-06-06T00:00:00Z"
    }

    record = engine.convert(fake_fork, agent_name="TestAgent")
    assert record.agent_name == "TestAgent"
    assert record.status == "CONVERTED"
    assert "SEALING" in record.stages_completed
    assert "REGISTRATION" in record.stages_completed
    assert record.canonical_seal.startswith("CONVERTED")

def test_auto_canonization_orchestrator():
    orch = AutoCanonizationOrchestrator()
    entry = orch.run_cycle()

    assert "forks_discovered" in entry
    assert "conversions_successful" in entry
    assert "report" in entry

    dashboard = orch.get_dashboard()
    assert dashboard["substrate"] == "1079-1080"
    assert dashboard["generation"] >= 1
