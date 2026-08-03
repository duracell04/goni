---
id: GONI-IMAP-DA34CA2920C0
title: 4. Responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Validate that requested workflow templates are declared and hash-addressed.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/visual-runtime.md
  heading: 4. Responsibilities
  revision: 4d6a56dfeb55430356f9e72b203b5df766df28e8
---

# 4. Responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Responsibilities

- Validate that requested workflow templates are declared and hash-addressed.
- Load only model bundles approved by MODEL-REG-01 for the requested visual
  task class and asset permission class.
- Keep raw private image content out of Control-plane logs and receipts.
- Preserve deterministic seeds or execution settings when an audit-grade visual
  run needs replayability.
- Return verification summaries and artifact hashes to the kernel.
- Support cancellation and budget checks for long visual jobs.
