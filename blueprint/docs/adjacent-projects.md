# Adjacent Projects - Agent Gateways, Local Runtimes, and AI OS Neighbors

This document captures projects that are adjacent to Goni's scope but sit
outside the "home cluster / distributed inference" focus of
`blueprint/docs/related-projects.md`.

It is meant to be a fast map of what exists, what it is good at, and where
Goni's "governed kernel + receipts + confinement" approach is distinct.

How this relates to `blueprint/docs/related-projects.md`:

- `blueprint/docs/related-projects.md` covers **home clusters + distributed inference**
  prior art (EXO, Cake, prima.cpp, Beowulf, llama.cpp).
- This document covers **ecosystem neighbors** (runtimes, apps, gateways, and
  "AI OS" framing) and the system/product insights they imply.

---

## 1. Key insights (systems-level)

- "AI OS" splits into four real categories: blueprint/runtime/server, local AI desktop app,
  agent gateway, and agent OS research. Goni is aiming for runtime + governance
  + operator appliance UX, which is rarer.
- Agent gateways (OpenClaw, Open Interpreter) are strongest at integrations and
  "doing things," but they are not OS-style governance layers.
- For a sovereign path, gateways should be treated as inspiration or optional
  untrusted/mediated seats, not as substitutes for the kernel.
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
- Useful for interaction and routing ideas, but not a sovereign base: Goni must
  still own authority, receipt semantics, corridor policy, and durable memory.

Refined Goni takeaway:

- "Goni Claw" is the pattern where Goni keeps the sovereign kernel and an
  OpenClaw-like layer supplies the front-door UX.
- That means:
  - chat/channel routing and action surfaces may look gateway-like,
  - but capability checks, corridor outcomes, receipts, and memory provenance
    still terminate in the Goni kernel.
- In short: OpenClaw as surface inspiration, Goni as control plane.

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

### Source confidence legend

Use this inventory as technology intelligence, not as an implementation
commitment. Confidence labels mean:

- `verified`: official project site, docs, repository, standards page, or
  primary paper was found in this pass.
- `needs verification`: likely real or widely referenced, but status,
  maintenance, or scope needs a direct official check before use.
- `stale/deprecated`: real project, but renamed, acquired, discontinued, or
  superseded enough that it should not be treated as a fresh default.
- `candidate/unverified`: named in ecosystem notes, but no reliable official
  source was confirmed in this pass.

### A) Runtime / model serving layer

These are backends for `llm-runtime` or model routing. None replaces Goni's
kernel, policy, receipt, or memory governance.

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| Ollama | `verified` | Simple local model runtime and model manager. |
| llama.cpp / llama-server | `verified` | Low-level GGUF inference baseline and OpenAI-compatible local server path. |
| LocalAI | `verified` | Local OpenAI-compatible API with broad backend and multimodal ambitions. |
| LM Studio | `verified` | Desktop runtime plus OpenAI-compatible local server. |
| Jan local server | `verified` | Local-first desktop app that can also expose local model serving. |
| vLLM | `verified` | Production-grade GPU serving and batching path for stronger nodes. |
| SGLang | `verified` | Serving/runtime layer for structured generation and high-throughput inference. |
| TensorRT-LLM | `verified` | NVIDIA-optimized high-throughput serving path. |
| TGI / Hugging Face Text Generation Inference | `verified` | Hugging Face production inference server, useful for GPU-backed deployments. |
| Mistral.rs | `verified` | Rust inference engine candidate for local and server use. |
| exo | `verified` | Everyday-device clustering and distributed local inference reference. |
| BitNet | `needs verification` | 1-bit inference/model-family direction; evaluate by concrete runtime maturity. |
| KTransformers | `verified` | Heterogeneous/MoE-optimized inference candidate, especially CPU+GPU mixes. |
| Hugging Face Transformers | `verified` | Research and experimentation baseline, not usually the appliance serving layer. |
| LMDeploy | `verified` | Efficient LLM deployment/serving framework candidate. |
| JittorInfer | `needs verification` | China-origin inference candidate tied to Jittor/Ascend-style deployment notes. |
| Xinference | `verified` | Model-serving platform that can wrap vLLM, SGLang, llama.cpp, MLX, and others. |
| Xuanwu CLI / 玄武CLI | `needs verification` | Ollama-like domestic CLI/server candidate; confirm license and backend maturity. |

