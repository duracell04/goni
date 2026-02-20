# Bibliography (annotated)

Key: [[liu2023-lost-middle]]
Claim: Long-context LMs show positional sensitivity; evidence in the middle is
used less reliably than evidence near prompt boundaries.
Relevance:
- Motivates bounded, curated context projection instead of transcript growth.
- Supports the TXT axiom and plane separation for context discipline.
Used in:
- `blueprint/software/50-data/10-axioms-and-planes.md` (Empirical motivation)

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
