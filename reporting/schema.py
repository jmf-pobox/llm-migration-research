"""Canonical metrics schema for migration analysis.

This module defines the standardized data structures for capturing
and analyzing migration metrics at scale.
"""

from dataclasses import dataclass, field, asdict
from datetime import datetime
from enum import Enum
from typing import Optional
import json
import uuid


SCHEMA_VERSION = "1.0.0"


class MigrationStatus(Enum):
    """Outcome status of a migration run."""
    SUCCESS = "success"
    PARTIAL = "partial"
    FAILURE = "failure"


class MigrationStrategy(Enum):
    """Migration strategy used."""
    MODULE_BY_MODULE = "module-by-module"
    FEATURE_BY_FEATURE = "feature-by-feature"


@dataclass
class IdentityMetrics:
    """Identifies a specific migration run."""
    run_id: str
    project_name: str
    source_language: str
    target_language: str
    strategy: str
    started_at: str  # ISO 8601
    completed_at: Optional[str] = None  # ISO 8601
    host_platform: Optional[str] = None
    sdk_version: Optional[str] = None
    model_id: Optional[str] = None

    @classmethod
    def create(
        cls,
        project_name: str,
        source_language: str,
        target_language: str,
        strategy: str,
    ) -> "IdentityMetrics":
        """Create new identity with generated run_id and current timestamp."""
        return cls(
            run_id=str(uuid.uuid4()),
            project_name=project_name,
            source_language=source_language,
            target_language=target_language,
            strategy=strategy,
            started_at=datetime.now().isoformat(),
        )


@dataclass
class ModuleTiming:
    """Timing for a single module migration."""
    module_name: str
    duration_ms: int
    attempts: int = 1


@dataclass
class TimingMetrics:
    """Timing information for the migration."""
    wall_clock_duration_ms: int = 0
    api_duration_ms: int = 0
    phase_durations_ms: dict[str, int] = field(default_factory=dict)
    module_durations: list[ModuleTiming] = field(default_factory=list)


@dataclass
class CostMetrics:
    """Cost breakdown for the migration."""
    total_cost_usd: float = 0.0
    input_tokens_cost_usd: float = 0.0
    output_tokens_cost_usd: float = 0.0
    cache_creation_cost_usd: float = 0.0


@dataclass
class TokenMetrics:
    """Token usage statistics."""
    input_tokens: int = 0
    output_tokens: int = 0
    cache_creation_input_tokens: int = 0
    cache_read_input_tokens: int = 0

    @property
    def cache_efficiency_ratio(self) -> float:
        """Ratio of cache reads to total input tokens."""
        total = self.cache_read_input_tokens + self.input_tokens
        if total == 0:
            return 0.0
        return self.cache_read_input_tokens / total


@dataclass
class AgentMetrics:
    """Agent and tool usage statistics."""
    total_turns: int = 0
    total_messages: int = 0
    subagent_invocations: dict[str, int] = field(default_factory=dict)
    tool_invocations: dict[str, int] = field(default_factory=dict)
    error_recovery_events: int = 0
    retry_count: int = 0


@dataclass
class CodeMetrics:
    """Code complexity and size metrics."""
    production_loc: int = 0
    test_loc: int = 0
    total_loc: int = 0
    module_count: int = 0
    function_count: int = 0
    avg_cyclomatic_complexity: float = 0.0
    max_cyclomatic_complexity: int = 0
    external_dependencies: int = 0
    maintainability_index: Optional[float] = None  # 0-100, higher is better


@dataclass
class CompilationResult:
    """Compilation quality gate result."""
    passed: bool = False
    error_count: int = 0
    warning_count: int = 0


@dataclass
class LintingResult:
    """Linting quality gate result."""
    passed: bool = False
    tool: str = ""
    error_count: int = 0
    warning_count: int = 0


@dataclass
class FormattingResult:
    """Formatting quality gate result."""
    passed: bool = False
    tool: str = ""


@dataclass
class TestResult:
    """Test execution quality gate result."""
    passed: bool = False
    total: int = 0
    passed_count: int = 0
    failed_count: int = 0
    skipped_count: int = 0


@dataclass
class CoverageResult:
    """Test coverage metrics."""
    line_coverage_pct: Optional[float] = None
    function_coverage_pct: Optional[float] = None
    branch_coverage_pct: Optional[float] = None


@dataclass
class QualityGates:
    """All quality gate results."""
    compilation: CompilationResult = field(default_factory=CompilationResult)
    linting: LintingResult = field(default_factory=LintingResult)
    formatting: FormattingResult = field(default_factory=FormattingResult)
    unit_tests: TestResult = field(default_factory=TestResult)
    coverage: CoverageResult = field(default_factory=CoverageResult)


@dataclass
class FeatureResult:
    """I/O contract result for a single feature."""
    feature: str
    test_count: int
    passed: int
    status: str  # SUPPORTED, NOT_SUPPORTED, PARTIAL


@dataclass
class IOContractMetrics:
    """I/O contract validation results."""
    total_test_cases: int = 0
    passed: int = 0
    failed: int = 0
    unsupported: int = 0
    feature_results: list[FeatureResult] = field(default_factory=list)

    @property
    def match_rate_pct(self) -> float:
        """Percentage of test cases that passed."""
        if self.total_test_cases == 0:
            return 0.0
        return (self.passed / self.total_test_cases) * 100


