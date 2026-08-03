---
id: GONI-IMAP-94D49438F542
title: 1. Philosophy
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**The API is the product.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/00-index.md
  heading: 1. Philosophy
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 1. Philosophy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Philosophy

> **The API is the product. The UI is a client.**

This directory defines how humans and programs talk to a Goni node.

We distinguish:

- A **normative contract**: the HTTP API that all clients rely on.
- **Optional sugar**: dashboards and UIs that *consume* that API but never bypass it.

The HTTP API semantics are defined in terms of the internal planes

N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E}),

so that we can reason about behaviour end-to-end.

The API layer is also constrained by the canonical kernel contracts:

- Tools are capability-scoped syscalls with mandatory audit envelopes: `blueprint/30-specs/tool-capability-api.md`
- Agents are local processes with manifests (permissions/triggers/budgets): `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Latent state is maintained independently of language decoding: `blueprint/30-specs/latent-state-contract.md`

---
