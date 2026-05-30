import pytest
import asyncio
from unittest.mock import patch, MagicMock
from node.passport_gateway import PassportGateway, HumanityProof

@pytest.fixture
def gateway():
    gw = PassportGateway()
    return gw

@pytest.mark.asyncio
async def test_is_human_with_high_score(gateway):
    # Mocking the HTTP requests
    mock_session = MagicMock()
    gateway.session = mock_session

    # Mock get_passport_score response
    def mock_score_get(*args, **kwargs):
        class MockResponse:
            status = 200
            async def json(self):
                return {"score": 25.0}
            async def __aenter__(self):
                return self
            async def __aexit__(self, exc_type, exc_val, exc_tb):
                pass
        return MockResponse()

    # Mock get_passport_stamps response
    def mock_stamps_get(*args, **kwargs):
        class MockResponse:
            status = 200
            async def json(self):
                return {"items": [{"credential": {"credentialSubject": {"provider": "Github"}}}]}
            async def __aenter__(self):
                return self
            async def __aexit__(self, exc_type, exc_val, exc_tb):
                pass
        return MockResponse()

    # Apply mocks conditionally based on URL
    def side_effect(*args, **kwargs):
        url = args[0] if args else kwargs.get("url", "")
        if "/score/" in url:
            return mock_score_get()
        elif "/stamps/" in url:
            return mock_stamps_get()

    mock_session.get.side_effect = side_effect

    proof = await gateway.is_human("0xRandomAddress")

    assert proof.is_human == True
    assert proof.score == 1.0 # Max normalized score is 1.0
    assert "Github" in proof.stamps
    assert proof.orcid_verified == False

@pytest.mark.asyncio
async def test_verify_dao_voter_with_orcid(gateway):
    # Setup mock to return low score but address starts with 0xAlice
    mock_session = MagicMock()
    gateway.session = mock_session

    def side_effect(*args, **kwargs):
        url = args[0] if args else kwargs.get("url", "")
        class MockResponse:
            status = 200
            async def json(self):
                if "/score/" in url:
                    return {"score": 5.0}
                elif "/stamps/" in url:
                    return {"items": []}
            async def __aenter__(self):
                return self
            async def __aexit__(self, exc_type, exc_val, exc_tb):
                pass
        return MockResponse()

    mock_session.get.side_effect = side_effect

    can_vote = await gateway.verify_dao_voter("0xAliceTheVoter")

    assert can_vote == True # Since address starts with 0xAlice, orcid_verified will be True

@pytest.mark.asyncio
async def test_is_human_with_low_score(gateway):
    # Setup mock to return low score
    mock_session = MagicMock()
    gateway.session = mock_session

    def side_effect(*args, **kwargs):
        url = args[0] if args else kwargs.get("url", "")
        class MockResponse:
            status = 200
            async def json(self):
                if "/score/" in url:
                    return {"score": 10.0}
                elif "/stamps/" in url:
                    return {"items": []}
            async def __aenter__(self):
                return self
            async def __aexit__(self, exc_type, exc_val, exc_tb):
                pass
        return MockResponse()

    mock_session.get.side_effect = side_effect

    proof = await gateway.is_human("0xRandomAddress2")

    assert proof.is_human == False
    assert proof.score == 0.5 # 10.0 / 20.0
    assert proof.orcid_verified == False
