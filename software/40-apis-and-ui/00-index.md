# 40 – APIs and UI: Index & Core Philosophy

Status: v0.1 (2025-11-30)  
Scope: External boundary of a single Goni node  
Audience: API consumers, UI designers, systems engineers

---

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

- Tools are capability-scoped syscalls with mandatory audit envelopes: `docs/specs/tool-capability-api.md`
- Agents are local processes with manifests (permissions/triggers/budgets): `docs/specs/agent-definition.md`, `docs/specs/agent-manifest.md`
- Latent state is maintained independently of language decoding: `docs/specs/latent-state-contract.md`

---

## 2. Documents

1. **[api-surface.md](./api-surface.md)**  
   **The contract.**  
   Defines the public HTTP endpoint(s), their request/response semantics, and how each request is mapped into the Control Plane \(\mathcal{K}\), Context Plane \(\mathcal{X}\), Execution Plane \(\mathcal{E}\), and Data Plane \(\mathcal{A}\).

2. **[dashboard-concepts.md](./dashboard-concepts.md)**  
   **The view.**  
   Describes what an operator dashboard is allowed to show and do. The dashboard is a *client* of the API, with no hidden privileges.

---

## 3. MVP vs future

For the MVP / prototype:

- The **HTTP API** is **mandatory** – a node is “alive” if and only if it serves /v1/chat/completions correctly.
- The **dashboard** is **optional** – the kernel must be fully usable without any UI running.

Future work (admin APIs, rich dashboards, multi-user UIs) will extend this directory but must not violate the invariants defined here.
