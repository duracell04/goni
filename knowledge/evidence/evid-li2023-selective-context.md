---
id: EVID-LI2023-SELECTIVE-CONTEXT
title: 'Source claim: Selective Context'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Selective Context identifies and prunes redundant input material and reports lower inference memory use and latency with relatively small downstream quality loss on evaluated tasks.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SPEC-A29366F3E2EA
sources:
- SRC-LI2023-SELECTIVE-CONTEXT
artifacts: []
uncertainty: The reported efficiency-quality trade-off is workload-specific and does not establish that low-surprisal or redundant-looking text is safe to discard in high-consequence Goni tasks.
legacy: []
---

# Source claim: Selective Context

Selective Context compresses long inputs by pruning information estimated to be
redundant before model inference.

## Goni relevance

The result supports evaluating context construction as an information-selection
problem rather than treating every available token as equally useful.

## Boundary

Rare or low-frequency evidence may still be decisive. Goni therefore needs
task-, evidence-, and consequence-sensitive omission controls.
