---
id: GONI-SYNTHESIS-9D421CB87853
title: Standard request spans (trace framing)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Instrument each request with spans so metrics are explainable.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: Standard request spans (trace framing)
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Standard request spans (trace framing)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Standard request spans (trace framing)
Instrument each request with spans so metrics are explainable.

- `ingress` (request received, validated)
- `policy_check` (capability evaluation and permission gating)
- `context_assemble` (Vault/RAG retrieval and prompt build)
- `prefill` (prompt processing)
- `decode_stream` (token generation)
- `tool_call` (one span per tool)
- `egress` (Network Gate decision and bytes)
- `receipt_write` (receipt creation and integrity)
- `complete` (finalization)
