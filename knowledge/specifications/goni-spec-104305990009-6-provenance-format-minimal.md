---
id: GONI-SPEC-104305990009
title: 6. Provenance format (minimal)
type: specification
status: draft
implementation_state: specified_only
proposition: Provenance records origin and derivation, while authenticity and freshness claims require explicit evidence and validity intervals rather than being inferred from a source label or timestamp.
domains:
- specs
- provenance
aliases: []
relations:
- type: refines
  target: RESULT-01
sources:
- SRC-W3C2013-PROV
artifacts: []
uncertainty: Source-specific authenticity mechanisms and freshness windows depend on connector, data class, and task semantics.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 6. Provenance format (minimal)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 6. Provenance format (minimal)

`provenance` is a structured object that can include:

- `source`: origin such as observer, encoder, tool, agent, connector, or external provider;
- `content_hash`: identity of the observed or derived content where appropriate;
- `observed_at`: when GONI observed the item;
- `valid_at`: the external time or state for which the item claims relevance;
- `expires_at` or `freshness_policy`: when the item requires refresh;
- `inputs`: references to upstream records;
- `permissions`: policy tags in effect;
- `authenticity_evidence_refs`: signatures, authenticated connector receipts, attestations, or other evidence where available;
- `trust_domain`: the authority or boundary within which the provenance claim is accepted.

## Semantic boundary

Provenance answers where a datum came from and how it was derived.

It does not automatically establish that:

- the originating source was truthful;
- the source was authorized;
- the data was fresh enough for the current decision;
- the external event described by the data actually occurred.

Those properties require separate evidence, connector guarantees, or policy assumptions. A timestamp alone is not a freshness guarantee, and a content hash alone is not an authenticity guarantee.
