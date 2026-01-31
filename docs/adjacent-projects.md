# Adjacent Projects - Agent Gateways, Local Runtimes, and AI OS Neighbors

This document captures projects that are adjacent to Goni's scope but sit
outside the "home cluster / distributed inference" focus of
`docs/related-projects.md`.

It is meant to be a fast map of what exists, what it is good at, and where
Goni's "governed kernel + receipts + confinement" approach is distinct.

How this relates to `docs/related-projects.md`:

- `docs/related-projects.md` covers **home clusters + distributed inference**
  prior art (EXO, Cake, prima.cpp, Beowulf, llama.cpp).
- This document covers **ecosystem neighbors** (runtimes, apps, gateways, and
  "AI OS" framing) and the system/product insights they imply.

---

## 1. Key insights (systems-level)

- "AI OS" splits into four real categories: runtime/server, local AI desktop app,
  agent gateway, and agent OS research. Goni is aiming for runtime + governance
  + operator appliance UX, which is rarer.
- Agent gateways (OpenClaw, Open Interpreter) are strongest at integrations and
  "doing things," but they are not OS-style governance layers.
- Local runtime servers (LM Studio, Ollama, LocalAI) are good backends for an
  `llm-runtime` abstraction, but they do not provide tool governance.
- Desktop "local AI apps" (Jan, GPT4All, AnythingLLM, Open WebUI) prioritize UX
  and convenience over kernel-style constraints.
- "Agent OS" research (AIOS) overlaps on scheduling and memory ideas, but it
  typically lacks Goni's data-plane constitution, receipts, and confinement.
- OpenClaw and LM Studio are complementary, not substitutes: gateway/tool seats
  vs local inference runtime.
- Goni's differentiator is governance: capability-scoped side effects, audited
  receipts tied to state snapshots, and text confinement.
- The local-first shift is driven more by latency, privacy, and memory bandwidth
  than by chasing peak benchmark scores; cloud remains an explicit escalation.
- Memory bandwidth and capacity are often harder constraints than peak TOPS.
- The "PC moment" for local AI is a sub-2000 box that handles 80-90% of daily
  operator tasks offline, with cloud as a logged exception path.
- Agent platforms increase attack surface; OS-style confinement and explicit
  egress syscalls become mandatory for safety.
- The best near-term appliance stack is hybrid: local runtime + governed tool
  layer + optional gateway adapters treated as untrusted seats.
- "Environment, not a chatbot" positioning (Harborne) shows a strong narrative
  around continuity, executive roles, and ROI framing that Goni can ground with
  receipts and confinement.

---

## 2. Mapping OpenClaw and LM Studio to Goni

### OpenClaw (agent gateway + channel integrations)

What it is:

- A self-hosted agent gateway that integrates chat channels and tools.
- A practical "operator front door" with messaging and browser automation.

Where it maps in Goni terms:

- Closest to "gateway + tool seats + channel adapters."
- Not a kernel-governed plane model (no receipts, confinement, or capability
  syscalls as first-class primitives in the public framing).

Links:

- https://openclaw.ai/
- https://docs.openclaw.ai/

### LM Studio (local runtime + API surface)

What it is:

- A local model runtime with OpenAI-compatible endpoints and a native REST API.
- A developer-friendly way to run and manage local models.

Where it maps in Goni terms:

- Closest to "llm-runtime" + model manager.
- Not an agent system; governance is owned by the caller.

Links:

- https://lmstudio.ai/
- https://lmstudio.ai/docs/developer/openai-compat
- https://lmstudio.ai/docs/developer/rest

---

## 3. Adjacent categories and examples

### A) Local AI apps (UX-first)

- Jan: https://jan.ai/
- GPT4All: https://github.com/nomic-ai/gpt4all
- AnythingLLM: https://anythingllm.com/
- Open WebUI: https://github.com/open-webui/open-webui

These projects are valuable for UI and workflows, but do not usually provide
kernel-level governance or audited tool execution.

### B) Local runtime servers (backend-first)

- Ollama: https://github.com/ollama/ollama
- LocalAI: https://localai.io/

These are strong candidates for `llm-runtime` backends, but still need
Goni's scheduling, budgets, and receipts on top.

### C) Agent gateways and operator shells

- OpenClaw: https://openclaw.ai/
- Open Interpreter: https://github.com/openinterpreter/open-interpreter

These are integration-heavy and "do things" well, but carry higher risk without
capability-gated syscalls and audit trails.

### D) "Agent OS" research framing

- AIOS: https://github.com/agiresearch/AIOS

Overlaps on scheduling and memory ideas, but does not define a data plane or
confinement story as a core contract.

### E) Agentic automation + executive-team positioning (Harborne)

- Harborne (Power5 / CoreBrain / JT1): https://harborne.ai/
- Agentic AI solution page (ROI framing): https://harborne.ai/agentic-ai/

What to learn:

