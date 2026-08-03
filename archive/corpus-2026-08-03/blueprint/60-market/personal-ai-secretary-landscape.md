# Local Personal AI Secretary: Verified Landscape and Goni Roadmap

Status: Specified only / roadmap (technology intelligence).

Evidence snapshot: 2026-08-02

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

Goni's preferred posture is owner-sovereign and open-weight: no mandatory
provider account, remote policy prompt, server-side classifier, silent
telemetry, revocable API entitlement, or vendor-controlled behavior update is
allowed on the default inference path. The owner chooses the checkpoint, chat
template, system prompt, decoding settings, memory, and optional content
controls. No mandatory viewpoint or content filter should sit between the
owner and locally generated text.

This is an architectural allocation of authority, not a promise of polite or
approved opinions. A sovereign assistant should answer owner-requested text
queries candidly, including controversial, heterodox, or offensive subjects,
without adding a third party's moral or political policy layer. Goni should
govern consequential actions at the capability boundary instead of treating
the model's speech as the action: generating text is not the same operation as
sending a message, spending money, deleting data, or controlling a machine.

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
infallible. "No filter" can therefore be a stack policy: no mandatory external
moderation or application-layer suppression--but cannot honestly guarantee
that learned weights will never hedge, refuse, omit, or moralize.

For a secretary, a capable instruction- and tool-tuned model is usually a
better starting point than a raw base model. A base model primarily predicts
continuations and may require extensive prompting or tuning to converse,
follow tool schemas, stop correctly, and maintain a role. Goni should select a
model through behavior and systems evaluation rather than assuming that
"base" means more loyal or useful.

## What the full stack must provide

Owning only the model is insufficient. A durable personal operator needs five
separable layers:

1. **Inference:** a local runtime with swappable, hash-pinned open-weight model
   bundles, bounded context, resource reporting, and fully offline operation.
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

## Local-inference runtime matrix

This ranking applies one specific criterion: practical usefulness for an
interactive local AI assistant on constrained hardware. It prioritizes usable
latency, hardware flexibility, maintainability, and permissive licensing. It
does not rank projects by the largest model they can technically initialize.
Latency labels are qualitative selection guidance, not Goni benchmark results.

### A. Local and constrained-hardware runtimes

