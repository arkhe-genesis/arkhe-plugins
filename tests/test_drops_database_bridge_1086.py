import pytest
import asyncio
from datetime import datetime
from arkhe.substrates.drops_database_bridge_1086 import DropsBridgeOrchestrator

@pytest.fixture
def orchestrator():
    return DropsBridgeOrchestrator()

def test_fuse_driver(orchestrator):
    inode = orchestrator.fuse_driver.exec_query("SELECT * FROM users", (1,))
    assert inode.startswith("/cathedral/drops/queries/")
    stats = orchestrator.fuse_driver.get_inode_stats()
    assert stats["total_inodes"] == 1
    assert stats["total_queries"] == 1

def test_typed_column_kernel(orchestrator):
    kernel = orchestrator.typed_kernel.register_column("users", "id", "int64", "int64")
    assert kernel["rkhs_dimension"] == 64
    eval_val = orchestrator.typed_kernel.evaluate_kernel("users", "id", 10, 10)
    assert eval_val == 1.0 # diff is 0, exp(0) is 1.0
    report = orchestrator.typed_kernel.get_kernel_report()
    assert report["total_kernels"] == 1

def test_proof_refactor(orchestrator):
    ast = orchestrator.proof_refactor.extract_expression("Select", "SELECT *", [])
    assert ast["expr_type"] == "Select"
    assert "lean_lemma" in ast
    assert "lean_theorem" in ast
    report = orchestrator.proof_refactor.get_extraction_report()
    assert report["total_expressions"] == 1
    assert report["total_lemmas"] == 1
    assert report["total_theorems"] == 1

def test_dna_decoder(orchestrator):
    orchestrator.dna_decoder.register_primer("drop:user_id", "id")
    row = {"id": 42}
    decoded = orchestrator.dna_decoder.decode_row(row, ["user_id"])
    assert decoded["user_id"] == 42
    report = orchestrator.dna_decoder.get_decoding_report()
    assert report["total_decoded"] == 1
    assert report["avg_match_rate"] == 1.0

def test_theosis_dashboard(orchestrator):
    orchestrator.theosis_dashboard.record_query("SELECT", "query_1", 50.0)
    dash = orchestrator.theosis_dashboard.get_dashboard()
    assert dash["total_queries"] == 1
    assert dash["current_theosis"] > 0.5 # should increase slowly

    orchestrator.theosis_dashboard.record_query("SELECT", "query_slow", 500.0)
    dash = orchestrator.theosis_dashboard.get_dashboard()
    assert dash["slow_queries"] == 1
    assert dash["current_theosis"] < 1.0 # should decrease

def test_tx_rollup(orchestrator):
    tx = orchestrator.tx_rollup.begin_transaction("tx1")
    assert tx["tx_id"] == "tx1"
    op_hash = orchestrator.tx_rollup.add_operation("tx1", "INSERT", ())
    assert op_hash is not None
    commit = orchestrator.tx_rollup.commit_transaction("tx1")
    assert commit["status"] == "COMMITTED"
    assert "merkle_root" in commit
    report = orchestrator.tx_rollup.get_rollup_report()
    assert report["total_transactions"] == 1
    assert report["committed"] == 1

def test_tx_rollup_rollback(orchestrator):
    orchestrator.tx_rollup.begin_transaction("tx2")
    rollback = orchestrator.tx_rollup.rollback_transaction("tx2")
    assert rollback["status"] == "ROLLED_BACK"
    assert "nullifier" in rollback
    report = orchestrator.tx_rollup.get_rollup_report()
    assert report["rolled_back"] == 1

def test_meta_extract(orchestrator):
    current = {"tables": {"users": {"columns": {"id": {"type": "int64"}}}}}
    desired = {"tables": {"users": {"columns": {"id": {"type": "int64"}, "name": {"type": "text"}}}}}
    stmts = orchestrator.meta_extract.diff_schema(current, desired)
    assert len(stmts) == 1
    assert "ADD COLUMN" in stmts[0]
    report = orchestrator.meta_extract.get_meta_extract_report()
    assert report["total_migrations"] == 1

