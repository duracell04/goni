---
id: GONI-PROPOSAL-5C09F1A8BA07
title: 1. Establish the inference baseline
type: proposal
status: draft
implementation_state: specified_only
proposition: Implement and exercise one local OpenAI-compatible backend behind the existing runtime abstraction, then add a second backend only when it exposes a meaningful hardware or serving difference.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: 1. Establish the inference baseline
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# 1. Establish the inference baseline

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1. Establish the inference baseline

- Implement and exercise one local OpenAI-compatible backend behind the
  existing runtime abstraction, then add a second backend only when it exposes
  a meaningful hardware or serving difference.
- Prefer downloadable open-weight checkpoints whose licenses permit the
  intended local use, modification, and redistribution; pin hashes, templates,
  tokenizer assets, and quantization metadata so the bundle cannot drift at a
  provider's discretion.
- Keep the default local text path free of mandatory moderation sidecars,
  provider policy prompts, remote entitlement checks, and hidden network
  dependencies. Any owner-selected content control must be explicit,
  inspectable, removable, and disabled independently of tool authorization.
- Evaluate current instruction/tool-capable model bundles on legitimate-request
  completion, schema-valid tool selection, correction following, latency,
  memory use, context pressure, and offline behavior.
- Keep the model engine stateless. Prompts, persona, memory, and policy remain
  versioned Goni inputs rather than hidden runtime state.
- Reject absolute "never refuses" claims. Measure unnecessary refusal and
  instruction-following rates on an approved, reproducible evaluation pack.