Runtime conclusion for Goni:

- Edge and consumer nodes should bias toward llama.cpp, Ollama-like UX,
  MLX/Apple paths, or Xinference-style wrappers.
- GPU servers should evaluate vLLM, SGLang, TGI, TensorRT-LLM, and LMDeploy.
- Goni's model router should treat all of these as swappable runtime targets
  behind policy, receipts, budgets, and local/cloud routing.

### B) Self-hosted chat, RAG UI, and app layer

These projects are useful surfaces for chat, document workflows, agent testing,
or operator UX. They are not the governed control plane.

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| Open WebUI | `verified` | Popular self-hosted chat/RAG UI. |
| AnythingLLM | `verified` | Private document AI, RAG, and agent workflows. |
| LibreChat | `verified` | Self-hosted multi-provider chat platform with agents/MCP-style integrations. |
| Jan | `verified` | Local-first desktop chat/runtime surface. |
| Msty | `verified` | Desktop/local AI app and model/provider UX reference. |
| Dify | `verified` | Visual workflow, RAG, and agent application platform. |
| LobeChat | `verified` | Self-hostable chat UI with plugin/provider ecosystem. |
| GPT4All | `verified` | Local desktop chat and model ecosystem. |
| Chatbox AI | `verified` | Cross-platform chat client reference. |
| BionicGPT | `verified` | Self-hosted private GPT/RAG deployment candidate. |
| Khoj | `verified` | Personal second-brain assistant with local/private knowledge workflows. |
| PrivateGPT | `verified` | Private document RAG reference implementation/product. |
| Chatbot UI | `verified` | Lightweight self-hostable chat UI base. |
| uni-ai-x | `needs verification` | DCloud cross-platform AI chat candidate; confirm license and server model. |
| AIChatWeb | `candidate/unverified` | Chinese enhanced ChatGPT-Next-Web-style commercial self-hosting candidate. |
| Alibaba ChatUI | `verified` | Frontend conversational UI component library, not a full operator. |

UI conclusion for Goni:

- AnythingLLM, Open WebUI, LibreChat, Jan, Dify, and Khoj are strong references
  for user workflows.
- Goni should keep the UI thin: channel and dashboard surfaces can be swapped,
  but action authority stays behind the Goni kernel.

### C) Agent gateway, operator, and orchestration layer

This layer is where tool use, channels, memory, planning, workflows, and
multi-agent behavior get confused. Goni should separate them:

- channel gateway: user reachability and sessions,
- reasoning/orchestration: state machine and tool loop,
- kernel: authority, mediation, receipts, policy, and rollback.

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| OpenClaw | `verified` | Agent/channel gateway reference for messaging, sessions, tools, and routing. |
| Jan Agents / OpenClaw integration | `needs verification` | Possible bridge between Jan UX and OpenClaw-style local agents. |
| OpenDAN | `verified` | Personal AI OS concept and comparison point. |
| LibreChat agents | `verified` | Agent features inside a self-hosted chat platform. |
| n8n + LLM tools | `verified` | Workflow automation and integration graph around LLM calls. |
| LangGraph | `verified` | Stateful graph orchestration, checkpointing, and human-in-the-loop design. |
| CrewAI | `verified` | Role/crew-based agent orchestration; useful for rapid prototypes. |
| AutoGen / AG2 | `verified` | Conversational multi-agent framework lineage. |
| Flowise | `verified` | Visual LLM workflow builder. |
| LangFlow | `verified` | Visual LangChain-style workflow builder. |
| Dify workflows | `verified` | Visual app/workflow layer, also listed in UI because it spans layers. |
| Superagent | `needs verification` | Agent platform candidate; verify current maintenance and deployment model. |
| OpenDevin | `stale/deprecated` | Coding agent lineage; check current project name/status before adoption. |
| Eigent | `needs verification` | Multi-agent/operator candidate; verify current official source and maturity. |
| Hermes Agent | `needs verification` | Server/structured agent candidate; verify official project identity. |
| NanoClaw | `needs verification` | Security-focused OpenClaw-adjacent candidate; verify exact repo and scope. |
| NemoClaw | `candidate/unverified` | Named security-focused variant; no reliable official source confirmed here. |
| memU | `needs verification` | Memory-heavy agent candidate; verify official source and data model. |
| Open Interpreter | `verified` | Local code/computer-use execution shell; high-risk without mediation. |
| Aiden | `candidate/unverified` | Personal AI OS/assistant candidate; ambiguous name, needs disambiguation. |
| CoPaw / QwenPaw | `needs verification` | Alibaba/Tongyi-style personal agent workstation candidate; naming changed in checked source. |
| LobsterAI / 有道龙虾 | `verified` | NetEase Youdao personal assistant agent with channel and local/sandbox execution notes. |
| MaxClaw | `candidate/unverified` | MiniMax/OpenClaw-adjacent candidate; no official source confirmed here. |
| WorkBuddy | `candidate/unverified` | Tencent-style OpenClaw-compatible candidate; no official source confirmed here. |
| AutoClaw | `needs verification` | Zhipu/OpenClaw installer candidate; verify official ownership and distribution source. |

