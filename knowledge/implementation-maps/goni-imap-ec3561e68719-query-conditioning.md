---
id: GONI-IMAP-EC3561E68719
title: Query conditioning
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The query/goal X_q is treated as a conditioning signal: selects which aspects of S_v matter, constrains what retrieval is relevant, narrows what "correct" latent update looks like.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Query conditioning
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Query conditioning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Query conditioning

The query/goal `X_q` is treated as a conditioning signal:
- selects which aspects of `S_v` matter,
- constrains what retrieval is relevant,
- narrows what "correct" latent update looks like.

In Goni terms, `X_q` can be:
- a user message,
- a system goal,
- a policy constraint,
- or a tool result that changes the task.
