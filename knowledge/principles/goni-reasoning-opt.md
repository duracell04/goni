---
id: GONI-PRINCIPLE-REASON-OPT
title: Optimization theory as a reusable reasoning lens
type: principle
status: draft
implementation_state: not_applicable
proposition: Allocate scarce resources to maximize a defined objective subject to explicit constraints.
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

# Optimization theory

**Diagnostic signal:** Inefficient resource allocation.

**Core proposition:** Allocate scarce resources to maximize a defined objective subject to explicit constraints.

**Inputs / assumptions:** Decision variables, objective function, constraints, costs, uncertainty and feasibility.

**Mathematical or theoretical families:** Linear/integer/convex optimization; constrained stochastic optimization.

**Decision contribution:** Choose schedules, model budgets, compute placement, and constrained task assignment.

**Epistemic boundary:** A numerical optimum depends on the specified objective and may omit stakeholder incentives or distributional consequences.

**Representation options:** (1) reusable knowledge-graph reasoning node, (2) prompt/skill with an explicit input-output contract, (3) deterministic mathematical solver when formal inputs are available, or (4) specialized model if matched evaluations justify one. Selection is conditional on task requirements and measured cost/quality.

**Evidence status:** This is a theory-to-problem mapping, not a claim that Goni currently implements or empirically benefits from this framework. Original research citations and benchmarks should be attached through separately reviewed source/evidence nodes.
