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

## 6. Productivity anchors (academic + standards mapping to Goni primitives)

These are canonical references that backstop Goni's existing primitives
(SLOs, interrupts, capabilities, receipts, confinement, Council, lab eval)
without claiming implementation. This section is meant to strengthen the
"productivity -> OS solutions" narrative with recognisable anchors.

### 6.1 Local-first as a first-class principle

What to add:
- A short "Local-first principles" box: offline utility, user data ownership,
  fast feedback loops, and collaboration patterns even if Goni starts
  single-user.

Why it matters:
- Anchors "on-device by default" in a research-backed framing that lines up
  with Arrow spine + confinement.

Source anchors:
- Ink & Switch, "Local-first software": https://www.inkandswitch.com/essay/local-first/
- PDF: https://www.inkandswitch.com/essay/local-first/local-first.pdf

### 6.2 SLOs and tail latency

What to add:
- Define latency SLOs explicitly as percentiles (p95/p99) for TTFT and
  cancellation, and treat worst-case latency as a design target.

Why it matters:
- Interactive tools feel broken because of tail latency; budgets + interrupts
  are the mitigation.

Source anchors:
- Google SRE book, SLO chapter: https://sre.google/sre-book/service-level-objectives/
- "The Tail at Scale" (for tail-latency framing): https://research.google/pubs/pub40801/

### 6.3 LLM serving as OS-style memory management

What to add:
- Cite PagedAttention / vLLM as precedent for treating KV cache like virtual
  memory and for schedulers that can preempt requests.

Why it matters:
- Supports the "LLM calls as interrupts" framing and the utilization reports
  as a principled systems approach.

Source anchors:
- vLLM / PagedAttention paper: https://arxiv.org/abs/2309.06180

### 6.4 Capability security + reference monitor lineage

What to add:
- Explicitly describe Tool Capability API + Network Gate as a reference monitor
  (complete mediation, tamper resistance, small TCB) with capability systems
  as the OS lineage.

Why it matters:
- Makes "no ambient permissions" academically legible.

Source anchors:
- Anderson, "Computer Security Technology Planning Study" (reference monitor):
  https://csrc.nist.rip/publications/history/ande72.pdf
- Capsicum capability mode (practical UNIX capabilities):
  https://www.usenix.org/legacy/event/sec10/tech/full_papers/Watson.pdf

### 6.5 Provenance standardization for receipts

What to add:
- Map receipts to W3C PROV concepts (Entity/Activity/Agent; derivation;
  attribution).

Why it matters:
- Makes receipts interoperable and gives established terminology for audit
  and traceability.

Source anchors:
- W3C PROV-DM: https://www.w3.org/TR/prov-dm/

### 6.6 Tamper-evident audit logs

What to add:
- A "receipt integrity" option: append-only log commitments (Merkle tree) so
  operators can verify logs were not rewritten.

Why it matters:
- For an operator appliance, integrity of receipts is as important as their
  existence.

Source anchors:
- RFC 6962 (Certificate Transparency): https://www.rfc-editor.org/rfc/rfc6962

### 6.7 Human-AI interaction and mixed-initiative control

What to add:
- An interaction contract: show progress, allow correction/undo, communicate
  uncertainty, request approval at the right moments, keep user in control.

Why it matters:
- Matches Action Cards + approvals + receipts and provides a research-backed UX
  rubric.

Source anchors:
- Amershi et al., "Guidelines for Human-AI Interaction":
  https://dl.acm.org/doi/10.1145/3290605.3300233
- Horvitz, "Mixed-initiative user interfaces":
  https://www.microsoft.com/en-us/research/publication/mixed-initiative-user-interfaces/

### 6.8 Evaluation discipline for RAG and hallucination

What to add:
- Adopt a standard evaluation loop for retrieval + grounding quality
  (faithfulness, context relevance, answer relevance) and treat hallucination
  risk as an evaluated property.

Why it matters:
- Strengthens the "measured routing improvements" claim and makes status-honest
  progress quantifiable.

