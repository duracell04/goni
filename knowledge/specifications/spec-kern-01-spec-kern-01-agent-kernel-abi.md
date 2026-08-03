---
id: SPEC-KERN-01
title: SPEC-KERN-01 - Agent Kernel ABI
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: SPEC-KERN-01 Status: Specified only / roadmap This spec defines the mandatory kernel boundary for all effectful agent operations.'
domains:
- agent
- kernel
- specs
aliases:
- SPEC-KERN-01-AGENT-KERNEL-ABI
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: SPEC-KERN-01 - Agent Kernel ABI
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# SPEC-KERN-01 - Agent Kernel ABI

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# SPEC-KERN-01 - Agent Kernel ABI
DOC-ID: SPEC-KERN-01
Status: Specified only / roadmap

This spec defines the mandatory kernel boundary for all effectful agent
operations. It is the reference-monitor choke point for tool actions, network
egress, and state mutation proposals.

External runtimes, gateways, or assistant frameworks may consume this ABI, but
they do not replace it. A third-party session model or tool registry is not a
valid substitute for the Goni kernel boundary.
