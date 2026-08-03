---
id: GONI-IMAP-5E1EB55F242D
title: 4.2 Encoders -> Predictor -> (Optional) Decoder (latent-first pipeline)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'This repo is model-agnostic, but the execution substrate supports a common pattern: **Encoders**: map one or more inputs into latent representations.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 4.2 Encoders -> Predictor -> (Optional) Decoder (latent-first pipeline)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 4.2 Encoders -> Predictor -> (Optional) Decoder (latent-first pipeline)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.2 Encoders -> Predictor -> (Optional) Decoder (latent-first pipeline)

This repo is model-agnostic, but the execution substrate supports a common pattern:

1) **Encoders**: map one or more inputs into latent representations.  
   - Examples: text encoder, vision encoder (screenshots/images), audio encoder, structured-data encoder.  
   - Output: latent vectors + lightweight structured features.

2) **Predictor (cognitive core)**: updates latent state and selects actions.  
   - Input: (a) current latent state, (b) new latent observations, (c) an optional query/goal.  
   - Output: updated latent state, tool calls, and optionally a latent "answer" representation.

3) **Optional Decoder (verbaliser / renderer)**: turns latent state into words or other outputs.  
   - Used for: explanations, drafts, chat UX, external communications.  
   - Not required for internal planning/tool use.

See `blueprint/software/30-components/latent-predictor.md` for the integration sketch and VL-JEPA-inspired block diagram.

#### Latent-space objective (conceptual)

Where a component is trained or fine-tuned, the preferred high-level objective is:

- learn an encoder representation `S(Â·)`,
- learn a predictor `P(Â·)` such that `P(S(context), S(observation), q) â‰ˆ S(target)`,
- compare predicted vs target latent representations with a similarity loss (e.g., cosine / contrastive).

This makes "meaning" the primary internal currency, while tokens remain an interface.

#### Why this fits Goni's infra stance

- Compatible with queues/planes: encoders emit events; predictor consumes events; decoder is a late-stage consumer.  
- Compatible with local-first: always-on encoders + predictor can be small; decoder can be on-demand.  
- Compatible with multi-model arbitration: different encoders/decoders can share the same latent state contract.
