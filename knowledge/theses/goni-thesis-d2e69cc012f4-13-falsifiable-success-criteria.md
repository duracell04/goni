---
id: GONI-THESIS-D2E69CC012F4
title: 13. Falsifiable Success Criteria
type: thesis
status: draft
implementation_state: specified_only
proposition: For Goni to move from manifesto to engineering program, it needs measurable claims.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: 13. Falsifiable Success Criteria
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 13. Falsifiable Success Criteria

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 13. Falsifiable Success Criteria

For Goni to move from manifesto to engineering program, it needs
measurable claims. The system's core claims are evaluated through
authority-layer metrics, not only model-quality metrics.

Important success criteria include:

- Receipt completeness: consequential mediated actions produce complete,
  reconstructable receipts.
- Unauthorized action prevention: actions lacking valid policy, capability, or
  corridor authorization are denied.
- Egress control: external network calls route through the network gate and
  produce appropriate evidence.
- Reconstruction success: reviewers can reconstruct why sampled actions
  occurred using receipts and memory references.
- Interruption reduction: Goni reduces unnecessary approval prompts while
  preserving the no-ambient-authority posture.
- Revocation latency: when a principal withdraws a mandate, related future
  actions are denied immediately or near-immediately.
- Autonomy safety: autonomous actions that violate policy approach zero, with
  incidents linked to receipts and repair paths where possible.

These metrics make the authority layer testable. They also clarify what Goni
needs to prove: not that it has the most capable model, but that it can safely
mediate delegation.
