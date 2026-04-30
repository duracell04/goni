# Bibliography (annotated)

Key: [[frugalgpt2023]]
Claim: LLM cascades, prompt adaptation, and LLM approximation can reduce
inference cost while preserving or improving task quality.
Relevance:
- Grounds Goni's cascade pattern: cheap/private/local routes should be tried
  before expensive external routes.
- Goni extends the objective from cost-quality to sovereignty, privacy,
  latency, energy, auditability, and policy compatibility.
Used in:
- `blueprint/10-product/15-delegation-doctrine.md` (Frugal Sovereign Routing)
- `blueprint/software/20-architecture.md` (Frugal sovereign model router)
Source:
- https://huggingface.co/papers/2305.05176

Key: [[routellm2024]]
Claim: Routers trained from preference data can select between stronger and
weaker LLMs to reduce cost while preserving much of strong-model performance.
Relevance:
- Supports treating routing as a learnable control problem rather than a fixed
  "always best model" policy.
- Provides a future path for Goni Lab traces to train routing policies.
Used in:
- `blueprint/50-evidence/eval/EVID-ROUTE-01-frugal-sovereign-routing.md`
Source:
- https://sky.cs.berkeley.edu/project/routellm/

Key: [[routerbench2024]]
Claim: Multi-LLM routing needs systematic evaluation because no single model is
optimal across all tasks and cost-quality points.
Relevance:
- Grounds Goni's evidence lane for measuring correct local routing, late
  escalation, wasted cloud calls, and Council value.
Used in:
- `blueprint/50-evidence/eval/EVID-ROUTE-01-frugal-sovereign-routing.md`
Source:
- https://huggingface.co/papers/2403.12031

Key: [[liu2023-lost-middle]]
Claim: Long-context LMs show positional sensitivity; evidence in the middle is
used less reliably than evidence near prompt boundaries.
Relevance:
- Motivates bounded, curated context projection instead of transcript growth.
- Supports the TXT axiom and plane separation for context discipline.
Used in:
- `blueprint/software/50-data/10-axioms-and-planes.md` (Empirical motivation)

Key: [[lewis2020-rag]]
Claim: Retrieval-augmented generation separates parametric model memory from
non-parametric retrieved evidence, improving knowledge-intensive generation
and making retrieved sources part of the generation path.
Relevance:
- Supports external, updateable memory rather than storing personal knowledge
  only in model weights.
- Grounds Goni's Work Order driven retrieval plane.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/2005.11401

Key: [[reimers2019-sbert]]
Claim: Sentence-BERT uses siamese/triplet structures to produce sentence
embeddings that can be compared efficiently for semantic similarity search.
Relevance:
- Supports dense semantic retrieval over user-owned chunks.
- Distinguishes meaning search from exact keyword lookup.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/1908.10084

Key: [[karpukhin2020-dpr]]
Claim: Dense Passage Retrieval shows learned dual-encoder representations can
serve as effective passage retrievers for open-domain QA.
Relevance:
- Supports dense retrieval as a practical memory-access primitive.
- Helps motivate reranking and retrieval evaluation as first-class concerns.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/2004.04906

Key: [[kleppmann2019-local-first]]
Claim: Local-first software treats local device state as primary while preserving
collaboration and sync, improving ownership, offline use, privacy, longevity,
and user control compared with server-primary cloud apps.
Relevance:
- Grounds Goni's local-first memory ownership and minimized remote context
  transfer.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://www.inkandswitch.com/essay/local-first/

Key: [[deng2023-proactive-dialogue]]
Claim: Proactive dialogue systems study agents that guide interaction toward
goals rather than only responding passively.
Relevance:
- Supports Goni's default of resolving memory context and open loops as system
  work when policy allows.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://www.ijcai.org/proceedings/2023/738

Key: [[hu2025-memoryagentbench]]
Claim: Memory-agent evaluation should cover accurate retrieval, test-time
learning, long-range understanding, and selective forgetting in incremental
multi-turn settings.
Relevance:
- Supports treating memory lifecycle, updates, and forgetting as evaluated
  system behavior rather than informal chat history.
Used in:
- `blueprint/30-specs/memory-retrieval.md` (Evidence anchors)
Source:
- https://arxiv.org/abs/2507.05257

Key: [[ntia2024-open-model-weights]]
Claim: Widely available model weights can broaden participation, decentralize
market control, and enable use without sharing data with third parties, while
also creating monitoring, misuse, oversight, and accountability challenges.
Relevance:
- Grounds the distinction between decentralized access to weights and unresolved
  governance over deployment, monitoring, and trust.
