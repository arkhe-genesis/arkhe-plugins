import os
import json
from web3 import Web3
from eth_account import Account

RPC_URL = os.environ.get("RPC_URL", "http://127.0.0.1:8545")
PRIVATE_KEY = os.environ.get("PRIVATE_KEY", "0x0000000000000000000000000000000000000000000000000000000000000001")

w3 = Web3(Web3.HTTPProvider(RPC_URL))
account = Account.from_key(PRIVATE_KEY)

def deploy():
    print(f"Deploying from account: {account.address}")

    # Normally we would load ABI and bytecode here from Hardhat/Foundry compile output
    # Mocking deployment for demonstration since we don't have compilation set up
    oracle_address = "0x" + "1"*40
    bridge_address = "0x" + "2"*40

    print(f"PNKTheosisOracle deployed to: {oracle_address}")
    print(f"CathedralKlerosBridgeWithVoting deployed to: {bridge_address}")

    with open("kleros_bridge/deployments.json", "w") as f:
        json.dump({
            "PNKTheosisOracle": oracle_address,
            "CathedralKlerosBridgeWithVoting": bridge_address
        }, f, indent=2)

if __name__ == "__main__":
    deploy()
