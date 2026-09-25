---
id: AGENT-DEF-01
title: AGENT-01 - Agent Definition (Goni OS)
type: specification
status: draft
implementation_state: specified_only
proposition: "An agent is a persistent governed userland process defined by identity, state, memory, mandate, policy, tools, budgets, and workflow; model or mechanism selection is a replaceable cognitive dependency and does not define agent identity."
domains:
- agent
- specs
aliases:
- AGENT-DEFINITION
relations: []
sources: []
artifacts: []
uncertainty: "This contract defines identity and responsibility boundaries, not a runtime process implementation. Concrete lifecycle and scheduler semantics remain governed elsewhere."
legacy:
- path: blueprint/30-specs/agent-definition.md
  heading: AGENT-01 - Agent Definition (Goni OS)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# AGENT-01 - Agent Definition (Goni OS)

An agent is a governed userland process bound to a principal-defined purpose and
kernel-mediated authority.

Its persistent identity is determined by properties such as:

- agent identity and role;
- Work Order or delegated objective;
- state and memory references;
- mandate and policy scope;
- tool and capability eligibility;
- budgets and deadlines;
- workflow or cognitive-graph state;
- provenance and receipts.

A model is not the agent. Models, classifiers, decision models, solvers,
retrievers, verifiers, and other cognitive mechanisms are replaceable resources
that the agent may invoke through the harness and router.

Changing the selected model or mechanism does not, by itself, change the
agent's identity, authority, memory ownership, mandate, or accountability.

Agents are userland processes; cognition is a governed substrate; expensive
solver or model calls remain budgeted operations.
