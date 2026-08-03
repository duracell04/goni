---
id: DAT-01
title: DAT-01 - Delegated Agent Treasury
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: DAT-01 Status: Specified only / roadmap This spec defines the control-plane contract for agents that search, bargain, contract, and pay under delegated financial authority.'
domains:
- agent
- specs
aliases:
- DELEGATED-AGENT-TREASURY
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegated-agent-treasury.md
  heading: DAT-01 - Delegated Agent Treasury
  revision: 2ff6ae6b5cc74e5c25b8ae959713f27d0c9bbfd0
---

# DAT-01 - Delegated Agent Treasury

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# DAT-01 - Delegated Agent Treasury
DOC-ID: DAT-01
Status: Specified only / roadmap

This spec defines the control-plane contract for agents that search, bargain,
contract, and pay under delegated financial authority. A commercial agent is
not treated as an entity that simply "has money"; it is a delegated economic
actor operating inside a bounded mandate from a principal.

DAT-01 is specified only. It does not add a shipping schema table, change the
`/v1/chat/completions` API, require a wallet implementation, or require any
specific payment rail. Implementations may later map this contract onto cards,
bank rails, stablecoins, smart accounts, signed mandate protocols, or HTTP
payment protocols, but the Goni control-plane authority model is independent of
those adapters.
