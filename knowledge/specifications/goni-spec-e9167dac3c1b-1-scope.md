---
id: GONI-SPEC-E9167DAC3C1B
title: 1. Scope
type: specification
status: draft
implementation_state: specified_only
proposition: This spec applies to tasks that inspect, transform, generate, annotate, compare, or remember image-like artifacts, including screenshots, diagrams, document pages, product photos, brand assets, mockups, evidence images, masks, and generated outputs.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 1. Scope
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 1. Scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope

This spec applies to tasks that inspect, transform, generate, annotate, compare,
or remember image-like artifacts, including screenshots, diagrams, document
pages, product photos, brand assets, mockups, evidence images, masks, and
generated outputs.

When the source asset is a live screen, desktop app, browser tab, accessibility
tree, or OS event stream, the task also carries Desktop Agent Firewall boundary
metadata. A screenshot audit may stop at observation and extraction; it may not
write memory or act unless separate grants authorize those powers.

Visual execution backends may include ComfyUI-compatible workflow engines,
diffusion runtimes, segmentation models, open-set detectors, OCR/layout
analyzers, visual-language models, and embedding models. These backends are
replaceable substrate. They do not own authority, approval corridors, asset
permissions, memory promotion, receipts, or rollback.
