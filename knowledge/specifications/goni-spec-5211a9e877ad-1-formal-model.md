---
id: GONI-SPEC-5211A9E877AD
title: 1. Formal model
type: specification
status: draft
implementation_state: specified_only
proposition: 'A Goni interaction is modeled as a trajectory: Where: x_t is the task context.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 1. Formal model
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 1. Formal model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Formal model

A Goni interaction is modeled as a trajectory:

```text
tau_t = (x_t, y_t_ai, y_t_user, a_t, o_t)
```

Where:

- `x_t` is the task context.
- `y_t_ai` is the agent draft.
- `y_t_user` is the corrected or final user-approved version.
- `a_t` is the user action: accept, edit, reject, send, ignore.
- `o_t` is the downstream outcome, if observable.

The useful signal is the correction delta:

```text
Delta_t = aligned_edit(y_t_ai, y_t_user)
```

This is not literal text subtraction. It is an aligned edit operation covering
deletions, insertions, tone changes, structure changes, factual corrections,
added sources, removed hedges, changed asks, shortened length, stronger
framing, softer tone, and privacy or safety edits.

The learning target is a latent preference state for the principal:

```text
p(theta_u | Delta_1:t, a_1:t, o_1:t, M_1:t)
```

Future outputs should maximize expected utility under the active preference,
memory, and policy state:

```text
y_star = argmax_y E[U_u(y, x) | theta_u, M_u, P]
```

In systems terms, CDC is online system identification for the principal's
delegation preferences. Corrections are preference gradients; CDC compiles them
into scoped, receipted procedural memory without silently drifting the system.