Agent-layer conclusion for Goni:

- OpenClaw-like systems can be gateways, not foundations.
- LangGraph-like systems are closer to the reasoning/state-machine layer.
- n8n, Dify, Flowise, and LangFlow are useful workflow surfaces.
- Goni's distinctive layer is the governed kernel above all of them:
  capability-checked tool execution, receipts, rollback, and autonomy corridors.

### D) Tool protocol and execution layer

| Project / protocol | Confidence | Goni relevance |
| --- | --- | --- |
| MCP servers / Model Context Protocol | `verified` | Open protocol for connecting AI apps to tools, data sources, and workflows. |
| Composio | `verified` | Large integration catalog for agent tools; supplier/adapter candidate. |
| E2B | `verified` | Sandboxed code execution for agents. |
| Daytona | `verified` | Development/workspace sandbox candidate. |
| browser-use | `verified` | Browser automation agent framework. |
| Stagehand | `verified` | Browser automation library using Playwright-style primitives. |
| Anthropic Computer Use | `verified` | Computer-use reference behavior/model interface, not a local governance layer. |
| Playwright | `verified` | Deterministic browser automation substrate. |
| Google A2A / Google ADK | `verified` | Agent-to-agent protocol/devkit direction; complement to MCP, not a replacement. |

Goni implication:

- MCP and A2A are interoperability surfaces.
- E2B, Daytona, Playwright, browser-use, and Stagehand are execution surfaces.
- Goni still needs non-bypassable mediation before any tool or browser action.

### E) Memory, RAG, state, and knowledge layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| mem0 | `verified` | Agent memory layer candidate. |
| Zep | `verified` | Long-term conversational memory/state service. |
| Chroma / ChromaDB | `verified` | Local vector store candidate. |
| Qdrant | `verified` | Current prototype-aligned vector DB candidate. |
| Milvus | `verified` | Scalable vector DB candidate. |
| Weaviate | `verified` | Vector DB and hybrid retrieval candidate. |
| pgvector | `verified` | Postgres vector extension path. |
| Redis | `verified` | Cache/state/checkpoint substrate. |
| Neon | `verified` | Serverless Postgres supplier candidate; more cloud than local-first. |
| LlamaIndex | `verified` | RAG and agentic retrieval framework. |
| RAGFlow | `verified` | Document understanding and RAG application framework. |
| Unstructured | `verified` | Document parsing and ingestion pipeline. |
| Docling | `verified` | Document conversion/parsing pipeline. |
| Marker | `verified` | PDF/document-to-Markdown extraction candidate. |
| Neo4j | `verified` | Graph database for knowledge graph / GraphRAG experiments. |
| Kuzu | `verified` | Embedded graph database candidate. |
| Marqo | `verified` | Vector search platform candidate. |
| LanceDB | `verified` | Embedded/vector data lake candidate. |
| Obsidian AI/plugin patterns | `needs verification` | Personal knowledge integration pattern, not one stable upstream component. |

Goni implication:

- VecDB is only one piece of memory.
- Durable state, checkpoints, selective forgetting, graph memory, and provenance
  must remain under Goni governance even when external stores are used.

