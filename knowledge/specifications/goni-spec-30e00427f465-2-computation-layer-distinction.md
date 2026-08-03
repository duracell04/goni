---
id: GONI-SPEC-30E00427F465
title: 2. Computation layer distinction
type: specification
status: draft
implementation_state: specified_only
proposition: SS-01 uses "symbolic" narrowly.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 2. Computation layer distinction
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 2. Computation layer distinction

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Computation layer distinction

SS-01 uses "symbolic" narrowly. It does not claim that AI, classical
computing, and quantum computing are competing logics of the same kind. They
operate at different explanatory layers:

- Classical computation is deterministic symbolic state transition.
- AI computation is statistical inference and representation learning running
  on classical hardware.
- Quantum computation is physical state evolution and measurement over quantum
  states.

For Goni, this distinction fixes the authority boundary. LLMs and other AI
systems may infer, rank, summarize, draft, classify, or propose state changes.
Those outputs remain advisory until SS-01 validates them against policy, facts,
schemas, constraints, and capabilities. Quantum computation is a distinct
physical substrate, not a stronger form of AI or a bypass around symbolic
validation.

| Dimension | Classical computing | AI computation | Quantum computation |
| --- | --- | --- | --- |
| Core object | Bit or symbolic state | Vector, parameter, or distribution | Qubit or quantum state |
| Main operation | Deterministic state transition | Statistical inference | Unitary state evolution and measurement |
| Probability type | Optional simulated randomness | Epistemic or model uncertainty | Physical measurement probability |
| Mathematical base | Logic, algorithms, automata | Statistics, optimization, linear algebra | Hilbert spaces, amplitudes, operators |
| Main output | Exact computation | Prediction, generation, or classification | Sampled classical result after measurement |
| Failure mode | Bug, wrong algorithm, wrong input | Hallucination, overfitting, bias, drift | Decoherence, noise, error accumulation |
| Hardware today | CPU, GPU, ASIC | GPU, TPU, NPU, accelerators | Quantum processors plus classical control |

Goni's runtime stack is therefore layered: classical systems control execution,
AI systems infer patterns and propose artifacts, and any future quantum system
would only accelerate specific mathematical subproblems where its physical
structure is useful. None of those layers replaces the deterministic
arbitration contract defined here.
