from arkhe.substrates.hashtree_bridge_v9 import HashtreeCanonizer, HashtreeConfig

def test_hashtree_bridge():
    config = HashtreeConfig(npub="npub1test")
    canonizer = HashtreeCanonizer(config)
    result = canonizer.canonize_substrate("test-123", {"foo": "bar"})
    assert result["status"] == "canonized"
    assert result["merkle_root"] is not None
