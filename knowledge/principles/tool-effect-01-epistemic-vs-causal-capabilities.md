---
id: TOOL-EFFECT-01
title: Separate epistemic access from causal capability
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should distinguish capabilities that primarily change what an agent can know from capabilities that can change external state because their authority, risk, rollback, and verification requirements differ.
domains:
- system
- tools
- authority
aliases:
- epistemic versus causal tools
relations:
- type: refines
  target: GONI-THESIS-F782C616C259
sources:
- SRC-YAO2023-REACT
- SRC-SCHICK2023-TOOLFORMER
artifacts: []
uncertainty: Some tools both reveal and change state; classification should follow the effectful capability actually requested rather than a permanent label on the integration.
legacy: []
---

# Separate epistemic access from causal capability

Goni distinguishes two capability families by their primary effect.

**Epistemic capabilities** primarily expand what the system can observe or retrieve: search, document retrieval, metadata reads, database reads, status queries, screenshots, and other observation operations.

**Causal capabilities** can change the external environment: file writes, database mutations, message sending, purchases, publishing, code execution with side effects, account changes, and device control.

The distinction is risk-relevant rather than cosmetic. Epistemic operations still require privacy, provenance, and prompt-injection controls, but causal operations additionally require effect authorization, postcondition verification, rollback or compensation analysis where feasible, and stronger receipt requirements.

A single integration may expose both classes. Goni therefore classifies the requested operation or capability, not merely the brand or protocol through which it is reached.