### F) Observability, evaluation, and model gateway/router layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| LiteLLM | `verified` | Model-agnostic proxy/router across local and cloud providers. |
| Portkey | `verified` | AI gateway, routing, guardrail, and observability candidate. |
| Helicone | `verified` | LLM observability and gateway candidate. |
| Langfuse | `verified` | Open-source LLM tracing, prompt, eval, and cost observability. |
| AgentOps | `verified` | Agent-specific observability candidate. |
| LangSmith | `verified` | LangChain ecosystem tracing/evals platform. |
| Arize Phoenix | `verified` | Open-source observability/evaluation stack. |
| Bench360 | `needs verification` | Academic benchmark candidate for local inference comparisons. |
| PASB | `candidate/unverified` | Personalized Agent Security Benchmark claim needs primary source check. |
| OpenClaw-RL | `candidate/unverified` | RL personalization extension claim needs primary source check. |

Goni implication:

- Receipts are not the same as observability traces; Goni should support both.
- Routers like LiteLLM are useful plumbing, but the policy decision remains in
  Goni's control plane.

### G) Smart-home, local voice, realtime voice, and multimodal layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| Home Assistant / Nabu Casa | `verified` | Smart-home integration baseline and practical voice/home ecosystem. |
| Home Assistant Voice Assist | `verified` | Native HA voice path. |
| Rhasspy | `stale/deprecated` | Offline voice assistant lineage; much of the practical path moved into HA/Wyoming. |
| OpenVoiceOS | `verified` | Privacy-oriented open voice platform. |
| Willow | `verified` | ESP32-S3/local voice hardware path. |
| Wyoming | `verified` | Protocol/integration layer for STT/TTS/wake-word services. |
| Whisper / Faster Whisper | `verified` | Local speech-to-text candidates. |
| Piper / Piper TTS | `verified` | Local text-to-speech candidate. |
| openWakeWord | `verified` | Local wake-word detection candidate. |
| LiveKit | `verified` | Realtime voice/video agent infrastructure. |
| Pipecat | `verified` | Realtime conversational AI/voice agent framework. |
| Daily | `verified` | Realtime voice/video infrastructure supplier. |
| ESPHome / ESP32 voice satellites | `verified` | Low-cost local voice satellite hardware path. |
| Linux Voice Assistant | `needs verification` | Possible Wyoming/HA-adjacent voice satellite lineage; verify exact project. |
| Leon | `verified` | Open-source personal assistant reference. |
| Home Guardian | `needs verification` | Academic/offline smart-home voice prototype; verify primary source before relying. |
| Mycroft AI | `stale/deprecated` | Important voice-assistant lineage; OpenVoiceOS is the more current path. |
| Snips | `stale/deprecated` | France-origin privacy voice lineage; acquired/legacy status. |
| Mingyuyue | `candidate/unverified` | Chinese modular voice framework claim needs official source check. |
| wukong-robot | `verified` | Chinese voice assistant/smart speaker project with HA/MQTT relevance. |

Goni implication:

- Home Assistant is the practical smart-home substrate.
- Wyoming, Piper, Whisper/Faster Whisper, and openWakeWord are the modular
  local voice stack.
- LiveKit/Pipecat are separate realtime-agent infrastructure and should not be
  collapsed into smart-home automation.

### H) Hardware, edge, and deployment substrate

| Project / platform | Confidence | Goni relevance |
| --- | --- | --- |
| NVIDIA CUDA / TensorRT | `verified` | GPU acceleration and production throughput path. |
| Apple Silicon / MLX | `verified` | Local Apple-device inference path and edge development reference. |
| Intel NUC | `verified` | Small-form-factor node class. |
| Beelink | `verified` | Mini-PC node class. |
| Coral TPU | `verified` | Edge accelerator path for small models/signals. |
| ESP32-S3 | `verified` | Voice satellite microcontroller class. |
| Off Grid-style phone stacks | `candidate/unverified` | Phone-local AI pattern; needs concrete project/source before use. |
| exo clustering | `verified` | Home/edge device clustering reference, also listed under runtime. |

Hardware conclusion:

