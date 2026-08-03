---
id: GONI-IMAP-6C8C68F14D04
title: 4.5 Base model as a governed three-speed bundle
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni does not treat the base LLM as one sacred, continuously changing blob.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 4.5 Base model as a governed three-speed bundle
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 4.5 Base model as a governed three-speed bundle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.5 Base model as a governed three-speed bundle

Goni does not treat the base LLM as one sacred, continuously changing blob.
Instead, it treats deployed model behavior as an immutable, governable bundle:
$$
\mathcal{B} = (v_A, v_B, H_D, I_C, H_E, \Sigma_F)
$$
where:

- \(v_A\) is the dense trunk version.
- \(v_B\) is the sparse expert mesh version.
- \(H_D\) is the ordered set of patch hashes.
- \(I_C\) is the set of knowledge snapshot identifiers.
- \(H_E\) is the set of evaluation artefact hashes.
- \(\Sigma_F\) is the set of governance signatures or approvals.

This bundle is realised by a layered stack:

- Layer A: dense constitutional trunk. Stable identity, safety, style, and
  durable reasoning priors.
- Layer B: sparse expert mesh. Domain skill, specialist modules, and routed
  computation.
- Layer C: external knowledge plane. Facts, retrieval state, tool receipts, and
  memories.
- Layer D: patch graph. Scoped, reversible deltas over declared seams.
- Layer E: compiler or sleep phase. Replay, benchmark, merge, reject, or roll
  back candidate changes.
- Layer F: governance ledger. Provenance, signatures, approvals, and deployable
  bill of materials.

Only Layers A and B execute as model weights inside the runtime. Layer C is
looked up at inference time through versioned objects. Layers D, E, and F live
in the control and governance path and may change the deployed bundle only
through an explicit promotion decision.

This yields a change-velocity gradient: the closer a component is to authority,
identity, memory integrity, privacy, or irreversible action, the slower it
moves. Edge artifacts can branch quickly; core artifacts move only through
evidence-backed promotion.

The operational form is a three-speed learning rule:

- P0: facts and retrieval tuning change fast in Layer C.
- P1: domain skill changes as scoped patches in Layers B and D.
- P2: core trunk changes are rare and require compilation into a new \(v_A\).

Applied to models, downloaded candidates live at the fast outer edge, locally
evaluated bundles move inward, policy-approved bundles move closer to the core,
and defaults for private memory require slow, receipted promotion. Applied to
memory, raw observations and chunk candidates move fast, while confirmed user
preferences move slower and policy or identity memory moves slowest.

Hard attachment seams are:

- S1 router seam: routing thresholds, gating rules, expert selection.
- S2 expert seam: adapters, sparse deltas, expert-local modules.
- S3 trunk-interface seam: system rules, output schema, refusal policy.
- S4 retrieval seam: index, reranker, citation rules, memory shaping.
- S5 tool-policy seam: capability corridors and two-phase write policy.

> **Invariant E3 (Seam-bounded model evolution).**
> Any change to deployed model behavior must declare one or more seams in
> \(\{S1, S2, S3, S4, S5\}\). Changes that cannot be expressed as seam-bounded
> patches are not deployable as patches and must be treated as P2 revisions.

> **Invariant E4 (Evidence-gated promotion).**
> A candidate bundle \(\mathcal{B}'\) may replace \(\mathcal{B}\) only if it
> shows benchmark improvement on its target suite, non-regression on safety and
> latency suites, and seeded replay reproducibility under the deterministic
> profile.

> **Invariant E5 (Facts stay external by default).**
> Fresh factual updates must enter through Layer C unless and until repeated
> evidence shows they are durable structure worth compiling into Layers A or B.

---
