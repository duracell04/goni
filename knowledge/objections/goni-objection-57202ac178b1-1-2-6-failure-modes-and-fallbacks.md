---
id: GONI-OBJECTION-57202AC178B1
title: 1.2.6 Failure modes and fallbacks
type: objection
status: draft
implementation_state: not_applicable
proposition: 'The platform MUST expose enough signals to detect and mitigate: memory-bound stall and tail-latency spikes, thermal runaway and prolonged throttling, swap thrash from oversubscription, write amplification during compaction.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.2.6 Failure modes and fallbacks
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.2.6 Failure modes and fallbacks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2.6 Failure modes and fallbacks

The platform MUST expose enough signals to detect and mitigate:

- memory-bound stall and tail-latency spikes,
- thermal runaway and prolonged throttling,
- swap thrash from oversubscription,
- write amplification during compaction.

Fallbacks include routing to CPU/iGPU, lowering duty cycle, and deferring
background compaction until safe conditions return.

---
