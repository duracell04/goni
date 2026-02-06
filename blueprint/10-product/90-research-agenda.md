3) Queueing theory & performance engineering (tail latency)

Purpose:
- Define the research questions and measurements needed to make tail latency a
  first-class design constraint for Goni's interactive-first behavior.

Research questions (academically framed):
- What queueing model best fits mixed interactive/background LLM workloads
  (e.g., heavy-tailed service times, bursty arrivals, and long-tail prompts)?
- How do different scheduling policies (priority, preemptive, aging, EDF)
  affect p95/p99 TTFT and end-to-end latency under realistic load?
- What is the minimal admission-control policy that preserves interactive SLOs
  while maximizing background throughput?
- How does context length distribution (prefill size) reshape the tail, and
  which chunking strategies actually reduce head-of-line blocking?
- What are the measurable interactions between KV cache pressure, queue depth,
  and latency spikes?
- When does resource partitioning (two pools) outperform single-pool priority?

Core metrics and SLOs:
- TTFT p50/p95/p99 (interactive class)
- End-to-end latency p50/p95/p99 (interactive class)
- Cancellation latency p95/p99 (interrupt effectiveness)
- Queue depth and wait time by class
- Service time distribution (tokens/sec and total tokens) by class
- Admission rate and rejection rate by class
- Tail amplification factor (p99 / p50)

Models and analysis:
- Fit arrival processes and service time distributions (test for heavy tails).
- Evaluate priority M/G/1 and preemptive-resume variants against traces.
- Quantify head-of-line blocking via long-prefill share of total service time.
- Use Little's Law to sanity-check queue size vs latency under steady load.

Experiment design (repeatable):
- Baselines: interactive-only, background-only, mixed steady-state.
- Stress: long-prefill bursts with short interactive requests injected.
- Mixed burst: background backlog + interactive burst with cancellation events.
- Resource partitioning: single pool vs dual pool (interactive/background).

Control mechanisms to evaluate:
- Preemptive priority scheduling (LLM request priority).
- Chunked prefill and long-prefill thresholds.
- Admission control with hysteresis (token bucket or WIP limits).
- Load shedding for background jobs under SLO threat.
- Class-based routing (separate LLM pools).

Evidence artifacts to produce:
- Trace logs with timestamps: arrival, start, first-token, completion, cancel.
- Scenario-specific latency distributions and queue depth timelines.
- A short "tail latency playbook" with recommended defaults and failure modes.

4) Distributed systems & offline-first synchronization (local-first durability)

Purpose:
- Define the research questions and system constraints for a durable, offline-first
  Vault that can later replicate safely across multiple devices.

Research questions (academically framed):
- What consistency guarantees are required per plane (capabilities/keys vs user data),
  and what is the minimal consistency model that preserves safety?
- Which data types benefit from CRDTs vs append-only logs vs transactional storage?
- How do we bound privacy leakage during replication while still achieving convergence?
- What is the cost of conflict resolution under realistic device churn and partitions?
- How does replication affect auditability and receipt integrity over time?

Core data categories (by consistency class):
- Strong: capability grants, device membership, key rotations, policy changes.
- Eventual: notes, summaries, derived embeddings, user artifacts.
- Append-only: receipts, audit logs, execution traces (tamper-evident chain).

Models and analysis:
- Formalize per-object merge semantics (CRDT types or LWW where justified).
- Model partition tolerance vs consistency for each category (CAP tradeoffs).
- Define a replication protocol that can operate without a central coordinator.

Experiment design (repeatable):
- Multi-device simulator: 3–7 nodes, randomized partitions, random edits.
- Chaos scenarios: clock skew, dropped messages, partial replays.
- Measure convergence time, storage bloat, and conflict rate.

Control mechanisms to evaluate:
- Plane-aware replication policy (redaction/minimization before sync).
- Strong-consistency paths for keys/capabilities (quorum or explicit approvals).
- Snapshot + log compaction strategy for long-running devices.

Evidence artifacts to produce:
- Replication traces showing convergence and conflict resolution.
- A "Vault durability" report: data loss bounds, recovery steps, and audit chain health.

5) Databases, IR, and data management (the Vault + receipts)

