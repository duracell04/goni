---
id: GONI-PRINCIPLE-74DF1AAD27E7
title: 6. Safety invariants
type: principle
status: draft
implementation_state: specified_only
proposition: Untrusted text MUST NOT become control-plane instruction without policy mediation.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/memory-retrieval.md
  heading: 6. Safety invariants
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 6. Safety invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Safety invariants

- Untrusted text MUST NOT become control-plane instruction without policy
  mediation.
- Parser output MUST NOT become durable memory or action context without source
  refs, parser identity, confidence metadata, and policy filtering.
- Stale, expired, or conflicted memory MUST be filtered, demoted, or surfaced as
  uncertainty.
- Private memory MUST NOT be sent to remote runtimes unless policy explicitly
  allows the destination and purpose.
- Observed or extracted screen/app context MUST NOT be stored, indexed, or
  reused unless a memory grant permits the memory class, retention posture, and
  source scope.
- Deletion or redaction MUST trigger reindexing or tombstoning sufficient to
  prevent normal retrieval.
- Memory answers that rely on retrieved evidence SHOULD expose source refs or
  waypoints sufficient for audit.