Used in:
- `blueprint/30-specs/model-registry.md` (Scientific framing)
Source:
- https://www.ntia.gov/programs-and-initiatives/artificial-intelligence/open-model-weights-report

Key: [[cyclonedx-mlbom]]
Claim: CycloneDX ML-BOM represents models, datasets, dependencies, dataset
provenance, training methodologies, and AI framework configuration for
transparency and risk assessment.
Relevance:
- Supports treating model provenance as machine-readable supply-chain metadata.
- Provides a basis for making ML-BOM data an input to local policy checks.
Used in:
- `blueprint/30-specs/model-registry.md` (Evaluation limits)
Source:
- https://cyclonedx.org/capabilities/mlbom/

Key: [[mitchell2019-model-cards]]
Claim: Model cards document intended use, evaluation procedures, performance
characteristics, and limitations of trained models.
Relevance:
- Supports requiring model card references in approved bundle manifests.
Used in:
- `blueprint/30-specs/model-registry.md` (Bundle manifest)
Source:
- https://arxiv.org/abs/1810.03993

Key: [[gebru2021-datasheets]]
Claim: Datasheets for datasets document dataset motivation, composition,
collection, recommended uses, distribution, maintenance, and other lifecycle
information to improve transparency and accountability.
Relevance:
- Supports recording dataset-lineage references when known.
Used in:
- `blueprint/30-specs/model-registry.md` (Bundle manifest)
Source:
- https://arxiv.org/abs/1803.09010

Key: [[slsa-framework]]
Claim: SLSA defines incrementally adoptable supply-chain controls and assurance
levels to prevent tampering and improve artifact integrity.
Relevance:
- Supports Goni's graded assurance model for model artifacts.
Used in:
- `blueprint/30-specs/model-registry.md` (Evaluation limits)
Source:
- https://slsa.dev/

Key: [[in-toto-framework]]
Claim: in-toto records and verifies supply-chain steps, actors, order, and
artifacts so users can inspect how a product moved from initiation to
installation.
Relevance:
- Supports signed attestation chains for model promotion and evaluation
  receipts.
Used in:
- `blueprint/30-specs/model-registry.md` (Evaluation limits)
Source:
- https://in-toto.io/

Key: [[spdx-overview]]
Claim: SPDX is an open standard for communicating bill-of-materials information
including provenance, license, security, and related supply-chain metadata.
Relevance:
- Supports license and provenance exchange for model bundle manifests.
Used in:
- `blueprint/30-specs/model-registry.md` (Evaluation limits)
Source:
- https://spdx.dev/about/overview/

Key: [[greshake2023-indirect-prompt-injection]]
Claim: Untrusted retrieved text can inject instructions that hijack tool use
and control flow in LLM-integrated systems.
Relevance:
- Motivates separating untrusted text from control/execution planes.
- Supports redaction and minimization before remote escalation.
Used in:
- `blueprint/software/50-data/40-privacy-and-text-confinement.md` (Empirical motivation)

Key: [[tworek2026-decoder]]
Claim: Reported view that deployed models do not robustly learn from mistakes in
normal use, and that this limits autonomous improvement without system support.
Relevance:
- Motivates explicit, system-managed learning layers and failure recovery.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Constraints and rationale)

Key: [[apple2025-illusion-thinking]]
Claim: Evaluation suggests accuracy can collapse as problem complexity grows,
with reasoning effort rising then dropping despite remaining budget.
Relevance:
- Motivates complexity guardrails and unstuck primitives in runtime design.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Engineering constraint)

Key: [[comment2025-illusion-thinking]]
Claim: Critique argues that some collapse results may be artifacts of evaluation
design (token limits, misclassification, or unsatisfiable instances).
Relevance:
- Supports a conservative stance: treat collapse as a risk, validate in-house.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Risk framing)

Key: [[anderson1972-reference-monitor]]
Claim: Defines core properties of reference validation mechanisms/reference
monitor style enforcement: always invoked, tamper resistant, and analyzable.
Relevance:
- Grounds "agentic kernel" as a minimal mediation core, not a large framework.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Kernel properties)
Source:
- https://csrc.nist.rip/publications/history/ande72.pdf

Key: [[saltzer1975-protection]]
Claim: Establishes protection design principles including least privilege and
economy of mechanism.
Relevance:
- Supports capability scoping and small TCB design goals.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Capabilities)
Source:
- https://www.cl.cam.ac.uk/teaching/1011/R01/75-protection.pdf

Key: [[lampson1974-protection]]
Claim: Classic formulation of protection models and access-control structure in
operating systems.
Relevance:
- Frames authority mediation and protection domains for tool execution.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Mechanisms)
Source:
- https://dl.acm.org/doi/pdf/10.1145/775265.775268

