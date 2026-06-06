import json

def generate_config():
    config = {
        "source_chain": "Arbitrum",
        "target_chain": "RBB",
        "relay_interval_seconds": 3600,
        "oracle_address": "0x123..."
    }
    with open("vea_relay.json", "w") as f:
        json.dump(config, f, indent=2)
    print("Vea Relay config generated.")

if __name__ == "__main__":
    generate_config()
