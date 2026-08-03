---
id: GONI-SYNTHESIS-DC3AFAC962DA
title: 1. Human and digital responsibilities
type: synthesis
status: draft
implementation_state: specified_only
proposition: The diagram describes responsibility, not physical nesting.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/60-cognitive-exocortex-model.md
  heading: 1. Human and digital responsibilities
  revision: cdf162b26a4fe7d78e6daa6039696e89ee0ef17f
---

# 1. Human and digital responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Human and digital responsibilities

```text
HUMAN SOVEREIGNTY
goals | values | judgement | delegated authority | correction | veto
                         |
                  governed interfaces
                         |
DIGITAL MENINGES (cross-cutting protection)
security | orchestration | provenance and epistemic controls
                         |
THIRD BRAIN (active, bounded cognition)
perceive | associate | plan | integrate | verify | propose or act
                         |
SECOND BRAIN (durable memory)
evidence | episodic | semantic | procedural | relational | project | policy
                         |
GOVERNED EVIDENCE
files | messages | records | media | observations | receipts
```

The diagram describes responsibility, not physical nesting. Digital meninges
are cross-cutting controls, and the second and third brain are operational
views over existing Goni planes and services.

| Model term | Responsibility | Goni mapping |
| --- | --- | --- |
| **First brain** | Defines goals and values, grants authority, supplies final judgement, corrects, and vetoes. | The human principal and the [delegation interface](/blueprint/30-specs/delegation-interface.md). |
| **Second brain** | Preserves governed evidence and durable memory for later retrieval. | The Vault plus the Knowledge and Memory abstractions in the [software architecture](/blueprint/software/20-architecture.md). It is not an additional canonical plane. |
| **Third brain** | Performs bounded perception, association, context assembly, planning, verification, and action preparation. | The Harness, Context, Control, and Execution responsibilities operating through kernel contracts. It does not own authority or canonical memory. |
| **Digital meninges** | Protect digital cognition with security, movement controls, provenance, and epistemic safeguards. | A cross-cutting view of the TCB, policy, capability, scheduling, egress, receipt, and memory-governance contracts. |

Human sovereignty does not mean approving every low-risk operation. The
principal may establish bounded, revocable autonomy corridors. Actions outside
those corridors, or actions whose risk requires approval, return to the human.
