---
id: GONI-SYNTHESIS-7CD00D3DE959
title: 2. Tokenization realism is a first-class criterion
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Tokenizer regime changes architecture behavior: character-level setups inflate sequence length and alter optimization dynamics, modern LLM quality baselines are built on subword tokenization.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: 2. Tokenization realism is a first-class criterion
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# 2. Tokenization realism is a first-class criterion

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Tokenization realism is a first-class criterion

Tokenizer regime changes architecture behavior:
- character-level setups inflate sequence length and alter optimization
  dynamics,
- modern LLM quality baselines are built on subword tokenization.

Evaluation implication:
- report quality under realistic tokenization (BPE/SentencePiece-like),
- treat large regressions under subword settings as non-competitive until
  resolved.

Related work:
- RWKV large-scale LM evaluations [[peng2023-rwkv]]
- Hyena LM evaluations with mainstream corpora [[poli2023-hyena]].