Purpose:
- Define the research agenda for a Vault that is durable, queryable, and auditable,
  with receipts as first-class provenance objects rather than log lines.

Research questions (academically framed):
- What storage model best preserves auditability while supporting low-latency retrieval?
- How should receipts be modeled to ensure integrity, minimal text retention, and
  provenance completeness?
- What indexing strategy (dense, hybrid, graph) best balances recall and latency
  for personal corpora under tight device budgets?
- How do we guarantee that derived artifacts (summaries, embeddings, decisions)
  can be traced back to source entities without leaking raw text?
- What is the minimal schema that still supports conformance and reproducibility?

Core objects and relations:
- Entities: Docs, Chunks, Embeddings, ContextItems, MemoryEntries, Receipts.
- Activities: ingest, chunk, embed, retrieve, summarize, redact, toolcall.
- Agents: user, kernel module, model provider, tool executor.
- Relations: used, wasGeneratedBy, wasDerivedFrom, wasAttributedTo.

Receipt requirements:
- Immutable, append-only storage with hash chaining.
- Pointers to inputs/outputs (entity IDs) and policy hash.
- Code + model identifiers and parameters used for the transformation.
- Deny/allow decisions for redaction or capability gates.

Retrieval modes to evaluate (same harness):
- Dense ANN (vector DB).
- Hybrid (BM25/FTS + dense with RRF or learned weights).
- Graph traversal (entities/threads/clauses) with budgeted BFS.

Quality + performance metrics:
- Recall@k, MRR, nDCG on synthetic + open demo corpora.
- p50/p95 end-to-end latency with stage breakdown.
- Groundedness proxy: answer spans map to cited chunk IDs.
- Storage overhead for receipts and provenance links.

Evidence artifacts to produce:
- A benchmark report comparing retrieval modes and budgets.
- A receipt integrity report (hash chain verification and completeness checks).

6) ML systems (efficient inference, model routing, compression)

Purpose:
- Define the research agenda for efficient local inference and routing across
  model sizes, quantization levels, and hardware backends.

Research questions (academically framed):
- What routing policy maximizes task quality under tight latency and power budgets?
- How do quantization and KV-cache pressure trade off against TTFT and tail latency?
- What is the minimal model portfolio that achieves target coverage of tasks?
- When does offloading (remote or larger local model) outperform small local models
  given real device constraints and privacy policies?
- How does compression (4-bit, 8-bit, mixed) affect retrieval-augmented tasks?

Core evaluation axes:
- Latency: TTFT, end-to-end, cancellation latency.
- Throughput: tokens/sec per class, concurrency scaling.
- Quality: task success metrics, groundedness where retrieval is used.
- Energy: joules/token or Wh per request (when power data is available).
- Cost: remote escalation frequency vs local token share.

Model portfolio experiments:
- Small local (interactive), medium local (quality), embedder model.
- Quantized vs full-precision variants on the same hardware.
- Context length scaling curves (1k, 4k, 16k, 32k tokens).

Routing policy experiments:
- Local-first with escalation on confidence or latency thresholds.
- Budget-aware routing with per-class caps and hysteresis.
- Priority routing under mixed interactive/background load.

Compression + serving tactics:
- Quantization sweeps (4/5/6/8-bit) with accuracy trade-off curves.
- Speculative decoding or prompt-caching (if supported) for TTFT control.
- Separate pools for interactive vs background to stabilize tail latency.

Evidence artifacts to produce:
- A routing report: quality vs latency vs local-token ratio.
- A compression report: accuracy loss vs latency/energy gains.

7) HCI & cognitive systems (mixed-initiative, interruption science, cockpit design)

Purpose:
- Define the research agenda for mixed-initiative behavior, interruption
  management, and operator cockpit UX that preserves user agency.

Research questions (academically framed):
- What policies minimize interruption cost while preserving task success?
- How do "propose vs act vs defer" thresholds affect user trust and regret?
- What UI cues best convey provenance and confidence without overload?
- How should the system adapt to user focus modes and time constraints?
- What undo/rollback guarantees are required for safe autonomy?

