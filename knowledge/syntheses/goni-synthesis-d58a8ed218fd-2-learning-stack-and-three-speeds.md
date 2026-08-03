---
id: GONI-SYNTHESIS-D58A8ED218FD
title: 2. Learning stack and three speeds
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni does not treat the base model as one mutable blob.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 2. Learning stack and three speeds
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 2. Learning stack and three speeds

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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

This maps to the PAL adaptation ladder:

1. Prompt or policy steering first, because it is cheap, inspectable, and easy
   to roll back.
2. Governed memory and retrieval second, because factual and preference context
   should remain external, citeable, and reversible where possible.
3. Adapter or preference-dataset generation third, only after repeated evidence,
   replay evaluation, and promotion review.
4. Full fine-tuning last and rarely, because it is harder to debug, evaluate,
   and reverse than prompt, memory, policy, or adapter changes.

This is the system form of Goni's change-velocity gradient: change velocity is
inversely proportional to governance centrality. Surface artifacts such as
prompts, drafts, context assemblies, eval packs, plugins, and candidate models
may move quickly. Control-plane policy, memory access classes, default model
permissions, and capability corridors move slowly. Kernel invariants,
constitutional policy, privacy posture, and receipt requirements move only
through strong evidence, review, versioned governance, and rollback plans.
