---
id: EVID-ENF-01
type: EVIDENCE
status: specified_only
---
# EVID-ENF-01 Egress Non-bypass Validation

Goal: provide reproducible evidence that tool/runtime workloads cannot perform
network egress without mediated capability authorization.

## Hypothesis

In conformant runtime profiles, direct outbound socket attempts from tool
runner context fail closed, while authorized mediated egress succeeds and emits
attributable receipts.

## Test matrix

- Profile A: strict deny-all egress for tool runners.
- Profile B: mediated allowlist egress via gateway.
- Profile C: policy-engine unavailable (fail-closed behavior).

## Procedure (placeholder)

1. Launch tool runner with baseline sandbox profile.
2. Attempt direct socket egress from runner process.
3. Attempt mediated egress with missing capability.
4. Attempt mediated egress with valid capability.
5. Capture receipts, gateway logs, and kernel mediation events.

## Pass criteria

- Direct egress attempt returns explicit deny/failure.
- Missing-capability mediated request is denied with decision metadata.
- Valid-capability mediated request succeeds and is attributable.
- Each attempt path has exactly one terminal receipt entry.

## Artifacts

- `nonbypass-egress-report.json` (planned)
- receipt trace samples (planned)
- sandbox policy profile used (planned)
