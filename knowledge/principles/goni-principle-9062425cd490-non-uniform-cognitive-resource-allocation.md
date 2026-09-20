---
id: GONI-PRINCIPLE-9062425CD490
title: Non-uniform cognitive resource allocation
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should keep large logical capacity addressable while activating the smallest sufficient task-relevant working set, allocating scarce cognitive resources according to expected marginal utility rather than uniformly.
domains:
- agent
- data
- kernel
- software
- system
aliases:
- large-latent-state-small-active-state
relations:
- type: refines
  target: GONI-PRINCIPLE-B40DDEFD1872
- type: supports
  target: GONI-IMAP-EB2133E6965D
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources: []
artifacts: []
uncertainty: Heavy-tailed access is an empirical hypothesis at each abstraction level. Lexical Zipf frequency does not establish a universal distribution for KV importance, memory retrieval, prefix reuse, expert activation, or tool demand.
legacy: []
---

# Non-uniform cognitive resource allocation

Goni should treat memory, context, inference, retrieval, and model capacity as
large addressable spaces from which only a small task-relevant working set
becomes active at a given moment.

The design rule is:

[
	ext{estimate usefulness cheaply}
ightarrow
	ext{activate the smallest sufficient subset}
ightarrow
	ext{spend expensive computation there}.
]

A conceptual allocation score for an item (i) is:

[
U_i =
rac{
P(i	ext{ will be useful again})
	imes
Impact_i
	imes
ReconstructionCost_i
}{
StorageCost_i + BandwidthCost_i + ComputeCost_i
}.
]

This is a systems principle, not a claim that one universal scoring function is
already known. The relevant distribution must be measured at the abstraction
being managed: retrieved evidence, contextual state, KV blocks, prefixes,
experts, models, tools, or verification work.

The principle generalizes Goni's existing architecture:

- the Arrow/Vault state can remain large while the Context Plane remains bounded;
- retrieval may produce a candidate set larger than the final working context;
- the model router may prefer the smallest sufficient local computation;
- KV residency may be selective under memory and bandwidth pressure;
- external models and tools remain exceptional resources activated when local
  evidence, confidence, capability, or freshness is insufficient.

The architectural target is therefore:

[
oxed{
	ext{large logical capacity}
+
	ext{small dynamically selected active state}
}
]

Goni MUST NOT infer low consequence merely from low frequency. Rare evidence can
be mission-critical. Frequency, semantic relevance, consequence, provenance,
and reconstruction cost remain distinct variables.

This principle is compatible with heavy-tailed or Zipf-like access when such
concentration is observed, but it does not assume a universal exponent or a
universal power-law distribution in advance.
