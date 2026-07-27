# Cognitive Exocortex Model
DOC-ID: SYS-04
Status: Specified only / roadmap
Maturity: Draft

Goni is a delegation OS and bounded digital representative. A **cognitive
exocortex** is the explanatory model for how its memory, reasoning, planning,
and governed action fit around the human operator.

This model is non-normative. It is not a claim of biological equivalence, a
deployment topology, a new trust boundary, or a replacement for Goni's planes
and kernel contracts. If the metaphor conflicts with a canonical specification,
data contract, or accepted decision, the canonical artifact wins.

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

## 2. Digital meninges

The meningeal names are mnemonic groupings, not separate runtime services.

| Metaphor | Question | Existing controls |
| --- | --- | --- |
| **Dura** | Who or what may enter, execute, or control the system? | Host and device security, identity, secrets isolation, sandboxing, encrypted storage and backups, egress policy, and the [trust model](/blueprint/20-system/20-trust-model.md). |
| **Arachnoid** | How may context and commands move between components? | Work Orders, jobs, capability tokens, scheduling, budgets, context isolation, agent manifests, and the [tool capability](/blueprint/30-specs/tool-capability-api.md) and [scheduler](/blueprint/30-specs/scheduler-and-interrupts.md) contracts. |
| **Pia** | What is a claim, where did it come from, and how may it be used? | Source references, parser identity, hashes, confidence, validity, permissions, contradiction state, review status, redaction, receipts, and [governed memory retrieval](/blueprint/30-specs/memory-retrieval.md). |

External text, model output, parser output, and connector responses remain
untrusted. Provenance supports inspection and challenge; it does not turn a
claim into truth. Receipts provide operational evidence and tamper indication,
not epistemic certainty.

## 3. Second-brain memory model

The memory labels below are a reader-facing view of existing storage and
lifecycle contracts.

| Memory view | Canonical placement and lifecycle |
| --- | --- |
| **Evidence** | Source material belongs in governed Knowledge-plane records and content-addressed artifacts, with source and integrity metadata. Evidence may be append-only or integrity-protected where policy requires, but remains subject to authorized retention, deletion, redaction, and tombstoning. |
| **Episodic** | Chronological events and interactions use the `episodic` MemoryEntry class and retain source and receipt references. |
| **Semantic** | Facts, claims, decisions, and derived understanding use governed MemoryEntries with confidence, validity, conflict, permission, and provenance metadata. The existing `relational`, `project`, and `policy` classes remain first-class and are not collapsed into this label. |
| **Procedural** | Reusable methods, preferences about how work is done, and governed skills use the `procedural` or `policy` classes according to authority and scope. |
| **Working** | Task-scoped context lives in the Context Plane and hot latent state. It expires or is discarded unless a separate, policy-mediated memory grant authorizes consolidation. |

The canonical fields and finite memory classes remain those in the
[MVP schemas](/blueprint/software/50-data/51-schemas-mvp.md) and
[memory retrieval contract](/blueprint/30-specs/memory-retrieval.md).
Observation, extraction, or appearance in working context never grants durable
memory authority by itself.

## 4. Third-brain cortex mapping

The cortical layers group cognitive responsibilities. Their numbering does not
define a required call sequence or add components to the formal plane model.

| Cortex layer | Cognitive role | Goni mapping |
| --- | --- | --- |
| **Layer IV: perception** | Receive and structure screen, file, message, audio, and other observations. | Observation adapters, parsers, and the [Visual Intelligence Plane](/blueprint/30-specs/visual-intelligence-plane.md). Outputs are candidates, not trusted knowledge or instructions. |
| **Layers II/III: association** | Resolve entities, compare claims, link time and relationships, and search bounded graph neighborhoods. | [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md), the [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md), and swappable dense, sparse, graph, and metadata indexes. |
| **Layer I: integration** | Assemble the temporary situation model used for the current Work Order. | Context Plane selection and materialization, including source waypoints, conflicts, omission reasons, and budget constraints. |
| **Layer V: action output** | Turn a conclusion into an answer, draft, proposal, or effectful tool request. | Thinking/proposal/commit discipline, capability-mediated tools, approval corridors, and receipts. The model cannot perform an effect by emitting text. |
| **Layer VI: executive feedback** | Classify work, create bounded plans, allocate compute, verify evidence, apply stopping criteria, and decide whether to propose memory changes. | Work Orders, Done Contracts, the Control Plane, [ITCR](/blueprint/30-specs/itcr.md), scheduling, and the [learning loop](/blueprint/20-system/50-learning-loop.md). |

