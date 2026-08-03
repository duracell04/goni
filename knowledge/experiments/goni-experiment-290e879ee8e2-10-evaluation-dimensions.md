---
id: GONI-EXPERIMENT-290E879EE8E2
title: 10. Evaluation dimensions
type: experiment
status: draft
implementation_state: not_applicable
proposition: Goni evaluates Desktop Agent Firewall behavior as separation of powers, not as vendor comparison.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 10. Evaluation dimensions
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 10. Evaluation dimensions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Evaluation dimensions

Goni evaluates Desktop Agent Firewall behavior as separation of powers, not as
vendor comparison. Evaluation SHOULD measure:

- latency and throughput of mediated capture/extraction/action paths,
- offline capability,
- privacy leakage rate,
- prompt-injection recovery,
- GPU/VRAM and memory pressure,
- rollback or repair success,
- receipt completeness,
- denied-transition fail-closed behavior.

These are measurement dimensions. This spec does not assert benchmark values.
