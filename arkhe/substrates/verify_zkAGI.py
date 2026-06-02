#!/usr/bin/env python3
import hashlib
import json
import torch
from zk_agi_model import ZkAGIModel

def generate_commitment(tensor):
    """Generates a SHA256 commitment of a PyTorch tensor"""
    tensor_bytes = tensor.detach().cpu().numpy().tobytes()
    return hashlib.sha256(tensor_bytes).hexdigest()

def verify_commitments(model, metadata_path):
    print(f"Verifying commitments against {metadata_path}...")
    with open(metadata_path, 'r') as f:
        metadata = json.load(f)

    commitments = metadata.get('tensor_commitments', {})

    verified_count = 0
    failed_count = 0

    for name, param in model.named_parameters():
        if name in commitments:
            computed_commitment = generate_commitment(param)
            expected_commitment = commitments[name]
            if computed_commitment == expected_commitment:
                print(f"✓ Verified: {name}")
                verified_count += 1
            else:
                print(f"✗ Failed: {name} (expected {expected_commitment}, got {computed_commitment})")
                failed_count += 1
        else:
            print(f"? Missing commitment for: {name}")

    print(f"\nVerification Complete: {verified_count} verified, {failed_count} failed.")
    return failed_count == 0

if __name__ == "__main__":
    model = ZkAGIModel(vocab_size=32000)
    # in practice, you'd load state_dict here

    verify_commitments(model, "zk_agi_metadata.json")