Core signals and policies:
- Interruptibility estimate (0–1), urgency class, and deferrable-until time.
- Focus modes: deep work, normal, idle.
- Initiative levels: notify, propose, act (with explicit permission thresholds).
- Confidence thresholds with hysteresis to reduce oscillation.

Evaluation design:
- Scenario-based user studies with scripted interruptions.
- A/B tests on notification timing and modality.
- Longitudinal "trust calibration" tracking (accept/reject/undo rates).

Metrics to track:
- Interruption cost (task switch time, error rate).
- Action regret rate and time-to-undo.
- Delegation comfort score and opt-out rate.
- Time-to-resolution for action cards.

Evidence artifacts to produce:
- A "proactivity policy" report with measured outcomes.
- Cockpit design rationale mapping UI elements to cognitive goals.

8) Security, privacy, and trustworthy computing (delegation without trust collapse)

Purpose:
- Define the research agenda for enforcing "no ambient authority" and proving
  that delegation remains safe under realistic threat models.

Research questions (academically framed):
- What is the minimal trusted computing base (TCB) for capability mediation?
- How can we prove all outbound effects are mediated and auditable?
- What receipt design provides integrity without leaking sensitive content?
- How should redaction and data classification be enforced at egress boundaries?
- What is the recovery story if a component inside the sandbox is compromised?

Core security objectives:
- Default-deny network for all non-gate components.
- Capability-scoped tool execution with explicit budgets and expiry.
- Tamper-evident, append-only receipt log with verifiable integrity.
- Plane-aware data minimization and redaction before any egress.

Threat model coverage:
- Untrusted tool outputs and prompt-injection via retrieved text.
- Bypass attempts (direct sockets, file exfiltration, side-channel signals).
- Compromised tool sandbox attempting privilege escalation.
- Accidental policy drift via config changes.

Enforcement and verification:
- Single egress gate; all HTTP flows through it.
- Sandboxed tools with no ambient network; only gate has egress.
- Integration tests that prove bypass failures and receipt emission.
- Key rotation and revocation for capability tokens.

Evidence artifacts to produce:
- A security conformance report with bypass test results.
- A receipt integrity report with hash-chain verification.

9) Economics, adoption, and market design (community-first, not marketing)

Purpose:
- Define how a high-trust builder community forms, governs itself, and sustains
  the project without relying on hype or pricing narratives.

Research questions (academically framed):
- What governance structures reduce coordination cost while preserving rigor?
- How do contribution norms (RFCs, receipts discipline, threat reviews) affect
  project quality and trust over time?
- What incentives best attract serious builders without diluting standards?
- How do communities prevent spec drift and maintain enforceable contracts?

Community operating system:
- Clear entry points: COMMUNITY.md, STATUS, and Truth Map.
- Lightweight governance: RFC flow for spec changes and security-sensitive topics.
- Trust ladder: transparent criteria for contributor roles and review authority.
- Receipts discipline: every capability change documents logging, redaction, and
  auditability impacts.

Adoption signals (non-marketing, observable):
- Contributor retention and review throughput.
- Time-to-merge for spec and prototype changes.
- Number of conformance tests added per release.
- Decline in "reality map" drift (STATUS vs actual implementation).

Evidence artifacts to produce:
- A community health report (review throughput, RFC cadence, conformance growth).
- A governance audit log (decisions, rationale, and spec linkage).

10) Reproducible system checklist (must-have vs nice-to-have)

Purpose:
- Define the concrete repo contents that make Goni runnable, measurable, and
  extendable by outsiders without guesswork.

Must-have (to be "real"):
- Single blessed entrypoint: README matches reality and links to STATUS.
- One-command demo: compose + smoke test + deterministic success criteria.
- Clear component boundaries: orchestrator, llm adapter, vector store, telemetry.
- Measurement harness: repeatable TTFT/tok-s benchmarks with JSON output.
- Scheduler v0: interactive vs background, preemption points, metrics.
- Contracts + CI: enforce invariants, smoke test, and minimal perf checks.
- Deployment parity: compose is canonical; k8s only if maintained.

Nice-to-have (next):
- Real embeddings backend with model/version metadata.
- Hardware compatibility matrix (CPU/CUDA/ROCm, known-good configs).
- Security and governance hygiene: SECURITY.md, TCB statement, retention policy.

