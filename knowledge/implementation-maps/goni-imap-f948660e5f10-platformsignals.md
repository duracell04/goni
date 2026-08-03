---
id: GONI-IMAP-F948660E5F10
title: PlatformSignals
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: signal_id = row_id Fields: timestamp: timestamp(ms), device_id: fixed_size_binary[16], session_id?: fixed_size_binary[16], thermal_throttled?: bool, thermal_domain?: dict<uint8, utf8>, dvfs_state?: dict<uint8, utf8>, free_ram_mb?: uint32, swap_in_mb?: uint32, major_faults?: uint32, bytes_written_today?: int64, waf_estimate?: float32, ssd_health?: float32,'
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
  heading: PlatformSignals
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# PlatformSignals

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### PlatformSignals
- PK: `signal_id = row_id`
- Fields: `timestamp: timestamp(ms)`, `device_id: fixed_size_binary[16]`, `session_id?: fixed_size_binary[16]`,
  `thermal_throttled?: bool`, `thermal_domain?: dict<uint8, utf8>`, `dvfs_state?: dict<uint8, utf8>`,
  `free_ram_mb?: uint32`, `swap_in_mb?: uint32`, `major_faults?: uint32`,
  `bytes_written_today?: int64`, `waf_estimate?: float32`, `ssd_health?: float32`,
  `npu_shape_buckets?: list<utf8>`, `supported_quant?: list<utf8>`,
  `gpu_active?: bool`, `gpu_wake_ms_p95?: uint32`,
  `solver_wake_count?: uint32`, `solver_active_ms?: uint32`, `encoder_active_ms?: uint32`
- Notes: Optional fields support partial telemetry. Use dict enums for domain/state tags.
