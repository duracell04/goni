---
id: GONI-EXPERIMENT-OMNIPARSER-V1
title: OmniParser V1 Feasibility Spike
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Before OmniParser becomes part of the default desktop-perception runtime, measure its installation friction, latency, resource use, and UI-element grounding quality against Windows UI Automation on the actual target machine."
domains:
- experiments
- vision
- desktop-agent
aliases:
- omniparser-spike
- ui-parser-benchmark
relations:
- type: tests
  target: ADR-VISION-V1-ROUTING
- type: depends_on
  target: VIS-01
sources: []
artifacts: []
uncertainty: "No conclusion is implied before the spike is run on the actual target machine. Windows UI Automation remains the V1 baseline."
---

# OmniParser V1 Feasibility Spike

## Goal

Determine whether OmniParser earns a place in the default Goni desktop-perception path.

The hypothesis is that OmniParser may improve visual grounding for applications that expose weak or incomplete accessibility trees, but the benefit must justify its latency, resource cost, installation complexity, and maintenance burden.

## Baseline

The V1 baseline is:

```text
mss / DXcam
-> OpenCV diff gate
-> PaddleOCR
-> Windows UI Automation
-> structured screen context
```

OmniParser is not a required runtime dependency until this experiment passes.

## Test set

Use representative screenshots from the actual target Windows machine covering at least:

- VS Code,
- browser pages,
- Windows settings/dialogs,
- Office-style ribbon UI,
- one Electron application,
- one visually rich application with weak accessibility metadata.

Avoid synthetic-only screenshots as the sole test basis.

## Measurements

For each screen, record:

1. installation success and setup time,
2. cold-start latency,
3. warm parse latency,
4. CPU utilization,
5. GPU/accelerator utilization where applicable,
6. peak RAM/VRAM use,
7. number of useful elements returned,
8. bounding-box accuracy,
9. label/type accuracy,
10. duplicate/noise rate,
11. incremental value over Windows UIA + OCR.

## Comparison

Evaluate three modes:

```text
A. UIA only
B. OCR + UIA
C. OCR + UIA + OmniParser
```

The question is not whether OmniParser works in isolation. The question is whether mode C materially improves Goni's downstream screen understanding enough to justify its runtime cost.

## Decision rule

After the spike, assign one of three outcomes:

### adopt

Use OmniParser in the default perception path if it delivers materially better grounding with acceptable latency/resource overhead.

### optional fallback

Keep OmniParser available only for applications/screens where UIA/OCR produce insufficient structure.

### drop

Do not include OmniParser in V1 if it adds little incremental grounding value or imposes disproportionate latency, hardware, setup, or maintenance cost.

## Evidence

The experiment result must include:

- machine specifications,
- software/model version,
- benchmark date,
- raw timing/resource logs,
- representative output samples,
- comparison against the same screenshots under the baseline,
- a short conclusion mapping directly to adopt / optional fallback / drop.

## Relation to V1 routing

Until this experiment is complete:

- Windows UI Automation remains the default structural parser;
- OmniParser stays out of the critical path;
- weak local structure is handled by the remote vision provider rather than by assuming OmniParser is always available.

## Expected value

This experiment prevents a heavy UI parser from becoming architectural baggage before its marginal value is demonstrated.
