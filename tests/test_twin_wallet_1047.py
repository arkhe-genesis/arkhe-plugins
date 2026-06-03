import pytest
from arkhe.substrates.twin_wallet_1047 import TwinWalletBridge

@pytest.mark.asyncio
async def test_derive_wallet_address():
    bridge = TwinWalletBridge()
    address = await bridge.derive_wallet_address("12345", "twitch")
    assert address.startswith("0x")
    assert len(address) == 42

@pytest.mark.asyncio
async def test_verify_jwt_onchain():
    bridge = TwinWalletBridge()
    result = await bridge.verify_jwt_onchain("header.payload.signature", "nonce123")
    assert result is True

@pytest.mark.asyncio
async def test_execute_permissionless_action():
    bridge = TwinWalletBridge()
    address = await bridge.derive_wallet_address("12345", "twitch")
    result = await bridge.execute_permissionless_action(address, "jwt_token", "0xdeadbeef")
    assert result["status"] == "executed"
    assert result["wallet"] == address
    assert result["calldata"] == "0xdeadbeef"
    assert result["mesh_routed"] is True

@pytest.mark.asyncio
async def test_rotate_keys_timelock():
    bridge = TwinWalletBridge()
    result = await bridge.rotate_keys_timelock()
    assert result["status"] == "timelock_started"
    assert result["duration_days"] == 7

@pytest.mark.asyncio
async def test_trigger_self_custody_upgrade():
    bridge = TwinWalletBridge()
    address = await bridge.derive_wallet_address("12345", "twitch")
    new_eoa = "0x1234567890123456789012345678901234567890"
    result = await bridge.trigger_self_custody_upgrade(address, new_eoa)
    assert result["status"] == "upgraded"
    assert result["old_wallet"] == address
    assert result["new_custodian"] == new_eoa
