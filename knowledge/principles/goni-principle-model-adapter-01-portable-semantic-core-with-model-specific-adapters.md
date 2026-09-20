---
id: GONI-PRINCIPLE-MODEL-ADAPTER-01
title: "Portable Semantic Core With Model-Specific Adapters"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should keep goals, canonical state, permissions, actions, evidence, tests, and receipts provider-independent while allowing thin model-specific adapters to exploit provider or runtime capabilities without transferring authority to the model layer."
domains:
- models
- system
aliases: []
relations:
- type: refines
  target: GONI-SPEC-9D8A0A7F0F70
- type: refines
  target: MODEL-REG-01
sources:
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "The adapter boundary must be validated against real providers; excessive abstraction can suppress capabilities while excessive specialization can reduce portability."
legacy: []
---

# Portable Semantic Core With Model-Specific Adapters

Goni's semantic contract should remain portable across model families and providers. The portable layer owns goals, state, authority, action semantics, evidence, receipts, and acceptance tests.

Thin adapters MAY exploit native tool schemas, reasoning-state APIs, context caching, structured output modes, KV-cache behavior, speculative decoding, and local runtime features.

The adapter improves execution efficiency or quality; it does not become the canonical owner of policy, memory, authority, or receipts.