Key: [[watson2010-capsicum]]
Claim: Demonstrates practical capability mode for compartmentalization in a
UNIX-like OS.
Relevance:
- Concrete precedent for capability-based confinement in familiar OS settings.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Capabilities)
Source:
- https://www.usenix.org/legacy/event/sec10/tech/full_papers/Watson.pdf

Key: [[miller2003-capability-myths]]
Claim: Clarifies object-capability security myths and delegation/attenuation
semantics.
Relevance:
- Supports explicit authority transfer model for agent tool actions.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Capabilities)
Source:
- https://classpages.cselabs.umn.edu/Fall-2021/csci5271/papers/SRL2003-02.pdf

Key: [[watson2014-cheri]]
Claim: Introduces CHERI capability extensions for hardware-supported memory and
authority safety.
Relevance:
- Future hardening path for capability enforcement below OS layer.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Capabilities)
Source:
- https://murdoch.is/papers/cl14cheriisa.pdf

Key: [[zeldovich2006-histar]]
Claim: Presents OS design with explicit information-flow control and small
trusted computing base goals.
Relevance:
- Maps directly to policy-gated declassification and egress confinement.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (IFC)
Source:
- https://www.scs.stanford.edu/~nickolai/papers/zeldovich-histar.pdf

Key: [[krohn2007-flume]]
Claim: Shows decentralized information-flow control integrated with standard OS
abstractions.
Relevance:
- Practical precedent for IFC in processes, pipes, and sockets.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (IFC)
Source:
- https://pdos.csail.mit.edu/papers/flume-sosp07.pdf

Key: [[klein2009-sel4]]
Claim: Demonstrates machine-checked functional correctness for a microkernel.
Relevance:
- Strong exemplar for "small trusted core" and verifiability claims.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Citation key list)
Source:
- https://www.sigops.org/s/conferences/sosp/2009/papers/klein-sosp09.pdf

Key: [[kwon2023-vllm]]
Claim: Identifies KV-cache memory management as central to LLM serving
throughput and introduces PagedAttention to reduce fragmentation.
Relevance:
- Justifies treating KV-cache residency as a first-class scheduling concern.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Scheduling)
Source:
- https://arxiv.org/pdf/2309.06180

Key: [[w3c2013-prov]]
Claim: Provides interoperable provenance data model (entities, activities,
agents) for describing derivation and accountability.
Relevance:
- Formal anchor for "receipts as structured provenance."
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Provenance)
Source:
- https://www.w3.org/TR/prov-overview/

Key: [[w3c2021-trace-context]]
Claim: Standardizes trace correlation headers across distributed systems.
Relevance:
- Anchor for end-to-end action correlation across kernel components.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Provenance)
Source:
- https://www.w3.org/TR/trace-context/

Key: [[agache2020-firecracker]]
Claim: Describes microVM design with reduced attack surface and low overhead
for multi-tenant serverless workloads.
Relevance:
- Anchor for isolation-boundary tradeoffs (container vs microVM).
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role A)
Source:
- https://www.usenix.org/system/files/nsdi20-paper-agache.pdf

Key: [[nist-reference-monitor-glossary]]
Claim: NIST glossary definition of reference monitor properties and concept.
Relevance:
- Standards-style wording for complete mediation and tamper resistance.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role B)
Source:
- https://csrc.nist.gov/glossary/term/reference_monitor

Key: [[roughgarden2020-eip1559]]
Claim: Mechanism-design analysis of Ethereum transaction fees with EIP-1559.
Relevance:
- Grounds fee-policy preflight and max-fee constraint reasoning.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Intent pipeline)
Source:
- https://arxiv.org/abs/2012.00854

Key: [[eip2335-keystore]]
Claim: Defines interoperable keystore format for BLS12-381 private keys used
in Ethereum validator contexts.
Relevance:
- Reference point for signer-keystore handling conventions.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Intent pipeline)
Source:
- https://eips.ethereum.org/EIPS/eip-2335

Key: [[haber1991-timestamp]]
Claim: Introduces cryptographic digital timestamping with chained commitments.
Relevance:
- Basis for tamper-evident receipt anchoring semantics.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role C)
Source:
- https://www.staroceans.org/e-book/Haber_Stornetta.pdf

Key: [[bayer1993-timestamp]]
Claim: Improves timestamping efficiency and reliability using Merkle-tree
batching.
Relevance:
- Supports scalable commitment anchoring for receipt logs.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role C)
Source:
- https://www.math.columbia.edu/~bayer/papers/Timestamp_BHS93.pdf

