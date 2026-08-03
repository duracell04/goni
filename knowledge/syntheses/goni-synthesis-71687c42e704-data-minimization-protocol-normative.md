---
id: GONI-SYNTHESIS-71687C42E704
title: Data minimization protocol (normative)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'When routing to the Council, the orchestrator MUST: Minimize payloads: remove non-essential chunks, collapse long artifacts to summaries, and apply the active redaction profile.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: Data minimization protocol (normative)
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# Data minimization protocol (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Data minimization protocol (normative)

When routing to the Council, the orchestrator MUST:
- Minimize payloads: remove non-essential chunks, collapse long artifacts to summaries, and apply the active redaction profile.
- Attach a provenance manifest: `request_id`, `prompt_hash`, `source_context_id`, and chunk IDs used.
- Emit a `RedactionEvents` row with before/after hashes and a structured summary (no raw text).
- Obey the configured egress mode from `goni-prototype-lab:config/council.yaml` (local-only, structured-only, redacted, or user-approved full context).
- Emit or update an `llm_route` receipt object that records why local execution
  was insufficient and what privacy class, if any, left the node.

These rules are enforced at the Network Gate and logged through the Control plane.
