# Model Registries - Supplier Note

## What it is
- Model hubs, private registries, and local runtimes that provide model weights
  or approved bundle delivery for Goni.

## Relevance to Goni
- Public hubs are useful discovery surfaces, not the source of execution
  authority.
- Goni should execute only approved model bundles whose license, hashes,
  quantization, task permissions, assurance level, private-memory permission,
  and evaluation receipts are known.
- The scientific question is the distribution-governance layer around
  open-weight AI, not whether any single public hub is good or bad.

## Current candidates
- Public discovery: Hugging Face and ModelScope.
- Private/self-hosted registry candidates: MatrixHub and KohakuHub, subject to
  local policy, deployment maturity, and audit review.
- Local runtimes: Ollama, llama.cpp, vLLM, and SGLang.

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

## Related docs
- [Model bundle registry governance](/blueprint/30-specs/model-registry.md)
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Local models](/blueprint/60-market/suppliers/local-models.md)
