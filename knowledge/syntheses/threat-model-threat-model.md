---
id: THREAT-MODEL
title: THREAT MODEL
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Status: specified only / roadmap Adversaries: malicious prompt or retrieved text compromised tool container compromised model provider operator error Assumptions: kernel is trusted for mediation tools and external text are untrusted covert channels are out of scope for v0'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/threat-model.md
  heading: THREAT MODEL
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# THREAT MODEL

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# THREAT MODEL

Status: specified only / roadmap

Adversaries:
- malicious prompt or retrieved text
- compromised tool container
- compromised model provider
- operator error

Assumptions:
- kernel is trusted for mediation
- tools and external text are untrusted
- covert channels are out of scope for v0
