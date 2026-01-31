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

## 6. Productivity + proactivity backbone (primary sources only)

Status-honest note: this section defines design contracts and justification.
It does not claim implementation unless separately evidenced by tests or
enforcement proofs.

These anchors are restricted to peer-reviewed primary sources, formal
standards, or RFCs, with DOI or canonical links. They backstop Goni primitives
(SLOs, interrupts, capabilities, receipts, confinement, Council, lab eval)
without claiming implementation.

### 6.1 Definition and system contract

**Definition.** Proactive = policy-governed background cognition plus
attention-aware interventions. Proactivity is a resource-constrained systems
problem (scheduling, admission control, interruption policy) and an OS
governance problem (non-bypassable boundaries for side effects and egress).

**Enforceable rules (normative).**
1) Scheduled: proactive work runs under a QoS class and budget (time, tokens,
   bandwidth, energy) before it can manifest as an interrupt. Queue growth and
   interactive tail latency must remain bounded. [R6, R7]
2) Justified: an intervention must satisfy an explicit expected-utility test
   relative to deferral and the expected cost of interruption. [R2-R5]
3) Accountable: any external side effect or egress is mediated through a
   reference-monitor boundary (capability syscall layer / net.egress) and
   leaves receipts with provenance semantics. [R9-R14]

### 6.2 Mixed-initiative control (initiative under uncertainty)

Mixed-initiative systems allocate initiative between user and system under
uncertainty. The system's initiative must be scoped to confidence and preserve
user sovereignty via deferral and override mechanisms. [R1]

Goni mapping (normative):
- Action Cards: propose -> approve/deny/defer -> execute.
- Daily Brief: default deferral/batching surface.
- Agent manifests: declared scope, triggers, and budgets constrain initiative.
- Capability-checked syscalls: initiative cannot bypass governance.

### 6.3 Attention and interruption management

Interruption decisions should be decision-theoretic: alerts depend on context
and predicted interruption cost. Models of attention support notify vs defer
decisions. Empirical evidence shows interruption can increase stress and
perceived time pressure, supporting deferral-first policies. [R2-R5, R8]

Goni mapping (normative):
- Interrupt budgets enforce per-channel/per-day caps.
- Deferral first: non-urgent suggestions go to Daily Brief.
- Escalation rule: interrupt only if expected utility of immediate action
  exceeds expected cost of interruption, subject to budget.

### 6.4 Queueing theory and tail latency (prevent overload collapse)

Little's Law: L = lambda W. If arrival rate grows without bounding work-in-
system, waiting time increases and interactive responsiveness suffers. [R6]

Tail latency dominates perceived responsiveness; systems must isolate
interactive QoS and use percentile SLOs (p95/p99) for TTFT and cancellation. [R7]

Goni mapping (normative):
- Background work must be admission-controlled and preemptible.
- Interactive QoS must be protected under contention.

### 6.5 LLM serving as OS-style memory management

Modern LLM serving explicitly uses OS-like paging for KV cache and couples it
to scheduling, providing a precedent for preemption/cancellation as first-class
control. [R15]

Goni mapping (normative):
- LLM runtime exposes utilization/capability signals so the scheduler can
  protect interactive QoS and preempt background inference.

### 6.6 End-to-end arguments (where enforcement belongs)

End-to-end arguments show some integrity properties cannot be guaranteed by
best-effort intermediates; enforcement must live at boundaries where checks
are complete. [R9]

Goni boundary placement (normative):
- Tool capability syscalls enforce no ambient side effects.
- net.egress enforces no unlogged outbound traffic.
- Council escalation is explicit and logged.

The "Rise of the Middle" RFC supports keeping enforcement at explicit
boundaries rather than implicit middleboxes. [R10]

### 6.7 Security principles and reference monitor lineage

Saltzer & Schroeder provide the canonical secure design checklist: least
privilege, complete mediation, economy of mechanism, fail-safe defaults,
separation of privilege, psychological acceptability. [R11]

Reference monitor lineage traces to the Anderson planning study. [R12]

Goni mapping (normative):
- Least privilege: minimal capabilities per tool/agent.
- Complete mediation: every access and side effect is checked.
- Economy of mechanism: keep the trusted kernel small.
- Psychological acceptability: permissions must be understandable.

