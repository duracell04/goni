---
id: GONI-IMAP-7AEADCA109AA
title: Context Plane
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '[ ] The objective \(F\) used by the selector is monotone submodular (facility location + modular term).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: Context Plane
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# Context Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Context Plane

- [ ] The objective \(F\) used by the selector is monotone submodular (facility location + modular term).  
- [ ] Greedy selection empirically achieves
  $$
  F(S_{\text{greedy}}) \ge 0.63\,F(S^\*)
  $$
  on small synthetic instances.
