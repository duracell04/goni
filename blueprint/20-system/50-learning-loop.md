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

## 2. Learning stack and three speeds
Goni does not treat the base model as one mutable blob. It separates learning
into a governed stack:

- Layer A: dense constitutional trunk. Stable identity, style, refusal policy,
  and durable reasoning priors. Slow-moving.
- Layer B: sparse expert mesh. Domain skill, specialist adapters, and routing-
  addressable modules. Medium-moving.
- Layer C: external knowledge plane. Facts, retrieval state, tool receipts, and
  memories. Fast-moving.
- Layer D: patch graph. Scoped, reversible deltas that target declared seams.
- Layer E: compiler or sleep phase. Replay, eval, promote, reject, merge, or
  roll back candidate changes.
- Layer F: governance ledger. Provenance, signatures, approvals, and deployable
  bill of materials.

Operationally, a serious LLM learns in three speeds:

- P0 fast path: fresh facts and retrieval tuning belong in Layer C. They update
  at inference time and do not imply weight changes.
- P1 medium path: domain skill belongs in Layer B or Layer D via scoped router
  changes, expert adapters, tool wrappers, and validators.
- P2 slow path: trunk changes belong in Layer A only after repeated durable
  gains survive replay, safety, and latency gates.

The governing rule is simple: facts default outward, skill patches stay scoped,
and core weights absorb only rare durable structure.

This is the system form of Goni's change-velocity gradient: change velocity is
inversely proportional to governance centrality. Surface artifacts such as
prompts, drafts, context assemblies, eval packs, plugins, and candidate models
may move quickly. Control-plane policy, memory access classes, default model
permissions, and capability corridors move slowly. Kernel invariants,
constitutional policy, privacy posture, and receipt requirements move only
through strong evidence, review, versioned governance, and rollback plans.

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
5. Emit experience packet for P0/P1/P2 promotion gates.

## 2.1a Harness observability
Agentic Harness Engineering maps onto Goni as **Harness Governance**: the
system improves by changing observable operating artefacts, not by letting
hidden prompts or tool glue drift.

The harness is governed through three observability requirements
[[lin2026-agentic-harness-engineering]]:

- **Component observability:** prompts, policies, tool manifests, retrieval
  rules, routing rules, approval corridors, context templates, and receipt
  formats are file-backed, versioned, and revertible.
- **Experience observability:** receipts, user edits, approvals, overrides,
  failed retrievals, wrong model routes, latency, cost, and rollback events are
  compressed into experience digests instead of replaying raw trajectory logs
  into the model.
- **Decision observability:** every harness edit declares its prediction, eval
  window, measurement signals, retention criteria, and rollback condition.

Example derived artefacts:

```yaml
experience_digest:
  task_class: "social_follow_up"
  outcome: "approved_after_minor_edit"
  assumption_error: false
  question_needed: false
  policy_change_candidate: "use softer reminder template for weak ties"
  evidence_refs:
    - receipt_id: "rec_..."
    - draft_diff_id: "diff_..."
```

```yaml
harness_change:
  id: "followup-template-exit-ramp-v2"
  target_component: "social_open_loop_policy"
  prediction:
    approval_rate: "+10%"
    user_edit_distance: "-20%"
    negative_feedback_rate: "no increase"
  eval_window: "next 30 follow-up drafts"
  rollback_condition: "negative_feedback_rate > baseline + 5%"
```

These examples are evaluation artefacts, not receipt-schema fields. Receipts
remain the canonical source evidence; harness digests are derived summaries used
for P0/P1/P2 promotion gates.

## 2.2 Patch seams and allowed attachment points
Candidate changes may attach only to declared seams:

- S1 router seam: routing rules, thresholds, and expert selection.
- S2 expert seam: per-expert adapters, sparse deltas, or small modules.
- S3 trunk-interface seam: stable output contracts, refusal policy, and schema.
- S4 retrieval seam: indexes, reranker config, citation rules, memory shaping.
- S5 tool-policy seam: capabilities, corridors, and two-phase write policy.

Delegation-policy bundles attach at S5. This includes clarification thresholds,
assumption-visibility rules, corridor defaults, and irreversible-action policy.
They are treated as reversible control-plane patches rather than hidden prompt
edits [[tomasev2026-intelligent-delegation]] [[zhang2025-ace]].

Undeclared attachment points are rejected. A patch that cannot name its seam is
not a valid patch.

## 3. Failure becomes a first-class artifact
Every failure produces an experience packet derived from receipts and runtime
state. This enables repeatable repairs without claiming the model "learns."

Minimum fields:
- failure_id, timestamp, request_id
- receipt_ids (immutable chain)
- failure_class (see Section 4)
- observed_symptoms (short tags)
- proposed_fix (promotion class + target seam)
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
2. RFC: structured proposal with target seam, promotion class, and rollback
   pointer.
3. SPEC: formal contract or schema change, if needed.
4. ADR: decision record with rationale and alternatives.
5. EVID: evaluation results, non-regression report, and replay artefacts.

Minimum promotion evidence by class:

- P0: scoped task improvement plus reproducible replay on the affected suite.
- P1: P0 evidence plus safety and latency non-regression.
- P2: P1 evidence plus explicit approval, durable gains across repeated runs,
  and signed bundle metadata for rollback.

Delegation-policy promotion specifically requires trace replay over vague-intent
episodes, comparison against prior policy bundles, and review of question rate,
override rate, unsafe autonomy, and surfaced-assumption coverage. Failed
bundles must be rollbackable through the governance ledger.

Harness-policy promotion additionally requires a falsifiable change statement:
target component, expected outcome delta, evidence sources, eval window,
retention rule, and rollback condition. A change that improves a narrow task but
increases interruption rate, policy violations, unreviewed egress, or rollback
frequency fails promotion unless an explicit higher-level policy accepts that
trade-off.

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
