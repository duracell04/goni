---
id: GONI-IMAP-B23C6DC7C543
title: 1. DSL Tokens (frozen v1.0)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Planes: Plane::Knowledge | Plane::Context | Plane::Control | Plane::Execution Types: FixedSizeBinary(16), Utf8, LargeUtf8, DictU8Utf8, ListUtf8, MapUtf8Utf8, Int32, UInt32, Int64, UInt16, UInt8, Float32, Float64, Boolean, FixedSizeListF32(N), TimestampMsUtc.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/53-schema-dsl-and-macros.md
  heading: 1. DSL Tokens (frozen v1.0)
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# 1. DSL Tokens (frozen v1.0)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. DSL Tokens (frozen v1.0)

- Planes: `Plane::Knowledge | Plane::Context | Plane::Control | Plane::Execution`
- Types: `FixedSizeBinary(16)`, `Utf8`, `LargeUtf8`, `DictU8Utf8`, `ListUtf8`, `MapUtf8Utf8`, `Int32`, `UInt32`, `Int64`, `UInt16`, `UInt8`, `Float32`, `Float64`, `Boolean`, `FixedSizeListF32(N)`, `TimestampMsUtc`.
- Note: `TimestampMsUtc` maps to `timestamp(ms, UTC)` in schema docs.
