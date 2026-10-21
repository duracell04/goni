---
id: INF-PLACE-01
title: Memory-Hierarchy-Aware Inference Placement
type: specification
status: draft
implementation_state: specified_only
proposition: "Goni selects an inference execution profile from policy-authorized and resource-feasible model/runtime/placement combinations using calibrated workload estimates and job budgets, while tensor movement remains a backend concern."
domains:
- hardware
- kernel
- software
aliases: []
relations:
- type: refines
  target: GONI-PRINCIPLE-9062425CD490
- type: depends_on
  target: GONI-SPEC-78027C2E0EED
- type: depends_on
  target: GONI-DECISION-0532C27DA3CC
- type: depends_on
  target: MODEL-REG-01
- type: derived_from
  target: EVID-AIRLLM-STREAMING
- type: derived_from
  target: EVID-SHENG2023-FLEXGEN
sources:
- SRC-AIRLLM-REPO
- SRC-SHENG2023-FLEXGEN
artifacts: []
uncertainty: "Backend support, memory estimates, throughput, energy, and SSD effects are hardware- and workload-dependent. This contract is specified only; eligibility for production requires independent benchmarks."
legacy: []
---

# Memory-Hierarchy-Aware Inference Placement (INF-PLACE-01)

## Scope and authority

A Goni model job carries an authorized model bundle, required capabilities,
quality requirements, QoS class, deadline, and resource budget. The kernel
selects a feasible execution profile and retains D-008 scheduling, admission,
budget, cancellation, egress, and receipt authority. Runtime adapters implement
the selected profile and manage their internal tensor transfers.

Placement and numerical precision are orthogonal. The selected precision or
quantization variant is declared in the governed model/runtime configuration.
The model bundle registry remains the authority for approved artifacts and
their integrity. Network-based execution follows existing egress policy.

## Placement modes

- `resident`: selected model weights and working state remain in the
  selected high-bandwidth memory tier for the execution.
- `offloaded`: selected weights or state execute from a managed mixture of
  accelerator and host memory.
- `layer_streamed`: dense-model layers are staged sequentially through
  limited accelerator memory during execution.
- `expert_streamed`: a compatible sparse-MoE backend stages the experts
  selected by model routing while respecting shared-layer and cache needs.

A backend MAY combine techniques. Each declared mode is available only for
model architecture, hardware, precision, and runtime versions the backend
reports as supported.

## Logical execution plan

```yaml
job_id: ...
model_bundle_id: ...
runtime_backend_id: ...
runtime_version: ...
placement_mode: resident | offloaded | layer_streamed | expert_streamed
precision_profile: ...
estimate_profile_ref: ...
estimated:
  peak_accelerator_memory_bytes: ...
  peak_system_memory_bytes: ...
  working_kv_activation_bytes: ...
  model_storage_bytes: ...
  storage_read_bytes: ...
  storage_write_bytes: ...
  wall_time_ms: ...
  energy_joules: ...
```

Resource estimates include context/KV-cache and activation demands as well as
weights, plus model preparation and cold-start costs when applicable. Unknown
values carry an explicit unmeasured status. Runtime profiles describe the
hardware/software version, batch and context conditions, measurement provenance,
and uncertainty range.

## Admission and selection

1. Compile authorized model bundles, permitted precision profiles, local or
   explicitly approved remote backends, and supported placement modes.
2. Check hardware capability, required storage provisioning, memory headroom,
   concurrency, job deadline, quality requirements, and enforceable budgets.
3. Compare eligible profiles using measured or conservatively estimated
   end-to-end latency, transfer cost, energy, storage wear, and expected quality.
4. Admit the best eligible profile under the job's priorities and D-008 queue
   discipline; defer, fail safely, or use an existing approval/egress route
   when the eligible set is empty.

The control plane reserves scarce resources, observes runtime pressure, and
supports cancellation and backpressure. The runtime performs fine-grained
layer/expert loading and prefetching within the selected envelope. Warm-state
reuse may improve performance; the initial admission estimate accounts for
the possibility of a cold start.

## Metering and receipts

Use existing METER-01 and execution-receipt contracts to link the job,
authorized bundle and runtime version, selected placement mode, estimate
profile, observed peak accelerator/system memory, wall time, storage reads and
writes, energy when measured, and any budget or fallback decision. Report
estimated values separately from measurements and from output-quality
evaluation.

## Conformance and promotion

A qualifying adapter demonstrates supported mode/model combinations,
resource-accounted admission, cooperative cancellation and backpressure,
reconstructable receipts, and its declared storage/network boundary. Measured
workload-specific performance establishes production eligibility. A successful
small-VRAM inference demonstration alone establishes feasibility, not
interactive suitability or overall resource efficiency.
