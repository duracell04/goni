---
id: GONI-SPEC-3140D38DA0F4
title: 8. Memory separation
type: specification
status: draft
implementation_state: specified_only
proposition: Observation and extraction do not imply memory.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 8. Memory separation
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 8. Memory separation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Memory separation

Observation and extraction do not imply memory. Screen frames, OCR text,
accessibility trees, summaries, embeddings, layout facts, screenshots, and audio
transcripts may enter durable memory only through a memory grant.

Memory writes MUST preserve source refs, permission scope, memory class,
retention policy, parser/extraction basis, and receipt refs. Memory layers that
record continuously are still governed memory writers; being passive does not
make storage authority ambient.
