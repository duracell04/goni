---
id: AGENT-STORE-01
type: SPEC
status: specified_only
---
# AGENT-STORE-01 - Agent Store Governance
DOC-ID: AGENT-STORE-01
Status: Specified only / roadmap

This spec defines governance requirements for any Goni "Agent Store" or
manifest marketplace. It treats store content as hostile supply chain input.

## 1. Scope

Applies to:
- Official Goni agent stores.
- Community or third-party stores.
- Any install or update flow that delivers agent packages or manifests.

Out of scope:
- Internal development registries (see separate policy).

## 2. Definitions

- **Store**: an index that lists agent packages and metadata.
- **Package**: a signed artifact containing code and a manifest.
- **Publisher**: the entity that signs packages.
- **Trust tier**: a store or publisher classification that controls policy.

## 3. Required package metadata

Every package entry MUST include:
- `package_id` (stable identifier)
- `version` (semver or monotonic)
- `publisher_id`
- `artifact_digest` (content hash)
- `signature` (publisher signature over digest)
- `manifest` (AGENT-MANIFEST-01 compliant)
- `capability_declarations` (requested tools + scopes)
- `sandbox_profile_id` (SANDBOX-01 mapping)
- `update_policy` (auto, prompt, or pinned)
- `provenance` (build or source attestations)

## 4. Trust and verification

The runtime MUST:
- treat store indices as untrusted input.
- verify signatures and digests before install or update.
- validate manifests against AGENT-MANIFEST-01.
- reject packages with undeclared capabilities or missing sandbox mappings.
- enforce store trust tiers for install and update policy.

## 5. Install and update flow

Install MUST:
- record a receipt for the install action.
- pin the exact artifact digest and publisher signature.
- bind the install to a capability policy snapshot.

Update MUST:
- record a receipt for the update action.
- enforce update_policy and allow rollback.
- require explicit confirmation for new capabilities.

## 6. Receipts and auditability

Install, update, and removal MUST emit receipts including:
- package_id, version, artifact_digest
- publisher_id, signature_id
- capability policy snapshot hash
- sandbox_profile_id
- trust tier at time of action

## 7. Revocation and quarantine

The system MUST support:
- publisher revocation lists.
- package quarantine status.
- forced disablement for revoked signatures.
- a safe rollback path to last known-good versions.

## 8. Community stores

Community stores MUST be labeled as untrusted by default and require explicit
opt-in. The runtime SHOULD allow per-store policies, including:
- allowlist and denylist controls.
- stricter sandbox defaults.
- blocked auto-updates.

## 9. Invariants

- **No bypass**: packages cannot be installed without signature verification.
- **Policy binding**: installs bind to a capability policy snapshot.
- **Auditability**: all store actions emit receipts.

## 10. Related specs

- [Agent manifest](/blueprint/30-specs/agent-manifest.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 11. Upstream

- [Agent definition](/blueprint/30-specs/agent-definition.md)
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)

## 12. Downstream

- [Runtime overview](/blueprint/runtime/00-overview.md)
- [Governance hub](/blueprint/docs/hubs/governance.md)

## Conformance tests
- TBD: store signature validation tests.
- TBD: update policy and rollback tests.
