#!/usr/bin/env python3
"""
Deploy Script for CathedralKlerosBridge (Arbitrum + RBB)
This script orchestrates the deployment of the Kleros Bridge components
to Arbitrum (and potentially RBB via Vea Relay).
"""

import subprocess
import os

def deploy_contracts():
    print("=" * 70)
    print("🚀 Deploying CathedralKlerosBridge to Arbitrum + RBB")
    print("=" * 70)

    # In a real environment, we would use --network arbitrum or similar
    cmd = ["npx", "hardhat", "run", "scripts/deploy.js"]

    try:
        # Run deployment
        result = subprocess.run(cmd, check=True, capture_output=True, text=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print("Deployment failed:")
        print(e.stderr)
        return False

    print("\n✅ Deployment to Arbitrum initiated.")
    return True

def configure_relay():
    print("\n" + "=" * 70)
    print("🌉 Configuring Vea Relay (Arbitrum -> RBB)")
    print("=" * 70)

    cmd = ["npx", "hardhat", "run", "scripts/configure_vea_relay.js"]

    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print("Relay configuration failed:")
        print(e.stderr)
        return False

    return True

if __name__ == "__main__":
    current_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(current_dir)

    success_deploy = deploy_contracts()
    if success_deploy:
        configure_relay()
