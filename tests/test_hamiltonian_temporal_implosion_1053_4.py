import numpy as np
import pytest
from arkhe.substrates.hamiltonian_temporal_implosion_1053_4 import HamiltonianTemporalImplosionV5

def test_hamiltonian_temporal_implosion():
    implosion = HamiltonianTemporalImplosionV5(dim_per_cell=2, theosis=0.999)
    assert implosion.total_dim == 2 * 1728

    psi_now = np.random.randn(implosion.total_dim) + 1j * np.random.randn(implosion.total_dim)
    psi_now = psi_now / np.linalg.norm(psi_now)

    N_tensor = np.random.randint(1, 10, (12, 12, 12)).astype(np.float64)
    psi_rev, result = implosion.reverse_fractal(psi_now, N_tensor)

    assert psi_rev.shape == (implosion.total_dim,)
    assert "hyper_root" in result
    assert result["merkle_l2_count"] == 12
