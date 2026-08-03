---
id: AGENT-MANIFEST-01
title: AGENT-02 - Agent Manifest
type: specification
status: draft
implementation_state: specified_only
proposition: "\uFEFF--- id: AGENT-MANIFEST-01 type: SPEC status: specified_only DOC-ID: AGENT-MANIFEST-01 Status: Specified only / roadmap The agent manifest is the single source of truth for agent identity, triggers, capabilities, and budgets."
domains:
- agent
- specs
aliases:
- AGENT-MANIFEST
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-manifest.md
  heading: AGENT-02 - Agent Manifest
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# AGENT-02 - Agent Manifest

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# AGENT-02 - Agent Manifest
﻿---
id: AGENT-MANIFEST-01
type: SPEC
status: specified_only
---
DOC-ID: AGENT-MANIFEST-01
Status: Specified only / roadmap

The agent manifest is the single source of truth for agent identity, triggers,
capabilities, and budgets. The kernel refuses to instantiate agents whose
requested capabilities exceed policy.

Canonical data contract: `blueprint/software/50-data/51-schemas-mvp.md` (MANIFEST-02).
