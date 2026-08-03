---
id: GONI-SYNTHESIS-E9DD07CB6184
title: 5. Explicit assumptions
type: synthesis
status: draft
implementation_state: specified_only
proposition: Host OS primitives (namespaces/cgroups/seccomp/capabilities or equivalent) are available and correctly configured.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/20-trust-model.md
  heading: 5. Explicit assumptions
  revision: 628398028a2ae5fe5696b6b3ec004da2314ddd96
---

# 5. Explicit assumptions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Explicit assumptions

- Host OS primitives (namespaces/cgroups/seccomp/capabilities or equivalent)
  are available and correctly configured.
- Only approved egress path is reachable from runtime components.
- Time source and key material are sufficient for receipt integrity checks.

If these assumptions fail, guarantees degrade to best effort.
