---
id: GONI-IMAP-EB2133E6965D
title: 3.4 Frugal sovereign model router
type: implementation-map
status: draft
implementation_state: specified_only
proposition: "Goni's router is local-first and sovereignty-aware; it allocates among heterogeneous cognitive mechanisms using calibrated evidence, selective acceptance, consequence-sensitive loss, and resource/privacy constraints while leaving authority decisions to the kernel."
domains:
- software
aliases: []
relations:
- type: depends_on
  target: CAL-01
- type: depends_on
  target: SELECTIVE-01
- type: depends_on
  target: LOSS-01
- type: refines
  target: GONI-PRINCIPLE-HET-INTEL-01
sources:
- SRC-FRUGALGPT2023
- SRC-ROUTELLM2024
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "The concrete route set, estimators, thresholds, and utility terms remain empirical and workload-specific. This node specifies an allocation shape rather than verified runtime behavior."
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.4 Frugal sovereign model router
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.4 Frugal sovereign model router

Goni's router is local-first and sovereignty-aware. Its objective is broader
than model cost and answer quality: route choice may also depend on privacy
leakage risk, latency, energy/thermal budget, audit burden, data locality,
external dependency cost, consequence class, and the active approval corridor.

The operating rule is to use the smallest sufficient computation whose evidence
supports the required operating point, then escalate when uncertainty,
consequence, freshness, capability, or verification requirements justify the
additional cost.

A default mechanism ordering may resemble:

rule/cache/memory
-> deterministic checker or retrieval
-> task-specific classifier or bounded decision model
-> local small generative model
-> local stronger reasoning model
-> local multi-model verification
-> policy-permitted remote model/council
-> principal review

This ordering is a policy template, not a claim that every task traverses every
stage or that the same order is optimal universally.

For a candidate route result r on input x, confidence may be used only when the
relevant calibration evidence is in scope under CAL-01. SELECTIVE-01 determines
whether the cognitive result is accepted or abstained from at the configured
risk-coverage operating point. LOSS-01 determines how the cost of error,
escalation, delay, and other consequences affects the control decision.

Remote escalation remains subject to network, privacy, and approval policy. Raw
private or sensitive context is not sent merely because a remote route has
higher expected model quality.

Each consequential routing decision SHOULD link its receipt to:

- mechanisms considered;
- mechanism selected;
- calibration/confidence evidence where used;
- acceptance or abstention rule;
- consequence class and applicable loss/control policy;
- redaction and privacy state;
- model or tool identities;
- resulting policy decision.

The router may still be modeled as a contextual bandit or related online
decision problem for suitable workloads. Regret is one evaluation axis among
several. Numeric regret targets are experiment-specific and require an explicit
loss definition, oracle, workload, and evaluation protocol.

Routing evidence can influence cognition and escalation. It cannot itself
expand mandates, capabilities, or authority.
