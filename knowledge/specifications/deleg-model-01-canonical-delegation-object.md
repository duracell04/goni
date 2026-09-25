---
id: DELEG-MODEL-01
title: Canonical delegation object
type: specification
status: draft
implementation_state: specified_only
proposition: A delegated relationship should be represented as an explicit principal-to-delegate grant binding objective, authority, constraints, budgets, validity, observability, verification, revocation, and accountability requirements rather than inferred from model context or tool access.
domains: [delegation, authority, system, specs]
aliases:
- delegation object
- delegation grant
relations:
- type: refines
  target: GONI-THESIS-826C859B5D30
- type: depends_on
  target: DELEG-INT-01
- type: depends_on
  target: GONI-SPEC-4B9D2670E434
sources:
- SRC-TOMASEV2026-INTELLIGENT-DELEGATION
- SRC-IBRAHIM2026-OVERLAY-GOVERNANCE
artifacts: []
uncertainty: This defines a provider-agnostic semantic contract. Concrete serialization, signatures, storage, and revocation propagation remain implementation questions governed by lower-level specifications.
legacy: []
---

# Canonical delegation object

Goni treats **delegation** as the root relationship connecting human intent to authorized digital action.

A delegation object SHOULD bind at least:

- **principal**: the authority source;
- **delegate**: the actor or runtime receiving bounded authority;
- **objective**: the outcome or task class for which authority is granted;
- **authority envelope**: permitted operations and resources;
- **constraints**: invariants, exclusions, policy conditions, and preconditions;
- **budgets**: bounded calls, time, spend, data, or other scarce resources;
- **validity**: start, expiry, event conditions, and state assumptions;
- **observability requirements**: what state must be inspectable before, during, or after execution;
- **verification requirements**: evidence required to accept consequential results;
- **revocation**: how the grant can be withdrawn or invalidated;
- **accountability requirements**: receipt and attribution fields needed to reconstruct the delegation chain.

The compact abstraction is:

[
D=(P,G,O,A,C,B,T,V,Q,R)
]

where (P) is principal, (G) delegate, (O) objective, (A) authority, (C) constraints, (B) budgets, (T) temporal/state validity, (V) observability and verification requirements, (Q) revocation semantics, and (R) accountability requirements.

## Invariants

1. **Explicit source** — authority derives from an identifiable principal grant, not from model confidence, context inclusion, tool visibility, or provider capability.
2. **Purpose binding** — an authority envelope is valid only for the objective and conditions under which it was granted.
3. **Attenuation** — a delegate may preserve or narrow inherited authority; expansion requires a new valid principal authorization path.
4. **State validity** — authorization is evaluated against current authoritative state rather than stale planning or cached representations.
5. **Revocability** — future actions become unauthorized when the governing delegation is revoked, expired, superseded, or otherwise invalidated.
6. **Reconstructability** — consequential execution remains linkable to the delegation, policy decision, capability, effect, observation, verification, and receipt.

WorkOrders describe bounded work. Capabilities describe executable permissions. Policies define governing rules. Receipts reconstruct consequential execution. The delegation object is the higher-level relationship that gives those objects a principal-owned authority lineage.
