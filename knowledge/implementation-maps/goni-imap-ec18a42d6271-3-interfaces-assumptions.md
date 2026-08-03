---
id: GONI-IMAP-EC18A42D6271
title: 3. Interfaces & assumptions
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The kernel assumes: A POSIX-like environment with: a writable, durable directory for data, a writable directory for models, a writable temp directory.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/os-and-base-image.md
  heading: 3. Interfaces & assumptions
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 3. Interfaces & assumptions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Interfaces & assumptions

The kernel assumes:

- A POSIX-like environment with:
  - a writable, durable directory for data,
  - a writable directory for models,
  - a writable temp directory.
- A process supervisor that can:
  - start the Goni service,
  - restart on failure,
  - expose environment variables for configuration.

We **do not** fix whether Goni runs on bare metal, in a VM, or in a container; the component spec is deliberately deployment-agnostic.

---
