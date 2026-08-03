---
id: GONI-IMAP-4171C0CCCC70
title: 2.2 Compatibility with OpenAI
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Let: \(\mathsf{Req}_{\text{OA}}\) = set of valid OpenAI chat/completions requests (for chat models), \(\mathsf{Req}_{\text{Goni}}\) = set of valid Goni requests.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 2.2 Compatibility with OpenAI
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 2.2 Compatibility with OpenAI

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Compatibility with OpenAI

Let:

* \(\mathsf{Req}_{\text{OA}}\) = set of valid OpenAI chat/completions requests (for chat models),
* \(\mathsf{Req}_{\text{Goni}}\) = set of valid Goni requests.

For supported model names, we require:

* \(\mathsf{Req}_{\text{OA}} \subseteq \mathsf{Req}_{\text{Goni}}\),
* Fields shared with OpenAI semantics retain those semantics.

Goni-specific extensions are prefixed with goni_ and must be ignored by generic OpenAI clients.

---
