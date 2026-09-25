---
id: EVID-JIANG2024-LONGLLMLINGUA
title: 'Source claim: LongLLMLingua'
type: evidence
status: draft
implementation_state: not_applicable
proposition: LongLLMLingua reports that task-aware prompt compression can reduce long-context cost and latency while also addressing position-sensitive use of key information on evaluated benchmarks.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SPEC-A29366F3E2EA
- type: supports
  target: GONI-PRINCIPLE-B40DDEFD1872
sources:
- SRC-JIANG2024-LONGLLMLINGUA
artifacts: []
uncertainty: Improvements are benchmark-, model-, and compression-policy-specific and do not imply that compression is always preferable to native long-context reading.
legacy: []
---

# Source claim: LongLLMLingua

LongLLMLingua targets long-context inference where cost, latency, and position
bias can jointly degrade useful access to key information.

## Goni relevance

It supports evaluating information density and placement, not context length
alone, when compiling bounded inference context.

## Boundary

Goni should compare compression against native-window reading, retrieval-only,
and hybrid strategies instead of assuming one universally superior lane.
