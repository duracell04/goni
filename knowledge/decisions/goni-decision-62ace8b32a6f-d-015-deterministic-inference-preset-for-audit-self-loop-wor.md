---
id: GONI-DECISION-62ACE8B32A6F
title: D-015 - Deterministic inference preset for audit/self-loop workloads
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** The Execution plane exposes a deterministic preset.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-015 - Deterministic inference preset for audit/self-loop workloads
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-015 - Deterministic inference preset for audit/self-loop workloads

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-015 - Deterministic inference preset for audit/self-loop workloads

**Formal statement**

The Execution plane exposes a deterministic preset. For any request marked deterministic:

- temperature = 0 and fixed seed (if supported by backend),  
- batch size = 1 with no continuous/dynamic batching,  
- single worker/thread (or CPU-only fallback) and TF32 disabled on NVIDIA,  
- deterministic backend flag enabled (e.g. vLLM `--enable-deterministic-inference`),  
- blueprint/hardware/driver hashes recorded with the run.

**Rationale**

- Self-loop/agent chains have positive Lyapunov exponents; tiny numeric noise can flip tokens and diverge trajectories.  
- Regulated or audited runs must be reproducible even at the cost of throughput.

**Consequence**

- Engines must provide a slower deterministic profile rather than silently ignoring the request.  
- CI includes a self-loop drift check (bitwise-stable tokens across two runs) under the deterministic preset.  
- Fast defaults may use batched/GPU paths, but the audit preset remains available and documented.

---
