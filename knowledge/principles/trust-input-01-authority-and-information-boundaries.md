---
id: TRUST-INPUT-01
title: Preserve authority and information trust boundaries
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should preserve the provenance and privilege of policy, authorized user intent, tool observations, retrieved material, and external content so that untrusted data cannot acquire execution authority merely by entering model context.
domains:
- system
- safety
- authority
- context
aliases:
- instruction data boundary
- trusted instruction boundary
relations:
- type: refines
  target: GONI-THESIS-349EB2E79136
sources:
- SRC-WALLACE2024-INSTRUCTION-HIERARCHY
- SRC-DEBENEDETTI2024-AGENTDOJO
- SRC-DOSHI2026-SAFE-TOOL-USE
artifacts: []
uncertainty: Model-level privilege handling remains probabilistic; Goni's stronger claims depend on implemented and tested runtime mediation outside the model.
legacy: []
---

# Preserve authority and information trust boundaries

The model context is not one uniform authority domain.

Goni should preserve at least the following semantic classes:

1. kernel and constitutional policy,
2. authenticated user mandate and current authorized intent,
3. approved workflow or work-order constraints,
4. trusted system observations,
5. tool outputs and retrieved documents,
6. arbitrary web, email, document, or third-party content.

A lower-trust class may provide evidence about the world without gaining the right to redefine higher-trust policy or authorize an effect.

## Runtime consequence

Prompt injection should be treated as an information-flow and authority-confusion problem. Model-level defenses can reduce risk, while the kernel must independently enforce capability, egress, policy, and effect boundaries.

The system therefore asks two separate questions:

- **What does this content tell us?**
- **What authority, if any, does this content possess?**

Only the second question can grant permission to act.
