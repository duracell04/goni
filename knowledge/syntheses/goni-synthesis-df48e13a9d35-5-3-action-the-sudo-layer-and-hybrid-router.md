---
id: GONI-SYNTHESIS-DF48E13A9D35
title: '5.3 Action: the sudo layer and hybrid router'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'When a job needs to act, two things happen: The **sudo layer** checks permissions: Is this job allowed to read this table?'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: '5.3 Action: the sudo layer and hybrid router'
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 5.3 Action: the sudo layer and hybrid router

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.3 Action: the sudo layer and hybrid router

When a job needs to act, two things happen:

1. The **sudo layer** checks permissions:
   - Is this job allowed to read this table?
   - Is it allowed to draft messages?
   - Is it allowed to send them?
   - Does this action require your approval or biometric confirmation?

2. The **router** chooses where to run intelligence:
   - Local models for:
     - classification, short summaries, anomaly detection.
   - Cloud models for:
     - nuanced writing, structured reports, complex reasoning,
     - only after anonymising sensitive data and under budget/privacy policies you control.

So the system doesn't just "call an LLM". It:

- uses local models to do the heavy, repetitive, privacy-sensitive reading,
- uses cloud models sparingly to get the "this sounds like me" polish when needed.

---
