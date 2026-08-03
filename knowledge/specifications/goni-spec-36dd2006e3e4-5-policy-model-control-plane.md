---
id: GONI-SPEC-36DD2006E3E4
title: 5. Policy model (Control Plane)
type: specification
status: draft
implementation_state: specified_only
proposition: Network policies are declared and bound to agents via capability tokens and policy_hash.
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 5. Policy model (Control Plane)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 5. Policy model (Control Plane)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Policy model (Control Plane)

Network policies are declared and bound to agents via capability tokens and
policy_hash. At minimum:

- Route constraints: DIRECT vs OVERLAY.
- Purpose constraints: REMOTE_LLM, WEB_SEARCH, MESSAGING, UPDATE, TIME_SYNC, etc.
- Budgets: bytes, time, concurrency, retries, daily quota.
- Retention constraints: minimal vs verbose receipt logging.

Default policy is deny-by-default for external egress; explicit capability is
required for any route outside the local node.
