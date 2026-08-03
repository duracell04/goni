---
id: GONI-SYNTHESIS-B51284675A2E
title: C) Agent gateways and operator shells
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'OpenClaw: https://openclaw.ai/ Open Interpreter: https://github.com/openinterpreter/open-interpreter These are integration-heavy and "do things" well, but carry higher risk without capability-gated syscalls and audit trails.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: C) Agent gateways and operator shells
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# C) Agent gateways and operator shells

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### C) Agent gateways and operator shells

- OpenClaw: https://openclaw.ai/
- Open Interpreter: https://github.com/openinterpreter/open-interpreter

These are integration-heavy and "do things" well, but carry higher risk without
capability-gated syscalls and audit trails.

For Goni, these should be modeled as optional seats or adapters behind kernel
mediation, not as the control plane itself.
