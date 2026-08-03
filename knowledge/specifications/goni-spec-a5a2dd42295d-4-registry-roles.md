---
id: GONI-SPEC-A5A2DD42295D
title: 4. Registry roles
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni distinguishes three roles: Public discovery: broad ecosystem search and metadata lookup.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 4. Registry roles
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 4. Registry roles

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Registry roles

Goni distinguishes three roles:

- Public discovery: broad ecosystem search and metadata lookup.
- Private registry: local or self-hosted cache of approved bundles.
- Runtime loader: engine-specific loading from approved bundle IDs only.

Public discovery may include Hugging Face or ModelScope. Private registry
candidates may include self-hosted registries such as MatrixHub or KohakuHub
when they satisfy Goni policy, storage, and audit requirements. Runtime loading
may use engines such as Ollama, llama.cpp, vLLM, or SGLang.

Public hubs can decentralize access while centralizing epistemic mediation. In
Goni, the hub is a discovery input; the local bundle registry is the execution
authority.
