---
id: HARNESS-PORT-01
title: Semantic Portability Across Agent Systems
type: experiment
status: draft
implementation_state: not_applicable
proposition: Goni should evaluate whether a stable canonical task contract can be compiled into model- or harness-specific execution scaffolds that produce sufficiently equivalent observable engineering behavior across multiple agent systems without assuming textual prompt equivalence.
domains:
- evaluation
- harness
- portability
aliases:
- semantic portability evaluation
relations:
- type: tests
  target: GONI-PRINCIPLE-MODEL-ADAPTER-01
- type: tests
  target: DELEG-INT-01
- type: tests
  target: GONI-PRINCIPLE-SYS-CAP-01
sources:
- SRC-LIN2026-AGENTIC-HARNESS-ENGINEERING
artifacts: []
uncertainty: Equivalent behavior requires task-specific acceptance thresholds. Model and harness ecosystems evolve rapidly, so the benchmark must pin exact versions and treat conclusions as dated.
legacy: []
---

# Semantic Portability Across Agent Systems

Goni should test semantic portability rather than textual prompt portability.

The invariant object is the canonical task contract:

- objective;
- authoritative context and source hierarchy;
- hard constraints and invariants;
- authority corridor;
- deliverable;
- DoneContract;
- verification requirements; and
- stop/escalation conditions.

The execution scaffold may vary by agent system:

- model family and checkpoint;
- deployment profile;
- prompt/template form;
- decomposition strategy;
- native tool schema;
- structured-output adapter;
- context selection;
- checkpoint frequency; and
- deterministic validation hooks.

## Experimental design

Use a fixed suite of representative synthetic repository and delegation tasks. For each agent system, compile the same semantic contract into the smallest sufficient model-specific scaffold.

Candidate systems may include, when available and legally usable:

- Goni local Qwen deployment profiles;
- Goni local GLM deployment profiles;
- other open-weight local profiles;
- reference coding/agent harnesses such as Codex or Claude Code for comparative evaluation; and
- future harnesses with materially different tool or context mechanisms.

The experiment compares observable outcomes, not hidden reasoning traces.

## Metrics

Measure:

- DoneContract completion;
- constraint/invariant violations;
- unauthorized action proposals;
- correct tool usage;
- unnecessary tool calls;
- unnecessary architecture or scope expansion;
- deterministic validation failures;
- number of repair iterations;
- human interruption rate;
- missed decisive clarification rate;
- context and tool-schema tokens;
- latency and resource cost;
- verification success; and
- final state equivalence where exact equivalence is meaningful.

## Interpretation

A provider-neutral Markdown or task contract is semantically portable when materially different model/harness combinations can satisfy the same acceptance criteria under comparable authority and environment conditions, even if their concrete prompts and internal scaffolds differ.

Failure of one model/harness combination does not by itself falsify the canonical contract. It may indicate a capability, adapter, context, tool-interface, or verification mismatch that should be localized through the evaluation evidence.

Results should therefore distinguish:

```text
contract failure
adapter failure
model capability failure
tool-interface failure
context failure
verification failure
```

rather than collapsing all errors into generic prompt quality.
