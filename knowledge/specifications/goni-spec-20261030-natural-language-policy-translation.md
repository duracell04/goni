---
id: GONI-SPEC-20261030-NATURAL-LANGUAGE-POLICY-TRANSLATION
title: Natural-language mandates to enforceable policy
type: specification
status: draft
implementation_state: specified_only
proposition: Goni translates natural-language delegation requests into typed, reviewable policy proposals and commits authority changes only through explicit kernel-mediated, human-authorized transitions.
domains:
- policy
- agent
aliases: []
relations:
- type: refines
  target: DELEG-INT-01
- type: depends_on
  target: GONI-SPEC-20261030-VISUAL-GOVERNANCE-TRACKS
- type: depends_on
  target: GONI-PRINCIPLE-20261030-CAPABILITY-GROWTH-WITH-BOUNDED-AUTHORITY
sources: []
artifacts: []
uncertainty: Exact policy schema, parsing accuracy, conflict handling, and semantic-equivalence checks remain to be implemented and empirically evaluated.
legacy: []
---

# Natural-language mandates to enforceable policy

A person can describe a delegation in ordinary language. Goni interprets it as a **proposal**, then converts the proposed mandate into a typed, versioned policy that the kernel can evaluate independently of the language model.

## Translation contract

1. **Capture intent:** Record the requested goal, principal, evidence, and stated constraints.
2. **Construct a typed proposal:** Identify agent or role, task class, operations, target resources and recipients, scope, budgets, duration, approval requirements, review timing, and revocation conditions.
3. **Resolve consequential ambiguity:** Where interpretation could enlarge authority, request a narrow clarification or keep the permission absent.
4. **Validate:** Check schema, scope boundaries, conflicting mandates, policy precedence, least privilege, and compatibility with canonical kernel contracts.
5. **Explain:** Render the effective-permission preview and a plain-language, before-and-after summary. Let the human inspect material changes.
6. **Authorize:** Activate a new or expanded delegation only through an authenticated, explicit decision by the authorized human principal. Preserve the previous effective policy until activation succeeds.
7. **Enforce and audit:** Compile or load the approved structured contract into kernel-owned policy state; link its version to mediated effects and receipts.

A natural-language description, prompt, generated code fragment, visual gesture, or model confidence cannot serve by itself as an executable permission. The structured policy and canonical authorization state determine the permitted effects.

## Representation boundaries

- Natural language is the preferred authoring and explanation interface.
- Typed declarative contracts are the authoritative policy representation.
- Conventional deterministic code implements enforcement and stable computations where needed.
- Generated workflows and code execute inside the same scoped capabilities as built-in tools.
- The interpretation and implementation of a policy must be compared for semantic agreement before authorization. The system reports material uncertainty instead of silently inferring a broader grant.

This contract enables natural-language programming of delegation while preserving independent, inspectable security semantics.
