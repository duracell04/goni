---
id: GONI-SYNTHESIS-CC4921DED1BB
title: 1. Threat model (high-level)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Adversarial or unreliable inputs include: prompt-injected retrieved text, compromised or malicious tool extensions, accidental over-privileged connectors, model outputs that attempt policy bypass.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 1. Threat model (high-level)
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 1. Threat model (high-level)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Threat model (high-level)

Adversarial or unreliable inputs include:
- prompt-injected retrieved text,
- compromised or malicious tool extensions,
- accidental over-privileged connectors,
- model outputs that attempt policy bypass.

Primary risk classes:
- unauthorized side effects,
- silent data exfiltration,
- untraceable actions,
- degraded responsiveness from memory or scheduler contention.
