---
id: GONI-PROPOSAL-3AD5E01EB3D1
title: Personal-agent frameworks and applications
type: proposal
status: draft
implementation_state: specified_only
proposition: '| Project | Maturity and local path | Memory and automation | External dependencies / telemetry | Relevance to Goni | | QwenPaw | Active AgentScope-team personal agent with a web console, TUI, desktop build, bundled llama.cpp path, Ollama and LM Studio support, and optional cloud providers | Working context, verbatim history, ReMe-based Markdown knowledge, cron, heartbeat, channels, and multi-agent features | Local models require no API key; qwenpaw init has anonymous telemetry, and --defaults a'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Personal-agent frameworks and applications
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Personal-agent frameworks and applications

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
