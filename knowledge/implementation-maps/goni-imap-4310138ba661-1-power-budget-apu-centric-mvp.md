---
id: GONI-IMAP-4310138BA661
title: 1. Power budget (APU-centric MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Assumptions (order-of-magnitude): APU board sustained: ~120–160 W class (depends on firmware limits and workload) Peaks (short): higher bursts are possible NVMe SSDs: 5–10 W each under heavy IO (short peaks higher) Fans / MCU / misc: 5–15 W Always-on encoder loop + sensor ingest: budgeted steady-state draw (measure, not assume)'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/40-electronics/power-and-psu.md
  heading: 1. Power budget (APU-centric MVP)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Power budget (APU-centric MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Power budget (APU-centric MVP)

Assumptions (order-of-magnitude):

- APU board sustained: ~120–160 W class (depends on firmware limits and workload)
- Peaks (short): higher bursts are possible
- NVMe SSDs: 5–10 W each under heavy IO (short peaks higher)
- Fans / MCU / misc: 5–15 W
- Always-on encoder loop + sensor ingest: budgeted steady-state draw (measure, not assume)

**Design for:** ~250 W sustained worst-case, with headroom for spikes.

---
