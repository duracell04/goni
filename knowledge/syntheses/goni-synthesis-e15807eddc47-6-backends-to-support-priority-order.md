---
id: GONI-SYNTHESIS-E15807EDDC47
title: 6. Backends to support (priority order)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Local**: vLLM/TGI seats; optional Ollama/LM Studio for hobby setups.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-lab.md
  heading: 6. Backends to support (priority order)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 6. Backends to support (priority order)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Backends to support (priority order)
- **Local**: vLLM/TGI seats; optional Ollama/LM Studio for hobby setups.
- **Cloud (via Council)**: seats already in `goni-prototype-lab:config/council.yaml` (OpenRouter IDs or direct providers).
- **Web-grounded**: Perplexity Sonar / Grok only when task tag demands live info; log separately.
- **Future**: Airtrain-style dataset evals for repeatable benchmarks; slot in when we have labelled task sets.
