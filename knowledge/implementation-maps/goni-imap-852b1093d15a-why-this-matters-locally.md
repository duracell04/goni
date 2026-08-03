---
id: GONI-IMAP-852B1093D15A
title: Why this matters locally
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Always-on components can be compact (encoders + predictor).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Why this matters locally
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Why this matters locally

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Why this matters locally

- Always-on components can be compact (encoders + predictor).
- Decoder can be invoked only when output is needed.
- Latent summaries reduce raw-text duplication and improve privacy boundaries.
