---
id: GONI-SPEC-90456A57BCA9
title: 3.1 Tool runner boundary
type: specification
status: draft
implementation_state: specified_only
proposition: 'Tool runners execute in constrained sandboxes: separate UID and mount namespace, restricted syscall profile, no direct credential material, no ambient privileged file descriptors.'
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
  heading: 3.1 Tool runner boundary
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 3.1 Tool runner boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Tool runner boundary

Tool runners execute in constrained sandboxes:
- separate UID and mount namespace,
- restricted syscall profile,
- no direct credential material,
- no ambient privileged file descriptors.
