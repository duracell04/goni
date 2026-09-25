---
id: MODEL-REG-01
title: Model Bundle Registry Governance
type: specification
status: draft
implementation_state: specified_only
proposition: "Approved model execution flows through a governed bundle registry whose immutable bundle records include provenance, licensing, hashes, permitted task classes, cognitive roles, assurance evidence, deployment constraints, and evaluation receipts."
domains:
- models
- specs
aliases:
- MODEL-REGISTRY
relations: []
sources: []
artifacts: []
uncertainty: "The registry is specified only. Role labels and evaluation profiles do not guarantee performance outside their measured domain."
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: Model Bundle Registry Governance
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# Model Bundle Registry Governance

Approved execution flows through a governed bundle registry. The runtime uses
immutable bundle identifiers whose relevant properties are known before use.

A promoted bundle should record or reference:

- model/checkpoint and artifact hashes;
- runtime compatibility;
- license and redistribution constraints;
- provider or local deployment identity;
- permitted task classes;
- supported cognitive roles under MODEL-ROLE-01;
- evaluation artifacts for each promoted role;
- latency, cost, memory, energy, and egress profiles where measured;
- calibration evidence where probabilities influence control;
- privacy and data-residency constraints;
- active adapters, prompt/policy bundles, retrieval bundles, and other seams;
- assurance level and promotion receipts.

For personalized behavior, the execution unit may be a governed model stack
consisting of a base bundle plus approved adapters, prompt/policy bundle, and
memory or retrieval references.

Role labels are routing metadata, not authority. Loading a stronger model or a
model with more cognitive roles cannot expand the current Work Order,
capability set, or approval corridor.

The unit of trust is the attested, evidence-linked model installation or remote
provider configuration, not an informal model name.
