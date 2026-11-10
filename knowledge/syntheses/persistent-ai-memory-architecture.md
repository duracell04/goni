---
id: "SYN-PERSIST-MEM-2026"
title: "Toward a complete architecture for persistent AI memory — research matrix"
type: "synthesis"
status: "draft"
implementation_state: "specified_only"
proposition: "A proposed persistent AI memory architecture links governed source and event storage, temporally versioned assertions, derived knowledge projections, model-independent agent context, and outcome-driven consolidation, while keeping epistemic claims distinct from kernel authority."
domains:
- "agent"
- "memory"
- "research"
- "system"
aliases:
- "PERSISTENT-MEMORY-RESEARCH-MATRIX"
relations:
- type: "synthesizes"
  target: "GONI-SPEC-33294E0D3306"
- type: "synthesizes"
  target: "MEM-RETR-01"
- type: "synthesizes"
  target: "CGG-01"
- type: "synthesizes"
  target: "GONI-SPEC-5CD378282A73"
- type: "synthesizes"
  target: "SKILL-REG-01"
- type: "synthesizes"
  target: "DREAM-01"
- type: "synthesizes"
  target: "GONI-SPEC-5FAD21682D7E"
sources:
- "SRC-CHALEF2026-SKILLS"
- "SRC-HE2026-MEMORYARENA"
- "SRC-KARPATHY2026-LLMWIKI"
- "SRC-LATIMER2026-HINDSIGHT"
- "SRC-LIN2026-LTMSEC"
- "SRC-PACKER2023-MEMGPT"
- "SRC-RASMUSSEN2025-ZEP"
- "SRC-SHEN2026-MEM2ACTBENCH"
- "SRC-W3C2013-PROV"
- "SRC-WU2025-LONGMEMEVAL"
- "SRC-YU2026-AGEMEM"
- "SRC-ZHANG2026-HIVEMEM"
artifacts: []
uncertainty: "This 16-dimension matrix is an engineering synthesis. The cited studies vary from peer-reviewed proceedings to preprints, proposed standards, and software projects. It is neither a validated unified memory system nor a promotion of the cited architecture into GONI's normative contracts."
legacy: []
---

# Toward a Complete Architecture for Persistent AI Memory

Consolidated Research Matrix · 10 November 2026

## Scope

This node consolidates sixteen research dimensions into one **proposed GONI memory architecture**. The matrix is a map of scientific precedent, implemented approaches, and proposed engineering principles. A source's existence or claimed performance does not establish a functioning GONI implementation.

## Consolidated research matrix

