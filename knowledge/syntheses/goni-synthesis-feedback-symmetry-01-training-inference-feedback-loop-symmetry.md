---
id: GONI-SYNTHESIS-FEEDBACK-SYMMETRY-01
title: "Training–Inference Feedback-Loop Symmetry"
type: synthesis
status: draft
implementation_state: not_applicable
proposition: "Training-time agent learning and inference-time agent execution share a feedback-loop structure in which actions produce feedback, while the update target differs: model weights during training and state or governed harness artifacts during execution."
domains:
- agent
- research
aliases: []
relations:
- type: synthesizes
  target: SYS-03
- type: synthesizes
  target: GONI-PRINCIPLE-HARNESS-EVOL-01
sources: []
artifacts: []
uncertainty: "This is a structural analogy, not a claim that training-time optimization and inference-time state updates are mathematically equivalent."
legacy: []
---

# Training–Inference Feedback-Loop Symmetry

At inference:

[
action_t ightarrow observation_t ightarrow state_{t+1}
]

During training:

[
action_t ightarrow feedback/reward_t ightarrow weights_{t+1}
]

The analogy matters because behaviors currently implemented in prompts, routing, retries, planning, or tool-selection scaffolds may later migrate into model weights through post-training.

Goni should keep authority, canonical state, evidence, and receipts outside that moving boundary.
