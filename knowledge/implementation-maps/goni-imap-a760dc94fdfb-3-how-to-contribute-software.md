---
id: GONI-IMAP-A760DC94FDFB
title: 3. How to contribute (software)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Start with 10-requirements.md to understand what the software **must** achieve.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/00-overview.md
  heading: 3. How to contribute (software)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 3. How to contribute (software)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. How to contribute (software)

1. Start with [`10-requirements.md`](/blueprint/software/10-requirements.md) to understand what the software **must** achieve.
2. For new architectural ideas:
   - sketch them in [20-architecture.md](/blueprint/software/20-architecture.md) or a new document under `30-components/`,
   - open a software issue summarising the proposal and linking to the doc.
3. For API or UI changes:
   - update the relevant files under [40-apis-and-ui/](/blueprint/software/40-apis-and-ui),
   - include example flows or payloads where helpful.
4. When a proposal is accepted:
   - add an entry to [90-decisions.md](/blueprint/software/90-decisions.md) to record the decision and its consequences.

The aim is to arrive at a **coherent, minimal software stack** that implements the agreed requirements and can evolve as hardware and AI tools advance.

---