| Research dimension | Architectural principle / knowledge contribution | Academic references and implementations |
| --- | --- | --- |
| 1. Memory taxonomy | Distinguish working, episodic, semantic, procedural and prospective memory by function, update semantics and retrieval needs. | [MemGPT (Packer et al., 2023)](https://arxiv.org/abs/2310.08560); [Hindsight (Latimer et al., ACL 2026)](https://aclanthology.org/2026.acl-demo.27/) |
| 2. Knowledge representation | Maintain structured, evolving knowledge derived from evidence; distinguish source observations, assertions, inferred beliefs and decisions; render human-readable projections. | [Karpathy, LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f); [gbrain](https://github.com/garrytan/gbrain); [Hindsight](https://aclanthology.org/2026.acl-demo.27/) |
| 3. Runtime memory management | Retrieve, compress and update information across bounded context windows while keeping durable state independent of model execution. | [MemGPT](https://arxiv.org/abs/2310.08560); [Agentic Memory / AgeMem (Yu et al., ACL 2026)](https://aclanthology.org/2026.acl-long.981/); [Mem0](https://github.com/mem0ai/mem0); [Letta](https://github.com/letta-ai/letta) |
| 4. Temporal and relational memory | Represent entities, assertions and relationships with explicit recording time and validity time; retain superseded versions. | [Zep temporal knowledge graph (Rasmussen et al., 2025)](https://arxiv.org/abs/2501.13956); [Graphiti](https://github.com/getzep/graphiti); [Cognee](https://github.com/topoteretes/cognee) |
| 5. Epistemic consistency | Track uncertainty, contradictions and belief revision; distinguish direct evidence, inferred claims, opinion and provenance lineage. | [Hindsight](https://aclanthology.org/2026.acl-demo.27/); [W3C PROV-DM](https://www.w3.org/TR/prov-dm/) |
| 6. Agent continuity | Serialize goal/task state, source-backed decisions, execution history and unresolved dependencies for cross-model and cross-runtime handoffs. | [MemGPT](https://arxiv.org/abs/2310.08560); [ai-memory](https://github.com/akitaonrails/ai-memory); [claude-second-brain](https://github.com/jessepinkman9900/claude-second-brain) |
| 7. Procedural learning | Compile verified execution traces and recurring user corrections into versioned, inspectable procedures without requiring weight updates. | [Chalef et al., Learning by Doing (2026 preprint)](https://arxiv.org/abs/2611.01386); [open-second-brain](https://github.com/itechmeat/open-second-brain); [COG second brain](https://github.com/huytieu/COG-second-brain) |
| 8. Multimodal memory | Keep original visual/audio evidence linked to derived text, captions and embeddings; choose evidence modalities per query. | [HiVe-Mem (Zhang et al., 2026 preprint)](https://arxiv.org/abs/2611.12872); [agent-second-brain](https://github.com/smixs/agent-second-brain) |
| 9. Memory lifecycle | Define policy for admission, consolidation, retrieval, supersession, retention, revocation and forgetting; distinguish history from current relevance. | [Lin et al., Long-Term Memory Security (EMNLP 2026)](https://aclanthology.org/2026.emnlp-main.2296/); [Agentic Memory](https://aclanthology.org/2026.acl-long.981/) |
| 10. Security and governance | Require authorized writes and principal-scoped retrieval, provenance, rollback and deletion verification through explicit policy/enforcement mechanisms. | [Lin et al. (EMNLP 2026)](https://aclanthology.org/2026.emnlp-main.2296/); [W3C PROV-DM](https://www.w3.org/TR/prov-dm/) |
| 11. Multi-agent consistency | Govern concurrent writers through versioned authoritative state, transactional updates, isolation, conflict resolution and auditable transformations. | Architectural synthesis informed by [MemGPT](https://arxiv.org/abs/2310.08560), [Lin et al. (2026)](https://aclanthology.org/2026.emnlp-main.2296/) and [W3C PROV-DM](https://www.w3.org/TR/prov-dm/) |
| 12. Retrieval evaluation | Test information extraction, multi-session synthesis, temporal reasoning, knowledge updates and justified abstention. | [LongMemEval (Wu et al., ICLR 2025)](https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html) |
| 13. Action-oriented evaluation | Test whether memory improves subsequent decisions, tool selection, grounded parameters and completion of interdependent tasks. | [MemoryArena (He et al., ICML 2026)](https://proceedings.mlr.press/v306/he26am.html); [Mem2ActBench (Shen et al., ACL 2026)](https://aclanthology.org/2026.acl-long.370/) |
| 14. Computational efficiency | Optimize verified downstream task utility against retrieval, indexing, generation, storage, verification and ongoing maintenance costs. | [MemGPT](https://arxiv.org/abs/2310.08560); [AgeMem](https://aclanthology.org/2026.acl-long.981/); [HiVe-Mem](https://arxiv.org/abs/2611.12872) |
| 15. Unified implementation | Proposed pipeline: governed evidence/events → temporally versioned claims and policy references → derived wiki/graph/search projections → model-independent execution → outcome-driven consolidation. | GONI architectural synthesis of preceding literature; **not independently validated** |
| 16. Application domains | JEV: agent continuity and procedural memory; EDELKI: temporal relationship intelligence and evidence-backed decisions; scientific knowledge network: claims, reproducible provenance and auditable derivation. | Proposed application mapping using MemGPT, Graphiti, Hindsight, W3C PROV-DM, MemoryArena and Mem2ActBench |

## Clarifying distinctions

**Functional memory classes and epistemic classes are orthogonal.** Working, episodic, semantic, procedural and prospective describe the information's role. GONI's `fact`, `preference`, `decision`, `hypothesis` and `derived` distinguish governance or epistemic treatment. For example, a semantic belief may remain unconfirmed, and a procedure may require separate approval to become a governed skill.

**Provenance, truth and authority are separate.** W3C PROV-DM describes sources, entities, activities, agents and derivations; it does not itself enforce permissions, retention, forgetting or correctness. Source lineage supports auditability, and independent corroboration is a separate evidentiary question. Model output may update hypotheses but cannot grant tool authority.

**Stored knowledge and working execution context are separate.** GONI's canonical Arrow/Memory substrate holds governed records, while graph, wiki, vector, keyword and multimodal representations are derived, rebuildable views. The Context Plane selects relevant material for each Work Order. The kernel mediates memory writes, model use and consequential action under existing policies.

**Persistent memory utility is measured downstream.** Successful retrieval is necessary for many tasks but insufficient for reliable grounded execution. LongMemEval, MemoryArena and Mem2ActBench cover complementary evaluation targets. Learned skill compilation, temporal claims, forgetting and multimodal retrieval require task-specific testing.

## Proposed implementation sequence and GONI mapping

1. **Canonical input and authority:** authorized source objects, event observations, asset hashes, memory grants and receipts; existing `GONI-SPEC-5CD378282A73` and `MEM-RETR-01` remain authoritative.
2. **Claims and time:** represent observation, assertion, belief, confidence, subject/entity role, recorded-at, validity interval, derivation, supersession and conflict with explicit epistemic typing. Bitemporal semantics are an evaluation candidate, not a new adopted schema here.
3. **Derived projections:** compile human-readable wiki pages, temporal graph views, embeddings, indexes and selective multimodal evidence packets from source-backed records. Preserve path back to original observations.
4. **Context and continuity:** compile Work Order-specific context, unresolved dependencies, procedural fragments and memory references for model-independent agent handoffs; the LLM remains replaceable.
5. **Learning and lifecycle:** consolidate evidence and successful verified traces into separately reviewed beliefs or `SKILL-REG-01` skills; use `DREAM-01` for bounded synthetic hypotheses. Retention, revocation and deletion require explicit tests.
6. **Outcome-based evaluation:** measure factual recall, temporal correctness, contradiction management, action success, authority compliance, per-principal isolation and complete resource cost using `MEM-EVAL-01`.

## Evidence classification and limitations

- **Conference proceedings:** ACL (including System Demonstrations), EMNLP, ICLR and ICML publications are scholarly venue publications; their reported measurements remain source-specific.
- **Technical standard:** W3C PROV-DM specifies interoperable provenance semantics; implementation and enforcement are separate obligations.
- **Preprints:** Rasmussen et al. (2025), Chalef et al. (2026), and Zhang et al. (2026) are used as research proposals/early evidence, not independent proof of product quality.
- **Implementations and ideas:** GitHub projects supply inspectable technical approaches; Karpathy's LLM Wiki is an architectural idea, not a peer-reviewed experiment.
- **GONI status:** architectural synthesis, specified only. No new kernel contract, database schema, memory authority or deployed dependency is authorized by this document.

## GONI canonical links

Existing concepts are described in `GONI-SPEC-33294E0D3306` (memory taxonomy), `MEM-RETR-01` (governed retrieval), `CGG-01` (relational context selection), `GONI-SPEC-5CD378282A73` (memory write policy), `SKILL-REG-01` (governed procedural skills), `DREAM-01` (bounded synthetic cognition), and `GONI-SPEC-5FAD21682D7E` (visual memory). The new `EVID-MEM-ACTION2026` source-evidence node and `MEM-EVAL-01` experiment specify the action-evaluation research extension.
