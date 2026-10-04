---
id: HCOMP-01
title: Semantic universality and physical specialization
type: principle
status: draft
implementation_state: specified_only
proposition: A common accepted-computation contract may route to physically specialized
  backends according to semantics, locality, memory, interconnect, evidence cost,
  and measured suitability.
domains:
- hardware
- compute
aliases: []
relations:
- type: refines
  target: GONI-PRINCIPLE-COMPUTE-SEMANTIC-NEUTRALITY
- type: depends_on
  target: COMP-REG-01
- type: depends_on
  target: GONI-SPEC-5819106486B4
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
- SRC-MLIR-OFFICIAL
- SRC-ROOFLINE-CACM
artifacts: []
uncertainty: Which specialized routes improve end-to-end performance and assurance
  requires workload-specific implementation and measurement.
legacy: []
---

# Semantic universality and physical specialization

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

\[
\boxed{Semantic\ universality+Physical\ specialization}
\]

Common computation semantics enable qualified substitution; execution remains
hardware-specific. Memory capacity, bandwidth, interconnect/transfer cost,
thermal headroom, backend support and evidence cost constrain useful capacity.
Operation count or nominal accelerator throughput alone is insufficient for an
end-to-end placement decision.

HCOMP-01 refines application-neutral compute semantics. Current local CPU/GPU/NPU
and APU-centric MVP requirements retain their existing scope. Additional crypto,
proof, FPGA, remote, or quantum profiles are research/extension possibilities
whose eligibility is established through COMP-REG-01 and the compute contracts.
The principle creates no new MVP device, minimum capacity, vendor selection,
performance promise, or quantum requirement.
