---
id: ADR-VISION-V1-ROUTING
title: V1 Vision Provider Routing
type: decision
status: draft
implementation_state: specified_only
proposition: "For the current desktop-vision prototype, Goni prioritizes low latency and low marginal cost over local-model sovereignty: perception remains local, Gemini Flash-Lite is the default on-demand vision provider, Grok is the stronger reasoning fallback, and local Qwen is deferred to optional offline/private mode."
domains:
- software
- vision
- desktop-agent
aliases:
- vision-provider-routing
- gemini-first-vision-v1
relations:
- type: refines
  target: VIS-01
- type: depends_on
  target: GONI-PRINCIPLE-D03F6E9577A1
sources: []
artifacts: []
uncertainty: "Provider choice is a current V1 optimization, not a permanent architectural commitment. Runtime model names, prices, and latency can change and must be re-benchmarked."
---

# V1 Vision Provider Routing

## Decision

For the current Goni desktop-vision prototype, optimize first for:

1. low user-perceived latency,
2. low marginal API cost,
3. simple integration,
4. reliable screen explanation.

Sovereignty remains a long-term architectural value, but it is not the binding optimization objective for this prototype stage.

The V1 runtime split is:

```text
LOCAL PERCEPTION
mss / DXcam
-> OpenCV diff gate
-> PaddleOCR
-> Windows UI Automation
-> structured screen context

ON-DEMAND COGNITION
-> Gemini Flash-Lite by default
-> Grok fallback for stronger reasoning
-> local Qwen only as optional offline/private mode
```

## Boundary

This decision does not collapse observation, extraction, memory, egress, and actuation into one permission.

BOUND-01 and the Vision, Memory, and Actuation principle remain authoritative:

- screen capture may stay local,
- OCR/UI extraction may stay local,
- model egress is a separate governed decision,
- memory persistence is separate,
- actuation requires separate authority.

The model provider is therefore replaceable. The authority boundary is not.

## Screen-call policy

The live desktop loop must not continuously stream frames to a remote model.

Remote vision calls occur on demand, for example when the user asks:

- "What am I looking at?"
- "Explain this error."
- "What should I click?"
- "Summarize this screen."
- "Turn this into a mind-map node."

The default request should include only the smallest sufficient context:

```text
relevant screenshot or crop
+ OCR text
+ active-window identity
+ UIA elements
+ user question
```

## V1 routing heuristic

A sophisticated provider confidence model is deferred.

Until measured routing evidence exists, use a simple explicit heuristic:

- if local OCR returns fewer than 50 useful characters, remote vision is preferred;
- if Windows UI Automation returns zero useful elements, remote vision is preferred;
- otherwise structured local context is included with the remote request and should be used to reduce ambiguity.

These thresholds are provisional and must be replaced by measured routing criteria when enough traces exist.

## Provider roles

### Gemini Flash-Lite

Default V1 vision provider because the current objective is speed and low price.

Provider-specific model identifiers belong in the implementation repository/configuration and may change without altering this architectural decision.

### Grok

Reasoning fallback for cases where the default provider returns an error, weak answer, or the task requires materially stronger planning/critique.

Grok is not the default high-frequency screenshot explainer while its marginal input cost is materially higher than the chosen Flash-Lite tier.

### Local Qwen

Deferred to optional offline/private mode.

Local multimodal inference remains architecturally supported, but it is not in the latency-critical default path until target-machine benchmarks justify it.

## Implementation evidence boundary

The experimental implementation lives in:

- https://github.com/duracell04/goni-cognitive-os
- pinned experiment baseline: https://github.com/duracell04/goni-cognitive-os/commit/909e1119a31a6b5500eda705f623ae2666ae7993

That repository can test this decision but cannot redefine canonical Goni authority or promote this node's status.

## Revisit triggers

Re-evaluate this decision when any of the following becomes true:

- local VLM latency on target hardware approaches cloud latency at acceptable quality;
- cloud vision spend becomes economically material;
- privacy/offline requirements become binding;
- provider pricing changes materially;
- desktop/session volume makes local inference cheaper at the margin;
- a provider materially outperforms the current default on the Goni screen-understanding benchmark.

## Consequence

Goni remains provider-agnostic at the architecture level while using a concrete, low-friction provider choice for V1.

The current objective is:

> Prove the interaction loop cheaply and quickly; preserve the right to localize later.
