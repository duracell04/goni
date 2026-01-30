# Adjacent Projects - Agent Gateways, Local Runtimes, and AI OS Neighbors

This document captures projects that are adjacent to Goni's scope but sit
outside the "home cluster / distributed inference" focus of
`docs/related-projects.md`.

It is meant to be a fast map of what exists, what it is good at, and where
Goni's "governed kernel + receipts + confinement" approach is distinct.

---

## 1. Key insights (systems-level)

- Agent gateways (OpenClaw, Open Interpreter) are strongest at integrations and
  "doing things," but they are not OS-style governance layers.
- Local runtime servers (LM Studio, Ollama, LocalAI) are good backends for an
  `llm-runtime` abstraction, but they do not provide tool governance.
- Desktop "local AI apps" (Jan, GPT4All, AnythingLLM, Open WebUI) prioritize UX
  and convenience over kernel-style constraints.
- "Agent OS" research (AIOS) overlaps on scheduling and memory ideas, but it
  typically lacks Goni's data-plane constitution, receipts, and confinement.
- The local-first shift is driven more by latency, privacy, and memory bandwidth
  than by chasing peak benchmark scores; cloud remains an explicit escalation.
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

## 4. Local-first shift: hardware and economics signals

Key drivers (system-level, not vendor-specific):

- Latency and privacy push personal workloads toward local inference.
- Memory bandwidth often limits LLM throughput more than peak FLOPS.
- The economic unit shifts from "GPU hour" to "device amortization."
- Cloud remains a logged escalation path for hard or fresh-web tasks.

Vendor spec anchors (illustrative, not requirements):

- Apple MacBook Pro specs: https://www.apple.com/macbook-pro/specs/
- AMD Ryzen AI 300 series: https://www.amd.com/en/partner/articles/ryzen-ai-300-series-processors.html
- NVIDIA Jetson Orin: https://www.nvidia.com/en-us/autonomous-machines/embedded-systems/jetson-orin/
