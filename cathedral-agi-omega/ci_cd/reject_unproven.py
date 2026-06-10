import os
import sys

def main():
    # In a real environment, this would parse git diff or PR files.
    # Here we mock the behavior for the CI to check if any critical file changes lack a .lean proof.

    # Mock changed files from environment or arguments
    changed_files = sys.argv[1:] if len(sys.argv) > 1 else []

    critical_dirs = [
        "ZK_REASONING_ENGINE/circuits",
        "COGNITIVE_CORTEX/agents",
        "DISTRIBUTED_COMPUTATION"
    ]

    critical_changes = [f for f in changed_files if any(d in f for d in critical_dirs)]
    lean_changes = [f for f in changed_files if f.endswith(".lean")]

    if critical_changes and not lean_changes:
        print("ERROR: PR touches critical directories but no Lean 4 proof (.lean) is attached.")
        print("Critical files changed:", critical_changes)
        sys.exit(1)

    print("CI Check Passed. Proofs are sufficient or no critical files changed.")
    sys.exit(0)

if __name__ == "__main__":
    main()