- The physical substrate determines which runtime candidates are realistic.
- vLLM/SGLang/TensorRT-LLM are GPU-server oriented.
- llama.cpp, MLX, Ollama-like UX, and Xinference-style wrappers are more
  plausible on edge, APU, CPU, or desktop-class nodes.

### Selected official source anchors

This list is intentionally biased toward official sites, official docs,
official repositories, standards pages, or primary papers. It is not an
endorsement list.

Runtime/model serving:

- Ollama: https://github.com/ollama/ollama
- llama.cpp: https://github.com/ggml-org/llama.cpp
- LocalAI: https://localai.io/
- LM Studio: https://lmstudio.ai/
- Jan: https://jan.ai/
- vLLM: https://docs.vllm.ai/
- SGLang: https://docs.sglang.ai/
- TensorRT-LLM: https://github.com/NVIDIA/TensorRT-LLM
- Hugging Face TGI: https://github.com/huggingface/text-generation-inference
- Mistral.rs: https://github.com/EricLBuehler/mistral.rs
- exo: https://github.com/exo-explore/exo
- KTransformers: https://github.com/kvcache-ai/ktransformers
- Hugging Face Transformers: https://github.com/huggingface/transformers
- LMDeploy: https://github.com/InternLM/lmdeploy
- Xinference: https://github.com/xorbitsai/inference
- Xuanwu CLI / 玄武CLI: https://xw.tsingmao.com/cli-reference

UI/application:

- Open WebUI: https://github.com/open-webui/open-webui
- AnythingLLM: https://anythingllm.com/
- LibreChat: https://github.com/danny-avila/LibreChat
- Msty: https://msty.app/
- Dify: https://github.com/langgenius/dify
- LobeChat: https://github.com/lobehub/lobe-chat
- GPT4All: https://github.com/nomic-ai/gpt4all
- Chatbox AI: https://github.com/chatboxai/chatbox
- BionicGPT: https://github.com/bionic-gpt/bionic-gpt
- Khoj: https://github.com/khoj-ai/khoj
- PrivateGPT: https://github.com/zylon-ai/private-gpt
- Chatbot UI: https://github.com/mckaywrigley/chatbot-ui
- Alibaba ChatUI: https://github.com/alibaba/ChatUI

Agent/workflow/tooling:

- OpenClaw docs: https://docs.openclaw.ai/
- OpenDAN: https://github.com/OpenDAN-Personal-AI-OS/OpenDAN
- n8n: https://github.com/n8n-io/n8n
- LangGraph: https://github.com/langchain-ai/langgraph
- CrewAI: https://github.com/crewAIInc/crewAI
- AutoGen: https://github.com/microsoft/autogen
- AG2: https://github.com/ag2ai/ag2
- Flowise: https://github.com/FlowiseAI/Flowise
- LangFlow: https://github.com/langflow-ai/langflow
- Open Interpreter: https://github.com/openinterpreter/open-interpreter
- LobsterAI: https://github.com/netease-youdao/lobsterai
- MCP: https://modelcontextprotocol.io/
- MCP organization: https://github.com/modelcontextprotocol
- Composio: https://github.com/ComposioHQ/composio
- E2B: https://e2b.dev/
- Daytona: https://www.daytona.io/
- browser-use: https://github.com/browser-use/browser-use
- Stagehand: https://github.com/browserbase/stagehand
- Playwright: https://playwright.dev/
- Google A2A: https://github.com/google-a2a/A2A
- Google ADK: https://github.com/google/adk-python

Memory/RAG/observability/router:

- mem0: https://github.com/mem0ai/mem0
- Zep: https://github.com/getzep/zep
- Chroma: https://github.com/chroma-core/chroma
- Qdrant: https://github.com/qdrant/qdrant
- Milvus: https://github.com/milvus-io/milvus
- Weaviate: https://github.com/weaviate/weaviate
- pgvector: https://github.com/pgvector/pgvector
- Redis: https://github.com/redis/redis
- Neon: https://neon.com/
- LlamaIndex: https://github.com/run-llama/llama_index
- RAGFlow: https://github.com/infiniflow/ragflow
- Unstructured: https://github.com/Unstructured-IO/unstructured
- Docling: https://github.com/docling-project/docling
- Marker: https://github.com/datalab-to/marker
- Neo4j: https://neo4j.com/
- Kuzu: https://github.com/kuzudb/kuzu
- Marqo: https://github.com/marqo-ai/marqo
- LanceDB: https://github.com/lancedb/lancedb
- LiteLLM: https://github.com/BerriAI/litellm
- Portkey: https://github.com/Portkey-AI/gateway
- Helicone: https://github.com/Helicone/helicone
- Langfuse: https://github.com/langfuse/langfuse
- AgentOps: https://github.com/AgentOps-AI/agentops
- LangSmith: https://www.langchain.com/langsmith
- Arize Phoenix: https://github.com/Arize-ai/phoenix

