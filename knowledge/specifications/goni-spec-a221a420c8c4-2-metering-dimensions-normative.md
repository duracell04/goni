---
id: GONI-SPEC-A221A420C8C4
title: 2. Metering dimensions (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: GONI should separate logical, physical, economic, and assurance dimensions of execution metering so result correctness, resource consumption, and payment claims are not conflated.
domains:
- specs
- compute
- metering
aliases: []
relations:
- type: refines
  target: METER-01
- type: depends_on
  target: EVID-01
sources: []
artifacts: []
uncertainty: Hardware energy counters, provider telemetry, and cost attribution vary by platform; individual fields require capability discovery and confidence labels.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: 2. Metering dimensions (normative)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2. Metering dimensions (normative)

For each execution, the runtime should capture applicable counters while keeping different claim types explicit.

## Logical metering

Examples:

- `tokens_in`, `tokens_out`;
- logical operation or workload units;
- input/output bytes;
- proof-system workload units where defined;
- tool-call count.

## Physical metering

Examples:

- wall-clock latency;
- accelerator or CPU occupancy time;
- peak memory;
- bytes transferred;
- measured energy where a supported counter exists;
- bandwidth or storage utilization where material.

## Economic metering

Examples:

- quoted or reserved amount;
- accepted execution price;
- verification cost;
- transfer/storage cost;
- settlement amount.

## Meter assurance

Every consequential meter may declare how it was obtained:

- `self_reported`;
- `runtime_measured`;
- `host_attested`;
- `externally_metered`;
- another explicitly defined mechanism.

## Non-equivalence rules

A proof that a result satisfies a computation relation does not by itself prove how many joules, seconds, or accelerator-hours a physical provider consumed.

Likewise, authenticated telemetry about physical resource use does not by itself prove computational correctness.

A result or proof may be reusable. Therefore:

[
valid\ result \neq proof\ of\ fresh\ physical\ expenditure
]

When a protocol specifically requires fresh work, freshness must be an explicit evidence claim with its own nonce, challenge, timing, or trusted-meter semantics.

Where the objective is delivery of an acceptable result, GONI should prefer result-based acceptance and allow permitted caching or reuse rather than rewarding unnecessary physical expenditure.
