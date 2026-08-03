---
id: GONI-PROPOSAL-FB27F56334F4
title: Selection guidance for Goni
type: proposal
status: draft
implementation_state: specified_only
proposition: Goni should not adopt one of these frameworks as its foundation.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Selection guidance for Goni
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Selection guidance for Goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Selection guidance for Goni

Goni should not adopt one of these frameworks as its foundation. That would
outsource the session, permission, or memory semantics that define sovereignty.
Instead:

- use QwenPaw, nanobot, Hivekeep, OpenHuman, meld, Elroy, and ZeroClaw as
  comparative references or optional mediated adapters;
- evaluate MemX, LightMem, and EverOS behind the Memory Plane contract;
- use llama.cpp as the default local foundation, ExLlamaV3 for VRAM-resident
  NVIDIA workloads, KTransformers for RAM-heavy sparse MoE experiments,
  vLLM/SGLang for serving scale, and AirLLM only for compatibility or
  minimum-allocation demonstrations, all behind the existing `LlmRuntime`
  abstraction; and
- promote only components that pass offline, provenance, resource, and
  conformance tests on Goni hardware.
