---
id: GONI-IMAP-AC64BA078497
title: 2.1 Responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Request ingestion** HTTP /v1/chat/completions (OpenAI-like).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 2.1 Responsibilities
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 2.1 Responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Responsibilities

1. **Request ingestion**
   - HTTP /v1/chat/completions (OpenAI-like).
   - Local RPC/CLI entry points.

2. **Normalisation & validation**
   - Check request size against global limits (tokens, tools, attachments).
   - Map model="goni-small" ? internal ModelTier::Small.
   - Map request type to TaskClass (interactive/background).
   - Compile interaction mode and Work Order metadata before submission.

3. **Job construction**
   - Build
     
     J = (\text{class}, \text{budget}, \text{tools}, \text{profile},
     \text{interaction\_mode}, \text{work\_order\_ref}, \dots)
     
     where udget encodes token/time/energy caps and class ? {interactive, background, maintenance}.

4. **Hand-off to Control Plane**
   - Submit J into scheduler queue.
   - Receive job completion / token stream from ??+??.

5. **Response handling**
   - Stream tokens back to clients.
   - Attach tool call traces / metadata.
   - Log metrics into the Data Plane (??) for observability.