Minimal MVP contents (lean but credible):
- make demo + smoke.sh
- JobSpec end-to-end
- 2-class scheduler with p95/p99 TTFT report
- Capability enforcement at a single choke point + negative tests
- Receipts/audit log (structured, hash-linked)

11) Serious systems repo structure (interactive-first QoS)

Purpose:
- Define the minimal file/folder structure and acceptance criteria that make
  QoS and tail-latency work reproducible, testable, and debuggable.

Core docs:
- README: problem, thesis, shipped components, measured metrics, 3-command run.
- VISION/THESIS: QoS class definitions, interrupt semantics, non-negotiable SLOs.
- ARCHITECTURE: data/control flow and trust boundaries.
- PERF: hardware profiles, backend flags, golden baselines, regression thresholds.

Scheduler artifacts:
- scheduler spec (QoS classes, WIP limits, hysteresis, preemption points).
- policies (interactive-first, burst protection).
- code module with admission control, priority mapping, cancel/abort propagation.

Benchmarks + observability:
- benchmark scenarios (interactive-only, background-only, mixed, burst).
- runner with JSON/CSV outputs.
- metrics doc + Prometheus/Grafana config.

Deploy layout:
- compose base + observability overlay + .env.example.
- k8s overlays only if maintained; no hidden logic.

Background jobs model:
- job types with schema, cost estimates, preemption points, idempotency rules.

CI guardrails:
- lint/test for all languages.
- bench smoke on PRs; perf regression nightly.

12) Local-first OS readiness (runnable + secure + multi-device growth)

Purpose:
- Define the minimal repo contents to make Goni runnable, secure, and ready
  for offline-first, multi-device evolution without overpromising.

Repo must-haves:
- README with one golden path, .env.example, and quickstart doc.
- Deterministic demo with expected outputs (ingest -> action card -> receipt).
- CONTRIBUTING, issue templates, and clear onboarding.

Core subsystems (first-class modules):
- Vault as source-of-truth (receipts + audit log, vector DB as index only).
- Orchestrator/council with policy-mediated execution.
- Plane enforcement with tests at boundaries.
- Local LLM + embeddings + vector store adapters.

Offline-first + sync foundations:
- Sync module (transport, event log/CRDT model, compaction).
- CRDT specs per artifact type with property tests.
- Plane-aware sync policy matrix + validator.
- Churn simulator for partitions and convergence.

Security essentials:
- SECURITY.md and dependency update automation.
- Device identity + key rotation + revocation semantics.
- Tamper-evident audit log with verification tests.

Specs and proof:
- Architecture index mapping spec -> code -> tests.
- Invariant tests for plane boundaries and append-only receipts.
- Versioning/migration spec for Vault and contracts.

Operational packaging:
- Compose and k8s manifests aligned to the golden path.
- Release tags, changelog, and published images.
- CI smoke test that proves the demo path runs.

13) Vault + receipts: minimum shippable system

Purpose:
- Define the minimal repo structure, artifacts, and CI checks needed for a
  fast, grounded, auditable local-first memory system.

Required structure:
- software/kernel: policy gates, receipt emitter, schema+migrations, goldens.
- software/vault: ingestion, chunking, index layer, context assembly.
- software/retrieval: dense/sparse/hybrid/graph via a unified interface.
- spec: planes, receipts, memory model, redaction profiles, retrieval contracts.
- goni-lab: conformance + benchmark harness with JSON reports.

Core artifacts:
- Receipt/Provenance model (PROV-DM shaped): Receipt, Entity, Activity, Agent.
- Vault tables: Documents, Chunks, Embeddings, SparseIndex, GraphEdges,
  ContextPacks, MemoryEntries, Receipts, RedactionEvents.
- Two enforced gates:
  - Memory write gate (evidence + receipt required; TTL for uncertain entries).
  - Egress/redaction gate (profile + plane rules + manifest required).

Retrieval requirements:
- Sparse baseline (FTS ok), dense embeddings (real, no placeholders), hybrid fusion.
- Optional graph expansion with budget limits.

Benchmarks + evaluation:
- Dataset format + synthetic corpus generator + small open demo corpus.
- Metrics: recall@k, nDCG/MRR, p50/p95 per stage latency.
- One command runner: goni-lab bench ... with deterministic output.

