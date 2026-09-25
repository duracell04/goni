---
id: HARNESS-BOUNDARY-01
title: Replaceable cognition below sovereign control
type: principle
status: draft
implementation_state: specified_only
proposition: Models, agent harnesses, orchestration frameworks, local runtimes, and provider-specific adapters are replaceable cognitive and execution substrates; principal authority, canonical delegation state, policy, identity, capability issuance, revocation, and receipt truth remain outside those substrates under Goni control.
domains: [system, harness, kernel, delegation]
aliases:
- replaceable harness boundary
- sovereign harness boundary
relations:
- type: refines
  target: GONI-IMAP-216164F1995B
- type: refines
  target: GONI-PRINCIPLE-80DD85BB01A0
- type: depends_on
  target: DELEG-MODEL-01
sources:
- SRC-PI2026-MINIMAL-AGENT-HARNESS
- SRC-SHARMA2026-AGENT-OPERATING-SYSTEMS
artifacts: []
uncertainty: Concrete adapter interfaces and runtime topology remain implementation choices. The principle constrains ownership of authority and canonical state, not the internal design of any chosen harness.
legacy: []
---

# Replaceable cognition below sovereign control

Goni separates **cognitive substrate** from **sovereign control**.

A provider-agnostic layering is:

```text
Human principal
    ↓
Delegation contract
    ↓
Goni delegation runtime
    ↓
Goni authority kernel
    ↕
Agent / cognitive harness
    ↓
Models, tools, sandboxes, external services
```

The harness may assemble context, route models, maintain working state, expose a bounded tool surface, generate proposals, and coordinate cognitive steps. It may be implemented by Goni-native components or by an external framework.

The harness does not become the source of:

- principal identity;
- principal goals or values;
- delegation validity;
- canonical mandates;
- policy authority;
- capability issuance;
- revocation;
- canonical receipts;
- durable authority-bearing memory.

Named frameworks are therefore implementation candidates or research references rather than constitutional dependencies. Replacing a model, provider, harness, or orchestration framework must not silently change the user's authority model.
