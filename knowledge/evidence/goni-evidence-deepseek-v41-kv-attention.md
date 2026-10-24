---
id: GONI-EVIDENCE-DEEPSEEK-V41-KV-ATTENTION
title: 'Source claim: DeepSeek-V4.1-Flash KV and sparse-attention architecture'
type: evidence
status: draft
implementation_state: not_applicable
proposition: >-
  DeepSeek reports that V4.1-Flash uses Causal Encoder-Decoder attention,
  SWA Bounded Replay, CSA2 Full/Reindex/Reuse modes, hierarchical sparse
  indexing, and FP4 main-KV caching; the first-party model card reports a
  global KV footprint of 890 bytes per token, about one quarter of
  DeepSeek-V4-Flash, while bounded replay reduces persistent SWA KV storage to
  roughly one eighth of the earlier model.
domains:
- research
- inference
relations:
- type: supports
  target: ATTN-LOCAL-01
sources:
- SRC-DEEPSEEK2026-V41-FLASH-HF
artifacts: []
uncertainty: >-
  Architecture and efficiency numbers are first-party reported results. They
  support a research comparison and implementation hypothesis, not a GONI
  runtime performance claim.
legacy: []
---

# Source claim: DeepSeek-V4.1-Flash KV and sparse-attention architecture

> Status boundary: this evidence records what the cited first-party release reports. It does not establish independent reproduction, local GONI compatibility, or a measured GONI performance improvement.

The DeepSeek-V4.1-Flash release material reports a 40-layer Causal Encoder-Decoder architecture with a 20-layer causal encoder and 20-layer decoder. The decoder's global KV state is projected from final encoder hidden states rather than independently derived from every decoder layer.

The same source reports:

- **SWA Bounded Replay**, which reconstructs missing sliding-window KV state by replaying only the most recent window and thereby avoids persisting that SWA KV state to SSD;
- **Compressed Sparse Attention 2 (CSA2)** with static per-layer **Full**, **Reindex**, and **Reuse** modes that share main KV and indexer state and reuse Top-K sparse-attention indices;
- a **Hierarchical Sparse Indexer** that bounds deeper indexer work through a candidate pool produced upstream; and
- **FP4 main-KV caching**.

The first-party model card reports a **global KV footprint of 890 bytes per token**, approximately **one quarter** of DeepSeek-V4-Flash, and reports that SWA Bounded Replay reduces persistent KV storage to roughly **one eighth** of DeepSeek-V4-Flash.

For GONI, these observations are evidence that the resource cost of long active contexts is architecture-dependent. They do not show that the exact DeepSeek mechanisms are optimal for GONI or that the model is suitable for the target local appliance.
