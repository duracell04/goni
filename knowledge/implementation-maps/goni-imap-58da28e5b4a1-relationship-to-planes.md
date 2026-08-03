---
id: GONI-IMAP-58DA28E5B4A1
title: Relationship to planes
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Data Plane (\(\mathcal{A}\) / \(\mathcal{X}\))**: observations, files, snapshots, events.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Relationship to planes
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Relationship to planes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Relationship to planes

- **Data Plane (\(\mathcal{A}\) / \(\mathcal{X}\))**: observations, files, snapshots, events.
- **Memory Plane**: vector store, structured facts, latent summaries, timelines.
- **Control Plane (\(\mathcal{K}\))**: policies about privacy, tool permissions, escalation rules.
- **Execution Substrate (\(\mathcal{E}\))**: orchestrates encoder/predictor/decoder workers as queued jobs.
