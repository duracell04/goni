---
id: GONI-SYNTHESIS-B681A094AF50
title: 7. How Goni fits in this landscape
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Putting all of this together: **EXO / MultiCortex EXO / Cake / prima.cpp** focus on **distributed inference**, with the cluster itself as the primary product: “take all your devices and turn them into one big GPU”.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 7. How Goni fits in this landscape
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 7. How Goni fits in this landscape

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. How Goni fits in this landscape

Putting all of this together:

- **EXO / MultiCortex EXO / Cake / prima.cpp** focus on  
  **distributed inference**, with the cluster itself as the primary product:
  - “take all your devices and turn them into one big GPU”.

- **Beowulf AI Cluster** focuses on  
  **deployment and benchmarking**: making it easy to test llama.cpp, EXO, distributed-llama, etc., on “random computers”.

- **llama.cpp** focuses on  
  **maximising single-node inference performance** across as many architectures as possible.
