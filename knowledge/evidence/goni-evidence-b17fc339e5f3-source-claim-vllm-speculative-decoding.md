---
id: GONI-EVIDENCE-B17FC339E5F3
title: 'Source claim: vllm-speculative-decoding'
type: evidence
status: draft
implementation_state: not_applicable
proposition: vLLM exposes speculative decoding as an inference feature where a draft path proposes candidate tokens and the target model verifies them.
domains:
- research
aliases: []
relations:
- type: supports
  target: LLM-RUNTIME
sources:
- SRC-VLLM-SPECULATIVE-DECODING
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[vllm-speculative-decoding]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: vllm-speculative-decoding

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[vllm-speculative-decoding]]
Claim: vLLM exposes speculative decoding as an inference feature where a draft
path proposes candidate tokens and the target model verifies them.
Relevance:
- Supports representing speculative decoding as an optional backend/runtime
  capability while keeping routing and escalation policy in Goni's control
  plane.
Used in:
- `blueprint/software/30-components/llm-runtime.md`
Source:
- https://docs.vllm.ai/en/stable/features/speculative_decoding/
