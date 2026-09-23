---
id: CONFIDENCE-01
title: Confidence Evidence Provenance
type: specification
status: draft
implementation_state: specified_only
proposition: "Confidence used in routing, abstention, verification intensity, or consequence-sensitive control should be represented as provenance-bearing evidence rather than as an unqualified scalar."
domains:
- audit
- evaluation
- models
- specs
aliases:
- UNCERTAINTY-EVIDENCE
relations:
- type: depends_on
  target: CAL-01
- type: depends_on
  target: SELECTIVE-01
- type: refines
  target: REC-01
sources: []
artifacts: []
uncertainty: "This is a specified-only logical record. Concrete storage-table promotion and receipt-schema migration should occur only with an implementation slice."
legacy: []
---

# Confidence Evidence Provenance

A scalar such as 0.92 is insufficient to reconstruct why a routing or autonomy
decision trusted a model output.

When confidence materially affects acceptance, abstention, escalation,
verification intensity, or review priority, Goni SHOULD preserve a logical
ConfidenceEvidence record containing at least:

- estimator or model identity;
- checkpoint/version/hash where applicable;
- harness, prompt, adapter, or decision-schema version where behavior depends on it;
- target event or decision definition;
- raw score and its declared semantics;
- calibrated probability, if calibration is claimed;
- calibration artifact/version reference;
- calibration/evaluation domain;
- selection threshold or control rule applied;
- timestamp;
- drift or validity status;
- links to the Work Order and relevant receipt.

Raw score and calibrated probability SHOULD remain separate fields.

Confidence evidence is cognitive provenance. It is not canonical policy state,
a capability, a mandate, or permission.

A human-readable receipt MAY project this record into a concise explanation,
while the underlying evidence retains the identifiers required for
reconstruction.
