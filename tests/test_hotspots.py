import pytest
from arkhe.substrates.interaction_hotspots import InteractionHotspotsAnalyzer

@pytest.mark.asyncio
async def test_analyzer_simulate():
    analyzer = InteractionHotspotsAnalyzer()
    res = await analyzer.analyze_trajectory("dummy.xtc", "dummy.pdb")
    assert res.num_atoms == 166
    assert res.mean_log_deviation > 0