- Lead with "environment, not chatbot" and "decisions connect over time".
- "Executive team" role bundles (CEO/CFO/Legal/etc.) compress UX and intent.
- ROI narratives (time/cost saved) are adoption accelerators when paired with
  an explicit calculator.

What to avoid:

- Big security and capability claims without verifiable enforcement or receipts.

---

## 4. Personal/local AI OS candidates (OS framing)

Closest to "real OS" (installable system or explicit OS substrate):

- Olares: https://github.com/beclab/Olares
- pAI-OS: https://paios.org/
- AIOS (agent OS research): https://github.com/agiresearch/AIOS

OS-like in marketing, but closer to agent appliance/platform:

- OpenClaw / Moltbot (Clawdbot): https://openclaw.ai/

OS-like framing, but not local-first:

- Warmwind OS: https://about.warmwind.space/we-built-an-operating-system-for-ai-but-is-it-really-one/

---

## 5. Local-first shift: hardware and economics signals

Key drivers (system-level, not vendor-specific):

- Latency and privacy push personal workloads toward local inference.
- Memory bandwidth often limits LLM throughput more than peak FLOPS.
- The economic unit shifts from "GPU hour" to "device amortization."
- Cloud remains a logged escalation path for hard or fresh-web tasks.

Vendor spec anchors (illustrative, not requirements):

- Apple MacBook Pro specs: https://www.apple.com/macbook-pro/specs/
- AMD Ryzen AI 300 series: https://www.amd.com/en/partner/articles/ryzen-ai-300-series-processors.html
- NVIDIA Jetson Orin: https://www.nvidia.com/en-us/autonomous-machines/embedded-systems/jetson-orin/

---

## 6. Productivity anchors (primary research + standards)

These references are restricted to peer-reviewed primary sources, standards,
or RFCs, with DOI or canonical PDF links. They backstop Goni primitives
(SLOs, interrupts, capabilities, receipts, confinement, Council, lab eval)
without claiming implementation.

### 6.1 Local-first and collaboration invariants

What to add:
- Frame local-first behavior as "offline-first plus convergent state" using
  strong eventual consistency; collaboration is safe when replicas can diverge
  and still converge deterministically.

Why it matters:
- Gives a primary academic foundation for local-first collaboration and
  conflict resolution (CRDTs) that aligns with Arrow spine + confinement.

Source anchors:
- Shapiro et al., "Conflict-Free Replicated Data Types" (SSS 2011).
  DOI: https://doi.org/10.1007/978-3-642-24550-3_29

### 6.2 SLOs and tail latency as percentiles

What to add:
- Define latency SLOs as percentiles (p95/p99) for TTFT and cancellation, and
  treat worst-case latency as a first-class design target.

Why it matters:
- Interactive systems feel "broken" because of tail latency; interrupts and
  budgets are the mitigation.

Source anchors:
- Dean & Barroso, "The Tail at Scale" (CACM 2013).
  DOI: https://doi.org/10.1145/2408776.2408794
- Li et al., "Tales of the Tail" (SoCC 2014).
  DOI: https://doi.org/10.1145/2670979.2670988

### 6.3 LLM serving as OS-style memory management

What to add:
- Cite PagedAttention / vLLM as precedent for treating KV cache like virtual
  memory and for schedulers that can preempt requests.

Why it matters:
- Supports the "LLM calls as interrupts" framing and utilization reports as a
  principled systems approach.

Source anchors:
- PagedAttention / vLLM paper (arXiv 2023).
  DOI: https://doi.org/10.48550/arXiv.2309.06180

### 6.4 Capability security + reference monitor lineage

What to add:
- Describe Tool Capability API + Network Gate as a reference monitor:
  complete mediation, tamper resistance, and a small trusted core.

Why it matters:
- Makes "no ambient permissions" academically legible and historically grounded.

Source anchors:
- Anderson, "Computer Security Technology Planning Study" (ESD-TR-73-51).
  PDF: https://csrc.nist.rip/publications/history/ande72.pdf
- Watson et al., "Capsicum: Practical Capabilities for UNIX" (USENIX Sec 2010).
  PDF: https://www.usenix.org/legacy/event/sec10/tech/full_papers/Watson.pdf

### 6.5 Provenance standardization for receipts

What to add:
- Map receipts to W3C PROV concepts (Entity/Activity/Agent; derivation;
  attribution).

Why it matters:
- Makes receipts interoperable and gives standard terminology for audit and
  traceability.

Source anchors:
- W3C PROV-DM (W3C Recommendation).
  https://www.w3.org/TR/prov-dm/

### 6.6 Tamper-evident audit logs

What to add:
- A "receipt integrity" option: append-only log commitments (Merkle tree) so
  operators can verify logs were not rewritten.

Why it matters:
- For an operator appliance, integrity of receipts is as important as their
  existence.

Source anchors:
- RFC 6962 (Certificate Transparency).
  https://www.rfc-editor.org/rfc/rfc6962

### 6.7 Human-AI interaction guidelines

