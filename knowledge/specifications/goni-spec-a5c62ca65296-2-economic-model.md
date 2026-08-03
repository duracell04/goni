---
id: GONI-SPEC-A5C62CA65296
title: 2. Economic Model
type: specification
status: draft
implementation_state: specified_only
proposition: 'An agent acting under DAT-01 optimizes expected net value, not nominal lowest price: Where: Utility is the principal''s expected value from the good, service, quality, timing, or contract terms.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegated-agent-treasury.md
  heading: 2. Economic Model
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# 2. Economic Model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Economic Model

An agent acting under DAT-01 optimizes expected net value, not nominal lowest
price:

```text
ExpectedNetValue = Utility - Price - Risk - MonitoringCost
```

Where:

- `Utility` is the principal's expected value from the good, service, quality,
  timing, or contract terms.
- `Price` is the monetary consideration paid or committed.
- `Risk` includes counterparty, fraud, delivery, compliance, legal, privacy,
  data leakage, settlement, and quality uncertainty.
- `MonitoringCost` includes approvals, audits, evidence collection,
  reconciliation, and transaction overhead.

The agent's goal is the best risk-adjusted price within the authorized mandate.
A lower nominal price MUST NOT be selected when it violates counterparty,
quality, legal, privacy, delivery, or approval constraints.
