---
id: GONI-SYNTHESIS-F264F7041069
title: 2.1b Correction Delta Compiler
type: synthesis
status: draft
implementation_state: specified_only
proposition: The Correction Delta Compiler (CDC) is Goni's P0/P1 path for learning the principal's delegation preferences from corrections.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 2.1b Correction Delta Compiler
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 2.1b Correction Delta Compiler

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1b Correction Delta Compiler
The Correction Delta Compiler (CDC) is Goni's P0/P1 path for learning the
principal's delegation preferences from corrections. It is an online
preference-estimation subsystem, not ordinary memory storage and not online
base-model training.

CDC treats an interaction as:

```text
tau_t = (x_t, y_t_ai, y_t_user, a_t, o_t)
```

and derives a correction delta:

```text
Delta_t = aligned_edit(y_t_ai, y_t_user)
```

The delta captures factual corrections, tone changes, structure changes,
source/evidence additions, privacy edits, shortened length, stronger framing,
softer tone, changed task scope, and accept/reject/send/ignore actions. The
system-identification target is the principal's latent preference state:

```text
p(theta_u | Delta_1:t, a_1:t, o_1:t, M_1:t)
```

CDC follows the governed path:

```text
interaction stream
-> draft/final alignment
-> correction delta extraction
-> classification
-> candidate preference rule
-> validation
-> updater
-> MemoryEntry + Receipt + RegressionTest
```

Fast CDC learning updates memory, retrieval, prompt assembly, and harness
policy through declared seams. Slow CDC learning may produce adapter, LoRA, or
DPO-style preference datasets only after enough evidence, replay evaluation,
and promotion review. Core policy and constitutional defaults require explicit
approval and slow promotion.

Worldview, tone, and ideology-shaped adapters require extra caution. Their goal
is to help the system understand and respect the principal's preferred framing,
not to collapse critique into agreement. Promotion evidence MUST check for
overfitting, confirmation bias, degraded general reasoning, refusal drift, and
loss of useful adversarial critique.

CDC MUST guard against overfitting. A single correction creates a scoped
hypothesis; repeated corrections create a preference candidate; an accepted
learning card creates a policy or memory candidate with receipt; stable defaults
require high-confidence repetition and replay coverage.

See [Correction Delta Compiler](/blueprint/30-specs/correction-delta-compiler.md)
for the normative contract.
