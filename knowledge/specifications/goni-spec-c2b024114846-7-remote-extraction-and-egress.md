---
id: GONI-SPEC-C2B024114846
title: 7. Remote extraction and egress
type: specification
status: draft
implementation_state: specified_only
proposition: Sending observed or extracted screen context to a remote model, service, connector, or API is egress.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 7. Remote extraction and egress
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 7. Remote extraction and egress

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Remote extraction and egress

Sending observed or extracted screen context to a remote model, service,
connector, or API is egress. It requires both:

- an extraction grant that permits remote submission for the observed content
  class, and
- a Network Gate capability that permits the destination, purpose, payload
  classification, budget, and redaction mode.

If either grant is absent, remote extraction is denied.
