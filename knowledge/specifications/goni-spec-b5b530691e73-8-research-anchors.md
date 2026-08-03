---
id: GONI-SPEC-B5B530691E73
title: 8. Research anchors
type: specification
status: draft
implementation_state: specified_only
proposition: 'CDC is closest to a hybrid of learning from human feedback, continual personalization, preference modeling, and managed memory systems: Christiano et al., "Deep reinforcement learning from human preferences": https://arxiv.org/abs/1706.03741 Rafailov et al., "Direct Preference Optimization": https://arxiv.org/abs/2305.18290'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 8. Research anchors
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 8. Research anchors

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Research anchors

CDC is closest to a hybrid of learning from human feedback, continual
personalization, preference modeling, and managed memory systems:

- Christiano et al., "Deep reinforcement learning from human preferences":
  https://arxiv.org/abs/1706.03741
- Rafailov et al., "Direct Preference Optimization":
  https://arxiv.org/abs/2305.18290
- Packer et al., "MemGPT: Towards LLMs as Operating Systems":
  https://arxiv.org/abs/2310.08560
- Liang et al., "Learning Personalized Agents from Human Feedback":
  https://arxiv.org/abs/2602.16173
- MemOS memory-as-managed-resource prior art:
  https://arxiv.org/abs/2507.03724

These references motivate the design. They do not prove that CDC improves Goni
until Goni-specific replay and longitudinal evaluation exists.