Key: [[rfc3161-tsp]]
Claim: Internet standard for trusted timestamping via Time-Stamp Protocol.
Relevance:
- Alternate anchoring path when PKI timestamp authority is preferred.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role C)
Source:
- https://www.ietf.org/rfc/rfc3161.txt

Key: [[gipp2015-btc-timestamp]]
Claim: Demonstrates decentralized trusted timestamping by anchoring to Bitcoin.
Relevance:
- Concrete example of blockchain-based commitment anchoring.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role C)
Source:
- https://arxiv.org/pdf/1502.04015

Key: [[ethereum-consensus-validator]]
Claim: Ethereum consensus spec guidance for honest validator behavior and
slashing-protection constraints.
Relevance:
- Grounds validator signing refusal invariants in Role D framing.
Used in:
- `blueprint/20-system/45-kernel-blockchain-mapping.md` (Role D)
Source:
- https://ethereum.github.io/consensus-specs/specs/phase0/validator/

Key: [[gu2021-s4]]
Claim: Structured state-space sequence models can handle long-range dependencies
efficiently and compete on long-sequence benchmarks.
Relevance:
- Baseline for evaluating non-attention long-sequence architectures.
Used in:
- `blueprint/20-system/30-performance.md` (scale and novelty criteria)
Source:
- https://arxiv.org/abs/2111.00396

Key: [[peng2023-rwkv]]
Claim: RWKV combines transformer-style training parallelism with recurrent
inference and reports scaling to large model sizes.
Relevance:
- Evidence that architecture claims should be evaluated at practical scale.
Used in:
- `blueprint/20-system/30-performance.md` (tokenization and scale realism)
Source:
- https://arxiv.org/abs/2305.13048

Key: [[poli2023-hyena]]
Claim: Hyena hierarchy proposes sub-quadratic implicit operators and reports
competitive language modeling and long-context behavior.
Relevance:
- Anchor for crossover analysis vs optimized attention.
Used in:
- `blueprint/20-system/30-performance.md` (runtime realism and novelty bar)
Source:
- https://arxiv.org/abs/2302.10866

Key: [[dao2022-flashattention]]
Claim: IO-aware exact attention implementation provides substantial practical
speed/memory gains without changing attention semantics.
Relevance:
- Shows wall-clock can dominate asymptotic arguments in practical regimes.
Used in:
- `blueprint/20-system/30-performance.md` (big-O vs wall-clock section)
Source:
- https://arxiv.org/abs/2205.14135

Key: [[sun2023-retnet]]
Claim: RetNet proposes a retentive architecture intended as an alternative
foundation sequence model with favorable inference properties.
Relevance:
- Expands prior-art set beyond attention and pure SSM families.
Used in:
- `blueprint/20-system/30-performance.md` (novelty bar section)
Source:
- https://arxiv.org/abs/2307.08621

Key: [[nguyen2023-hyenadna]]
Claim: HyenaDNA reports efficient long-range genomic sequence modeling and
large context handling.
Relevance:
- Example of long-context evaluation beyond toy text benchmarks.
Used in:
- `blueprint/20-system/30-performance.md` (acceptance checklist)
Source:
- https://proceedings.neurips.cc/paper_files/paper/2023/file/86ab6927ee4ae9bde4247793c46797c7-Paper-Conference.pdf

Key: [[wang2024-mmneedle]]
Claim: MMNeedle benchmarks long-context retrieval/use in multimodal settings
and highlights non-trivial failure modes.
Relevance:
- Supports need for task-level long-context evaluation beyond window length.
Used in:
- `blueprint/20-system/30-performance.md` (checklist and product implications)
Source:
- https://arxiv.org/abs/2406.11230

Key: [[horvitz1999-mixed-initiative]]
Claim: Mixed-initiative interfaces should decide when to automate vs interrupt
the user using uncertainty and utility/cost tradeoffs.
Relevance:
- Basis for "auto unless risky" delegation thresholds.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-01)
Source:
- https://www.microsoft.com/en-us/research/publication/principles-mixed-initiative-user-interfaces/

Key: [[bradshaw2004-adjustable-autonomy]]
Claim: Human-agent teamwork needs adjustable autonomy with policy constraints
and explicit authority boundaries.
Relevance:
- Grounds corridor-based delegation (`no_go`, `soft_gate`, `autopilot`).
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-02)
Source:
- https://www.researchgate.net/publication/2914496_Human-Agent_Teamwork_and_Adjustable_Autonomy_in_Practice

Key: [[shneiderman2020-hcai-thci]]
Claim: Human-centered AI should target high automation and high human control.
Relevance:
- Supports policy-level governance over per-action click workflows.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-03)
Source:
- https://aisel.aisnet.org/thci/vol12/iss3/1/

