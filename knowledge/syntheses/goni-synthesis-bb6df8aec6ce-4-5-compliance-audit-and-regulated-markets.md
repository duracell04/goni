---
id: GONI-SYNTHESIS-BB6DF8AEC6CE
title: 4.5 Compliance, audit, and regulated markets
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Arrow spine + deterministic logging create: a replayable history of what the system did and why.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 4.5 Compliance, audit, and regulated markets
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 4.5 Compliance, audit, and regulated markets

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.5 Compliance, audit, and regulated markets

- Arrow spine + deterministic logging create:
  - a replayable history of what the system did and why.
- Deterministic inference preset (batch=1, seed, single worker/CPU option, vLLM `--enable-deterministic-inference`) reduces drift in self-loop/agent runs, strengthening auditability.
- Potential to:
  - serve as a compliant "AI record system",
  - integrate with legal / financial workflows.

---
