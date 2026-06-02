import pytest
import sys
sys.exit(pytest.main(["test_suite.py::TestKyber768", "test_suite.py::TestDilithium3", "-v", "--tb=short"]))
