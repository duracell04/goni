---
id: GONI-IMAP-AAFACA09224C
title: 2. Backend substrate
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Visual execution may use ComfyUI-compatible node graphs, diffusion pipelines, segmentation services, OCR/layout analyzers, open-set detectors, visual-language models, or embedding extractors.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/visual-runtime.md
  heading: 2. Backend substrate
  revision: 4d6a56dfeb55430356f9e72b203b5df766df28e8
---

# 2. Backend substrate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Backend substrate

Visual execution may use ComfyUI-compatible node graphs, diffusion pipelines,
segmentation services, OCR/layout analyzers, open-set detectors, visual-language
models, or embedding extractors. These backends are adapters behind the Visual
Runtime interface.

ComfyUI-style graphs are useful because they make visual work explicit as
workflow nodes: load model, load reference, segment, mask, inpaint, control,
sample, upscale, verify, and save. Goni treats those graphs as hashed execution
templates. A graph hash is receipt evidence, not policy authority
[[comfyui-repo]].
