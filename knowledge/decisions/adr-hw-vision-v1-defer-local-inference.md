---
id: ADR-HW-VISION-V1-DEFER-LOCAL-INFERENCE
title: Defer Dedicated Local Vision Inference Hardware
type: decision
status: draft
implementation_state: specified_only
proposition: "During V1, Goni defers dedicated local multimodal inference hardware and validates the desktop-vision product loop with low-cost cloud vision first; Framework Desktop / Ryzen AI Max-class hardware remains a later candidate for sovereignty and offline inference, not a current procurement commitment."
domains:
- hardware
- vision
- desktop-agent
aliases:
- defer-local-vlm-hardware
- framework-later-candidate
relations:
- type: depends_on
  target: ADR-VISION-V1-ROUTING
- type: refines
  target: VIS-01
sources: []
artifacts: []
uncertainty: "No local hardware platform has been selected or benchmarked for this workload. Framework is retained as a candidate class, not an approved build."
---

# Defer Dedicated Local Vision Inference Hardware

## Decision

Do not buy or standardize dedicated local multimodal inference hardware for the V1 desktop-vision prototype.

The immediate objective is to validate the interaction loop:

```text
screen context
-> fast visual explanation
-> conversational guidance
-> canvas / memory integration
-> later safe action
```

At this stage, speed of iteration and low total cost matter more than maximizing local inference sovereignty.

## Current execution posture

Use the existing developer machine for local perception:

- screen capture,
- frame differencing,
- OCR,
- Windows UI Automation,
- SQLite state,
- orchestration.

Use the current low-cost remote vision provider for on-demand multimodal reasoning.

This avoids making hardware procurement a prerequisite for proving the product.

## Framework status

Framework Desktop and similar high-unified-memory desktop systems remain candidates for a later local-inference phase.

Framework is not selected hardware and this decision makes no claim that it will meet Goni's future latency, thermal, memory-bandwidth, driver, or model-compatibility requirements.

Any future adoption requires a hardware benchmark and a separate decision update.

Reference candidate:

- https://frame.work/desktop

## Why defer

A dedicated local VLM box creates immediate costs and constraints:

- capital expenditure before product validation,
- hardware-specific optimization work,
- model/runtime compatibility constraints,
- thermal and power design questions,
- local inference latency that may still be worse than remote low-latency inference.

The V1 should first establish measured usage patterns:

- visual questions per day,
- typical screenshot/crop sizes,
- acceptable response latency,
- model quality requirements,
- cloud spend,
- privacy/offline demand.

Those measurements determine whether local inference creates positive marginal value.

## Revisit triggers

Open a hardware evaluation when one or more of these conditions becomes material:

1. cloud vision spend approaches the amortized cost of a local inference box;
2. cloud latency becomes the dominant UX bottleneck;
3. privacy or offline operation becomes a product requirement;
4. session volume is high enough that local marginal cost is attractive;
5. a candidate platform demonstrates acceptable latency and quality on Goni's actual screen-understanding benchmark.

## Required evidence before adoption

Any future local-inference hardware decision should record:

- exact tested hardware,
- test date,
- model/runtime and quantization,
- screenshot-to-first-token latency,
- total response latency,
- sustained throughput,
- RAM/VRAM/unified-memory use,
- power draw where measurable,
- thermal behavior during sustained sessions,
- quality comparison against the current remote baseline,
- estimated break-even versus measured cloud spend.

## Consequence

The hardware path remains open without slowing the current software experiment.

The current sequence is:

```text
prove the UX with cheap remote vision
-> measure real workload
-> benchmark local hardware
-> localize only when the economics or constraints justify it
```

This is a timing decision, not a rejection of sovereign local inference.
