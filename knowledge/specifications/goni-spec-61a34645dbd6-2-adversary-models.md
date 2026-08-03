---
id: GONI-SPEC-61A34645DBD6
title: 2. Adversary models
type: specification
status: draft
implementation_state: specified_only
proposition: 'This spec uses the following threat categories: Local attacker: untrusted local process/container attempting to exfiltrate data or bypass policy.'
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
  heading: 2. Adversary models
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 2. Adversary models

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Adversary models

This spec uses the following threat categories:

- Local attacker: untrusted local process/container attempting to exfiltrate
  data or bypass policy.
- Network observer: ISP or on-path observer correlating traffic and destinations.
- Provider adversary: cloud provider or API endpoint that sees request content.
- Global passive adversary: state-level observer with broad traffic visibility.

Out of scope by default: active hardware compromise, physical tamper, or a
global active adversary that can modify traffic without detection.
