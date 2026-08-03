---
id: MODEL-REG-01
title: Model Bundle Registry Governance
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: MODEL-REG-01 Status: Specified only / roadmap Open-weight release decentralizes access to model parameters, but not necessarily governance over discovery, metadata, provenance, evaluation, licensing, deployment, or runtime permissions.'
domains:
- specs
aliases:
- MODEL-REGISTRY
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: Model Bundle Registry Governance
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# Model Bundle Registry Governance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Model Bundle Registry Governance
DOC-ID: MODEL-REG-01

Status: Specified only / roadmap

Open-weight release decentralizes access to model parameters, but not
necessarily governance over discovery, metadata, provenance, evaluation,
licensing, deployment, or runtime permissions. Goni may discover models through
public ecosystems, but approved execution flows through a governed bundle
registry. The runtime executes immutable bundle IDs whose provenance, license,
hashes, task permissions, assurance level, and evaluation receipts are known
before use. For personalized behavior, the execution unit may be a governed
model stack: base bundle plus approved adapters, prompt/policy bundle, and
memory or retrieval bundle refs.

Scientific framing:
- Observed fact: widely available model weights can broaden participation and
  enable local inference, while model openness still depends on documentation,
  code, data, licenses, and access structure. [[ntia2024-open-model-weights]]
- Theoretical inference: model hubs are governance infrastructure, not only file
  storage. They shape discovery, naming, metadata conventions, reputation,
  access restrictions, and takedown paths.
- Goni hypothesis: the unit of trust in open AI should shift from the hosted
  model repository to the locally attested model installation.
