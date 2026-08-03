---
id: GONI-SYNTHESIS-A6F2FA1A5658
title: 2.4 Arrow spine (shared, high-performance memory)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Unified, columnar data layer** Emails, events, tasks, transactions, metrics… all in a shared Arrow-based schema.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 2.4 Arrow spine (shared, high-performance memory)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2.4 Arrow spine (shared, high-performance memory)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.4 Arrow spine (shared, high-performance memory)

- **Unified, columnar data layer**  
  - Emails, events, tasks, transactions, metrics… all in a shared Arrow-based schema.
- **Zero-copy access for agents**  
  - Multiple agents can scan the same data without re-parsing.

**Why this matters:**  
Everyone else is reinventing storage: SQLite, vector DBs, JSON blobs. Goni can credibly claim **analytics-grade infra for personal data**.

---
