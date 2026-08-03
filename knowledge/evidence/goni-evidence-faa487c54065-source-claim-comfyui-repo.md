---
id: GONI-EVIDENCE-FAA487C54065
title: 'Source claim: comfyui-repo'
type: evidence
status: draft
implementation_state: not_applicable
proposition: ComfyUI is a modular diffusion model GUI, API, and backend with a graph/node interface for visual workflows.
domains:
- research
aliases: []
relations:
- type: supports
  target: VIS-01
- type: supports
  target: VIS-RUNTIME-01
sources:
- SRC-COMFYUI-REPO
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[comfyui-repo]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: comfyui-repo

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[comfyui-repo]]
Claim: ComfyUI is a modular diffusion model GUI, API, and backend with a
graph/node interface for visual workflows.
Relevance:
- Grounds the Visual Runtime's use of hashed node-graph workflows as execution
  substrate while keeping Goni policy and receipts outside the backend.
Used in:
- `blueprint/30-specs/visual-intelligence-plane.md` (Execution substrate)
- `blueprint/software/30-components/visual-runtime.md` (Backend substrate)
Source:
- https://github.com/Comfy-Org/ComfyUI
