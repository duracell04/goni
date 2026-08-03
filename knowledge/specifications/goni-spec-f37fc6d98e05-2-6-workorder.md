---
id: GONI-SPEC-F37FC6D98E05
title: 2.6 WorkOrder
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every executable turn MUST compile a WorkOrder with: goal done_contract inputs constraints assumptions plan tools risk_class output_schema work_quality_mode For audit_grade work, the Work Order MUST additionally carry: evidence_scope: sources, refs, paths, time windows, artifacts, and explicit exclusions.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.6 WorkOrder
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.6 WorkOrder

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.6 WorkOrder

Every executable turn MUST compile a `WorkOrder` with:

- `goal`
- `done_contract`
- `inputs`
- `constraints`
- `assumptions`
- `plan`
- `tools`
- `risk_class`
- `output_schema`
- `work_quality_mode`

For `audit_grade` work, the Work Order MUST additionally carry:

- `evidence_scope`: sources, refs, paths, time windows, artifacts, and explicit
  exclusions.
- `search_strategy`: the planned coverage pattern, including branches, repos,
  PRs/issues, logs, local/remote deltas, or other relevant surfaces.
- `negative_claim_policy`: how absence-of-evidence claims may be phrased.
- `claim_strength_target`: the strongest claim the current scope can support.
- `missing_evidence_plan`: what remains unchecked and what would close the
  loop.
- `audit_sticky`: whether audit-grade mode persists across follow-up turns.

The Work Order is the canonical pre-execution object. Downstream components may
store summarized or referenced forms, but the logical object MUST preserve all
of the fields above.
