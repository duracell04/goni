---
id: GONI-SPEC-D268994A7CBE
title: 4. Effect classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'Effects requiring kernel mediation: external network calls, filesystem writes and deletes, connector side effects (email send, calendar mutation, payments), mutable state commits in local durable stores, externally visible job/control-plane actions.'
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 4. Effect classes
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 4. Effect classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Effect classes

Effects requiring kernel mediation:
- external network calls,
- filesystem writes and deletes,
- connector side effects (email send, calendar mutation, payments),
- mutable state commits in local durable stores,
- externally visible job/control-plane actions.

Read-only in-memory transformations without external side effects may bypass
transaction commit, but not capability checks when protected resources are read.