## 5. Specialized-module mapping

These names describe responsibilities that may span multiple implementations.
They do not require one service per anatomical label.

| Module metaphor | Goni responsibility |
| --- | --- |
| **Thalamus** | Query classification, context routing, retrieval-mode selection, and attention allocation under the Work Order and budget. |
| **Hippocampus** | Governed consolidation of observed episodes into durable memory candidates, including contradiction checks, provenance, memory grants, and write receipts. |
| **Basal ganglia** | Capability, policy, risk, approval, and commit gating at the non-bypassable tool boundary. |
| **Cerebellum** | Deterministic routines such as parsing, conversion, deduplication, scheduled indexing, backup, schema validation, and reproducible calculations. |

## 6. Governed cognitive cycle

Goni expresses the cognitive cycle through existing contracts:

1. **Observe** inputs without granting them authority.
2. **Create or bind a Work Order** that states goal, constraints, risk, expected
   result, policy, and budget.
3. **Retrieve** policy-permitted evidence and memory through bounded dense,
   sparse, graph, and metadata paths.
4. **Verify** selected context against source boundaries, parser confidence,
   validity, contradictions, permissions, and the Done Contract.
5. **Reason and integrate** inside a temporary Context Plane projection. Model
   output remains untrusted until checked by the harness.
6. **Propose or commit** through capability and approval gates. Every mediated
   effect produces the required receipt and rollback or compensation reference.
7. **Consolidate** only approved memory candidates through a separate memory
   grant; discard ordinary working context.
8. **Audit** outcomes, stale claims, contradictions, retrieval quality, policy
   decisions, and receipt completeness.

This cycle is bounded by scheduler, token, time, tool, privacy, and risk budgets.
Stopping, escalation, correction, and human veto are normal outcomes rather
than failures of autonomy.

## 7. Architectural invariants

- The human principal remains the source of goals, values, delegated authority,
  correction, and veto.
- The LLM is a replaceable execution substrate, not the owner of state, policy,
  memory, identity, approvals, or receipts.
- Observation and perception do not imply permission to retain, retrieve, or
  act on the observed material.
- No effectful action bypasses capability, policy, budget, egress, and receipt
  mediation applicable to its risk class.
- Working context is bounded and ephemeral unless an explicit memory grant
  authorizes durable consolidation.
- Evidence is provenance-bearing and integrity-checkable, while still obeying
  retention, redaction, deletion, and legal policy.
- Cloud or council routes remain optional, minimized, policy-approved, and
  receipted; local authority remains canonical.

## 8. Contract relationships

Upstream:

- [Software Architecture](/blueprint/software/20-architecture.md)
- [Axioms and Planes](/blueprint/software/50-data/10-axioms-and-planes.md)
- [Trust Model](/blueprint/20-system/20-trust-model.md)
- [Agentic Kernel Foundations](/blueprint/20-system/40-agentic-kernel-foundations.md)

Downstream:

- [Goni Story](/blueprint/docs/goni-story.md)
- [Goni Whitepaper](/blueprint/docs/goni-whitepaper.md)
- [Glossary](/blueprint/docs/glossary.md)

Adjacent:

- [Governed Memory Retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Tool Capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Scheduler and Interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Learning Loop](/blueprint/20-system/50-learning-loop.md)
