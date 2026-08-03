---
id: GONI-IMAP-CC046D4FB74E
title: 4. Redaction Hooks
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Prompts.is_redacted signals PII scrubbing; derived datasets must honor it when materializing contexts or logs.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/40-privacy-and-text-confinement.md
  heading: 4. Redaction Hooks
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4. Redaction Hooks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Redaction Hooks
- `Prompts.is_redacted` signals PII scrubbing; derived datasets must honor it when materializing contexts or logs.
- No downstream pipeline may reconstruct prompts from hashes; hashes serve correlation only.
