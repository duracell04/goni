---
id: EVID-CONTINUAL-EXPERIENCE-COLLAPSE-01
title: 'Source claim: naive repeated experience internalization can degrade capability'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Chen et al. report progressive capability collapse under existing multi-iteration experience-internalization methods and show that stability depends on experience granularity, injection pattern, and internalization regime.
domains:
- research
- learning
- agents
aliases: []
relations:
- type: supports
  target: ADAPTATION-LEVELS-01
sources:
- SRC-CHEN2026-CONTINUAL-EXPERIENCE
artifacts: []
uncertainty: The reported collapse concerns the studied internalization methods and does not imply that all continual-learning methods degrade.
legacy: []
---

# Source claim: naive repeated experience internalization can degrade capability

Repeated self-learning is not automatically cumulative improvement. Chen et al. report progressive degradation under several existing experience-internalization approaches.

For Goni, persistent changes should therefore pass replay, independent evaluation, promotion, provenance, and rollback gates rather than being accepted merely because they were derived from prior experience.