| Priority | Project | Runtime class | Where the model weights live | Hardware sweet spot | Latency profile | Setup burden | Best use | Decisive limitation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | [llama.cpp](https://github.com/ggml-org/llama.cpp) | General local runtime | GGUF weights distributed between VRAM and system RAM through partial GPU offload and memory mapping | CPUs, Apple Silicon, NVIDIA, AMD, and Intel; especially 4-24 GB GPUs with adequate RAM | Interactive when most computation remains on GPU; progressively slower as more layers remain on CPU | Low to medium | Default local assistant, broad hardware support, experimentation, and reliable deployment | Extremely large models still require enough aggregate RAM and become CPU-bandwidth constrained when heavily offloaded. |
| 2 | [ExLlamaV3](https://github.com/turboderp-org/exllamav3) | GPU-native quantized runtime | EXL3 weights reside substantially in aggregate GPU memory, with tensor and expert parallelism across GPUs | Modern NVIDIA GPUs, especially one or several 24 GB cards | Excellent when the model fits in VRAM | Medium | Maximum interactive speed, long conversations, and multi-user local agents on NVIDIA | It is not primarily a CPU or NVMe spillover system; performance depends on fitting the quantized model and cache substantially inside available VRAM. |
| 3 | [KTransformers](https://github.com/kvcache-ai/ktransformers) | Heterogeneous MoE runtime | Attention and selected components use the GPU; experts can remain in system RAM and execute through optimized CPU kernels and scheduling | One strong GPU plus 128-512 GB of fast RAM, preferably modern DDR5 with high memory bandwidth | Conditionally interactive, depending heavily on model, CPU, and RAM bandwidth | High | Very large sparse MoE models that exceed GPU memory by a wide margin | Architecture support is model-specific, installation is demanding, and memory bandwidth becomes the principal bottleneck. |
| 4 | [ik_llama.cpp](https://github.com/ikawrakow/ik_llama.cpp) | Experimental high-performance llama.cpp fork | GGUF weights split selectively between CPU and CUDA, with specialized MoE kernels, tensor overrides, and additional quantization formats | Technical users with strong AVX2/AVX-512 CPUs and NVIDIA CUDA GPUs | Good to conditional, particularly for supported MoE workloads | High | Additional CPU or hybrid-MoE performance beyond mainline llama.cpp | The fork diverges from mainline, and its documentation identifies CPU and CUDA as the fully functional, performant backends; compatibility and configuration risk are higher. |
| 5 | [Chitu](https://github.com/thu-pacman/chitu) | Heterogeneous enterprise inference engine | Supports CPU, single-GPU, CPU-GPU hybrid, and distributed accelerator configurations, including FP4 and FP8 conversion paths | NVIDIA or Chinese accelerator infrastructure, from one card to multi-node deployments | Conditional to production-grade, depending on configuration | High | Large MoE deployment where heterogeneous hardware and later scaling matter | It is more operationally complex than desktop-focused runtimes, with a smaller international user ecosystem and more platform-specific deployment paths. |
| 6 | [Hugging Face Accelerate](https://huggingface.co/docs/accelerate/concept_guides/big_model_inference) | Generic model dispatcher and offloader | Modules can be allocated across GPU, CPU, and memory-mapped disk through an explicit or automatic device map | Researchers supporting uncommon Transformers architectures | Usually slow under heavy offload | Medium to high | Compatibility experiments, custom architectures, and implementation work | Multi-GPU model parallelism is deliberately basic and can leave GPUs operating sequentially; Accelerate is a compatibility layer, not a highly optimized local inference engine. |
| 7 | [FlexLLMGen](https://github.com/FMInference/FlexLLMGen) | Throughput-oriented offload engine | GPU, CPU, and storage are jointly scheduled, using compression and large effective batches | One commodity GPU running large offline jobs | Batch-oriented, not conversational | High | Overnight extraction, benchmarking, data processing, and high-volume offline inference | Its optimization target is aggregate throughput over long jobs rather than low first-token latency or interactive chat. |
| 8 | [AirLLM](https://github.com/lyogavin/airllm) | Layer-wise or expert-wise streaming runtime | Individual layers or routed experts are repeatedly loaded from storage into GPU memory | Small VRAM, very large SSD capacity, and workloads where execution matters more than response time | Proof-of-execution for extreme models | Medium | Architecture inspection, compatibility testing, and demonstrating that a checkpoint can technically execute | It minimizes peak VRAM by transferring the bottleneck to storage traffic, weight movement, and token latency. Its Kimi K3 demonstration used 3.72 GB peak allocation on an RTX 6000 Ada, not an actual 4 GB card. |

### B. Production-serving engines

These engines solve a different problem and therefore remain outside the
constrained-hardware ranking.

| Project | License | Primary optimization | Hardware assumption | Best use | Why it is not an AirLLM replacement |
| --- | --- | --- | --- | --- | --- |
| [vLLM](https://docs.vllm.ai/) | Apache 2.0 | Continuous batching, cache management, tensor parallelism, pipeline parallelism, and expert parallelism | The model is sensibly distributed across sufficient accelerator memory | High-throughput OpenAI-compatible serving, multi-user systems, and large production deployments | It optimizes execution after weights have been properly placed; it does not primarily turn a tiny GPU into a practical host for a trillion-parameter checkpoint. |
| [SGLang](https://docs.sglang.ai/) | Apache 2.0 | RadixAttention, prefix caching, structured generation, speculative decoding, and distributed serving | One adequately sized GPU through large clusters | Agent systems, repeated prefixes, structured outputs, and production inference | Its strengths are latency and throughput at serving scale, rather than continuous disk streaming under severe VRAM constraints. |

### Kimi K3 status on 2026-08-02

| Runtime | Snapshot status | Practical interpretation |
| --- | --- | --- |
| [AirLLM](https://github.com/lyogavin/airllm) | Publicly demonstrated with 3.72 GB peak VRAM on one RTX 6000 Ada through per-expert streaming | Lowest documented GPU allocation in this comparison, but primarily an execution demonstration rather than a fast assistant path. |
| [vLLM](https://vllm-project.github.io/2026/07/27/k3.html) | Live K3 support includes multimodal processing, tool calling, reasoning output, structured output, and production deployment recipes | Strongest current production path, provided substantial accelerator infrastructure is available. |
| [llama.cpp](https://github.com/ggml-org/llama.cpp/pull/26185) | Text-model support remains an open, unmerged upstream pull request; a full-size multimodal fork reports functional layer splitting while row and tensor splitting remain requested | Promising for GGUF and hybrid deployment, but too fresh and fork-dependent to call stable. See the current [split-mode request](https://github.com/ggml-org/llama.cpp/issues/26365). |
| [KTransformers](https://github.com/kvcache-ai/ktransformers/issues/2109) | No released K3 support was evident; the support request opened on 2026-07-28 remains unresolved | Potentially interesting for a RAM-heavy workstation, but not K3-ready at this snapshot. |

### How to read the matrix

- **Weight placement matters:** a small VRAM number can hide a large system-RAM
  or storage requirement. The physical location of the remaining weights is
  part of the hardware cost.
- **Capacity is not latency:** initialization, eventual token generation, and
  productive interactive response are three different thresholds. This matrix
  ranks for the third.
- **Serving is not offloading:** vLLM and SGLang improve execution and
  concurrency after a model has been provisioned. AirLLM, Accelerate, and
  FlexLLMGen instead explore increasingly aggressive memory hierarchies.
- **Operations matter:** installation, supported architectures, backend
  maturity, and configuration risk affect the reliability of a daily personal
  assistant as much as a peak benchmark does.

Four-bit arithmetic also sets a useful lower bound. Seventy billion weights at
four bits require `70,000,000,000 x 4 / 8 = 35,000,000,000` bytes, approximately
35 GB before quantization metadata, higher-precision tensors, KV cache, and
runtime buffers. A 24 GB GPU therefore requires CPU offload, multiple GPUs, a
lower effective bitrate, or a smaller model; ordinary 4-bit quantization alone
does not make a 70B model VRAM-resident on that card.

All engine codebases in the two tables use permissive MIT or Apache 2.0
licenses. The license for any selected model checkpoint is separate and must be
reviewed independently.

### Backend decision hierarchy

1. Use llama.cpp as the safest overall foundation for a dependable local AI
   secretary.
2. Prefer ExLlamaV3 when the deployment is NVIDIA-only and the selected model
   plus cache fit substantially in aggregate VRAM.
3. Use KTransformers as the specialist option for very large sparse MoE models
   on a RAM-heavy workstation.
4. Move to vLLM or SGLang when the assistant becomes a multi-user or production
   service with adequate accelerator memory.
5. Treat AirLLM as a research, compatibility, and minimum-allocation instrument
   rather than the primary interactive backend.

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
- use llama.cpp as the default local foundation, ExLlamaV3 for VRAM-resident
  NVIDIA workloads, KTransformers for RAM-heavy sparse MoE experiments,
  vLLM/SGLang for serving scale, and AirLLM only for compatibility or
  minimum-allocation demonstrations, all behind the existing `LlmRuntime`
  abstraction; and
- promote only components that pass offline, provenance, resource, and
  conformance tests on Goni hardware.

## Goni implementation roadmap

This roadmap records intended work. It does not upgrade any component's current
implementation status.

### 1. Establish the inference baseline

- Implement and exercise one local OpenAI-compatible backend behind the
  existing runtime abstraction, then add a second backend only when it exposes
  a meaningful hardware or serving difference.
- Prefer downloadable open-weight checkpoints whose licenses permit the
  intended local use, modification, and redistribution; pin hashes, templates,
  tokenizer assets, and quantization metadata so the bundle cannot drift at a
  provider's discretion.
- Keep the default local text path free of mandatory moderation sidecars,
  provider policy prompts, remote entitlement checks, and hidden network
  dependencies. Any owner-selected content control must be explicit,
  inspectable, removable, and disabled independently of tool authorization.
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

**Conformant Goni** makes the owner the root authority. The kernel enforces the
owner's declared capability scopes, risk corridors, egress policy, and receipt
requirements; it does not enforce a model vendor's worldview. Routine and
reversible work should run without paternalistic confirmation loops. Any limit
that remains must be attributable to an owner-selected policy, a concrete
resource boundary, or a documented legal/technical constraint.

A **sovereign local-expression profile** is conformant: it may remove
application-level output filters, remote moderation, and provider-authored
policy prompts for local text generation. This does not weaken file, network,
credential, financial, communications, or device-control permissions. Optional
owner-defined filters remain pluggable rather than mandatory.

An **unrestricted-execution research profile** may also remove action gates,
but only inside a disposable, credential-free, offline sandbox containing
synthetic or replaceable data. Because it bypasses kernel mediation, it is
non-conformant and must not be represented as a production mode. Containment
belongs outside that runtime: no personal vault, network route, reusable
credentials, mounted home directory, or real communications account.

The dividing line is therefore liberty of local computation and expression
versus authority to impose effects on other systems or people. The first is
owner-controlled by default; the second remains explicitly delegated and
receipted.

## Acceptance evidence for a future implementation

A secretary milestone is credible only when evidence demonstrates:

- a complete local/offline workflow with no undeclared egress;
- operation without a mandatory provider account, entitlement check, remote
  moderation service, or hidden policy prompt;
- a reproducible, hash-pinned open-weight bundle with checkpoint and engine
  licenses recorded separately;
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
- [ExLlamaV3 official repository](https://github.com/turboderp-org/exllamav3)
- [KTransformers official repository](https://github.com/kvcache-ai/ktransformers)
- [ik_llama.cpp official repository](https://github.com/ikawrakow/ik_llama.cpp)
- [Chitu official repository](https://github.com/thu-pacman/chitu)
- [Hugging Face Accelerate big-model inference documentation](https://huggingface.co/docs/accelerate/concept_guides/big_model_inference)
- [FlexLLMGen official repository](https://github.com/FMInference/FlexLLMGen)
- [AirLLM official repository](https://github.com/lyogavin/airllm)
- [vLLM official documentation](https://docs.vllm.ai/)
- [SGLang official documentation](https://docs.sglang.ai/)
- Meta, [Our responsible approach to Meta AI and Meta Llama
  3](https://ai.meta.com/blog/meta-llama-3-meta-ai-responsibility/), for the
  distinction between pretrained, instruction-tuned, and system safeguard
  layers.
