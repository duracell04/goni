---
id: SKILL-REG-01
title: Governed Skill Registry
type: specification
status: draft
implementation_state: specified_only
proposition: A Goni skill is a versioned, provenance-bearing procedural object whose relevant fragments may be selectively materialized for cognition while execution authority remains independently governed by kernel-owned capabilities and policy.
domains:
- agent
- memory
- software
- system
aliases:
- governed-skills
relations:
- type: refines
  target: GONI-SPEC-33294E0D3306
- type: refines
  target: GONI-SYNTHESIS-D58A8ED218FD
- type: depends_on
  target: MODEL-REG-01
sources:
- SRC-LI2021-PREFIX-TUNING
- SRC-HU2022-LORA
artifacts: []
uncertainty: This contract defines a logical skill and fragment model. Concrete schema tables, routing algorithms, scoring thresholds, and adapter boundaries remain specified only until promoted with implementation and evaluation evidence.
legacy: []
---

# Governed Skill Registry

Goni separates **procedural cognition** from **execution authority**. A skill may
teach or condition how a model approaches a task, but loading a skill never
grants tool, filesystem, network, memory-write, approval, or external-effect
authority.

The governing invariant is:

[
oxed{	ext{skill availability} 
eq 	ext{authority}}
]

## 1. Logical Skill object

A conforming representation MUST preserve the following logical fields:

```yaml
skill_id:
version:
task_classes:
scope:
activation_conditions:
fragment_refs:
dependency_refs:
tool_requirement_refs:
capability_requirement_refs:
model_constraints:
verification_contract_ref:
eval_refs:
content_hash:
provenance:
```

The representation may live in canonical rows, content-addressed artifacts, or
another governed substrate, but one authoritative version and hash MUST be
resolvable before materialization.

## 2. Skill fragments

A skill MAY be decomposed into selectively materializable fragments:

```yaml
fragment_id:
skill_id:
role:
content_ref:
content_hash:
token_estimate:
dependency_refs:
activation_conditions:
provenance:
```

Allowed fragment roles are:

- `principle`
- `procedure`
- `example`
- `tool_contract`
- `verification`
- `output_schema`

The Context Plane SHOULD materialize only fragments justified by the current
Work Order, model/runtime constraints, dependencies, and available budget.

## 3. Procedural memory boundary

Durable personal procedural memory may describe how the principal prefers work
to be done. It does not silently become executable instruction. Promotion from
memory into a reusable governed skill requires an explicit versioned skill
object, provenance, review/evaluation state, and a stable content hash.

## 4. Learned adaptation boundary

Textual skills, learned prefixes, adapters, and model weights are different
representations of recurring behavior:

[
	ext{explicit skill}
ightarrow
	ext{learned prefix}
ightarrow
	ext{adapter}
ightarrow
	ext{weights}.
]

`SKILL-REG-01` governs explicit procedural semantic modules.
`MODEL-REG-01` governs checkpoints, learned prefixes, adapters, and model
bundles. Moving behavior deeper into learned state requires separate evaluation
and does not transfer authority.

## 5. Tool boundary

A skill may declare required tools or capabilities, but declarations are
requirements, not grants. Kernel mediation resolves actual authority against
canonical policy and capability state at execution time.

## 6. Selective materialization

Given Work Order (W_t), eligible skills (S), and budget (B_t), the
Context Plane may compute:

[
S_t^* = operatorname{SelectSkillFragments}(W_t,S,B_t).
]

The selection policy SHOULD minimize irrelevant instruction load while
preserving task quality, verification requirements, provenance, and dependency
closure. False-negative omission of a decisive procedural rule is an explicit
evaluation risk.
