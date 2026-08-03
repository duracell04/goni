---
id: GONI-SYNTHESIS-00B5C471FF0F
title: 2.2 Local as in data gravity
type: synthesis
status: draft
implementation_state: specified_only
proposition: Your life isn't just one chat history.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: 2.2 Local as in data gravity
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 2.2 Local as in data gravity

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Local as in data gravity

Your life isn't just one chat history. It's:

- emails, messages, and documents,
- calendars and tasks,
- bank transactions and invoices,
- health exports and logs,
- random CSVs and scraped reports.

Goni ingests all of that into a **single, structured memory layer**, built on Apache Arrow. Think of it as:

> "A proper, columnar database of your life, sitting on a disk in your home."

No separate SQLite file per app, no random vector stores per feature.  
Just one **Arrow spine** that everything reads from.
