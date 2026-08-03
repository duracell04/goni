---
id: GONI-SYNTHESIS-A68046EBFCAE
title: 3. When Goni uses the Council
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Default rule: Goni tries local memory, local tools, local retrieval, and local models first.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 3. When Goni uses the Council
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 3. When Goni uses the Council

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. When Goni uses the Council
- Default rule: Goni tries local memory, local tools, local retrieval, and local
  models first. The Council is an escalation tier, not the default brain.
- Council rule: use councils only at uncertainty boundaries, not for routine
  local work. A council call must buy external freshness, material model
  diversity, high-stakes review, or a clear confidence/risk reduction.
- Explicit user request: UI/API flag `provider="council"` or `mode="paranoid"` routes to council.
- Orchestrator heuristics (examples):
  - Task classified "high difficulty" or "safety-critical".
  - Context length beyond local model comfort.
  - Sensitive domains where a second opinion is required (e.g., medical/financial summaries).
  - Current public web or external model knowledge is required and the outgoing
    payload is public, redacted, or explicitly approved.
  - Local retrieval has insufficient evidence or local model confidence is low.
- Hard constraint: if council is unavailable (no network/keys), orchestrator degrades gracefully ("I can only use local models right now") rather than blocking or crashing.
