---
id: SOV-AUDIT-01
type: ANALYSIS
status: specified_only
---
# Sovereign Operator Audit Gap Map
DOC-ID: SOV-AUDIT-01
Status: Specified only / roadmap

This note integrates the sovereign-operator audit into the blueprint without
turning it into a new stack prescription.

Core thesis:

> The stack is not Goni. The governance layer is Goni.

Goni is a private, local, receipt-backed personal operator. Third-party tools
may provide orchestration, inference, retrieval, user interfaces, voice,
workflow automation, observability, model gateways, and hardware acceleration.
They do not own authority.

## 1. What the audit gets right

- Goni must be defined by governance, not by a bundle of tools such as
  LangGraph, MCP, LiteLLM, Ollama, OpenClaw, Qdrant, Home Assistant, or any
  future substrate.
- Retrieval must become governed **Knowledge & Context Engineering**, not demo
  RAG (`chunk -> embed -> top-k -> generate`).
- Model routing is a policy decision over privacy, evidence quality, latency,
  energy, assurance, and fallback chains; it is not only cost optimization.
- Tool protocols such as MCP are useful, but every effectful call still needs
  Goni-issued authority, sandbox classification, approval/corridor evaluation,
  and receipts.
- Sandboxing is part of the trust boundary. Isolation level follows action
  risk, reversibility, and external side effects.
- Observability is insufficient without evaluation gates that answer whether a
  decision was allowed, reversible, properly routed, and supported by permitted
  memory.

## 2. What already exists

The audit is directionally right, but the repo already contains partial P0
contracts:

- Work Orders and Done Contracts:
  [Delegation interface](/blueprint/30-specs/delegation-interface.md).
- Receipts:
  [Receipts](/blueprint/30-specs/receipts.md).
- Capability tokens and mediated tools:
  [Tool Capability API](/blueprint/30-specs/tool-capability-api.md).
- Autonomy corridors:
  [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md).
- Governed memory retrieval:
  [Memory retrieval](/blueprint/30-specs/memory-retrieval.md).
- Model provenance and bundle governance:
  [Model registry](/blueprint/30-specs/model-registry.md).
- Kernel-owned authority over external gateways:
  [Software decision D-023](/blueprint/software/90-decisions.md).

Therefore the immediate work is not to invent these objects from scratch. It is
to harden them, connect them, and add missing taxonomy where the audit exposes
underspecified behavior.

## 3. Gap map

### P0 - Kernel contract hardening

- Keep Work Order, Done Contract, Receipt, CapabilityToken, AutonomyCorridor,
  MemoryClass, ModelManifest, InstallReceipt, EvalReceipt, and RollbackRef as
  Goni-owned concepts.
- Ensure mutating work preserves the chain:
  `intent -> Work Order -> Done Contract -> capability -> policy decision ->
  execution -> receipt -> rollback reference`.
- Ensure third-party framework logs never replace Goni receipts.

### P1 - Missing taxonomies and pipelines

- Memory classes need policy-level aliases for personal/private,
  project/shared, relationship, model/system, ephemeral, and quarantine memory.
- Background autonomy needs explicit job classes, triggers, receipts, and
  interrupt rules.
- Parser outputs need receipt fields because parsing errors can become bad
  memory and bad actions.
- Model provenance needs explicit install, eval, and rollback artifacts around
  model manifests.

### P2 - Governance evaluation gates

- Routing policy must be evaluated, not only individual models.
- Evaluation must cover policy decisions, retrieval permissions, autonomy
  gates, parser/ingestion fidelity, and rollback paths.
- Receipt tiers must prevent high-volume telemetry from drowning
  governance-relevant evidence.

## 4. Architecture consequence

The stable core moves slowly. Tools, models, workflows, parsers, UIs, and
runtimes can move quickly at the edge, but promotion inward requires evidence:

- install receipt,
- eval receipt,
- policy compatibility,
- sandbox classification,
- rollback path,
- and measured improvement.

This is the operational form of sovereign modularity: every external dependency
is replaceable, and every governance decision remains local, inspectable, and
receipted.

## 5. Downstream updates

This gap map is implemented by targeted updates to:

- [Memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Model registry](/blueprint/30-specs/model-registry.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Eval lane](/blueprint/50-evidence/eval/README.md)