Key: [[shneiderman2020-hcai-ijhci]]
Claim: Reliable, safe, and trustworthy AI requires strong user authority and
inspectable controls.
Relevance:
- Supports policy-and-anomaly-first operator UX contract.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-03)
Source:
- https://arxiv.org/abs/2002.04087

Key: [[clark1998-extended-mind]]
Claim: Cognitive processes can extend into external artifacts when coupling is
stable and reliable.
Relevance:
- Conceptual basis for personal AI as extended cognition.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-04)
Source:
- https://academic.oup.com/analysis/article-abstract/58/1/7/153111

Key: [[risko2016-cognitive-offloading]]
Claim: Cognitive offloading improves performance but can shift cognitive load
and behavior over time.
Relevance:
- Grounds offloading safeguards and longitudinal risk monitoring.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-04)
Source:
- https://www.cell.com/trends/cognitive-sciences/fulltext/S1364-6613(16)30098-5

Key: [[haegner2020-rpa-slr]]
Claim: Routine, structured digital work is broadly automatable with RPA.
Relevance:
- Evidence for SOP-first automation in admin-heavy task classes.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-05)
Source:
- https://www.fh-wedel.de/fileadmin/Mitarbeiter/Records/Haegner_2020_-_Robotic_Process_Automation_-_A_Systematic_Literature_Review.pdf

Key: [[smagul2023-rpa-review]]
Claim: RPA adoption patterns consistently move humans from execution to
exception handling.
Relevance:
- Supports anomaly-first oversight design in Goni.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-05)
Source:
- https://ceur-ws.org/Vol-3966/W2Paper2.pdf

Key: [[guner2020-rpa-capability]]
Claim: Organizations gain more from automation when routine capability is
institutionalized instead of ad-hoc scripting.
Relevance:
- Supports reusable SOP packs over one-off manual automations.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-05)
Source:
- https://aisel.aisnet.org/ecis2020_rp/153/

Key: [[gaffinet2025-human-digital-twin]]
Claim: Human digital twin literature converges on modeling + decision support
loops, but concepts need disambiguation.
Relevance:
- Grounds Goni positioning as a local, user-owned personal twin.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-06)
Source:
- https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4911522

Key: [[zafar2024-hdt-business-review]]
Claim: Human digital twin applications span sectors and highlight governance and
ownership concerns.
Relevance:
- Supports explicit policy, portability, and audit commitments.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-06)
Source:
- https://lutpub.lut.fi/bitstream/handle/10024/166844/MastersThesis_Zafar_MuhammadHarisShahid.pdf

Key: [[tomasev2026-intelligent-delegation]]
Claim: Delegation should be treated as adaptive task allocation with explicit
authority, accountability, role boundaries, and intent clarity rather than as
simple handoff heuristics.
Relevance:
- Grounds Goni's policy-first view of delegation engineering and visible intent
  repair.
Used in:
- `blueprint/30-specs/delegation-and-autonomy.md` (delegation-engineering contract)
- `blueprint/20-system/50-learning-loop.md` (delegation-policy bundles)
Source:
- https://arxiv.org/abs/2602.11865

Key: [[zhang2025-ace]]
Claim: Contexts can be evolved as structured playbooks, allowing systems to
improve via curated context updates rather than weight changes.
Relevance:
- Supports delegation policy bundles and context-first repair of vague intent.
Used in:
- `blueprint/30-specs/delegation-and-autonomy.md` (delegation-engineering contract)
- `blueprint/20-system/50-learning-loop.md` (patch seam rationale)
Source:
- https://arxiv.org/abs/2510.04618

Key: [[yang2025-contextagent]]
Claim: Proactive agents benefit from richer contextual signals and explicit
benchmarks for deciding when proactive help and tool use are warranted.
Relevance:
- Supports context acquisition on behalf of the user and proactive delegation
  evaluation via trace replay.
Used in:
- `blueprint/30-specs/delegation-and-autonomy.md` (delegation-engineering contract)
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-07)
Source:
- https://arxiv.org/abs/2505.14668

Key: [[grinschgl2023-cognitive-offloading]]
Claim: Cognitive offloading can shift available resources and change
performance under concurrent-task conditions, not merely reduce effort in the
abstract.
Relevance:
- Supports measuring delegation benefits and cognitive-load transfer over time,
  not just immediate task completion.
Used in:
- `blueprint/docs/references/personal-twin-autonomy-map.md` (C-AUTON-08)
Source:
- https://pmc.ncbi.nlm.nih.gov/articles/PMC10728259/
