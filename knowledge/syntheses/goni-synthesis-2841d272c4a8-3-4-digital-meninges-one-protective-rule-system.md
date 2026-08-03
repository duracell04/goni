---
id: GONI-SYNTHESIS-2841D272C4A8
title: '3.4 Digital meninges: one protective rule system'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'This is where "personal" becomes **governed**: "You can read my email automatically." "You can draft replies to certain types of messages." "You can send an email only if I approve it." "You can never touch my bank account without biometric confirmation." Goni OS has a **sudo layer** - a permission broker that turns free-wheeling "agents" into tools with clearly defined powers.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: '3.4 Digital meninges: one protective rule system'
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 3.4 Digital meninges: one protective rule system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Digital meninges: one protective rule system

This is where "personal" becomes **governed**:

- "You can read my email automatically."
- "You can draft replies to certain types of messages."
- "You can send an email only if I approve it."
- "You can never touch my bank account without biometric confirmation."

Goni OS has a **sudo layer** - a permission broker that turns free-wheeling "agents" into tools with clearly defined powers.

The result is a relationship that feels less like a toy model and more like a junior colleague you're training up:

> "Here's what you're allowed to see.  
> Here's what you're allowed to do.  
> Ask me when you're not sure."

The architecture groups these protections as **digital meninges**:

- a hard security perimeter for identity, secrets, storage, sandboxing, and
  network egress;
- an orchestration layer for capabilities, context movement, budgets, and
  scheduling;
- provenance and epistemic controls around evidence, claims, memory, and
  receipts.

These are reader-facing names for existing Goni controls, not extra services or
a second set of contracts. The detailed mapping is in the
[Cognitive Exocortex Model](/blueprint/20-system/60-cognitive-exocortex-model.md).

---
