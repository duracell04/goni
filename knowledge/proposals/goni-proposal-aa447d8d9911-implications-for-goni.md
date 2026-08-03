---
id: GONI-PROPOSAL-AA447D8D9911
title: Implications for Goni
type: proposal
status: draft
implementation_state: specified_only
proposition: 'Goni should treat media provenance as evidence, not as a truth oracle: preserve C2PA/IPTC records when ingesting or exporting media; validate signatures and retain the signer, actions, timestamps, and trust decision as separate fields; distinguish generated, edited, disclosed, and classifier_inferred evidence instead of collapsing them into one boolean;'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/ai-media-provenance.md
  heading: Implications for Goni
  revision: fc0e3881f67979ba52b15bdcf2c3bc651981fbd8
---

# Implications for Goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Implications for Goni

Goni should treat media provenance as evidence, not as a truth oracle:

- preserve C2PA/IPTC records when ingesting or exporting media;
- validate signatures and retain the signer, actions, timestamps, and trust
  decision as separate fields;
- distinguish `generated`, `edited`, `disclosed`, and `classifier_inferred`
  evidence instead of collapsing them into one boolean;
- record missing or stripped provenance as `unknown`, not `human_created`;
- show source and confidence when provenance affects a brief or action; and
- keep media truth assessment separate from production-history verification.

These are technology-intelligence implications only. Any normative Goni media
schema or API change requires a separate specification decision.
