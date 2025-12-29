"""Java target language configuration."""

from .base import LanguageTarget


class JavaTarget(LanguageTarget):
    """Java-specific configuration for the migration framework (Gradle build)."""

    @property
    def name(self) -> str:
        return "java"

    @property
    def file_extension(self) -> str:
        return ".java"

    def get_project_subdir(self, project_name: str) -> str:
        return f"{project_name}-java"

    def get_quality_gates(self) -> list[str]:
        return [
            "./gradlew compileJava",
            "./gradlew checkstyleMain || true",  # Don't fail on style initially
            "./gradlew test",
        ]

    def get_migrator_idioms(self) -> str:
        return """
## JAVA IDIOM REQUIREMENTS

Apply these patterns for modern, idiomatic Java (17+):

1. **Package structure**:
   - Follow Gradle standard layout: `src/main/java/`, `src/test/java/`
   - Package name: `com.rpn2tex` or similar
   - One public class per file, filename matches class name

2. **Naming conventions**:
   - PascalCase for classes and interfaces
   - camelCase for methods and variables
   - UPPER_SNAKE_CASE for constants
   - Package names all lowercase

3. **Documentation**:
   - Javadoc on all public classes and methods
   - Use `@param`, `@return`, `@throws` tags
   - Include usage examples in class-level Javadoc

4. **Modern Java features**:
   - Use `record` for immutable value types
   - Use `sealed` classes/interfaces where appropriate
   - Use pattern matching in switch expressions
   - Use `var` for local variables with obvious types

5. **Null safety**:
   - Use `Optional<T>` for nullable return values
   - Never return null from public methods
   - Use `Objects.requireNonNull()` for parameter validation

6. **Exception handling**:
   - Checked exceptions for recoverable conditions
   - Unchecked (RuntimeException) for programming errors
   - Create custom exception hierarchy if needed
   - Never catch and ignore exceptions

7. **Collections**:
   - Use interface types (List, Map, Set) not implementations
   - Prefer immutable collections (List.of(), Map.of())
   - Use Stream API for transformations
"""

    def get_reviewer_checks(self) -> str:
        return """
### Java-Specific Checks
- Proper exception handling (no empty catch blocks)
- Resources closed with try-with-resources
- No raw types (always use generics)
- Thread safety considerations documented
- No mutable static fields
- Proper equals/hashCode implementation for value types
- Optional used instead of returning null
"""

    def get_file_mapping(self, python_file: str) -> str:
        mappings = {
            "tokens.py": "Token.java",
            "ast_nodes.py": "Expr.java",
            "errors.py": "RpnException.java",
            "lexer.py": "Lexer.java",
            "parser.py": "Parser.java",
            "latex_gen.py": "LaTeXGenerator.java",
            "cli.py": "Main.java",
        }
        if python_file in mappings:
            return mappings[python_file]
        # Default: capitalize and change extension
        base = python_file.replace(".py", "")
        return "".join(word.capitalize() for word in base.split("_")) + ".java"

    def get_project_init_commands(self, project_dir: str) -> list[str]:
        return [
            f"cd {project_dir} && gradle init --type java-application --dsl kotlin --test-framework junit-jupiter --project-name rpn2tex --package com.rpn2tex",
        ]

    def get_source_dir(self, project_dir: str) -> str:
        return f"{project_dir}/src/main/java/com/rpn2tex"
