---
id: GONI-SYNTHESIS-E56DD0C22C30
title: 2.5 Permission broker (sudo layer)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Granular capabilities** Read, annotate, draft, send, delete, transfer, configure, etc.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 2.5 Permission broker (sudo layer)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2.5 Permission broker (sudo layer)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.5 Permission broker (sudo layer)

- **Granular capabilities**  
  - Read, annotate, draft, send, delete, transfer, configure, etc.
- **Policies per capability**  
  - “Read automatically”, “draft automatically”, “send only with approval”, “never move money without biometric”.

**Why this matters:**  
Agent systems are scary because they’re either paper-tigers or overpowered. Goni can say: *“We make autonomous agents safe to deploy by wrapping them in a permission kernel.”*

---
