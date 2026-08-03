---
id: GONI-SPEC-8F3E6C9C50FF
title: 2.1 Product Requirements Inferred from Reference Systems (Olares)
type: specification
status: draft
implementation_state: specified_only
proposition: These requirements are derived from reference product patterns (see blueprint/docs/reference-products/olares.md) and are treated as enforceable behavior.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 2.1 Product Requirements Inferred from Reference Systems (Olares)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 2.1 Product Requirements Inferred from Reference Systems (Olares)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1 Product Requirements Inferred from Reference Systems (Olares)

These requirements are derived from reference product patterns (see
`blueprint/docs/reference-products/olares.md`) and are treated as enforceable behavior.

- **R-UX-OWNERSHIP:** Default local-first; network access is an explicit, user-granted
  capability with visible policy and logs.
- **R-APP-ECOSYSTEM:** Agents are installable packages with manifests, permissions,
  and budgets; install/uninstall is first-class.
- **R-IDENTITY-SSO:** A single identity plane governs UI, agents, and tools; no
  per-app auth silos.
- **R-REMOTE-PRESENCE:** Secure remote access is a first-class feature with safe
  defaults and clear status.
- **R-ADMIN-OBSERVABILITY:** Provide a dashboard for node state, agents,
  permissions, and resource budgets.
