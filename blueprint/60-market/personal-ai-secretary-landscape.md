# Local Personal AI Secretary: Verified Landscape and Goni Roadmap

Status: Specified only / roadmap (technology intelligence).

Evidence snapshot: 2026-08-01

This brief evaluates the claim that an owner-controlled, persistent local AI
secretary is practical and maps that capability to Goni. It is not an
implementation-status claim. Goni remains a blueprint plus prototype lab, not a
complete personal secretary product.

## Executive verdict

A highly personalized local secretary is practical because the owner can
control the inference runtime, prompts, memory, tools, data retention, and
network boundary. Downloadable model weights remove many controls imposed by a
hosted product: remote system instructions, provider-side input/output
classifiers, account enforcement, service availability, and provider-owned
tool restrictions no longer have to sit on the live inference path.

That does **not** create absolute obedience or remove every constraint:

- model behavior remains learned and probabilistic;
- pretrained and instruction-tuned weights can retain refusal, bias, and
  uncertainty patterns;
- model and software licenses still apply;
- local integrations can still send data to cloud services or telemetry
  endpoints;
- a system prompt cannot guarantee a permanent personality or perfect
  instruction adherence; and
- tool execution creates ordinary computer-security and accidental-action
  risks regardless of how the model was trained.

There is no single refusal switch to disable. Refusal and helpfulness behavior
is distributed across model weights, prompting, chat templates, decoding,
optional guard models, and the surrounding application. Local ownership makes
those layers configurable; it does not make the model deterministic or
infallible.

For a secretary, a capable instruction- and tool-tuned model is usually a
better starting point than a raw base model. A base model primarily predicts
continuations and may require extensive prompting or tuning to converse,
follow tool schemas, stop correctly, and maintain a role. Goni should select a
model through behavior and systems evaluation rather than assuming that
"base" means more loyal or useful.

## What the full stack must provide

Owning only the model is insufficient. A durable personal operator needs five
separable layers:

1. **Inference:** a local model runtime with swappable model bundles, bounded
   context, resource reporting, and offline operation.
2. **Memory:** working, episodic, semantic, relational/project, and procedural
   records with provenance, retrieval, consolidation, export, and forgetting.
3. **Orchestration:** scheduled jobs, resumable work, interruption, retries,
   and explicit completion criteria.
4. **Tools:** scoped access to files, calendar, email, browser, code, and other
   systems, with credentials kept outside prompts.
5. **Authority:** a clear statement of which actor can approve effects, which
   policies bind execution, and how actions are reconstructed afterward.

Goni already specifies these concerns across its [LLM runtime](/blueprint/software/30-components/llm-runtime.md),
[governed memory retrieval](/blueprint/30-specs/memory-retrieval.md),
[scheduler](/blueprint/30-specs/scheduler-and-interrupts.md),
[tool capability API](/blueprint/30-specs/tool-capability-api.md), and
[receipts](/blueprint/30-specs/receipts.md). The missing work is integrated,
tested product behavior, not another personality prompt.

## Hardware is a workload profile, not one VRAM minimum

"At least 8 GB VRAM" is not a universal requirement. Memory use depends on
parameter count, quantization, context length and KV cache, concurrency,
multimodal encoders, and how much work is offloaded to a GPU. Runtimes such as
llama.cpp can use CPU inference and CPU/GPU hybrid offload for models that do
not fit entirely in VRAM.

| Deployment profile | Practical model envelope | Expected trade-off |
| --- | --- | --- |
| Low-VRAM developer machine | Small 2B-4B quantized models, or larger 7B-9B-class models split across GPU and system RAM | Useful for plumbing and tool-loop tests; lower speed, context, or quality |
| 8-16 GB discrete GPU | Many 7B-14B-class quantized instruction models | Responsive single-user assistant, with context and multimodal limits determined by the bundle |
| 24-48 GB accelerator memory | Larger 14B-32B-class quantized models and more resident context | Better reasoning and tool reliability at higher power and cost |
| 64-128 GB unified memory | Larger quantized models, including the 30B-40B class targeted by the Goni reference design | Broad local coverage; bandwidth, thermal limits, and time-to-first-token still matter |

These are evaluation bands, not guarantees. Every promoted model bundle must
be measured on the target backend and hardware. Model weights fitting in
memory does not guarantee acceptable latency or enough room for KV cache,
embeddings, OCR, and concurrent system services.

## Verified project landscape

The projects below are real and active or available as of the evidence
snapshot. Feature descriptions are based on their official repository or
project documentation. "Local" refers to deployability or model support, not a
guarantee that every default feature is offline.

### Personal-agent frameworks and applications

