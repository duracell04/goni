---
id: GONI-SPEC-B453D8E80962
title: 7. Future-Proofing Expectations
type: specification
status: draft
implementation_state: specified_only
proposition: 'The hardware design should anticipate: That compute boards / accelerators may be **swapped or upgraded** over the product''s lifetime without redesigning the entire enclosure and power domain.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 7. Future-Proofing Expectations
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 7. Future-Proofing Expectations

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Future-Proofing Expectations

The hardware design should anticipate:

- That compute boards / accelerators may be **swapped or upgraded** over the product's lifetime without redesigning the entire enclosure and power domain.
- That external **â€œheavy nodesâ€** (e.g. more powerful accelerators in separate devices) can be attached over the network in future, and Goni should be ready from a power and networking standpoint.

The constraints in this document should remain valid even as specific chip generations change.

---
