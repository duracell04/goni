---
id: GONI-SYNTHESIS-47F21BE73E86
title: Operational auditability (PAL framing)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Goni adopts the Personal AI Ledger (PAL) framing for personal agent safety: the model''s neural internals may remain opaque, but its interactions with the user''s world can be made operationally auditable.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Operational auditability (PAL framing)
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Operational auditability (PAL framing)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Operational auditability (PAL framing)

Goni adopts the Personal AI Ledger (PAL) framing for personal agent safety:
the model's neural internals may remain opaque, but its interactions with the
user's world can be made operationally auditable. The practical question is not
only "why did this token appear?", but "what context did the system use, which
memory or tool boundary was crossed, what was proposed, who approved it, what
changed, and how can the change be inspected or reversed?"

In Goni terms, PAL is not a separate product layer. It is the academic framing
for the local reference-monitor kernel, capability-mediated tools, provenance
receipts, policy gates, and rollback references already used throughout the
blueprint:

```text
The AI may think freely,
act only through receipts,
and change itself only through versioned adaptation receipts.
```

This is operational auditability, not mechanistic interpretability. Hash-linked
receipts provide tamper evidence, not truth. Git-backed rollback applies to
versioned file state, not to every real-world consequence such as sent messages,
published content, or external social effects. For those actions, Goni records
compensation or repair references rather than promising perfect undo.
