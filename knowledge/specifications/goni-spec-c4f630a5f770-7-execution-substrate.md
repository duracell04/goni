---
id: GONI-SPEC-C4F630A5F770
title: 7. Execution substrate
type: specification
status: draft
implementation_state: specified_only
proposition: Visual models enter through MODEL-REG-01 as governed model bundles.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 7. Execution substrate
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 7. Execution substrate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Execution substrate

Visual models enter through MODEL-REG-01 as governed model bundles. Examples of
substrate families include FLUX-style image generators, Qwen-Image-style
typography/editing models, Stable Diffusion-family workflows, SAM-style
segmentation, open-set detectors, Florence/Qwen-VL/InternVL-style visual
reasoning, and CLIP/OpenCLIP/DINO-style embeddings.

Workflow engines such as ComfyUI-compatible node graphs are execution backends,
not control planes. Goni compiles the visual Work Order into a workflow
template, supplies policy-approved model bundle IDs and asset refs, receives
output hashes and intermediate refs, performs verification, and emits the
canonical receipt.

Reference anchors: FLUX licensing and bundle variants [[bfl-flux-repo]],
Qwen-Image typography/editing direction [[qwen-image-2-2026]], SAM 2
segmentation [[sam2-2024]] [[meta-sam2-page]], and ComfyUI-style node workflows
[[comfyui-repo]].
