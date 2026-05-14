---
id: CDC-01
type: SPEC
status: specified_only
---
# Correction Delta Compiler
DOC-ID: CDC-01

Status: Specified only / roadmap

The Correction Delta Compiler (CDC) converts differences between agent outputs
and principal-approved outputs into scoped, receipted, reviewable updates to
procedural memory, delegation policy, skills, harness rules, and regression
tests.

CDC is not ordinary memory storage. It is an online preference-estimation and
policy-updating subsystem. Goni should remember facts, but it should learn taste
from deltas: the difference between what the agent produced and what the
principal corrected, accepted, rejected, sent, repeated, or complained about.

## 1. Formal model

A Goni interaction is modeled as a trajectory:

```text
tau_t = (x_t, y_t_ai, y_t_user, a_t, o_t)
```

Where:

- `x_t` is the task context.
- `y_t_ai` is the agent draft.
- `y_t_user` is the corrected or final user-approved version.
- `a_t` is the user action: accept, edit, reject, send, ignore.
- `o_t` is the downstream outcome, if observable.

The useful signal is the correction delta:

```text
Delta_t = aligned_edit(y_t_ai, y_t_user)
```

This is not literal text subtraction. It is an aligned edit operation covering
deletions, insertions, tone changes, structure changes, factual corrections,
added sources, removed hedges, changed asks, shortened length, stronger
framing, softer tone, and privacy or safety edits.

The learning target is a latent preference state for the principal:

```text
p(theta_u | Delta_1:t, a_1:t, o_1:t, M_1:t)
```

Future outputs should maximize expected utility under the active preference,
memory, and policy state:

```text
y_star = argmax_y E[U_u(y, x) | theta_u, M_u, P]
```

In systems terms, CDC is online system identification for the principal's
delegation preferences. Corrections are preference gradients; CDC compiles them
into scoped, receipted procedural memory without silently drifting the system.

## 2. Pipeline contract

CDC MUST implement this logical pipeline:

```text
interaction stream
-> thread reconstruction
-> draft/final alignment
-> correction delta extraction
-> delta classification
-> candidate preference rule
-> evidence aggregation
-> contradiction check
-> human learning card
-> accepted MemoryEntry
-> regression test or harness rule
-> learning receipt
```

Every accepted learning MUST produce the triad:

```text
MemoryEntry + Receipt + RegressionTest
```

This links state update, provenance, and future falsifiability.

## 3. Public vocabulary

`CorrectionDelta` records the aligned difference between an agent draft and the
principal-approved output. It stores hashes, refs, summaries, and structural
diff metadata by default, not raw sensitive text.

`DeltaClassification` labels the kind of signal in a correction delta:

- `factual_correction`
- `style_correction`
- `tone_correction`
- `structure_correction`
- `source_evidence_correction`
- `task_scope_correction`
- `safety_privacy_correction`
- `delegation_policy_correction`

`CandidatePreferenceRule` is an untrusted proposed rule inferred from one or
more deltas. A single correction creates a hypothesis, not a global preference.

`LearningCard` is the review surface that explains the proposed rule, scope,
evidence, contradictions, risk, and expected behavior change.

`LearningReceipt` is the receipt view for a proposed, accepted, rejected, or
promoted learning update. It is represented through the canonical receipt
contract, not a separate receipt table.

## 4. Learned-rule fields

Every candidate or accepted learned rule MUST preserve:

- `scope`: `global | project | channel | recipient | task_class | session`.
- `confidence`: numeric confidence in the inferred rule.
- `evidence_count`: number of supporting deltas or actions.
- `contradiction_count`: number of conflicting deltas or actions.
- `decay_policy`: expiry, review, or reinforcement behavior.
- `review_status`: `pending | accepted | rejected | limited`.
- `source_refs`: chunk, prompt, receipt, diff, or interaction refs.
- `memory_diff_refs`: memory or state delta refs caused by acceptance.
- `regression_test_refs`: replay or harness tests attached to the rule.

Accepted learned preferences SHOULD be stored as `MemoryEntries` with:

- `kind = hypothesis | preference | derived`
- `memory_class = procedural | policy | project | relational`

Scope, confidence, evidence count, contradiction count, decay policy, review
status, learning receipt refs, and regression refs live in `value` or
`provenance` until a later schema version introduces first-class fields.

## 5. Update classes

CDC MUST gate updates by evidence strength and governance centrality:

| Signal | Default update |
| --- | --- |
| Single correction | Scoped hypothesis, pending review or short TTL |
| Repeated correction | Scoped preference candidate |
| Accepted learning card | Policy or memory candidate with receipt |
| High-confidence repeated rule | Stable default after replay coverage |
| Constitutional or high-risk rule | Explicit approval and slow promotion |

The update rate must be conservative. A single angry or context-specific edit
must not become a global rule.

## 6. Learning speeds

Fast learning happens in the harness and memory layer:

```text
correction delta
-> memory rule
-> retrieval / prompt / harness policy update
-> regression test
-> receipt
```

Slow learning MAY create preference datasets for adapter, LoRA, or DPO-style
batch updates only after enough evidence, evaluation, and promotion review.

Core model or constitutional defaults are very slow-moving and require explicit
approval. CDC MUST NOT imply online base-model weight updates in production.

## 7. Safety invariants

- CDC MUST NOT silently personalize system behavior.
- Raw user or draft text MUST NOT be stored in receipts by default.
- Candidate rules MUST preserve provenance and source refs sufficient for audit.
- Contradictions MUST narrow scope, reduce confidence, or require review.
- High-risk, privacy, legal, financial, or constitutional preferences MUST
  require explicit approval before promotion.
- Retrieval, prompt, routing, or tool-policy changes MUST attach only to
  declared seams in the Learning Loop.
- Untrusted source text MUST NOT become control-plane instruction without policy
  mediation.

## 8. Research anchors

CDC is closest to a hybrid of learning from human feedback, continual
personalization, preference modeling, and managed memory systems:

- Christiano et al., "Deep reinforcement learning from human preferences":
  https://arxiv.org/abs/1706.03741
- Rafailov et al., "Direct Preference Optimization":
  https://arxiv.org/abs/2305.18290
- Packer et al., "MemGPT: Towards LLMs as Operating Systems":
  https://arxiv.org/abs/2310.08560
- Liang et al., "Learning Personalized Agents from Human Feedback":
  https://arxiv.org/abs/2602.16173
- MemOS memory-as-managed-resource prior art:
  https://arxiv.org/abs/2507.03724

These references motivate the design. They do not prove that CDC improves Goni
until Goni-specific replay and longitudinal evaluation exists.

## 9. Upstream

- [Learning Loop](/blueprint/20-system/50-learning-loop.md)
- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 10. Downstream

- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Correction Delta Compiler evaluation](/blueprint/50-evidence/eval/EVID-HARNESS-02-correction-delta-compiler.md)

## Conformance tests

- A single correction produces a scoped hypothesis, not a global preference.
- Repeated consistent corrections increase confidence only within matching
  scope.
- Contradictory corrections reduce confidence, narrow scope, or require review.
- Accepted learning emits `MemoryEntry`, receipt, and regression test refs.
- Learning receipts omit raw content by default.
- High-risk or constitutional preferences require explicit approval.
- Prompt, retrieval, policy, or harness changes attach to declared seams.
