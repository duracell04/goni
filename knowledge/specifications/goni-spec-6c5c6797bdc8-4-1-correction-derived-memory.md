---
id: GONI-SPEC-6C5C6797BDC8
title: 4.1 Correction-derived memory
type: specification
status: draft
implementation_state: specified_only
proposition: Correction deltas are first-class observed inputs.
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
  heading: 4.1 Correction-derived memory
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 4.1 Correction-derived memory

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4.1 Correction-derived memory

Correction deltas are first-class observed inputs. A correction delta is the
structured difference between an agent-produced draft and a principal-approved
or principal-corrected output. It may indicate factual error, tone
miscalibration, structure preference, missing evidence, privacy risk, excessive
branching, or task-scope misunderstanding.

Correction-derived memory MUST flow through the Correction Delta Compiler before
it can affect durable memory, retrieval, prompt assembly, or harness policy.
The compiler proposes scoped rules, validates support, checks contradictions,
and emits learning receipts for accepted updates.

Accepted learned preferences SHOULD map to existing memory classes:

| Preference scope | Memory class |
| --- | --- |
| How the principal wants work done | `procedural` |
| Delegation authority, approval, or safety defaults | `policy` |
| Project-specific style or operating preference | `project` |
| Recipient, relationship, or channel-specific preference | `relational` |

Single corrections SHOULD be stored as scoped hypotheses with TTL or review
status. Repeated corrections may become preferences after evidence aggregation
and contradiction checks. Raw correction text MUST remain confined to allowed
Knowledge or Context plane fields; memory entries and receipts store hashes,
refs, summaries, and governance metadata by default.
