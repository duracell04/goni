---
id: GONI-IMAP-DE08CFCDEEA4
title: 4.4 Deterministic inference profile
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Self-loop / agentic runs have positive Lyapunov exponents (small numeric noise can change token choice).
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 4.4 Deterministic inference profile
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 4.4 Deterministic inference profile

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.4 Deterministic inference profile

Self-loop / agentic runs have positive Lyapunov exponents (small numeric noise can change token choice). We therefore define a **deterministic profile** for engines:

> **Invariant E2 (Deterministic preset).**
> If a request is marked deterministic, the engine must execute with:
> - temperature 0 and fixed seed (if backend supports `seed`),
> - batch size 1 and no continuous/dynamic batching,
> - single worker / single thread (or CPU-only path) and TF32 disabled on NVIDIA,
> - fixed backend flags (e.g. vLLM `--enable-deterministic-inference`), and
> - recorded blueprint/hardware/driver hashes in the log.
>
> A compliant engine may fall back to a slower profile to satisfy E2, but must not silently drop the request.
