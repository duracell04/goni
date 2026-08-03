---
id: GONI-SYNTHESIS-916DD241D0F9
title: C) Agent gateway, operator, and orchestration layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: This layer is where tool use, channels, memory, planning, workflows, and multi-agent behavior get confused.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: C) Agent gateway, operator, and orchestration layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# C) Agent gateway, operator, and orchestration layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### C) Agent gateway, operator, and orchestration layer

This layer is where tool use, channels, memory, planning, workflows, and
multi-agent behavior get confused. Goni should separate them:

- channel gateway: user reachability and sessions,
- reasoning/orchestration: state machine and tool loop,
- kernel: authority, mediation, receipts, policy, and rollback.

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| OpenClaw | `verified` | Agent/channel gateway reference for messaging, sessions, tools, and routing. |
| QwenPaw | `verified` | AgentScope-team personal agent with local llama.cpp/Ollama/LM Studio paths, Markdown memory, channels, scheduling, and multi-agent features; useful as a reference or mediated seat. |
| nanobot | `verified` | Lightweight Python personal-agent runtime with WebUI, self-hosted endpoints, long-term memory, tools, subagents, and scheduled automation. |
| Hivekeep | `verified` | Self-hosted multi-agent household/team application with continuous sessions, hybrid memory, triggers, connected accounts, and encrypted secret handling. |
| OpenHuman | `verified` | Early-beta desktop orchestrator with local memory and optional local models, but a managed service path remains part of the default product story. |
| meld | `verified` | Local-first Markdown/Obsidian knowledge agent with Ollama/BYOK modes, autonomous note edits, and Git safety commits. |
| Elroy | `verified` | Scriptable terminal memory, reminder, document, and goal assistant with an MCP surface. |
| ZeroClaw | `verified` | Rust single-binary agent runtime with local/hosted providers, tools, MCP, and many communication channels. |
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
