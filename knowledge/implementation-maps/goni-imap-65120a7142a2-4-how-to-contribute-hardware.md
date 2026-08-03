---
id: GONI-IMAP-65120A7142A2
title: 4. How to contribute (hardware)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Read 10-requirements.md to understand constraints.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 4. How to contribute (hardware)
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 4. How to contribute (hardware)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. How to contribute (hardware)

1. Read [`10-requirements.md`](/blueprint/hardware/10-requirements.md) to understand constraints.
2. If you propose a change that affects constraints:
   - open a hardware issue, and
   - suggest concrete edits to [`10-requirements.md`](/blueprint/hardware/10-requirements.md).
3. For new designs or experiments:
   - add them to [`20-architecture-options.md`](/blueprint/hardware/20-architecture-options.md) or under `30-mechanical/` / `40-electronics/`,
   - link them from an issue or pull request.
4. For accepted decisions:
   - add or update an ADR entry in [`90-decisions.md`](/blueprint/hardware/90-decisions.md).

The aim is to converge on a **buildable, testable Goni node** that meets shared requirements, not just an idealised spec. Cross-check software constraints in `blueprint/software/` (LLM runtime backends, networking, storage layout) so the box and stack stay aligned.
