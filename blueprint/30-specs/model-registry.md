---
id: MODEL-REG-01
type: SPEC
status: specified_only
---
# Model Bundle Registry Governance
DOC-ID: MODEL-REG-01

Status: Specified only / roadmap

Open-weight release decentralizes access to model parameters, but not
necessarily governance over discovery, metadata, provenance, evaluation,
licensing, deployment, or runtime permissions. Goni may discover models through
public ecosystems, but approved execution flows through a governed bundle
registry. The runtime executes immutable bundle IDs whose provenance, license,
hashes, task permissions, assurance level, and evaluation receipts are known
before use.

Scientific framing:
- Observed fact: widely available model weights can broaden participation and
  enable local inference, while model openness still depends on documentation,
  code, data, licenses, and access structure. [[ntia2024-open-model-weights]]
- Theoretical inference: model hubs are governance infrastructure, not only file
  storage. They shape discovery, naming, metadata conventions, reputation,
  access restrictions, and takedown paths.
- Goni hypothesis: the unit of trust in open AI should shift from the hosted
  model repository to the locally attested model installation.

## 1. Scope

This spec applies to model bundles used by local or remote inference runtimes.
It covers model provenance and approval metadata, not model training or runtime
scheduling.

## 2. Bundle manifest

Each approved bundle MUST have a manifest with:

- `bundle_id`
- upstream registry or source
- upstream model id and revision
- model card URI
- datasheet URI when dataset lineage is known
- ML-BOM or SBOM URI when available
- license id or license URI
- publisher or maintainer
- weight file hashes
- manifest hash
- quantization or precision
- runtime compatibility (`llama.cpp`, `Ollama`, `vLLM`, `SGLang`, or other)
- approved task classes
- private-memory permission (`deny | distilled_only | allowed_by_policy`)
- assurance level (`L0 | L1 | L2 | L3 | L4`)
- eval receipt refs
- attestation refs
- policy hash
- policy pack refs
- created-at timestamp

The manifest MUST be hash-addressed. A runtime MUST reject undeclared or
mutated bundle contents.

## 3. Registry roles

Goni distinguishes three roles:

- Public discovery: broad ecosystem search and metadata lookup.
- Private registry: local or self-hosted cache of approved bundles.
- Runtime loader: engine-specific loading from approved bundle IDs only.

Public discovery may include Hugging Face or ModelScope. Private registry
candidates may include self-hosted registries such as MatrixHub or KohakuHub
when they satisfy Goni policy, storage, and audit requirements. Runtime loading
may use engines such as Ollama, llama.cpp, vLLM, or SGLang.

Public hubs can decentralize access while centralizing epistemic mediation. In
Goni, the hub is a discovery input; the local bundle registry is the execution
authority.

## 4. Assurance levels

Goni uses graded assurance, not a trusted/untrusted binary:

| Level | Evidence | Maximum default use |
|-------|----------|---------------------|
| L0 | Hash only | Sandbox testing |
| L1 | Hash + manifest + license state | Public or low-risk tasks |
| L2 | Local eval receipt | Personal low-sensitivity memory |
| L3 | Signed third-party or community eval | Broader tool use |
| L4 | Reproducible provenance + ML-BOM + audit trail | Sensitive memory or enterprise use |

Assurance levels are permission ceilings, not guarantees. Policy may further
restrict a bundle below its assurance level.

## 5. Policy gates

Before a bundle may process private memory, policy MUST check:

- license compatibility,
- source and publisher trust,
- hash match,
- assurance level,
- eval receipt coverage for the requested task class,
- private-memory permission,
- network and retention policy for the runtime destination,
- policy pack provenance and override rules.

If any gate fails, the router MUST choose a safer approved bundle or block the
request.

Policy sources MUST be transparent, inspectable, and provenance-bearing. Goni
MUST support user-editable policies, signed policy packs, community or
enterprise overlays, and override receipts. Otherwise the local registry would
replace one hidden governance layer with another.

## 6. Evaluation limits

Local evaluation receipts are attestations, not proofs. They can show which
tests ran, under which environment, against which model hash, with which
results. They do not prove absence of backdoors, lawful training data, future
safety, or semantic equivalence to upstream claims.

Distributed trust remains a separate problem: one Goni node SHOULD NOT accept
another node's eval receipt without signature validation, evaluator identity,
environment disclosure, failure disclosure, and policy-approved reputation or
attestation rules. SLSA, in-toto, SPDX, and CycloneDX are relevant source
patterns for this supply-chain evidence model. [[slsa-framework]]
[[in-toto-framework]] [[spdx-overview]] [[cyclonedx-mlbom]]

## 7. Receipt contract

Model execution receipts SHOULD include:

- `provider`
- `model_id`
- `bundle_id`
- `manifest_hash`
- `assurance_level`
- `ml_bom_ref`
- `attestation_refs`
- `policy_hash`
- `eval_receipt_refs` when the route depends on prior evaluation.

Bundle promotion, rollback, deletion, and policy override are mediated actions
and MUST emit receipts.

## 8. Upstream

- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 9. Downstream

- [Local models supplier card](/blueprint/60-market/suppliers/local-models.md)
- [Goni Lab](/blueprint/docs/goni-lab.md)

## Conformance tests

- Runtime rejects a bundle with a mismatched manifest hash.
- Runtime rejects private-memory use when permission is `deny`.
- Runtime rejects sensitive-memory use below the configured assurance floor.
- Bundle promotion emits a receipt with bundle and manifest hashes.
- Route selection can explain why a bundle was eligible for the task class.
- Local eval receipts state test environment, model hash, result summary, and
  limits of inference.
