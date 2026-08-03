---
id: GONI-SYNTHESIS-8577DD9A5AD1
title: 1. Core posture
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni defaults to open-weight, offline-capable, account-free inference.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/65-local-sovereign-knowledge-runtime.md
  heading: 1. Core posture
  revision: 43414875152ae18f9977f21c9786b2d7025081ac
---

# 1. Core posture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Core posture

Goni defaults to open-weight, offline-capable, account-free inference. The
owner controls the model bundle, keys, prompts, policy, memory, and data.
Mandatory remote moderation, entitlement checks, hidden telemetry,
provider-controlled policy prompts, or viewpoint filters that the owner cannot
remove are hostile dependencies, not sovereignty features.

Promoted model bundles SHOULD be hash-pinned and reproducible. Engine and
checkpoint licenses remain separate evidence and SHOULD be recorded
independently under MODEL-REG-01 rather than inferred from one another.

Local text is local expression. Generating, analyzing, criticizing, imagining,
or drafting controversial, heterodox, offensive, or politically sensitive
material does not itself move money, contact another person, publish content,
delete evidence, or control a machine. Goni does not treat private model output
as an effectful tool action merely because somebody dislikes the content.

"No filter" is a stack property: no mandatory external moderation or
application-layer suppression sits between the principal and local output. It
is not a false promise that every checkpoint will comply perfectly. Learned
weights may still hedge, refuse, omit, or moralize; model selection, prompting,
templates, adaptation, and evaluation remain owner-controlled remedies.

A deployment is sovereign when inference and canonical authority remain local
and the owner controls its keys, weights, policy, and data. Staffing, backup,
liability, and operational maturity affect reliability, but they are not
ideological admission tests for sovereignty.
