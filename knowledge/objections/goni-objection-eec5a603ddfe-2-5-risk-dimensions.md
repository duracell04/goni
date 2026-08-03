---
id: GONI-OBJECTION-EEC5A603DDFE
title: 2.5 Risk dimensions
type: objection
status: draft
implementation_state: not_applicable
proposition: 'A normalized risk_score in [0,1] MUST be computed from, at minimum: reversibility and compensation path quality, blast radius (financial, legal, or reputational magnitude), ambiguity/uncertainty, policy sensitivity (regulated domain, restricted counterparty, or consent gate).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 2.5 Risk dimensions
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 2.5 Risk dimensions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.5 Risk dimensions

A normalized `risk_score` in `[0,1]` MUST be computed from, at minimum:

- reversibility and compensation path quality,
- blast radius (financial, legal, or reputational magnitude),
- ambiguity/uncertainty,
- policy sensitivity (regulated domain, restricted counterparty, or consent gate).
