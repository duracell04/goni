---
id: GONI-SYNTHESIS-40887F2A1811
title: '5.1 Second-brain memory: the Arrow spine'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Instead of scattering data across dozens of app-specific stores, Goni OS builds a **single schema** for your life: messages (emails, DMs, notifications), events (calendar, reminders), transactions (bank, cards, invoices), artifacts (files, docs, notes), signals (computed metrics: meeting load, recurring expenses, etc.).'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-story.md
  heading: '5.1 Second-brain memory: the Arrow spine'
  revision: 21a992cf402108cf9aad54faaec42d06d9ca0f3a
---

# 5.1 Second-brain memory: the Arrow spine

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 Second-brain memory: the Arrow spine

Instead of scattering data across dozens of app-specific stores, Goni OS builds a **single schema** for your life:

- `messages` (emails, DMs, notifications),
- `events` (calendar, reminders),
- `transactions` (bank, cards, invoices),
- `artifacts` (files, docs, notes),
- `signals` (computed metrics: meeting load, recurring expenses, etc.).

It stores them in a columnar format (Apache Arrow) so that:

- agents can ask "show me all transactions > $X in the last 90 days" efficiently,
- jobs can scan for patterns without re-parsing blobs,
- everything shares the same source of truth.

It's like giving your life a proper database, instead of treating it as an afterthought.
