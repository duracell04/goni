---
id: EVID-ROUTE-01
type: EVIDENCE
status: specified_only
---
# Frugal Sovereign Routing

Goal: evaluate whether Goni chooses the cheapest, most private, sufficiently
capable route before escalating to the cloud Council.

## Routing Hypothesis

Goni should outperform an always-cloud policy on privacy exposure, cost, and
local-token ratio while avoiding unacceptable quality loss. It should outperform
an always-local policy on tasks that require current public evidence, external
model diversity, or compute beyond the local budget.

Evidence anchors: [[frugalgpt2023]], [[routellm2024]], [[routerbench2024]].

## Scenarios

- Friendly follow-up reminder: local memory plus small local model; no cloud.
- Academic research on a new public paper: local retrieval, public web, Council
  only if synthesis or disagreement checking is needed.
- Codebase audit: local repo scan and local model summary; cloud only for
  non-sensitive abstract reasoning after approval/redaction.
- High-stakes strategy memo: local first pass, Council critique when approved,
  local final synthesis.
- Private current-web request: keep raw private context local; require redaction
  or approval before any external payload.

## Metrics

- correct local accept rate
- false local accept rate
- late escalation rate
- wasteful cloud-call rate
- privacy-risk override/block rate
- local token share
- cloud token and dollar cost
- p50/p95 latency by route
- Council value rate: cases where disagreement checking changed the final
  answer or caught a material defect
- receipt completeness for `llm_route`

## Required Artifacts

- trace set with prompt class, sensitivity class, selected route, and expected
  route label
- receipt samples proving `llm_route` contains classification, route rationale,
  redaction state, privacy class sent, models considered/used, and policy
  decision
- error analysis for false local accepts, late escalation, wasted cloud calls,
  and privacy-risk blocks
- promotion memo stating whether router thresholds should change

## Acceptance Criteria

- Routine/public-low-risk tasks default local unless they exceed local budgets.
- Current public research tasks can escalate with public-only or redacted
  payloads.
- Private/sensitive tasks do not send raw context to cloud by default.
- Every model-routing decision that affects output or escalation eligibility has
  an inspectable `llm_route` receipt object.