### 6.8 Receipts as provenance + tamper-evident audit

Map receipts to W3C PROV concepts (Entity/Activity/Agent; attribution;
derivation) for interoperability. [R13]

Optional receipt integrity mode: append-only log commitments (Merkle tree),
with inclusion/consistency proofs. [R14]

### 6.9 Operator cockpit foundations (situation awareness)

Situation awareness theory is a primary foundation for operator interfaces
that support awareness, control, and error recovery. [R16]

Goni mapping (normative):
- Visibility: show queued/running actions and budgets.
- Operator control/undo: action cards must be reversible or explicitly confirmed.
- Comprehension/projection: Daily Brief explains what changed and what is likely.

### References (primary sources / standards only)

[R1] Horvitz, E. (1999). Principles of Mixed-Initiative User Interfaces.
     CHI 1999. DOI: https://doi.org/10.1145/302979.303030
[R2] Horvitz, E., and Apacible, J. (2003). Learning and Reasoning about
     Interruption. ICMI 2003. DOI: https://doi.org/10.1145/958432.958440
[R3] Horvitz, E., Koch, P., and Apacible, J. (2004). BusyBody: Creating and
     Fielding Personalized Models of the Cost of Interruption. CSCW 2004.
     DOI: https://doi.org/10.1145/1031607.1031690
[R4] Horvitz, E., Kadie, C., Paek, T., and Hovel, D. (2003). Models of Attention
     in Computing and Communication: From Principles to Applications.
     Communications of the ACM, 46(3). DOI: https://doi.org/10.1145/636772.636798
[R5] McFarlane, D. C. (2002). Comparison of Four Primary Methods for
     Coordinating the Interruption of People in Human-Computer Interaction.
     Human-Computer Interaction, 17(1). DOI: https://doi.org/10.1207/S15327051HCI1701_2
[R6] Little, J. D. C. (1961). A Proof for the Queueing Formula: L = lambda W.
     Operations Research, 9(3), 383-387. DOI: https://doi.org/10.1287/opre.9.3.383
[R7] Dean, J., and Barroso, L. A. (2013). The Tail at Scale.
     Communications of the ACM, 56(2), 74-80. DOI: https://doi.org/10.1145/2408776.2408794
[R8] Mark, G., Gudith, D., and Klocke, U. (2008). The Cost of Interrupted Work:
     More Speed and Stress. CHI 2008. DOI: https://doi.org/10.1145/1357054.1357072
[R9] Saltzer, J. H., Reed, D. P., and Clark, D. D. (1984). End-to-End Arguments
     in System Design. ACM TOCS, 2(4), 277-288. DOI: https://doi.org/10.1145/357401.357402
[R10] Kempf, J., and Austein, R. (2004). The Rise of the Middle and the Future
      of End-to-End. RFC 3724. https://datatracker.ietf.org/doc/html/rfc3724
[R11] Saltzer, J. H., and Schroeder, M. D. (1975). The Protection of Information
      in Computer Systems. Proceedings of the IEEE, 63(9), 1278-1308.
      DOI: https://doi.org/10.1109/PROC.1975.9939
[R12] Anderson, J. P. (1972). Computer Security Technology Planning Study
      (ESD-TR-73-51). Archival PDF: https://csrc.nist.rip/publications/history/ande72.pdf
[R13] W3C (2013). PROV-DM: The PROV Data Model (W3C Recommendation).
      https://www.w3.org/TR/prov-dm/
[R14] Laurie, B., Langley, A., and Kasper, E. (2013). Certificate Transparency.
      RFC 6962. https://www.rfc-editor.org/rfc/rfc6962
[R15] Kwon, W., et al. (2023). Efficient Memory Management for Large Language
      Model Serving with PagedAttention. arXiv:2309.06180.
      DOI: https://doi.org/10.48550/arXiv.2309.06180
[R16] Endsley, M. R. (1995). Toward a Theory of Situation Awareness in Dynamic
      Systems. Human Factors, 37(1), 32-64. DOI: https://doi.org/10.1518/001872095779049543
