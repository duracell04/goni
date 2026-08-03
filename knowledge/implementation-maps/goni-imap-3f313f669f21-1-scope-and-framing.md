---
id: GONI-IMAP-3F313F669F21
title: 1. Scope and framing
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'A local agentic kernel can interact with blockchain systems in at least four roles: hardened runtime host for a full node, reference monitor for signing and value-moving actions, provenance receipt emitter with external timestamp anchoring, validator safety guard for slashable consensus actions.'
domains:
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/45-kernel-blockchain-mapping.md
  heading: 1. Scope and framing
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 1. Scope and framing

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope and framing

A local agentic kernel can interact with blockchain systems in at least four
roles:
- hardened runtime host for a full node,
- reference monitor for signing and value-moving actions,
- provenance receipt emitter with external timestamp anchoring,
- validator safety guard for slashable consensus actions.

The core architectural stance remains:
- keys are not held by general agents,
- side effects are mediated through a small trusted core,
- authority is capability-scoped and revocable.
