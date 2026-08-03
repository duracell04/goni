---
id: GONI-PRINCIPLE-99ECE17758BF
title: 4. Invariants (from 30-conformance)
type: principle
status: draft
implementation_state: specified_only
proposition: '**Local-first invariant** Core operation (chat + RAG with local models) must be possible with *no* external network connectivity.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/os-and-base-image.md
  heading: 4. Invariants (from 30-conformance)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 4. Invariants (from 30-conformance)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Invariants (from 30-conformance)

- **Local-first invariant**  
  Core operation (chat + RAG with local models) must be possible with *no* external network connectivity.

- **Durability invariant**  
  Reboot does not lose:
  - model files,
  - indices/embeddings,
  - configuration.


- **State hygiene invariant**
  Latent state pages must not be swapped in plaintext, and crash dumps must redact or exclude state buffers.

- **Isolation invariant**  
  Default permissions ensure Goni data directories are not world-readable; GPU/NPU access is restricted to the Goni service user where applicable.

---
