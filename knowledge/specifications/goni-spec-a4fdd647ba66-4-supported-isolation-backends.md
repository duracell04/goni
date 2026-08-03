---
id: GONI-SPEC-A4FDD647BA66
title: 4. Supported isolation backends
type: specification
status: draft
implementation_state: specified_only
proposition: 'Conformant implementations may use one or more: namespace/cgroup/seccomp profile sets, microVM isolation, sandbox runtimes with syscall mediation, WASI-style execution for selected tools.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 4. Supported isolation backends
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 4. Supported isolation backends

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Supported isolation backends

Conformant implementations may use one or more:
- namespace/cgroup/seccomp profile sets,
- microVM isolation,
- sandbox runtimes with syscall mediation,
- WASI-style execution for selected tools.

Regardless of backend, mediation invariants are identical.
