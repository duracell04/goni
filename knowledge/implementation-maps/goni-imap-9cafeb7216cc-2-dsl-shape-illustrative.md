---
id: GONI-IMAP-9CAFEB7216CC
title: 2. DSL Shape (illustrative)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 2. DSL Shape (illustrative)
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
  heading: 2. DSL Shape (illustrative)
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# 2. DSL Shape (illustrative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. DSL Shape (illustrative)

```rust
define_tables! {
    table Docs {
        plane: Plane::Knowledge,
        kind: "Docs",
        fields: {
            doc_id: FixedSizeBinary(16),
            source_uri: LargeUtf8,
            mime_type: Utf8,
            title: Utf8,
            tags: List<Utf8>,
            metadata: Map<Utf8, Utf8>,
        }
    },

    table Requests {
        plane: Plane::Control,
        kind: "Requests",
        fields: {
            request_id: FixedSizeBinary(16),
            session_id: FixedSizeBinary(16),
            task_class: Dict(UInt8, Utf8),
            prompt_hash: FixedSizeBinary(32),
            budget_tokens: UInt32,
        }
    },

    // ... remaining tables ...
}
```
