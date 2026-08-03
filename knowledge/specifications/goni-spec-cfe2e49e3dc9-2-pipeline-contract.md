---
id: GONI-SPEC-CFE2E49E3DC9
title: 2. Pipeline contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'CDC MUST implement this logical pipeline: Every accepted learning MUST produce the triad: This links state update, provenance, and future falsifiability.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 2. Pipeline contract
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 2. Pipeline contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Pipeline contract

CDC MUST implement this logical pipeline:

```text
interaction stream
-> thread reconstruction
-> draft/final alignment
-> correction delta extraction
-> delta classification
-> candidate preference rule
-> evidence aggregation
-> contradiction check
-> human learning card
-> accepted MemoryEntry
-> regression test or harness rule
-> learning receipt
```

Every accepted learning MUST produce the triad:

```text
MemoryEntry + Receipt + RegressionTest
```

This links state update, provenance, and future falsifiability.
