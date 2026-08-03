---
id: GONI-EXPERIMENT-560F1D86253B
title: Core checks
type: experiment
status: draft
implementation_state: not_applicable
proposition: observation does not imply extraction extraction does not imply memory memory does not imply actuation synthetic input requires a capability token, sandbox profile, autonomy corridor, and receipt remote extraction of screen/app context requires Network Gate permission denied boundary transitions fail closed and remain auditable
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-DESK-01-desktop-agent-firewall.md
  heading: Core checks
  revision: aac7c2d833bd8db8894cb18deb97d6bc13e0b7b3
---

# Core checks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Core checks

- observation does not imply extraction
- extraction does not imply memory
- memory does not imply actuation
- synthetic input requires a capability token, sandbox profile, autonomy
  corridor, and receipt
- remote extraction of screen/app context requires Network Gate permission
- denied boundary transitions fail closed and remain auditable
- receipts omit raw private screenshots, full OCR text, accessibility dumps,
  audio transcripts, and unbounded prompts by default
