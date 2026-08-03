---
id: GONI-IMAP-EF8673885ADE
title: 6. Resolved questions
type: implementation-map
status: draft
implementation_state: specified_only
proposition: These were ???open questions???
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 6. Resolved questions
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 6. Resolved questions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Resolved questions

These were ???open questions??? in earlier drafts; they are now resolved for MVP:

1. **Memory capacity:** MVP requires **128 GB** unified memory. 64 GB is ???dev-only??? and not performance-representative for the product story.
2. **Mechanical envelope:** target **~7 L**, allowing 6???8 L to accommodate real cooling/PSU/fans.
3. **Networking:** prefer **5 GbE** on the compute module; 2.5 GbE is acceptable only as a fallback.
4. **Board choice:** design around a **Mini-ITX-style mounting + ATX PSU** assumption so we are not hard-locked to one vendor, but use Framework as the first concrete reference.
5. **External heavy node story:** GN100-class nodes are explicitly **out-of-scope for MVP**; revisit once the offload API and mesh are stable.

---
