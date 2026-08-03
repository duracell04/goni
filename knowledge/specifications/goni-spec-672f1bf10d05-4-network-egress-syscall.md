---
id: GONI-SPEC-672F1BF10D05
title: 4. Network egress syscall
type: specification
status: draft
implementation_state: specified_only
proposition: 'Networking is a capability-scoped syscall: Requirements: Caller MUST present a network capability token (TOOL-01).'
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
  heading: 4. Network egress syscall
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 4. Network egress syscall

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Network egress syscall

Networking is a capability-scoped syscall:

```
net.egress(
  route,
  purpose,
  classification,
  budgets,
  retention_mode,
  payload_ref | stream_ref
) -> receipt_id
```

Requirements:

- Caller MUST present a network capability token (TOOL-01).
- Gate MUST validate route/purpose/budget/retention constraints.
- Gate MUST emit an audit receipt for every external transfer.
- If the payload derives from observed desktop, browser, or vision context, the
  caller MUST also present boundary refs proving extraction permission and
  permitted remote submission under BOUND-01.
