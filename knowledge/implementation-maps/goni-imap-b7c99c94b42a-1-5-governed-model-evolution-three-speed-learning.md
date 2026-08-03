---
id: GONI-IMAP-B7C99C94B42A
title: 1.5 Governed model evolution - three-speed learning
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Object.** A deployable model bundle $$ \mathcal{B} = (v_A, v_B, H_D, I_C, H_E, \Sigma_F) $$ where: \(v_A\) is the trunk version.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 1.5 Governed model evolution - three-speed learning
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 1.5 Governed model evolution - three-speed learning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.5 Governed model evolution - three-speed learning

**Object.**
A deployable model bundle
$$
\mathcal{B} = (v_A, v_B, H_D, I_C, H_E, \Sigma_F)
$$
where:

- \(v_A\) is the trunk version.
- \(v_B\) is the expert mesh version.
- \(H_D\) is the ordered set of patch hashes.
- \(I_C\) is the set of knowledge snapshot IDs used by the bundle.
- \(H_E\) is the set of evaluation report hashes.
- \(\Sigma_F\) is the set of approval signatures.

The intended learning split is:

- fast facts in Layer C (knowledge plane),
- medium-speed domain skill in Layer B or D (experts and patches),
- rare durable structure in Layer A (trunk).

**Invariant G1 (seam-bounded change).**

Every patch declares one or more target seams from:

- S1 router seam,
- S2 expert seam,
- S3 trunk-interface seam,
- S4 retrieval seam,
- S5 tool-policy seam.

Any change that cannot be expressed through these seams is not a valid patch and
must be treated as a higher-risk compiled revision.

**Invariant G2 (promotion classes).**

Every candidate change is classified before evaluation:

- P0: retrieval tuning, citation rules, prompts, memory formatting, reranker
  changes.
- P1: router changes, expert adapters, scoped tool-policy updates.
- P2: trunk revisions or authority/policy changes that alter global behavior.

**Invariant G3 (evidence-gated promotion).**

No candidate bundle may be promoted unless it satisfies all of:

- benchmark improvement on the declared target suite,
- non-regression on safety and latency suites,
- seeded replay reproducibility under the deterministic profile,
- compatibility check proving that touched artifacts match the declared seams.

P2 additionally requires explicit approval signatures and a rollback pointer to
the previous bundle.

**Invariant G4 (bill of materials and rollback).**

Every deployed bundle records:

- trunk version,
- expert mesh version,
- patch set hashes,
- knowledge snapshot IDs,
- evaluation report hashes,
- approver signatures.

Rollback must be possible by redeploying the previous bundle ID without editing
the live runtime state in place.

**Proof obligation (theoretical).**

1. Define a validator that maps each patch to an allowed seam set.
2. Show that bundle replacement is append-only in the ledger: deploy
   \(\mathcal{B}'\) by reference, never by mutating \(\mathcal{B}\) in place.
3. Show that deterministic replay is sufficient to compare candidate vs
   baseline on at least one fixed evaluation suite.

**Empirical check (MVP).**

- For each candidate patch, run a seeded replay against a fixed baseline bundle.
- Require positive delta on a scoped benchmark set.
- Require no regression on safety and latency suites.
- Validate that the bundle receipt contains all bill-of-material fields.
- Exercise rollback by redeploying the previous bundle and rerunning the replay
  suite.

---
