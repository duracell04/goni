---
id: GONI-SYNTHESIS-E3ECE662464B
title: References (evidence anchors)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Purpose: keep a small, stable list of evidence keys that specs can cite without repeating full citations or long summaries.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/references/README.md
  heading: References (evidence anchors)
  revision: aec2b42c5d6e89d472d26952aa70be944ae50228
---

# References (evidence anchors)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# References (evidence anchors)

Purpose: keep a small, stable list of evidence keys that specs can cite
without repeating full citations or long summaries.

Conventions:
- Use stable keys like `[[liu2023-lost-middle]]`.
- Each key maps to a short, annotated entry in `bibliography.md`.
- Specs cite keys inline or as footnotes.
- Do not add citations to vision docs (`README.md`, `blueprint/docs/goni-story.md`,
  `blueprint/docs/goni-whitepaper.md`).

Additions:
1) Add a new key to `bibliography.md`.
2) Reference it from the relevant spec section.

Delegation research traceability:
- [Personal twin autonomy map](/blueprint/docs/references/personal-twin-autonomy-map.md)

Research-neighbor synthesis:
- [GoniOS research neighbor map](/blueprint/docs/references/gonios-research-neighbor-map.md)

See `bibliography.md` for the current list.
