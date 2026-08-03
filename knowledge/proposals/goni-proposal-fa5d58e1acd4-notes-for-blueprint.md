---
id: GONI-PROPOSAL-FA5D58E1ACD4
title: Notes for blueprint
type: proposal
status: draft
implementation_state: specified_only
proposition: Treat public hubs as discovery and metadata infrastructure, and Goni's private registry as the governed source of executable bundles.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/suppliers/model-registries.md
  heading: Notes for blueprint
  revision: c3ac67574bcb5917a0c9e09412b51cc8ae763259
---

# Notes for blueprint

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Notes for blueprint
- Treat public hubs as discovery and metadata infrastructure, and Goni's private
  registry as the governed source of executable bundles.
- Do not claim a self-hosted registry is production-ready for Goni until it has
  passed bundle hash checks, license capture, private-memory policy gates, and
  rollback tests.
- Record model provenance in manifests, ML-BOM/SBOM references, attestations,
  and receipts, not only in runtime logs.
- Treat local eval receipts as bounded behavioral evidence under specified test
  conditions, not global safety proofs.
