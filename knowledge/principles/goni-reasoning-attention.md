---
id: GONI-PRINCIPLE-REASON-ATTENTION
title: Rational inattention as a reusable reasoning lens
type: principle
status: draft
implementation_state: not_applicable
proposition: Allocate limited attention and information acquisition according to expected decision value and information cost.
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

# Rational inattention

**Diagnostic signal:** Excessive inference and information-processing cost.

**Core proposition:** Allocate limited attention and information acquisition according to expected decision value and information cost.

**Inputs / assumptions:** Prior beliefs, possible signals, information costs, downstream decision and utility/loss.

**Mathematical or theoretical families:** Information-constrained choice; mutual information I(S;A); value of information.

**Decision contribution:** Select when to retrieve, deliberate, call a costly model, or stop gathering information.

**Epistemic boundary:** Mutual information is a possible formal cost proxy, not an empirically established token-price function.

**Representation options:** (1) reusable knowledge-graph reasoning node, (2) prompt/skill with an explicit input-output contract, (3) deterministic mathematical solver when formal inputs are available, or (4) specialized model if matched evaluations justify one. Selection is conditional on task requirements and measured cost/quality.

**Evidence status:** This is a theory-to-problem mapping, not a claim that Goni currently implements or empirically benefits from this framework. Original research citations and benchmarks should be attached through separately reviewed source/evidence nodes.
