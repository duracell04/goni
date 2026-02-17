# Learning Loop (System-Managed Adaptation)
DOC-ID: SYS-03
Status: Specified only / roadmap
Maturity: Draft

This document defines how Goni improves over time without assuming online
weight updates. The system manages adaptation explicitly, with safety gates and
auditability.

## 1. Core constraint (no online weight updates)
Goni must not assume that a deployed model will learn its way out of failures.
Runtime improvement is achieved by system-managed artifacts (memory, playbooks,
checkers, and policies), not by changing model weights in production.
[[tworek2026-decoder]]

## 2. Learning layers (A/B/C)
Improvement is separated into three risk tiers:

- Layer A (fast, safe): preferences, retrieval memory, playbooks, allow/deny
  rules. These are reversible and low-risk.
- Layer B (gated): skill patches, tool wrappers, deterministic checkers, schema
  validators. These require review and explicit promotion.
- Layer C (rare, high risk): offline model fine-tunes or weight updates. These
  require evaluation, rollback plans, and governance approval.

## 2.1 Formal state tuple and OS transition loop (normative)
Operationally, the runtime is modeled as a partially observable control loop.
Let the kernel-visible state at step `t` be:

`X_t = (S_core_t, F_sparse_t, M_t, C_t, B_t, P_t, H_t)`

Where:
- `S_core_t`: dense working latent state.
- `F_sparse_t`: symbolic facts/flags.
- `M_t`: memory index references.
- `C_t`: active capability token set.
- `B_t`: budget ledger state.
- `P_t`: active policy hash/version.
- `H_t`: current receipt-chain head hash.

The system treats hidden world factors as unobserved variables and assumes
state transitions are Markov with respect to the kernel state:

`Pr(X_{t+1} | X_{0:t}, a_t) = Pr(X_{t+1} | X_t, a_t)`

Kernel loop per step:
1. Ingest observation/event and snapshot `X_t`.
2. Select action under policy + capability constraints.
3. Execute action/tool in a mediated transaction.
4. Commit delta + receipt on success, or rollback + failure receipt on reject.
5. Emit experience packet for Layer A/B/C promotion gates.

## 3. Failure becomes a first-class artifact
Every failure produces an experience packet derived from receipts and runtime
state. This enables repeatable repairs without claiming the model "learns."

Minimum fields:
- failure_id, timestamp, request_id
- receipt_ids (immutable chain)
- failure_class (see Section 4)
- observed_symptoms (short tags)
- proposed_fix (Layer A/B/C)
- evidence_links (metrics, logs, or traces)

## 4. Failure taxonomy (starter set)
- tool_permission_denied
- wrong_tool_choice
- hallucinated_assumption
- brittle_multi_step_plan
- long_horizon_drift
- complexity_overload

Reasoning collapse at higher complexity is treated as an engineering constraint,
not a debate. Build guardrails that assume phase transitions in reliability.
[[apple2025-illusion-thinking]] [[comment2025-illusion-thinking]]

## 5. Promotion gates (how fixes become durable)
Fixes follow an explicit pipeline:

1. Inbox: raw failure packet, untrusted.
2. RFC: structured proposal with minimal risk analysis.
3. SPEC: formal contract or schema change, if needed.
4. ADR: decision record with rationale and alternatives.
5. EVID: evaluation results and rollback plan.

## 6. Unstuck primitives (runtime recovery)
The runtime must implement explicit recovery behaviors:
- bounded retries with strategy changes (not "try again")
- plan repair vs plan restart
- backtracking checkpoints
- multi-model critique (Council path) when budgeted
- human escalation mode for high-risk tasks

## 7. Interfaces and links
Upstream:
- [Receipts](/blueprint/30-specs/receipts.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)

Downstream:
- [Metrics scorecard](/blueprint/docs/metrics.md)
- [Observability](/blueprint/docs/observability.md)

Adjacent:
- [LLM Council](/blueprint/docs/llm-council.md)
- [Memory architecture (legacy)](/blueprint/docs/memory-architecture.md)
