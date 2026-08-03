---
id: TOOL-01
title: TOOL-01 - Tool Capability API (Auditable Syscalls)
type: specification
status: draft
implementation_state: specified_only
proposition: "\uFEFF--- id: TOOL-01 type: SPEC status: specified_only DOC-ID: TOOL-01 Status: Specified only / roadmap Tools are kernel-mediated, capability-scoped syscalls."
domains:
- specs
aliases:
- TOOL-CAPABILITY-API
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: TOOL-01 - Tool Capability API (Auditable Syscalls)
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# TOOL-01 - Tool Capability API (Auditable Syscalls)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# TOOL-01 - Tool Capability API (Auditable Syscalls)
﻿---
id: TOOL-01
type: SPEC
status: specified_only
---
DOC-ID: TOOL-01
Status: Specified only / roadmap

Tools are kernel-mediated, capability-scoped syscalls. All tool invocations are
audited and attributable to an agent and a state snapshot.
