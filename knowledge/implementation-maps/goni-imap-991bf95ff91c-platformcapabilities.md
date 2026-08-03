---
id: GONI-IMAP-991BF95FF91C
title: PlatformCapabilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: capability_id = row_id Fields: timestamp: timestamp(ms), device_id: fixed_size_binary[16], npu_shape_buckets?: list<utf8>, supported_quant?: list<utf8> Notes: Static or infrequently changing capability snapshot.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: PlatformCapabilities
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# PlatformCapabilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### PlatformCapabilities
- PK: `capability_id = row_id`
- Fields: `timestamp: timestamp(ms)`, `device_id: fixed_size_binary[16]`,
  `npu_shape_buckets?: list<utf8>`, `supported_quant?: list<utf8>`
- Notes: Static or infrequently changing capability snapshot.
