---
id: EVID-DELEGATION-SCOPE-ATTENUATION-01
title: 'Source claim: recursive delegation requires explicit scope attenuation'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Ibrahim and Li model agentic delegation as a compositional authorization problem requiring explicit delegation semantics, recursive chains, contextual boundaries, and resource-scope attenuation.
domains: [research, delegation, authority]
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-DF627E719B9D
sources: [SRC-IBRAHIM2026-OVERLAY-GOVERNANCE]
artifacts: []
uncertainty: The paper proposes one formal authorization framework; Goni should reuse the attenuation principle without binding its architecture to that specific formalism.
legacy: []
---

# Recursive delegation requires explicit scope attenuation

Delegating work downstream should preserve or narrow the authority inherited from the original grant. Recursive delegation is therefore an authority-lineage problem rather than simple task decomposition.
