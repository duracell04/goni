---
id: GONI-IMAP-FF3B3ABFC280
title: v1.1 – 2026-01-15
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Added Prompts table and materialization metadata (hash, redaction flags).
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/99-changelog.md
  heading: v1.1 – 2026-01-15
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# v1.1 – 2026-01-15

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## v1.1 – 2026-01-15
- Added Prompts table and materialization metadata (hash, redaction flags).
- Added RedactionProfiles and RedactionEvents tables for minimization/audit.
- Added MemoryEntries table with gating fields (kind, confidence, review/ttl).
