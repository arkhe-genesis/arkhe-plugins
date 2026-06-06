import time
import os
import json
from prometheus_client import start_http_server, Gauge
from web3 import Web3

RPC_URL = os.environ.get("RPC_URL", "http://127.0.0.1:8545")
ORACLE_ADDRESS = os.environ.get("ORACLE_ADDRESS")
JUROR_ADDRESSES = os.environ.get("JUROR_ADDRESSES", "").split(",")

theosis_score = Gauge('pnk_theosis_score', 'Theosis score of jurors', ['juror'])

# Minimal ABI for PNKTheosisOracle getScore
ORACLE_ABI = json.loads('[{"inputs":[{"internalType":"address","name":"user","type":"address"}],"name":"getScore","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]')

def metrics_loop():
    w3 = Web3(Web3.HTTPProvider(RPC_URL))
    if not w3.is_connected():
        print("Failed to connect to Web3 node")
        return

    if not ORACLE_ADDRESS:
        print("ORACLE_ADDRESS environment variable not set")
        return

    contract = w3.eth.contract(address=ORACLE_ADDRESS, abi=ORACLE_ABI)

    while True:
        try:
            for juror in JUROR_ADDRESSES:
                if juror:
                    score = contract.functions.getScore(w3.to_checksum_address(juror)).call()
                    theosis_score.labels(juror=juror).set(score)
        except Exception as e:
            print(f"Error fetching scores: {e}")

        time.sleep(15)

if __name__ == "__main__":
    start_http_server(8000)
    print("Prometheus exporter running on port 8000")
    metrics_loop()
