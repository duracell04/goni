---
id: GONI-SPEC-7F27A26D2065
title: 1. Scope
type: specification
status: draft
implementation_state: specified_only
proposition: 'This spec applies to: local GUI agents using screenshot or framebuffer loops, cloud computer-use systems that request screenshots and return actions, browser automation and browser-isolated tools, embodied robot sensor and actuator systems, OS memory layers that record screen, audio, accessibility, or OCR history, desktop integrations that read app context through accessibility APIs,'
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 1. Scope
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 1. Scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Scope

This spec applies to:

- local GUI agents using screenshot or framebuffer loops,
- cloud computer-use systems that request screenshots and return actions,
- browser automation and browser-isolated tools,
- embodied robot sensor and actuator systems,
- OS memory layers that record screen, audio, accessibility, or OCR history,
- desktop integrations that read app context through accessibility APIs,
- permissioned-view assistants that observe and annotate only,
- future OS-native vision, capture, and desktop-agent APIs.

The same boundary model applies whether inference is local, remote, hybrid, or
absent.
