# Unsupported Assertions in comparative_analysis.tex

This document lists assertions in the research paper that are not directly supported by our experimental data and require external citations.

## Assertions Needing Citations

### 1. LLM Code Generation Quality (Line 42)

**Assertion**: "Recent advances in Large Language Models have enabled automated code generation at unprecedented quality levels."

**Type**: General claim about state of the field

**Recommended Citation**:
- Chen et al. (2021). "Evaluating Large Language Models Trained on Code." arXiv:2107.03374. [arxiv.org/abs/2107.03374](https://arxiv.org/abs/2107.03374)
  - Introduced Codex and HumanEval benchmark
  - Codex solved 28.8% of HumanEval problems; with repeated sampling, 70.2%
  - Established that LLMs can generate functional code from docstrings

**Status**: FOUND

---

### 2. Transformer Attention Complexity (Line 198)

**Assertion**: "In transformer-based models, attention complexity scales quadratically with context length."

**Type**: Technical claim about model architecture

**Recommended Citation**:
- Vaswani et al. (2017). "Attention Is All You Need." NeurIPS 2017. [arxiv.org/abs/1706.03762](https://arxiv.org/abs/1706.03762)
  - Foundational transformer paper
  - Self-attention requires computing n×n attention matrix, O(n²) complexity
  - Cited 173,000+ times as of 2025

**Status**: FOUND

---

### 3. Context Dilution Effect (Line 200)

**Assertion**: "large contexts dilute the signal of specific instructions"

**Type**: Claim about model behavior

**Recommended Citation**:
- Liu et al. (2023). "Lost in the Middle: How Language Models Use Long Contexts." TACL 2024, Volume 12, pp. 157-173. [arxiv.org/abs/2307.03172](https://arxiv.org/abs/2307.03172)
  - Performance highest when relevant info at beginning or end of context
  - Performance degrades when relevant information in middle ("U-shaped curve")
  - Explains via rotary positional embeddings and serial-position effect

**Status**: FOUND

---

### 4. Emergent Agent Behavior (Line 202)

**Assertion**: "agents exhibit emergent behavior that may not follow explicit instructions"

**Type**: Claim about agent behavior

**Recommended Citations**:
- Wei et al. (2022). "Emergent Abilities of Large Language Models." TMLR 2022. [jasonwei.net/blog/emergence](https://www.jasonwei.net/blog/emergence)
  - Defines emergent abilities as those "not present in small models but present in large models"
  - Documents 100+ emergent abilities in scaled LLMs

- Schaeffer et al. (2023). "Are Emergent Abilities of Large Language Models a Mirage?" NeurIPS 2023.
  - Provides alternative perspective on emergence attribution

**Status**: FOUND

---

### 5. Prior Work on Code Generation (Line 325)

**Assertion**: "Prior work on code generation has focused primarily on single-function or single-file generation from natural language descriptions."

**Type**: Claim about prior research scope

**Recommended Citations**:
- Chen et al. (2021). "Evaluating Large Language Models Trained on Code." (HumanEval focuses on function-level problems)
- Li et al. (2022). "Competition-Level Code Generation with AlphaCode." Science. [arxiv.org/abs/2203.07814](https://arxiv.org/abs/2203.07814)
  - AlphaCode generates single-problem solutions, not multi-file systems

**Contrast with repository-level benchmarks (emerged 2024)**:
- Jimenez et al. (2024). "SWE-bench: Can Language Models Resolve Real-world Github Issues?" ICLR 2024. [swebench.com](https://www.swebench.com/SWE-bench/)
  - Repository-level benchmark that evaluates patches to real codebases
  - Noting this is newer work helps justify our contribution

**Status**: FOUND

---

### 6. Multi-Agent Systems for Software Engineering (Line 327)

**Assertion**: "Multi-agent LLM systems have been explored for complex reasoning tasks, but their application to software engineering tasks with tool use remains limited."

**Type**: Claim about research landscape

**Recommended Citations**:
- Hong et al. (2023). "MetaGPT: Meta Programming for A Multi-Agent Collaborative Framework." ICLR 2024 Oral. [arxiv.org/abs/2308.00352](https://arxiv.org/abs/2308.00352)
  - Encodes SOPs into multi-agent software development
  - Includes product managers, architects, engineers

- Qian et al. (2023). "ChatDev: Communicative Agents for Software Development." arXiv:2307.07924. [arxiv.org/abs/2307.07924](https://arxiv.org/abs/2307.07924)
  - Multi-agent collaboration for waterfall software development
  - CEO, CTO, programmer, reviewer, tester roles

**Note**: These are contemporaneous works; our assertion may need softening to "remained limited until recently" or cite these as parallel developments.

**Status**: FOUND (but assertion may need revision)

---

### 7. Traditional Migration Approaches (Line 329)

**Assertion**: "Automated software migration has traditionally relied on rule-based transformation systems or statistical translation models."

**Type**: Claim about prior migration approaches

**Recommended Citations**:
- TXL, Stratego/XT, Rascal - rule-based transformation systems
  - Wikipedia: [Source-to-source compiler](https://en.wikipedia.org/wiki/Source-to-source_compiler)

- For learning-based transpilers (contrast):
  - ACM TOSEM paper on code translation: [dl.acm.org/doi/10.1145/3660778](https://dl.acm.org/doi/10.1145/3660778)
    - "Latest learning-based transpilers have shown impressive enhancement against rule-based counterparts"
    - Notes that rule-based approaches are "costly" and "practically prohibitive to predict all potential uses"

**Status**: FOUND

---

## Summary

| # | Topic | Line | Status | Primary Citation |
|---|-------|------|--------|------------------|
| 1 | LLM code generation quality | 42 | FOUND | Chen et al. 2021 (Codex/HumanEval) |
| 2 | Transformer attention complexity | 198 | FOUND | Vaswani et al. 2017 |
| 3 | Context dilution effect | 200 | FOUND | Liu et al. 2023 (Lost in the Middle) |
| 4 | Emergent agent behavior | 202 | FOUND | Wei et al. 2022 (Emergence) |
| 5 | Prior code generation work | 325 | FOUND | Chen 2021, Li 2022 (AlphaCode) |
| 6 | Multi-agent systems | 327 | FOUND* | Hong 2023 (MetaGPT), Qian 2023 (ChatDev) |
| 7 | Traditional migration approaches | 329 | FOUND | TXL/Stratego/Rascal, Wikipedia |

*Assertion #6 may need revision since MetaGPT/ChatDev are contemporaneous, not prior work.

## Recommended BibTeX Additions

```bibtex
@article{chen2021codex,
  title={Evaluating Large Language Models Trained on Code},
  author={Chen, Mark and Tworek, Jerry and Jun, Heewoo and others},
  journal={arXiv preprint arXiv:2107.03374},
  year={2021}
}

@inproceedings{vaswani2017attention,
  title={Attention is All You Need},
  author={Vaswani, Ashish and Shazeer, Noam and Parmar, Niki and others},
  booktitle={Advances in Neural Information Processing Systems},
  volume={30},
  year={2017}
}

@article{liu2024lost,
  title={Lost in the Middle: How Language Models Use Long Contexts},
  author={Liu, Nelson F. and Lin, Kevin and Hewitt, John and others},
  journal={Transactions of the Association for Computational Linguistics},
  volume={12},
  pages={157--173},
  year={2024}
}

@inproceedings{hong2024metagpt,
  title={MetaGPT: Meta Programming for A Multi-Agent Collaborative Framework},
  author={Hong, Sirui and others},
  booktitle={International Conference on Learning Representations},
  year={2024}
}

@article{qian2023chatdev,
  title={ChatDev: Communicative Agents for Software Development},
  author={Qian, Chen and others},
  journal={arXiv preprint arXiv:2307.07924},
  year={2023}
}

@inproceedings{jimenez2024swebench,
  title={SWE-bench: Can Language Models Resolve Real-world Github Issues?},
  author={Jimenez, Carlos E. and others},
  booktitle={International Conference on Learning Representations},
  year={2024}
}
```
