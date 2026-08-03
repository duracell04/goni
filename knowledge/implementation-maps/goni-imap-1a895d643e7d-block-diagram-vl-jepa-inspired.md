---
id: GONI-IMAP-1A895D643E7D
title: Block diagram (VL-JEPA inspired)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Inputs: Visual input X_v (image/screen/video) Text query X_q (user intent, task goal) Text target Y (optional during training; e.g., answer/draft) Modules: E_x: X-Encoder maps X_v -> S_v E_q: Query encoder maps X_q -> S_q (or reuse text encoder) P: Predictor maps (S_v, S_q, S_state) -> S_y_hat + S_state'' + actions E_y: Y-Encoder maps Y -> S_y (training / supervision only)'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Block diagram (VL-JEPA inspired)
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Block diagram (VL-JEPA inspired)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Block diagram (VL-JEPA inspired)

Inputs:
- Visual input `X_v` (image/screen/video)
- Text query `X_q` (user intent, task goal)
- Text target `Y` (optional during training; e.g., answer/draft)

Modules:
- `E_x`: X-Encoder maps `X_v -> S_v`
- `E_q`: Query encoder maps `X_q -> S_q` (or reuse text encoder)
- `P`: Predictor maps `(S_v, S_q, S_state) -> S_y_hat + S_state' + actions`
- `E_y`: Y-Encoder maps `Y -> S_y` (training / supervision only)
- `D_y`: Y-Decoder maps `S_y_hat -> text` (optional runtime)

Loss (training-time conceptual):
- compare `S_y_hat` vs `S_y` in latent space (similarity / contrastive loss)

Runtime:
- `E_y` and explicit loss are absent; only encoders + predictor + optional decoder run.
