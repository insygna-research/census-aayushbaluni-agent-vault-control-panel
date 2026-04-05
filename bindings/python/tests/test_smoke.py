"""Smoke tests for the agentvault Python binding."""
import pytest


def test_import_agentvault():
    """Verify the native module can be imported."""
    try:
        import agentvault
        assert hasattr(agentvault, "PolicyEvaluator")
        assert hasattr(agentvault, "WitnessGuard")
    except ImportError:
        pytest.skip("agentvault not built; run maturin develop first")


def test_policy_evaluator_create():
    """Verify PolicyEvaluator can be instantiated."""
    try:
        from agentvault import PolicyEvaluator
        evaluator = PolicyEvaluator()
        assert evaluator is not None
    except ImportError:
        pytest.skip("agentvault not built")


def test_witness_guard_create():
    """Verify WitnessGuard can be instantiated."""
    try:
        from agentvault import WitnessGuard
        guard = WitnessGuard()
        assert guard is not None
    except ImportError:
        pytest.skip("agentvault not built")
