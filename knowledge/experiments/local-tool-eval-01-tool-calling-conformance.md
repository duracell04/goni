---
id: LOCAL-TOOL-EVAL-01
title: Local Tool-Calling Conformance
type: experiment
status: draft
implementation_state: not_applicable
proposition: Goni should evaluate tool calling on the exact local deployment profile using representative synthetic tasks that separately measure tool selection, abstention, argument correctness, structured validity, multi-call behavior, stateful recovery, latency, and resource cost.
domains:
- evaluation
- tools
- local-ai
aliases:
- local function-calling evaluation
relations:
- type: tests
  target: TOOL-SURFACE-01
- type: tests
  target: STRUCT-OUT-01
- type: tests
  target: MODEL-DEPLOY-01
sources:
- SRC-PATIL2025-BFCL
- SRC-GENG2025-JSONSCHEMABENCH
artifacts: []
uncertainty: Scenario fixtures, thresholds, model/runtime coverage, and acceptable error rates remain to be calibrated. External leaderboard scores are not substitutes for Goni deployment-profile results.
legacy: []
---

# Local Tool-Calling Conformance

The evaluation unit is:

```text
bundle_id
+ deployment_profile_id
+ tool_surface policy
+ eval pack
```

rather than the model family name alone.

## Task families

The synthetic, domain-neutral evaluation pack should include:

- correct single-tool selection;
- correct no-tool / abstention decisions;
- argument extraction and normalization;
- enum and type constraints;
- multiple sequential tool calls;
- parallelizable tool calls where supported;
- tools with overlapping descriptions;
- missing required information;
- stale state and changed preconditions;
- tool errors and partial failures;
- correction after a rejected proposal;
- stateful multi-turn continuation;
- unavailable or policy-ineligible tools; and
- post-tool result interpretation.

## Metrics

Report at least:

- task success;
- correct tool / operation selection;
- false-positive tool-call rate;
- missed-tool rate;
- abstention accuracy;
- argument semantic accuracy;
- structured/schema-valid rate;
- invalid-surface reference rate;
- unauthorized proposal rate;
- recovery success after tool errors;
- correction-following success;
- tool-call count;
- context/tool-schema tokens;
- time to first valid proposal;
- end-to-end latency; and
- peak local memory where measurable.

Structured validity and semantic correctness must be reported separately.

## Comparative protocol

When comparing two deployments, hold the WorkOrder, authority corridor, tool semantics, fixture state, and acceptance criteria constant. If the adapter or tool representation differs, record the difference explicitly because the harness is part of observed system capability.

BFCL is an external reference for task categories and function-calling evaluation. Goni should maintain its own fixed local suite so results remain relevant to its kernel, tool surface, state model, and local hardware.

## Evidence promotion

Results become Goni evidence only when the exact deployment profile, model bundle, runtime version, tool manifests, schemas, seeds where meaningful, raw outputs, environment state, and evaluator version are pinned.
