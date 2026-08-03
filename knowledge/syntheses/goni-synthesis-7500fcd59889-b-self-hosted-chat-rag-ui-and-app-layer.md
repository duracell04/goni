---
id: GONI-SYNTHESIS-7500FCD59889
title: B) Self-hosted chat, RAG UI, and app layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: These projects are useful surfaces for chat, document workflows, agent testing, or operator UX.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: B) Self-hosted chat, RAG UI, and app layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# B) Self-hosted chat, RAG UI, and app layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
