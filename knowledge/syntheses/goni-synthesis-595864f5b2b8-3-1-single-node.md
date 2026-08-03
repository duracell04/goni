---
id: GONI-SYNTHESIS-595864F5B2B8
title: 3.1 Single node
type: synthesis
status: draft
implementation_state: specified_only
proposition: User -> gateway -> orchestrator -> {llm-local, vecdb, tools}, all running in containers on Ubuntu Server.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 3.1 Single node
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 3.1 Single node

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Single node

User -> gateway -> orchestrator -> {llm-local, vecdb, tools}, all running in containers on Ubuntu Server.

- **OS**: Ubuntu Server LTS (or similar), encrypted disk optional.  
- **Runtime**: containers (Docker / containerd).  
- **Core services**:
  - `llm-local`: vLLM/TGI bound to the APU (iGPU + CPU).  
  - `vecdb`: Qdrant/Milvus for embeddings and RAG.  
  - `orchestrator`: decides when to use local models vs cloud models; manages tools and workflows.  
  - `gateway`: HTTPS API (OpenAI-compatible) + web dashboard.  
  - `wg-mesh`: WireGuard for secure remote access and node-to-node mesh.
