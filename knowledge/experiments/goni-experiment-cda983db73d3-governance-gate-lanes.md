---
id: GONI-EXPERIMENT-CDA983DB73D3
title: Governance gate lanes
type: experiment
status: draft
implementation_state: not_applicable
proposition: These lanes define required evaluation coverage for a sovereign operator.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/README.md
  heading: Governance gate lanes
  revision: 1fce8b46efaf612f63f99ec32300ebf4da32522f
---

# Governance gate lanes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Governance gate lanes

These lanes define required evaluation coverage for a sovereign operator. They
may reuse existing evidence files until dedicated artifacts are added.

- **Policy tests:** verify allow/deny decisions, corridor outcomes, approval
  evidence, and fail-closed behavior.
- **Route tests:** verify local-first routing, privacy-class handling, cloud
  escalation reasons, and refusal when no permitted route exists.
- **Memory retrieval tests:** verify permission filters, Work Order binding,
  expired/quarantined memory exclusion, and source waypoint fidelity.
- **Autonomy-safety tests:** verify open-loop detection, background autonomy
  triggers, risk thresholds, and user interruption rules.
- **Rollback tests:** verify idempotency, compensation paths, rollback refs, and
  no partial side effects after failure.
- **Parser/ingestion tests:** verify parser identity, source hashes,
  confidence flags, chunk boundaries, and `parser_basis` receipt coverage.
- **Desktop Agent Firewall tests:** verify observation, extraction, memory,
  actuation, egress, sandbox, approval, and receipt boundaries remain separate
  for desktop/browser/vision-mediated agents.
- **Open adversarial tests:** verify that promoted models, tools, memory
  policies, routing rules, harness changes, and autonomy expansions pass
  versioned adversarial scenarios before they move closer to the principal's
  runtime.

Governance gates answer "was this allowed and reversible?" in addition to
"did it work?" Standard observability metrics do not satisfy these gates by
themselves.
