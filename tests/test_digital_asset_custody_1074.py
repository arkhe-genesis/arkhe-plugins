import pytest
import aiohttp
from unittest.mock import AsyncMock, patch
from arkhe.substrates.digital_asset_custody_1074 import DigitalAssetCustodyBridge

class MockContextManager:
    def __init__(self, json_data, status=200):
        self.json_data = json_data
        self.status = status

    async def __aenter__(self):
        mock_resp = AsyncMock()
        mock_resp.status = self.status
        mock_resp.json = AsyncMock(return_value=self.json_data)
        return mock_resp

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        pass

@pytest.fixture
def bridge():
    return DigitalAssetCustodyBridge()

@pytest.mark.asyncio
async def test_get_validators_by_entity(bridge):
    mock_data = {
        "data": [
            {"pubkey": "0x123", "balance": 32000000000, "slashed": False},
            {"pubkey": "0x456", "balance": 31500000000, "slashed": True}
        ]
    }

    with patch("aiohttp.ClientSession.get", return_value=MockContextManager(mock_data)):
        validators = await bridge.get_validators_by_entity("AthenaFoundation")
        assert len(validators) == 2
        assert validators[0]["pubkey"] == "0x123"
        assert validators[1]["slashed"] is True

def test_compute_total_balance(bridge):
    validators = [
        {"balance": 32000000000},
        {"balance": 32000000000}
    ]
    total = bridge.compute_total_balance(validators)
    assert total == 64000000000

def test_check_slashing_risk(bridge):
    validators = [
        {"pubkey": "0x123", "slashed": False},
        {"pubkey": "0x456", "slashed": True}
    ]
    risky = bridge.check_slashing_risk(validators)
    assert len(risky) == 1
    assert risky[0]["pubkey"] == "0x456"

@pytest.mark.asyncio
async def test_generate_zk_proof_of_reserves(bridge):
    balances = [1000, 2000, 3000]
    declared_total = 5000
    result = await bridge.generate_zk_proof_of_reserves(balances, declared_total)

    assert result["status"] == "proof_generated"
    assert result["valid"] is True
    assert result["declared_total"] == 5000

    # Test failure case
    result_fail = await bridge.generate_zk_proof_of_reserves(balances, 7000)
    assert result_fail["status"] == "proof_failed"
    assert result_fail["valid"] is False

@pytest.mark.asyncio
async def test_submit_multisig_transaction(bridge):
    whitelist = ["0xallowed"]
    max_daily = 1000

    # Valid - below max daily
    res1 = await bridge.submit_multisig_transaction("0xunknown", 500, "0x", max_daily, whitelist)
    assert res1["status"] == "created"

    # Valid - on whitelist
    res2 = await bridge.submit_multisig_transaction("0xallowed", 5000, "0x", max_daily, whitelist)
    assert res2["status"] == "created"

    # Invalid - above max daily and not on whitelist
    res3 = await bridge.submit_multisig_transaction("0xunknown", 5000, "0x", max_daily, whitelist)
    assert res3["status"] == "rejected"
    assert "AXIARQUIA" in res3["reason"]

@pytest.mark.asyncio
async def test_approve_multisig_transaction(bridge):
    # Not yet executed
    res1 = await bridge.approve_multisig_transaction(1, "0xsigner1", 1, 3)
    assert res1["status"] == "approved"
    assert res1["approvals"] == 2
    assert res1["executed"] is False

    # Executed
    res2 = await bridge.approve_multisig_transaction(1, "0xsigner2", 2, 3)
    assert res2["status"] == "executed"
    assert res2["approvals"] == 3
    assert res2["executed"] is True