def test_wormgraph(orchestrator):
    orchestrator.wormgraph.add_table_node("users", ["id"])
    orchestrator.wormgraph.add_table_node("posts", ["id"])
    edge = orchestrator.wormgraph.add_relation_edge("users", "posts", "HasMany", "user_id", "id")
    assert edge["relation_type"] == "HasMany"
    metrics = orchestrator.wormgraph.compute_wormgraph_metrics()
    assert metrics["nodes"] == 2
    assert metrics["edges"] == 1

def test_pgvector_bridge(orchestrator):
    kernel = orchestrator.pgvector_bridge.register_vector_column("items", "emb", 10)
    assert kernel["rkhs_dimension"] == 20
    results = orchestrator.pgvector_bridge.search_vector("items", "emb", [0.1]*10, 5)
    assert len(results) == 1
    assert results[0]["ensemble_confidence"] > 0
    report = orchestrator.pgvector_bridge.get_vector_report()
    assert report["total_searches"] == 1

def test_qdrant_oracle(orchestrator):
    orchestrator.qdrant_oracle.create_collection("col1", 10)
    res = orchestrator.qdrant_oracle.search_with_filter("col1", [0.1]*10, [{"op": "Must", "field": "a", "value": 1}], 5)
    assert res["gate_axiarquia"] == "PASS"
    assert len(res["results"]) == 5
    report = orchestrator.qdrant_oracle.get_oracle_report()
    assert report["total_collections"] == 1
    assert report["total_queries"] == 1

def test_cache_checkpoint(orchestrator):
    # Set with low max_entries to test eviction
    orchestrator.cache_checkpoint.max_entries = 1
    orchestrator.cache_checkpoint.set_cache("k1", "v1", 100)
    hit = orchestrator.cache_checkpoint.get_cache("k1")
    assert hit is not None

    # Second should evict first
    orchestrator.cache_checkpoint.set_cache("k2", "v2", 100)
    miss = orchestrator.cache_checkpoint.get_cache("k1")
    assert miss is None
    hit2 = orchestrator.cache_checkpoint.get_cache("k2")
    assert hit2 is not None

    report = orchestrator.cache_checkpoint.get_checkpoint_report()
    assert report["total_entries"] == 1
    assert "M" in report["state_counts"]

def test_redis_liquidity(orchestrator):
    acq = orchestrator.redis_liquidity.acquire_connection("c1")
    assert acq["status"] == "ACQUIRED"
    rel = orchestrator.redis_liquidity.release_connection("c1")
    assert rel["status"] == "RELEASED"
    assert rel["merkle_leaf"] is not None
    report = orchestrator.redis_liquidity.get_liquidity_report()
    assert report["total_ticks"] == 1

    # test throttling
    orchestrator.redis_liquidity.max_conns = 1
    orchestrator.redis_liquidity.acquire_connection("c2")
    rej = orchestrator.redis_liquidity.acquire_connection("c3")
    assert rej["status"] == "REJECTED"

def test_clickhouse_hamiltonian(orchestrator):
    part = orchestrator.clickhouse_hamiltonian.create_partition("p1", [{"a": 1}])
    assert part["partition_key"] == "p1"
    op1 = orchestrator.clickhouse_hamiltonian.apply_prewhere("p1", "a > 0")
    assert op1["type"] == "PREWHERE"
    op2 = orchestrator.clickhouse_hamiltonian.apply_sample("p1", 0.1)
    assert op2["type"] == "SAMPLE"
    op3 = orchestrator.clickhouse_hamiltonian.apply_asof_join("p1", "p2", "t")
    assert op3["type"] == "ASOF_JOIN"
    op4 = orchestrator.clickhouse_hamiltonian.apply_aggregate("p1", "sum", "a")
    assert op4["type"] == "AGGREGATE"
    report = orchestrator.clickhouse_hamiltonian.get_hamiltonian_report()
    assert report["total_partitions"] == 1
    assert report["total_operators"] == 4

def test_dashboard(orchestrator):
    dash = orchestrator.get_dashboard()
    assert dash["substrato"] == "1086"
    assert "seal" in dash
    assert "timestamp" in dash
    assert dash["nome"] == "DROPS-DATABASE-BRIDGE"
