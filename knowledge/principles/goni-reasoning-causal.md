---
id: GONI-PRINCIPLE-REASON-CAUSAL
title: Causal inference as a reusable reasoning lens
type: principle
status: draft
implementation_state: not_applicable
proposition: Distinguish causal effects from observed correlations using explicit identification assumptions.
domains:
  - reasoning
  - system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Conceptual mapping supplied for research; task-specific evidence, solver selection and performance remain to be established.
legacy: []
---

# Causal inference

**Diagnostic signal:** Unclear root cause.

**Core proposition:** Distinguish causal effects from observed correlations using explicit identification assumptions.

**Inputs / assumptions:** Treatment/intervention, outcome, confounders, causal graph, estimand and evidence.

**Mathematical or theoretical families:** Potential outcomes; structural causal models; do-calculus under applicable assumptions.

**Decision contribution:** Choose intervention, counterfactual, identification strategy, and falsification checks.

**Epistemic boundary:** Observational data alone often cannot identify causal effects; uncertainty and confounding must be explicit.

**Representation options:** (1) reusable knowledge-graph reasoning node, (2) prompt/skill with an explicit input-output contract, (3) deterministic mathematical solver when formal inputs are available, or (4) specialized model if matched evaluations justify one. Selection is conditional on task requirements and measured cost/quality.

**Evidence status:** This is a theory-to-problem mapping, not a claim that Goni currently implements or empirically benefits from this framework. Original research citations and benchmarks should be attached through separately reviewed source/evidence nodes.
