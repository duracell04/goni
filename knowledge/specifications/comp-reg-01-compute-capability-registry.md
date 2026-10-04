---
id: COMP-REG-01
title: Compute capability registry contract
type: specification
status: draft
implementation_state: specified_only
proposition: Scheduler-visible compute capabilities describe supported operations
  and semantic/evidence profiles with resource bounds, locality, provenance, freshness,
  and assurance, permitting placement only among policy-eligible resources.
domains:
- compute
- scheduler
- hardware
aliases: []
relations:
- type: refines
  target: SCHED-01
- type: depends_on
  target: COMP-01
- type: depends_on
  target: EXEC-01
- type: depends_on
  target: EVID-01
- type: depends_on
  target: NET-COMP-01
- type: refines
  target: GONI-IMAP-FBBFE14FE1A3
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
- SRC-RAY-SCHEDULING-LOCALITY
- SRC-ROOFLINE-CACM
artifacts: []
uncertainty: Capability encoding, confidence/freshness policies, objective weights,
  provider discovery, and counter availability remain profile-specific design questions.
legacy: []
---

# Compute capability registry contract

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Capability description

The registry is a logical scheduler-facing view over existing discovery and
telemetry. An implementation may use an in-process collection for the local MVP;
the contract does not prescribe a discovery service or database.

\[
H_i=(Ops,Memory,BW,Latency,Energy,Trust,Proof).
\]

Each term is a scoped descriptor: supported operations/shapes and numerical
profiles; available memory; observed or estimated bandwidth/latency/energy;
trust-domain and evidence references; and available proof/evidence mechanisms.
Trust denotes named assumptions rather than a universal scalar. Capability and
resource estimates state their source, validity interval, measurement/advertisement
status, uncertainty, and material runtime/driver/hardware versions.

## Eligibility and placement

\[
\mathcal H_J=\{H:Cap(H)\models Req(J)\land PolicyAllows(J,H)\},
\]
\[
H^*=\arg\min_{H\in\mathcal H_J}
(\alpha L+\beta Cost+\gamma Energy+\delta V+\eta PrivacyRisk).
\]

\(\models\) includes set membership for operations and comparisons for numeric
capacity, deadlines, and bounds. The accepted semantics, trust class, privacy
policy, evidence path, and current authority are eligibility conditions. The
weighted objective compares eligible routes; residual permitted privacy risk is
distinct from violation of a hard disclosure constraint.

The scheduler's named objective profile MUST define units, normalization,
weights, and the meaning of \(V\) (evidence production/verification burden).
Execution, evidence, verification, transfer/storage, coordination/settlement,
expected failure and retry costs are included where material. Terms are counted
once, and measured post-execution costs remain distinct from pre-execution estimates.

## Conformance and updates

A conforming scheduler MUST compare requirements with fresh, sufficiently assured
capabilities, including dynamic memory pressure and thermal/resource headroom.
An unsupported profile, unresolved required property, or unavailable eligible
backend returns the job's ordinary deferred, rejected, or unresolved outcome.
Backend failure and capability changes trigger bounded re-evaluation through the
existing job lifecycle. A provider's advertised evidence mechanism is checked
against EVID-01 at result acceptance.

Brand names and application labels may be descriptive metadata; eligibility is
defined by capabilities and contracts. A future specialized or quantum backend
is eligible only after its operation, stochastic/numerical acceptance semantics,
and evidence profile are explicitly defined and supported.