@dataclass
class OutcomeMetrics:
    """Final outcome of the migration."""
    status: str = "failure"  # success, partial, failure
    modules_completed: int = 0
    modules_total: int = 0
    blocking_issues: list[str] = field(default_factory=list)
    notes: Optional[str] = None


@dataclass
class MigrationMetrics:
    """Complete metrics for a single migration run.

    This is the canonical data structure for all migration analysis.
    All reports, database entries, and comparisons derive from this.
    """
    identity: IdentityMetrics
    timing: TimingMetrics = field(default_factory=TimingMetrics)
    cost: CostMetrics = field(default_factory=CostMetrics)
    tokens: TokenMetrics = field(default_factory=TokenMetrics)
    agent: AgentMetrics = field(default_factory=AgentMetrics)
    source_metrics: CodeMetrics = field(default_factory=CodeMetrics)
    target_metrics: CodeMetrics = field(default_factory=CodeMetrics)
    quality_gates: QualityGates = field(default_factory=QualityGates)
    io_contract: IOContractMetrics = field(default_factory=IOContractMetrics)
    outcome: OutcomeMetrics = field(default_factory=OutcomeMetrics)
    schema_version: str = SCHEMA_VERSION

    @property
    def loc_expansion_ratio(self) -> float:
        """Ratio of target LOC to source LOC."""
        if self.source_metrics.production_loc == 0:
            return 0.0
        return self.target_metrics.production_loc / self.source_metrics.production_loc

    @property
    def cost_per_loc(self) -> float:
        """Cost per line of source code."""
        if self.source_metrics.production_loc == 0:
            return 0.0
        return self.cost.total_cost_usd / self.source_metrics.production_loc

    def to_dict(self) -> dict:
        """Convert to dictionary for JSON serialization."""
        data = asdict(self)
        # Add computed properties
        data["loc_expansion_ratio"] = self.loc_expansion_ratio
        data["cost_per_loc"] = self.cost_per_loc
        data["tokens"]["cache_efficiency_ratio"] = self.tokens.cache_efficiency_ratio
        data["io_contract"]["match_rate_pct"] = self.io_contract.match_rate_pct
        return data

    def to_json(self, indent: int = 2) -> str:
        """Serialize to JSON string."""
        return json.dumps(self.to_dict(), indent=indent)

    @classmethod
    def from_dict(cls, data: dict) -> "MigrationMetrics":
        """Create from dictionary."""
        # Handle nested dataclasses
        identity = IdentityMetrics(**data["identity"])
        timing = TimingMetrics(
            wall_clock_duration_ms=data["timing"].get("wall_clock_duration_ms", 0),
            api_duration_ms=data["timing"].get("api_duration_ms", 0),
            phase_durations_ms=data["timing"].get("phase_durations_ms", {}),
            module_durations=[
                ModuleTiming(**m) for m in data["timing"].get("module_durations", [])
            ],
        )
        cost = CostMetrics(**data.get("cost", {}))
        tokens = TokenMetrics(
            input_tokens=data["tokens"].get("input_tokens", 0),
            output_tokens=data["tokens"].get("output_tokens", 0),
            cache_creation_input_tokens=data["tokens"].get("cache_creation_input_tokens", 0),
            cache_read_input_tokens=data["tokens"].get("cache_read_input_tokens", 0),
        )
        agent = AgentMetrics(**data.get("agent", {}))
        source_metrics = CodeMetrics(**data.get("source_metrics", {}))
        target_metrics = CodeMetrics(**data.get("target_metrics", {}))

        # Quality gates
        qg_data = data.get("quality_gates", {})
        quality_gates = QualityGates(
            compilation=CompilationResult(**qg_data.get("compilation", {})),
            linting=LintingResult(**qg_data.get("linting", {})),
            formatting=FormattingResult(**qg_data.get("formatting", {})),
            unit_tests=TestResult(**qg_data.get("unit_tests", {})),
            coverage=CoverageResult(**qg_data.get("coverage", {})),
        )

        # I/O contract
        io_data = data.get("io_contract", {})
        io_contract = IOContractMetrics(
            total_test_cases=io_data.get("total_test_cases", 0),
            passed=io_data.get("passed", 0),
            failed=io_data.get("failed", 0),
            unsupported=io_data.get("unsupported", 0),
            feature_results=[
                FeatureResult(**f) for f in io_data.get("feature_results", [])
            ],
        )

        outcome = OutcomeMetrics(**data.get("outcome", {}))

        return cls(
            identity=identity,
            timing=timing,
            cost=cost,
            tokens=tokens,
            agent=agent,
            source_metrics=source_metrics,
            target_metrics=target_metrics,
            quality_gates=quality_gates,
            io_contract=io_contract,
            outcome=outcome,
            schema_version=data.get("schema_version", SCHEMA_VERSION),
        )

    @classmethod
    def from_json(cls, json_str: str) -> "MigrationMetrics":
        """Deserialize from JSON string."""
        return cls.from_dict(json.loads(json_str))

    def save(self, path: str) -> None:
        """Save metrics to JSON file."""
        with open(path, "w") as f:
            f.write(self.to_json())

    @classmethod
    def load(cls, path: str) -> "MigrationMetrics":
        """Load metrics from JSON file."""
        with open(path) as f:
            return cls.from_json(f.read())
