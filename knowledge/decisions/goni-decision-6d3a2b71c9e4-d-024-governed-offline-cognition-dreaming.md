---
id: GONI-DECISION-6D3A2B71C9E4
title: D-024 - Governed offline cognition ("dreaming")
type: decision
status: draft
implementation_state: specified_only
proposition: Goni may run kernel-scheduled offline cognitive cycles that replay authorized experience, consolidate memory, derive abstractions, discover associations, generate counterfactuals, test competing explanations, and produce hypotheses or evidence requests, while treating all synthetic outputs as cognitive candidates rather than authoritative facts or permissions.
domains:
- agent
- kernel
- memory
- research
- system
aliases:
- governed-dreaming
- offline-cognition
relations:
- type: refines
  target: GONI-DECISION-FEB39440C565
- type: refines
  target: GONI-DECISION-9AF49466170D
- type: depends_on
  target: SCHED-01
- type: depends_on
  target: GONI-SPEC-A742123055E0
- type: depends_on
  target: GONI-SPEC-33294E0D3306
sources:
- SRC-HA2018-WORLD-MODELS
- SRC-HAFNER2023-DREAMERV3
- SRC-PARK2023-GENERATIVE-AGENTS
- SRC-VANDEVEN2020-BRAIN-REPLAY
- SRC-OPENAI2026-DREAMING
artifacts: []
uncertainty: The value, scheduling policy, calibration, and contamination risk of governed offline cognition are unvalidated. The term "dreaming" has multiple research and product meanings; Goni uses it only as an alias for the stricter architectural concept defined here and in DREAM-01.
legacy: []
---

# D-024 - Governed offline cognition ("dreaming")

Goni adopts **governed offline cognition** as a background cognitive regime.

"Offline" means decoupled from an immediate user turn or latency-critical
interaction. It does not mean that network policy is bypassed or that the
machine must be physically disconnected.

The Control Plane may schedule bounded cognitive cycles that operate over
authorized memory and context to:

- replay and consolidate prior experience;
- derive abstractions from multiple observations;
- discover associations across otherwise separate contexts;
- identify contradictions or stale assumptions;
- generate hypotheses and alternative explanations;
- construct counterfactuals or simulated futures;
- falsify current hypotheses or seek disconfirming evidence;
- identify evidence that would discriminate among competing hypotheses.

This is not a fifth architectural plane. It is a protocol spanning the existing
Data, Context, Control, and Execution responsibilities.

The governing distinction is:

> **Dreaming may expand the hypothesis space. It may not expand the authority space.**

A synthetic output is a cognitive candidate. It does not become true because a
model generated it, because the same model generated it repeatedly, because
multiple derivative generations agree, or because its source material is real.

Accordingly:

- dream output is not automatically fact;
- derivation provenance is not confirmation of the derived proposition;
- repeated synthesis is not independent evidence;
- confidence is not authority;
- simulated consequences are not observed consequences;
- a dream cycle cannot create or widen mandates, capabilities, approval state,
  policy, budgets, revocation state, or other kernel-owned authority.

A useful dream result may become a hypothesis, a derived memory, a contradiction
flag, an evidence request, a proposal, or a candidate Work Order. Any
consequential effect still enters Goni's ordinary authority path.

## Rationale

D-017 already requires recurring Observation → Reflection → Planning
consolidation. D-018 establishes latent-first cognition, while
GONI-DECISION-9AF49466170D separates probabilistic cognition from canonical
authority. Governed offline cognition makes the missing boundary explicit when
reflection moves beyond summarization into synthetic possibility generation.

Research on world models, imagined trajectories, reflective agents, and replay
provides precedent for the component mechanisms. OpenAI's 2026 use of
"Dreaming" for background memory synthesis also provides a contemporary product
precedent for the term. None of these sources validates Goni's authority or
epistemic contracts; those remain Goni-specific design claims requiring
evaluation.

## Consequences

- DREAM-01 defines the normative contract.
- Existing MemoryEntries remain the canonical persistence substrate; no
  DreamStore or separate hypothesis database is introduced.
- Dream workloads are kernel-scheduled, budgeted, cancellable, and subordinate
  to interactive work.
- Synthetic outputs retain an explicit epistemic distinction until independent
  evidence supports promotion under the memory contract.
- Dreaming does not bypass local-first, egress, receipt, tool, or capability
  rules.
- DREAM-EVAL-01 must test useful-hypothesis yield, synthetic-memory
  contamination, provenance, resource cost, and authority preservation before
  any implementation claim is promoted.
