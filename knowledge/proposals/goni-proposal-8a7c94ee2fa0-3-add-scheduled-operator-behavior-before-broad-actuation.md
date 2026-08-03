---
id: GONI-PROPOSAL-8A7C94EE2FA0
title: 3. Add scheduled operator behavior before broad actuation
type: proposal
status: draft
implementation_state: specified_only
proposition: Start with Daily Brief, open-loop detection, memory consolidation, and anomaly/audit summaries scheduled through the Control Plane.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: 3. Add scheduled operator behavior before broad actuation
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# 3. Add scheduled operator behavior before broad actuation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3. Add scheduled operator behavior before broad actuation

- Start with Daily Brief, open-loop detection, memory consolidation, and
  anomaly/audit summaries scheduled through the Control Plane.
- Add local files and read-only calendar/email ingestion before draft creation
  and external side effects.
- Introduce file writes, calendar changes, email sending, browser actions, and
  desktop control only through the canonical tool, scheduler, and receipt
  contracts.
- Keep credentials in an OS or encrypted secret store and substitute them only
  at the execution boundary; never persist them in prompts or semantic memory.
