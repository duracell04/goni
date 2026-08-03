---
id: GONI-SPEC-44CC8B8A6B1B
title: 2. Scope
type: specification
status: draft
implementation_state: specified_only
proposition: 'This spec applies to broad embodied robot systems, including: humanoid robots, mobile manipulators, home and eldercare assistance robots, service, retail, reception, and guide robots, warehouse and logistics robots, industrial pilots and robot workcells, robot fleets that combine autonomous execution with remote supervision,'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 2. Scope
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 2. Scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Scope

This spec applies to broad embodied robot systems, including:

- humanoid robots,
- mobile manipulators,
- home and eldercare assistance robots,
- service, retail, reception, and guide robots,
- warehouse and logistics robots,
- industrial pilots and robot workcells,
- robot fleets that combine autonomous execution with remote supervision,
- vendor SDKs, ROS-style middleware, and robot-cloud adapters exposed to Goni.

This spec covers robot observation, extraction, memory, actuation, cloud use,
telemetry, remote operator access, and physical safety mediation. It does not
replace specialized standards for industrial safety, medical devices,
functional safety, machinery directives, workplace rules, building access,
labor law, privacy law, or insurance.

If a robot action purchases, negotiates, contracts, reserves funds, or pays, it
also falls under [DAT-01](/blueprint/30-specs/delegated-agent-treasury.md).