What to add:
- An interaction contract: show progress, allow correction/undo, communicate
  uncertainty, request approval at the right moments, keep user in control.

Why it matters:
- Provides a peer-reviewed UX rubric for Action Cards, approvals, and receipts.

Source anchors:
- Amershi et al., "Guidelines for Human-AI Interaction" (CHI 2019).
  DOI: https://doi.org/10.1145/3290605.3300233

### 6.8 Evaluation discipline for RAG and hallucination

What to add:
- Adopt a standard evaluation loop for retrieval + grounding quality
  (faithfulness, context relevance, answer relevance) and treat hallucination
  risk as an evaluated property.

Why it matters:
- Strengthens "measured routing improvements" with explicit methodology.

Source anchors:
- Lewis et al., "Retrieval-Augmented Generation" (arXiv 2020).
  DOI: https://doi.org/10.48550/arXiv.2005.11401
- Es et al., "RAGAS" (arXiv 2023).
  DOI: https://doi.org/10.48550/arXiv.2309.15217
- Ji et al., "Survey of Hallucination in Natural Language Generation" (arXiv 2023).
  DOI: https://doi.org/10.48550/arXiv.2311.05232

### 6.9 Trustworthiness governance

What to add:
- Map Goni controls to a lifecycle AI governance framework (risk mgmt,
  monitoring, transparency, incident handling).

Why it matters:
- Provides a compliance-ready narrative without overstating regulatory status.

Source anchors:
- NIST AI RMF 1.0 (NIST standard).
  https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf

### 6.10 Privacy by design (PbD)

What to add:
- Frame confinement + minimized retention as PbD principles (privacy by
  default, data minimization, end-to-end lifecycle protection).

Why it matters:
- Makes the confinement story legible to privacy and policy audiences.

Source anchors:
- Cavoukian, "Privacy by Design" (foundational principles).
  https://www.ipc.on.ca/wp-content/uploads/Resources/7foundationalprinciples.pdf

### 6.11 Proactivity as mixed-initiative control

What to add:
- Define "proactive" explicitly: policy-governed background cognition plus
  attention-aware interventions. Everything proactive is scheduled; every
  intervention has an expected-value justification; every action has receipts.

Why it matters:
- Proactivity is not "more automation"; it is initiative under uncertainty
  with user control.

Source anchors:
- Horvitz, "Principles of Mixed-Initiative User Interfaces" (CHI 1999).
  DOI: https://doi.org/10.1145/302979.303030

### 6.12 Interruption management (when to interrupt, defer, bundle)

What to add:
- Proactivity is budgeted and attention-aware: suggestions are queued,
  bundled into a Daily Brief, and only escalate to interrupts when expected
  value exceeds expected attention cost.

Why it matters:
- A proactive operator that interrupts at the wrong time is less productive.

Source anchors:
- Horvitz & Apacible, "Learning and Reasoning about Interruption" (ICMI 2003).
  DOI: https://doi.org/10.1145/958432.958440
- Horvitz, Koch, Apacible, "BusyBody" (CSCW 2004).
  PDF: https://interruptions.net/literature/Horvitz-CSCW04-p507-horvitz.pdf

### 6.13 Overload and queueing discipline

What to add:
- Background work is opportunistic and preemptible; interactive QoS is
  protected with strict head-of-line blocking controls.

Why it matters:
- Proactive background work creates queues; without backpressure, interactive
  latency collapses.

Source anchors:
- Little, "A Proof for the Queueing Formula: L = lambda W" (Operations Research, 1961).
  DOI: https://doi.org/10.1287/opre.9.3.383

### 6.14 End-to-end arguments (where to enforce guarantees)

What to add:
- Explicitly position kernel boundaries so guarantees (no ambient side
  effects, no unlogged egress, confinement) are enforceable rather than
  best-effort.

Why it matters:
- End-to-end arguments justify enforcement at the right boundary: tool
  syscalls, network gate, and Council mediation.

Source anchors:
- Saltzer, Reed, Clark, "End-to-End Arguments in System Design" (ACM TOCS, 1984).
  DOI: https://doi.org/10.1145/357401.357402

### 6.15 Secure design principles checklist

What to add:
- A short checklist aligned to least privilege, complete mediation,
  separation of privilege, and psychological acceptability.

Why it matters:
- Provides a canonical security-design lens that maps to capability tokens,
  audit receipts, and user-visible approvals.

Source anchors:
- Saltzer & Schroeder, "The Protection of Information in Computer Systems"
  (Proceedings of the IEEE, 1975).
  DOI: https://doi.org/10.1109/PROC.1975.9939

### 6.16 Usability heuristics for an operator cockpit

What to add:
- Explicit UX heuristics: visibility of system status, user control/undo,
  error prevention, and recognition over recall.

Why it matters:
- Grounds Action Cards and approvals in a peer-reviewed usability method.

Source anchors:
- Nielsen & Molich, "Heuristic Evaluation of User Interfaces" (CHI 1990).
  DOI: https://doi.org/10.1145/97243.97281
