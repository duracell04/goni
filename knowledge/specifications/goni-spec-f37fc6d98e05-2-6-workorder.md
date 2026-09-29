---
id: GONI-SPEC-F37FC6D98E05
title: 2.6 WorkOrder
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every executable turn MUST compile a WorkOrder with goal, done_contract, inputs, constraints, assumptions, plan, tools, harness_requirements, risk_class, output_schema, and work_quality_mode. The selected execution harness must be able both to perform the authorized action and to observe the evidence required by the DoneContract.'
domains:
- specs
aliases: []
relations:
- type: depends_on
  target: GONI-SPEC-59E3DFC23CC8
sources: []
artifacts: []
uncertainty: Harness capability matching is specified-only. Concrete capability vocabularies, routing algorithms, and fallback behavior require implementation and evaluation.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.6 WorkOrder
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.6 WorkOrder

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

### 2.6 WorkOrder

Every executable turn MUST compile a `WorkOrder` with:

- `goal`
- `done_contract`
- `inputs`
- `constraints`
- `assumptions`
- `plan`
- `tools`
- `harness_requirements`
- `risk_class`
- `output_schema`
- `work_quality_mode`

`harness_requirements` declares the minimum operating environment required to
close the delegated loop. It SHOULD distinguish:

- **actuation capabilities** needed to perform authorized state changes;
- **observation capabilities** needed to inspect the resulting state;
- **verification capabilities** needed to evaluate the DoneContract;
- any sandbox, rollback, network, identity, or environment constraints needed by
  the task.

The selected harness MUST be capable of satisfying both the action path and the
required verification path. A harness that can mutate the target system but
cannot collect qualifying evidence for the DoneContract is insufficient for
autonomous completion certification.

When no available harness can close the verification path, the WorkOrder may
still permit a bounded action if policy allows it, but the runtime must preserve
the result as executed-but-unverified or verification-incomplete rather than
completed.

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
