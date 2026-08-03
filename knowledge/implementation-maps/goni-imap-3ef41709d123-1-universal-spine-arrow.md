---
id: GONI-IMAP-3EF41709D123
title: 1. Universal Spine (Arrow)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Every table is Spine + Payload.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/20-spine-and-ids.md
  heading: 1. Universal Spine (Arrow)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Universal Spine (Arrow)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Universal Spine (Arrow)

```arrow
struct Spine {
  row_id         : fixed_size_binary[16]  // UUIDv7 – globally unique, monotonic
  tenant_id      : fixed_size_binary[16]  // single-node: fixed 000…001
  plane          : uint8                  // 0=𝒜 1=𝒳 2=𝒦 3=ℰ
  kind           : dictionary<uint8, utf8> // table name for debugging
  schema_version : uint16                 // bump on breaking change
  ts_created     : timestamp[ms, UTC]
  ts_valid_from  : timestamp[ms, UTC]     // SCD-2 facts
}
```

Every table is `Spine + Payload`. `row_id` is the canonical primary key; table-specific ID fields are aliases.
