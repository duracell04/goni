---
id: GONI-EXPERIMENT-F2AB32D85916
title: Experimental long-context reading lane
type: experiment
status: draft
implementation_state: not_applicable
proposition: As a research lane, the predictor may also choose **programmatic corpus inspection** over large external text environments.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Experimental long-context reading lane
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Experimental long-context reading lane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Experimental long-context reading lane

As a research lane, the predictor may also choose **programmatic corpus
inspection** over large external text environments. This does not replace the
retrieval baseline; it is a strategy to compare against it.

Experimental action family:

- `corpus.search`
- `corpus.slice`
- `corpus.filter`
- `corpus.subread`
- `corpus.merge_evidence`

These actions are treated like any other mediated tool plan:

- they are budgeted,
- they must preserve provenance,
- and they remain auditable through receipts if promoted beyond research mode.
