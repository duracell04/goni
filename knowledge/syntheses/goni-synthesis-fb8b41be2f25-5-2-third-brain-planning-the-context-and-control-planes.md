---
id: GONI-SYNTHESIS-FB8B41BE2F25
title: '5.2 Third-brain planning: the context and control planes'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'On top of that memory, Goni OS runs: a **context layer** that selects the minimum relevant slice of data for any given question or job, a **control plane** that schedules and runs jobs.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: '5.2 Third-brain planning: the context and control planes'
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 5.2 Third-brain planning: the context and control planes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.2 Third-brain planning: the context and control planes

On top of that memory, Goni OS runs:

- a **context layer** that selects the minimum relevant slice of data for any given question or job,
- a **control plane** that schedules and runs jobs.

Jobs look like this, conceptually:

- **Nightly Inbox Triage**
  - Input: emails from the last 24 hours.
  - Steps: classify by importance, draft obvious replies, flag anomalies.
  - Output: morning digest + drafts, stored back into the spine.

- **Weekly Logistics Check**
  - Input: events + tasks for the next 14 days.
  - Steps: detect overbooked days, conflicts, "crunch weeks".
  - Output: suggestions for rescheduling, summary of risks.

- **Subscription Watchdog**
  - Input: recurring transactions.
  - Steps: detect new or increased subscriptions.
  - Output: alerts, draft cancellation emails, updated budget view.

Behind these is a **consolidation loop** inspired by Generative Agents: Observation (raw events) → Reflection (summaries + long-term facts) → Planning (jobs/actions). Nightly runs keep the box "thinking" even when you're offline, so plans and drafts stay aligned with what actually happened.

These jobs run **even if you never open the UI**. They're the difference between "AI as a prompt box" and "AI as a background service".
