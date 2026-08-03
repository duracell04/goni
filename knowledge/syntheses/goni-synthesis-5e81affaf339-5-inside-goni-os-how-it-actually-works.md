---
id: GONI-SYNTHESIS-5E81AFFAF339
title: '5. Inside Goni OS: How It Actually Works'
type: synthesis
status: draft
implementation_state: specified_only
proposition: Under the narrative, Goni OS is pretty simple conceptually.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: '5. Inside Goni OS: How It Actually Works'
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 5. Inside Goni OS: How It Actually Works

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Inside Goni OS: How It Actually Works

Under the narrative, Goni OS is pretty simple conceptually. It has three big responsibilities:

1. **Understand your world** (ingestion + memory).  
2. **Decide what matters and what to do** (judgement + scheduling).  
3. **Act within strict boundaries** (permissions + routing).

In the exocortex model, durable memory is the second brain and the governed
active layer is the third brain. The human remains the first brain: Goni can
rank, plan, and recommend, but goals, values, delegated authority, correction,
and veto stay with the operator.
