---
id: GONI-SPEC-B29890704DAE
title: 3. Public vocabulary
type: specification
status: draft
implementation_state: specified_only
proposition: CorrectionDelta records the aligned difference between an agent draft and the principal-approved output.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 3. Public vocabulary
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 3. Public vocabulary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