CI invariants:
- TXT boundary lint (no forbidden plane leaks).
- Receipt completeness for any external call or memory write.
- Goldens for memory-write and redaction gates.
- Spec/code sync (schemas/migrations).
- Benchmark smoke on PRs, full suite nightly.

14) Local-first ML kernel scope (what must exist vs what must not)

Purpose:
- Define the minimum repo contents that make the kernel runnable, testable,
  measurable, and extensible, and explicitly exclude half-built surfaces.

Must-have:
- Single blessed run path (README + compose or Make/justfile).
- Kernel contracts: router, scheduler, execution adapters, schemas.
- CI smoke test that exercises stream + non-stream + cancel.
- Measurement harness with CSV/JSON outputs and a tiny eval set.
- Traceable specs (architecture + threat model + routing + scheduler invariants).
- License + contribution hygiene.

Recommended:
- Runtime adapter boundary (vLLM, llama.cpp, etc.) with engine interface.
- Real embeddings path and retrieval budgets if RAG is claimed.
- Roadmap/scope statement to prevent kernel/product creep.

Must-not:
- Half-built frontend without build/CI support.
- Policy claims without enforced gates + tests.
- Infra (k8s/terraform) that isn't validated by CI or users.

15) Operator appliance repo minimum (proactivity + receipts)

Purpose:
- Define the minimal repo contents to make Goni a real operator appliance with
  proactive assistance, receipts, and a runnable demo on a new machine.

Must-have run path:
- make demo + make test wrappers.
- compose stack + .env.example.
- smoke test that asserts kernel endpoint, receipts, and retrieval.
- goni-prototype-lab:demo/demo_expected.md with "what you should see."

Core docs:
- ARCHITECTURE (components, dataflow, boundaries).
- DECISIONS/ (ADRs) + TRACEABILITY/STATUS tied to code/tests.

Operator primitives (schemas + APIs):
- Schemas: events, action_cards, daily_brief, receipts, capabilities, policy.
- APIs: cards list/approve/reject, brief, receipts, policy get/set.

Policy engine:
- initiative ladder (defer/propose/ask/act) with deterministic tests.
- rationale objects included in receipts.

Governance layer:
- capability tokens required for any side effect.
- receipts emitted for tool calls and egress.
- network gate as sole egress path with bypass tests.

Retrieval:
- real embeddings path with metadata.
- retrieval sanity tests (no placeholders).

Cockpit surface (pick one):
- minimal web UI with working build/CI, or
- CLI/TUI that can list/approve cards and inspect receipts.

Evaluation harness:
- simulated event streams with interruption metrics.
- EVALUATION.md with reproduction steps.

16) OS for agency checklist (safe delegation + receipts)

Purpose:
- Define the minimum repo contents that make "OS for agency" guarantees
  enforceable, auditable, and testable.

Guarantees + threat model:
- README: testable guarantees + TCB box.
- docs/threat-model.md and docs/tcb.md.

Enforcement core (small TCB):
- goni-kernel: capabilities, policy eval, budget ledger, receipt emission.
- goni-policy: policy language + reference profiles.
- goni-receipts: schema, signing/commit, verification library.

Non-bypassable mediation:
- egress-gate service as the only outbound path.
- runtime sandbox configs that block ambient network.
- integration tests proving bypass fails and receipts are emitted.

Tools as capabilities:
- tool manifests declaring required authorities.
- structured inputs/outputs with hashes.
- docs/tools.md with authoring + receipt rules.

Receipts (v1):
- receipt schema with hashed IO, capability id, policy decision, budget delta.
- CLI for tail/show/verify receipts.
- docs/receipts.md for interpretation + privacy defaults.

Privacy enforcement:
- classifier + redactor (fail-closed at egress).
- docs/privacy.md with data classes and retention rules.

Run + prove path:
- compose/k8s with secure defaults.
- smoke test validates receipts and blocked direct egress.
- examples showing local, remote via gate, tool call + verification.

Governance:
- SECURITY.md, CONTRIBUTING.md, CODEOWNERS.
- docs/architecture.md + roadmap for enforcement milestones.