Source anchors:
- RAG paper: https://arxiv.org/abs/2005.11401
- RAGAS eval framework: https://arxiv.org/abs/2309.15217
- Hallucination survey: https://arxiv.org/abs/2311.05232

### 6.9 Trustworthiness governance

What to add:
- Map Goni controls to a lifecycle AI governance framework (risk mgmt,
  monitoring, transparency, incident handling).

Why it matters:
- Provides a compliance-ready narrative without overstating regulatory status.

Source anchors:
- NIST AI RMF 1.0: https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf

### 6.10 Privacy by design (PbD)

What to add:
- Frame confinement + minimized retention as PbD principles (privacy by
  default, data minimization, end-to-end lifecycle protection).

Why it matters:
- Makes the confinement story legible to policy and privacy audiences.

Source anchors:
- Cavoukian, "Privacy by Design": https://www.ipc.on.ca/wp-content/uploads/Resources/7foundationalprinciples.pdf

### 6.11 Proactivity as mixed-initiative control

What to add:
- Define "proactive" explicitly: policy-governed background cognition plus
  attention-aware interventions. Everything proactive is scheduled; every
  intervention has an expected-value justification; every action has receipts.

Why it matters:
- Proactivity is not "more automation"; it is initiative under uncertainty
  with user control.

Source anchors:
- Horvitz, "Principles of Mixed-Initiative User Interfaces":
  https://www.microsoft.com/en-us/research/publication/principles-mixed-initiative-user-interfaces/

### 6.12 Interruption management (when to interrupt, defer, bundle)

What to add:
- Proactivity is budgeted and attention-aware: suggestions are queued,
  bundled into a Daily Brief, and only escalate to interrupts when expected
  value exceeds expected attention cost.

Why it matters:
- A proactive operator that interrupts at the wrong time is less productive.

Source anchors:
- Horvitz, "Learning and Reasoning about Interruption":
  https://erichorvitz.com/iw.pdf
- Horvitz, "BusyBody" (cost of interruption models):
  https://www.interruptions.net/literature/Horvitz-CSCW04-p507-horvitz.pdf

### 6.13 Overload and queueing discipline

What to add:
- Background work is opportunistic and preemptible; interactive QoS is
  protected with strict head-of-line blocking controls.

Why it matters:
- Proactive background work creates queues; without backpressure, interactive
  latency collapses.

Source anchors:
- Little's Law (queueing fundamentals):
  https://web.eng.ucsd.edu/~massimo/ECE158A/Handouts_files/Little.pdf
- "The Tail at Scale" (tail latency in large systems):
  https://research.google/pubs/the-tail-at-scale/

### 6.14 End-to-end arguments (where to enforce guarantees)

What to add:
- Explicitly position kernel boundaries so guarantees (no ambient side
  effects, no unlogged egress, confinement) are enforceable rather than
  best-effort.

Why it matters:
- End-to-end arguments justify putting enforcement at the right boundary:
  tool syscalls, network gate, and Council mediation.

Source anchors:
- Saltzer, Reed, Clark, "End-to-End Arguments in System Design":
  https://web.mit.edu/saltzer/www/publications/endtoend/endtoend.pdf

### 6.15 Secure design principles checklist

What to add:
- A short checklist aligned to least privilege, complete mediation,
  separation of privilege, and psychological acceptability.

Why it matters:
- Provides a canonical security-design lens that maps directly to capability
  tokens, audit receipts, and user-visible approvals.

Source anchors:
- Saltzer & Schroeder, "The Protection of Information in Computer Systems":
  https://www.cs.virginia.edu/~evans/cs551/saltzer/

### 6.16 Usability heuristics for an operator cockpit

What to add:
- Explicit UX heuristics: visibility of system status, user control/undo,
  error prevention, and recognition over recall.

Why it matters:
- Grounds Action Cards and approvals in widely recognized usability practice.

Source anchors:
- Nielsen Norman Group, "10 Usability Heuristics":
  https://www.nngroup.com/articles/ten-usability-heuristics/