Voice/home/hardware:

- Home Assistant: https://www.home-assistant.io/
- Nabu Casa: https://www.nabucasa.com/
- Home Assistant Voice: https://www.home-assistant.io/voice_control/
- Rhasspy: https://rhasspy.readthedocs.io/
- OpenVoiceOS: https://openvoiceos.org/
- Willow: https://heywillow.io/
- Wyoming: https://github.com/rhasspy/wyoming
- Whisper: https://github.com/openai/whisper
- Faster Whisper: https://github.com/SYSTRAN/faster-whisper
- Piper: https://github.com/rhasspy/piper
- openWakeWord: https://github.com/dscripka/openWakeWord
- LiveKit: https://github.com/livekit/livekit
- Pipecat: https://github.com/pipecat-ai/pipecat
- Daily: https://www.daily.co/
- ESPHome: https://esphome.io/
- Leon: https://github.com/leon-ai/leon
- Mycroft AI: https://github.com/MycroftAI/mycroft-core
- Snips legacy: https://github.com/snipsco
- wukong-robot: https://github.com/wzpan/wukong-robot
- NVIDIA CUDA: https://developer.nvidia.com/cuda-zone
- NVIDIA TensorRT: https://developer.nvidia.com/tensorrt
- Apple MLX: https://github.com/ml-explore/mlx
- Intel NUC: https://www.intel.com/content/www/us/en/products/details/nuc.html
- Beelink: https://www.bee-link.com/
- Coral TPU: https://coral.ai/
- ESP32-S3: https://www.espressif.com/en/products/socs/esp32-s3

---

## 4. Original adjacent categories and examples

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

For Goni, these should be modeled as optional seats or adapters behind kernel
mediation, not as the control plane itself.

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

## 5. Personal/local AI OS candidates (OS framing)

Closest to "real OS" (installable system or explicit OS substrate):

- Olares: https://github.com/beclab/Olares
- pAI-OS: https://paios.org/
- AIOS (agent OS research): https://github.com/agiresearch/AIOS

OS-like in marketing, but closer to agent appliance/platform:

- OpenClaw / Moltbot (Clawdbot): https://openclaw.ai/

OS-like framing, but not local-first:

- Warmwind OS: https://about.warmwind.space/we-built-an-operating-system-for-ai-but-is-it-really-one/

---

## 6. Local-first shift: hardware and economics signals

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

## 7. Productivity + proactivity backbone (primary sources only)

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

---

## 8. Operational Readiness and Adoption

### 7.1 What researchers would research

- Runtime utilization models: local execution share vs remote escalation share.
- Throughput and latency envelopes under realistic duty cycles.
- Operational tradeoffs: privacy exposure, latency, and uptime dependence.

### 7.2 Local-first readiness

- Study when local-first execution is sufficient for daily operator tasks.
- Model thresholds where remote escalation materially improves outcomes.

### 7.3 Externalities and incentive alignment

- Quantify privacy externalities (data leakage risk) and how local-first changes
  them.
- Measure trust gains from auditability and sovereignty primitives.

### 7.4 Deliverables

- Technology readiness matrix for a Goni-like appliance.
- Adoption story grounded in measurable user outcomes (time saved, risk reduced).

### 7.5 Enables Goni

This field explains why a local personal AI OS becomes the default and what
minimum "it just works" reliability/UX thresholds must be hit.

### 7.6 Primary anchors

- Armbrust et al. (cloud economics and trade-offs).
- Licklider (human-computer symbiosis vision: tight coupling of human + machine,
  historically motivating "personal computing" evolution).
