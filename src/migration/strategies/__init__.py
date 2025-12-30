"""Migration strategies."""

from .base import MigrationStrategy
from .feature_by_feature import FeatureByFeatureStrategy
from .module_by_module import ModuleByModuleStrategy

STRATEGY_REGISTRY: dict[str, type[MigrationStrategy]] = {
    "module-by-module": ModuleByModuleStrategy,
    "feature-by-feature": FeatureByFeatureStrategy,
}


def get_strategy(name: str) -> MigrationStrategy:
    """Get a strategy by name."""
    if name not in STRATEGY_REGISTRY:
        available = ", ".join(STRATEGY_REGISTRY.keys())
        raise ValueError(f"Unknown strategy: {name}. Available: {available}")
    return STRATEGY_REGISTRY[name]()
