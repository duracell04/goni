---
id: GONI-IMAP-A7DED0B58365
title: 3.1 Request
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We model a request as: R = (\text{messages}, \text{model}, \text{tools}, \text{stream}, \text{extras}) **messages** – ordered list of chat messages \(m_k = (\text{role}, \text{content}, \text{name?})\), with role ∈ {system, user, assistant, tool}.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 3.1 Request
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.1 Request

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Request

We model a request as:

R = (\text{messages}, \text{model}, \text{tools}, \text{stream}, \text{extras})


* **messages** – ordered list of chat messages
  \(m_k = (\text{role}, \text{content}, \text{name?})\), with role ∈ {system, user, assistant, tool}.
* **model** – string identifier, e.g. "goni-small", "goni-large", "goni-auto".
* **tools** *(optional)* – OpenAI-style function calling schema.
* **stream** – boolean; 	rue → server-sent events; 
alse → single JSON.
* **extras** *(Goni extensions)* – optional fields such as:

  * goni_profile: "interactive" | "background" | "maintenance" (hint for \(\mathcal{K}\)),
  * goni_rag_mode: "off" | "auto" | "strict" (hint for \(\mathcal{X}\)),
  * goni_interaction_mode: "delegation" | "co_creation" (optional override or
    inspection hint),
  * goni_output_schema: client-declared desired output contract,
  * goni_preview_only: request a reconstruction/approval preview without
    committing side effects.

We do not prescribe a full JSON schema here; the implementation follows OpenAI’s spec plus these extensions.
