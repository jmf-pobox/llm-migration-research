"""Language target definitions for the migration framework."""

from .base import LanguageTarget
from .go import GoTarget
from .java import JavaTarget
from .rust import RustTarget

LANGUAGE_REGISTRY: dict[str, type[LanguageTarget]] = {
    "rust": RustTarget,
    "java": JavaTarget,
    "go": GoTarget,
}


def get_language_target(name: str) -> LanguageTarget:
    """Get a language target by name."""
    if name not in LANGUAGE_REGISTRY:
        available = ", ".join(LANGUAGE_REGISTRY.keys())
        raise ValueError(f"Unknown language: {name}. Available: {available}")
    return LANGUAGE_REGISTRY[name]()


__all__ = [
    "LANGUAGE_REGISTRY",
    "GoTarget",
    "JavaTarget",
    "LanguageTarget",
    "RustTarget",
    "get_language_target",
]
