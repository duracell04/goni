---
id: GONI-PROPOSAL-1C3A0FDFD58A
title: Cross-repo references (policy)
type: proposal
status: draft
implementation_state: specified_only
proposition: This repository is blueprint-only.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: SCOPE-CONTRACT.md
  heading: Cross-repo references (policy)
  revision: 86d76976933ac0bfb8fe18839e67f17a8fc63531
---

# Cross-repo references (policy)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Cross-repo references (policy)

This repository is blueprint-only. Any runnable artifact lives in `goni-prototype-lab`.

**Canonical reference format (guard-safe):**
- Use plain text: `goni-prototype-lab:<relative-path>`
  - Example: `goni-prototype-lab:goni-lab/STATUS.md`
  - Example: `goni-prototype-lab:deploy/k8s/`

**Do NOT:**
- Do not create markdown links that look like local paths for prototype-lab content
  (e.g., `[STATUS](/goni-prototype-lab:...)` or `(/blueprint/deploy/...)`).
  The blueprint guard treats these as in-repo paths and will flag them as broken.

**If you need clickability:**
- Use a full GitHub URL (allowed by the guard), e.g.
  `https://github.com/duracell04/goni-prototype-lab/blob/main/goni-lab/STATUS.md`
  `https://github.com/duracell04/goni-prototype-lab/tree/main/deploy/k8s`
