---
id: GONI-IMAP-25EC1418A29D
title: 1. What the Goni box should be
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'At a high level, the Goni hardware should: **Look and feel** like a high-end, minimalistic device: compact rectangular box (target: ~7 L, allowed: 6???8 L), matte, neutral finish (e.g.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1. What the Goni box should be
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1. What the Goni box should be

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. What the Goni box should be

At a high level, the Goni hardware should:

- **Look and feel** like a high-end, minimalistic device:
  - compact rectangular box (target: ~7 L, allowed: 6???8 L),
  - matte, neutral finish (e.g. black / dark grey),
  - one power button and one calm status light bar on the front,
  - no RGB, no gamer aesthetics.

- **Fit into real homes and offices**:
  - small enough to sit next to a router, speaker, or monitor,
  - quiet enough to live under or on a desk,
  - no special power requirements (standard wall outlet).

- **Deliver enough performance** to:
  - run medium-to-large language models locally with interactive latency,
  - index and search through a personal document / email corpus,
  - handle multiple tasks in parallel (chat, indexing, small jobs),
  - train lightweight adapters (LoRA-style) on personal data.

- **Act as a cluster node**:
  - multiple Goni boxes on a network should combine into one logical AI system,
  - nothing beyond normal networking hardware should be required for a small cluster (2???4 nodes).

---
