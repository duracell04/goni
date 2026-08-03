---
id: GONI-PROPOSAL-3BAF5ED1FE15
title: 'Repo structure may change (Discovery Protocol: do not hallucinate paths)'
type: proposal
status: draft
implementation_state: specified_only
proposition: Paths are allowed to change.
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/AGENTS.md
  heading: 'Repo structure may change (Discovery Protocol: do not hallucinate paths)'
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Repo structure may change (Discovery Protocol: do not hallucinate paths)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Repo structure may change (Discovery Protocol: do not hallucinate paths)
Paths are allowed to change. Agents MUST use discovery instead of guessing.

When a referenced path is missing, resolve in this order:
1) Truth Map (below)
2) Filename search (canonical filenames)
3) H1 title search (exact document title)
4) Unique keyword anchors (distinctive phrases)

If multiple candidates match:
1) prefer docs under `blueprint/30-specs/` (if present)
2) else prefer docs linked from `README.md` or `blueprint/docs/README.md`
3) else prefer the file whose content matches the expected H1 title most closely

If discovery fails:
- STOP. Do not invent content.
- Report: "Canonical file not found in current tree."
- List searches performed (filename / title / keywords) and propose the smallest safe remediation (update links, restore from history, or add redirect stub).

---