| Project | Maturity and local path | Memory and automation | External dependencies / telemetry | Relevance to Goni |
| --- | --- | --- | --- | --- |
| [QwenPaw](https://github.com/agentscope-ai/QwenPaw) | Active AgentScope-team personal agent with a web console, TUI, desktop build, bundled llama.cpp path, Ollama and LM Studio support, and optional cloud providers | Working context, verbatim history, ReMe-based Markdown knowledge, cron, heartbeat, channels, and multi-agent features | Local models require no API key; `qwenpaw init` has anonymous telemetry, and `--defaults` accepts it automatically according to the reviewed README | Strong UX, memory, channel, and sandbox reference; any integration remains a mediated seat rather than Goni's authority source |
| [nanobot](https://github.com/HKUDS/nanobot) | Active lightweight Python agent with WebUI, terminal, chat channels, OpenAI-compatible API, and support for self-hosted endpoints such as llama.cpp | Session history, Dream-managed long-term files, tools, subagents, goals, and scheduled automation | Provider, channel, and search configuration can add external calls; no general no-telemetry guarantee was established in the reviewed README | Useful small-loop and integration reference; do not replace the Goni scheduler, memory provenance, or kernel |
| [Hivekeep](https://hivekeep.app/) | Self-hosted Bun/SQLite application distributed as one container, with configurable hosted or OpenAI-compatible providers | Continuous per-agent sessions, hybrid semantic/full-text memory, knowledge bases, collaborating agents, cron, webhooks, email triggers, and kanban work | Model and integration calls depend on configured providers; secrets are stored in an encrypted vault and substituted at the tool boundary; telemetry status was not established from the reviewed documentation | Strong multi-agent household UX and secret-handling reference; early project maturity requires verification before any adapter commitment |
| [OpenHuman](https://github.com/tinyhumansai/openhuman) | Early beta desktop application with optional Ollama/custom providers and a managed subscription path | SQLite/Markdown Memory Tree, goals, durable workflows, background context refresh, research, and agent orchestration | The default experience can use managed sign-in, routing, search, and OAuth services; fully local behavior requires explicit local/privacy configuration; telemetry status was not established from the reviewed repository | Valuable memory-tree and workflow UX reference, but "local-first" must not be misread as fully offline by default |
| [meld](https://meld.kizz.me/) | Local-first AGPL desktop application for Markdown, Obsidian, and Logseq vaults; supports Ollama or bring-your-own provider keys | Reads and writes the shared vault, creates and links notes, performs research, and makes Git safety commits | Ollama can keep inference local; BYOK and web research deliberately introduce network dependencies; telemetry status was not established from the reviewed documentation | Strong human-editable memory and diff/versioning reference; not a replacement for Goni's governed Memory Plane |
| [Elroy](https://elroy.bot/) | Scriptable terminal assistant focused on memory, reminders, and goals | Automatic recall, document ingestion, goal tracking, memory consolidation, scripting, and an MCP server | The official quickstart centers on model-provider credentials; local-model behavior is provider/configuration dependent; telemetry status was not established from the reviewed documentation | Useful minimal CLI and goal-memory reference; narrower than a complete operator runtime |
| [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) | Rust agent runtime distributed as a single binary or container, with Ollama and many hosted providers plus 30+ channels | Swappable providers, channels, tools, MCP connections, and long-running personal-agent infrastructure | Network behavior follows enabled providers, channels, tunnels, and tools; local execution alone does not imply offline operation; telemetry status was not established from the reviewed repository | Useful low-overhead runtime and channel abstraction reference; remains outside the sovereign Goni control plane |

### Memory components

| Project | Maturity and local path | Memory behavior | External dependencies / telemetry | Relevance to Goni |
| --- | --- | --- | --- | --- |
| [MemX](https://memx.me/) | Local-first Rust memory service centered on a single libSQL file | Hybrid retrieval, importance/confidence handling, and low-confidence rejection | Embedding endpoint requirements must be checked for the selected deployment; telemetry status was not established from the reviewed documentation | Candidate embedded-memory experiment, not an end-to-end secretary or authority layer |
| [LightMem](https://github.com/zjunlp/LightMem) | Research-backed memory framework with Ollama, vLLM, and hosted-provider support | Separates lightweight online work from deferred ("offline") consolidation in the processing sense and supports storage/retrieval/update experiments | Network locality follows the configured model and embedding providers; telemetry status was not established from the reviewed repository | Evaluation and consolidation reference; adoption requires mapping outputs to Goni provenance and lifecycle contracts |
| [EverOS](https://github.com/EverMind-AI/EverOS) | Active local-first memory ecosystem with an offline educational demo and broader flows that may use external model APIs | Markdown-, SQLite-, and index-backed memory methods, recall, self-evolution experiments, and memory benchmarks | Broader flows may depend on external model APIs; telemetry status was not established from the reviewed repository | Useful backend and benchmark candidate; cannot own Goni memory authority or replace receipt semantics |

Projects without an authoritative URL establishing identity, license,
architecture, and current status are excluded. Volatile GitHub star counts are
also omitted because they do not establish technical maturity.

## Selection guidance for Goni

Goni should not adopt one of these frameworks as its foundation. That would
outsource the session, permission, or memory semantics that define sovereignty.
Instead:

- use QwenPaw, nanobot, Hivekeep, OpenHuman, meld, Elroy, and ZeroClaw as
  comparative references or optional mediated adapters;
- evaluate MemX, LightMem, and EverOS behind the Memory Plane contract;
- keep local runtimes such as llama.cpp, Ollama, LM Studio, or vLLM behind the
  existing `LlmRuntime` abstraction; and
- promote only components that pass offline, provenance, resource, and
  conformance tests on Goni hardware.

## Goni implementation roadmap

This roadmap records intended work. It does not upgrade any component's current
implementation status.

### 1. Establish the inference baseline

- Implement and exercise one local OpenAI-compatible backend behind the
  existing runtime abstraction, then add a second backend only when it exposes
  a meaningful hardware or serving difference.
- Evaluate current instruction/tool-capable model bundles on legitimate-request
  completion, schema-valid tool selection, correction following, latency,
  memory use, context pressure, and offline behavior.
- Keep the model engine stateless. Prompts, persona, memory, and policy remain
  versioned Goni inputs rather than hidden runtime state.
- Reject absolute "never refuses" claims. Measure unnecessary refusal and
  instruction-following rates on an approved, reproducible evaluation pack.

### 2. Build durable memory through the Memory Plane

- Preserve transient working context separately from episodic events,
  semantic facts, project/relational knowledge, and versioned procedures.
- Store source, timestamp, permissions, confidence, validity, and expiry with
  every durable entry.
- Provide exact/sparse, semantic, metadata, and relationship retrieval, with
  reranking and low-confidence rejection.
- Make durable records inspectable and exportable; keep any vector or graph
  index rebuildable from canonical records.
- Implement consolidation, correction, pinning, expiry, forgetting, and audit
  as explicit jobs rather than model-side intuition.

### 3. Add scheduled operator behavior before broad actuation

- Start with Daily Brief, open-loop detection, memory consolidation, and
  anomaly/audit summaries scheduled through the Control Plane.
- Add local files and read-only calendar/email ingestion before draft creation
  and external side effects.
- Introduce file writes, calendar changes, email sending, browser actions, and
  desktop control only through the canonical tool, scheduler, and receipt
  contracts.
- Keep credentials in an OS or encrypted secret store and substitute them only
  at the execution boundary; never persist them in prompts or semantic memory.

### 4. Personalize in the least destructive order

1. owner profile and communication preferences;
2. retrieved project and relationship context;
3. versioned workflow/SOP examples;
4. prompt and chat-template evaluation; and
5. curated LoRA/adapter training only if the previous layers cannot meet the
   behavior target.

Raw personal archives and unreviewed conversations should not automatically
become fine-tuning data. Any adapter must be versioned, removable, evaluated
against its parent bundle, and separable from current facts stored in memory.

### 5. Keep authority profiles explicit

**Conformant Goni** remains the supported product path: maximum-safe autopilot,
kernel-owned capability checks, risk corridors, egress policy, and receipts.
Routine work should be governed at policy level rather than interrupted by
needless confirmations, but authority remains with the owner through the Goni
kernel.

An **unsupported unrestricted research profile** may remove application-level
prompt filters and action gates only inside a disposable, credential-free,
offline sandbox containing synthetic or replaceable data. Because it bypasses
kernel mediation, it is non-conformant and must not be represented as a Goni
production mode. Containment belongs outside that runtime: no personal vault,
network route, reusable credentials, mounted home directory, or access to a
real communications account.

This separation lets researchers measure model behavior without silently
amending Goni's accepted safety and sovereignty contracts.

## Acceptance evidence for a future implementation

A secretary milestone is credible only when evidence demonstrates:

- a complete local/offline workflow with no undeclared egress;
- model and tool behavior measured on a versioned evaluation pack;
- memory recall that shows sources and rejects unsupported matches;
- correction, export, deletion, and rebuild of durable memory;
- scheduled jobs yielding to interactive work without hidden queues;
- reconstructed receipts for every conformant side effect;
- credentials absent from prompts, memory, and receipts; and
- explicit status labels separating implemented, tested, and roadmap behavior.

## Primary sources

- [QwenPaw official repository](https://github.com/agentscope-ai/QwenPaw)
- [nanobot official repository](https://github.com/HKUDS/nanobot)
- [Hivekeep official site and documentation](https://hivekeep.app/)
- [OpenHuman official repository](https://github.com/tinyhumansai/openhuman)
- [meld official site](https://meld.kizz.me/)
- [MemX official site](https://memx.me/)
- [LightMem official repository](https://github.com/zjunlp/LightMem)
- [EverOS official repository](https://github.com/EverMind-AI/EverOS)
- [Elroy official documentation](https://elroy.bot/)
- [ZeroClaw official repository](https://github.com/zeroclaw-labs/zeroclaw)
- [llama.cpp official repository](https://github.com/ggml-org/llama.cpp)
- Meta, [Our responsible approach to Meta AI and Meta Llama
  3](https://ai.meta.com/blog/meta-llama-3-meta-ai-responsibility/), for the
  distinction between pretrained, instruction-tuned, and system safeguard
  layers.
