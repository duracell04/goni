---
id: GONI-IMAP-63C6C47522CB
title: 3. Storage Boundaries
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Plane 𝒜 text is persisted (Parquet + Lance v2).
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
  heading: 3. Storage Boundaries
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Storage Boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Storage Boundaries
- Plane 𝒜 text is persisted (Parquet + Lance v2). Plane 𝒳 text is ephemeral (memory, optional encrypted spill).
- Control/Execution planes never store raw text; hashes/IDs only.
