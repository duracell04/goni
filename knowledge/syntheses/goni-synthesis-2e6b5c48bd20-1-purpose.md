---
id: GONI-SYNTHESIS-2E6B5C48BD20
title: 1. Purpose
type: synthesis
status: draft
implementation_state: specified_only
proposition: Give builders a fast way to **compare models side-by-side** on real Goni tasks (chat, coding, RAG, tools).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-lab.md
  heading: 1. Purpose
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose
- Give builders a fast way to **compare models side-by-side** on real Goni tasks (chat, coding, RAG, tools).
- Produce **structured evidence** (latency, cost, refusals, faithfulness) that feeds `goni-router` thresholds and `goni-prototype-lab:config/council.yaml` seating.
- Stay **local-first**: include LM Studio / Ollama seats and offline-only runs; use cloud seats only when allowed.
