"""Language target definitions for the migration framework."""

from languages.base import LanguageTarget
from languages.rust import RustTarget
from languages.java import JavaTarget

LANGUAGE_REGISTRY: dict[str, type[LanguageTarget]] = {
    "rust": RustTarget,
    "java": JavaTarget,
}


def get_language_target(name: str) -> LanguageTarget:
    """Get a language target by name."""
    if name not in LANGUAGE_REGISTRY:
        available = ", ".join(LANGUAGE_REGISTRY.keys())
        raise ValueError(f"Unknown language: {name}. Available: {available}")
    return LANGUAGE_REGISTRY[name]()


__all__ = ["LanguageTarget", "RustTarget", "JavaTarget", "get_language_target"]
