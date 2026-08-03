---
id: GONI-SYNTHESIS-E329A28EADBF
title: AI-2027 as knowledge source
type: synthesis
status: draft
implementation_state: specified_only
proposition: Keep a snapshot of AI-2027 (PDF/HTML) under blueprint/docs/assets/ai-2027/ with a manifest.json (see manifest.json.example) capturing source URL, retrieved_at, format, file name, and sha256 for deterministic ingestion.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: AI-2027 as knowledge source
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# AI-2027 as knowledge source

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## AI-2027 as knowledge source
- Keep a snapshot of AI-2027 (PDF/HTML) under `blueprint/docs/assets/ai-2027/` with a `manifest.json` (see `manifest.json.example`) capturing source URL, retrieved_at, format, file name, and sha256 for deterministic ingestion.
- Index it into the Arrow spine for RAG so safety/strategy answers cite this corpus instead of generic web takes.
- It also motivates constrained/offline modes and multi-provider indirection.
