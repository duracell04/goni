---
id: GONI-SYNTHESIS-E7D604F11670
title: Quickstart (local stack)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Prototype/dev stack is specified only; things may change between commits.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: Quickstart (local stack)
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# Quickstart (local stack)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Quickstart (local stack)

Prototype/dev stack is specified only; things may change between commits.

Prereqs: Docker and docker compose. See goni-prototype-lab:goni-lab/quickstart.md for the intended workflow.

Services:  
- `llm-local` (vLLM) at `http://localhost:8000/v1`  
- `vecdb` (Qdrant) at `http://localhost:6333`  
- `orchestrator` (goni-http) at `http://localhost:7000`  

Env vars of interest:  
- `LLM_LOCAL_URL` (default: `http://llm-local:8000/v1`)  
- `LLM_MODEL` (default: `mistralai/Mistral-7B-Instruct-v0.3`)  
- `QDRANT_HTTP_URL` (default: `http://vecdb:6333`)  
- `QDRANT_COLLECTION` (default: `default`)  
- `EMBED_DIM` (default: `1024`)

Example call (against llm-local):  
`curl http://localhost:8000/v1/models`
