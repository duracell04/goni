---
id: GONI-DECISION-1A750217A0DF
title: Backend decision hierarchy
type: decision
status: draft
implementation_state: specified_only
proposition: Use llama.cpp as the safest overall foundation for a dependable local AI secretary.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Backend decision hierarchy
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Backend decision hierarchy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Backend decision hierarchy

1. Use llama.cpp as the safest overall foundation for a dependable local AI
   secretary.
2. Prefer ExLlamaV3 when the deployment is NVIDIA-only and the selected model
   plus cache fit substantially in aggregate VRAM.
3. Use KTransformers as the specialist option for very large sparse MoE models
   on a RAM-heavy workstation.
4. Move to vLLM or SGLang when the assistant becomes a multi-user or production
   service with adequate accelerator memory.
5. Treat AirLLM as a research, compatibility, and minimum-allocation instrument
   rather than the primary interactive backend.
