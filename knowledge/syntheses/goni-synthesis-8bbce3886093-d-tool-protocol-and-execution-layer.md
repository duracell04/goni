---
id: GONI-SYNTHESIS-8BBCE3886093
title: D) Tool protocol and execution layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: | Project / protocol | Confidence | Goni relevance | | MCP servers / Model Context Protocol | verified | Open protocol for connecting AI apps to tools, data sources, and workflows.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: D) Tool protocol and execution layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# D) Tool protocol and execution layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
