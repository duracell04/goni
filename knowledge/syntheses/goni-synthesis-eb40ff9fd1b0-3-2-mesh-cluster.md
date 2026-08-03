---
id: GONI-SYNTHESIS-EB40FF9FD1B0
title: 3.2 Mesh / cluster
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Every Goni node is a **cluster node**: 1st node -> control plane (e.g.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 3.2 Mesh / cluster
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 3.2 Mesh / cluster

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Mesh / cluster

Every Goni node is a **cluster node**:

- 1st node -> control plane (e.g. k3s server).  
- Additional nodes -> join as workers via "join token / URL".

The orchestrator sees accelerators abstractly:

- `apu:0` -> local Ryzen AI APU (iGPU+CPU).  
- `npu:0` -> local NPU (XDNA 2).  
- `gn100:0` -> (future) Grace Blackwell GN100 node in the same mesh.

Tasks:

- **Interactive** -> run on the node closest to the user.  
- **Batch** (embeddings, long research, nightly updates) -> can be offloaded to other nodes or GN100.

The prototype code models this routing, even if not fully implemented yet. GN100-class nodes are treated as just another accelerator in the mesh.

---
