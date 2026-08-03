---
id: GONI-SYNTHESIS-4A526B81A8D2
title: 5. Integration points in this repo
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Council config**: keep goni-prototype-lab:config/council.yaml as the ground truth; Lab “promote” steps propose edits, not auto-merge.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-lab.md
  heading: 5. Integration points in this repo
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 5. Integration points in this repo

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Integration points in this repo
- **Council config**: keep `goni-prototype-lab:config/council.yaml` as the ground truth; Lab “promote” steps propose edits, not auto-merge.
- **Router training data**: logs become supervised data for `goni-router` regret tests in `blueprint/software/30-conformance.md` (label = chosen/best seat).
- **Data plane**: store logs as Arrow batches (schema lives beside other metrics in `blueprint/software/50-data`); keep prompts hashed with minimal snippets for privacy.
- **Runtime hooks**: Lab is just another mode of the orchestrator UI/CLI; reuse the existing OpenAI-compatible gateways for seat calls.
