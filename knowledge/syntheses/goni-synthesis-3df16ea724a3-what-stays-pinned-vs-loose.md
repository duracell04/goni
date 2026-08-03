---
id: GONI-SYNTHESIS-3DF16EA724A3
title: What stays pinned vs loose
type: synthesis
status: draft
implementation_state: specified_only
proposition: '| Where | What is allowed to be specific / pinned | What must stay loose or move out | | README top level | Only the gut-punch feeling we are chasing right now (currently: perfect offline memory of the repo) + the 60-second run command | Never model sizes, never exact hardware specs, never timelines, never “30–40B”, never “silent”, never “6–8 L” |'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-agility-rules.md
  heading: What stays pinned vs loose
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# What stays pinned vs loose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## What stays pinned vs loose

| Where | What is allowed to be specific / pinned | What must stay loose or move out |
|-------|------------------------------------------|----------------------------------|
| README top level | Only the gut-punch feeling we are chasing right now (currently: perfect offline memory of the repo) + the 60-second run command | Never model sizes, never exact hardware specs, never timelines, never “30–40B”, never “silent”, never “6–8 L” |
| blueprint/docs/goni-story.md & blueprint/docs/goni-whitepaper.md | Vision, invariants, planes, math — these can be bold and long-term | Can say “we believe future APUs will…” but never “Goni will ship with Ryzen AI Max+ 395” |
| /hardware/10-requirements.md | Current reference envelope only (volume, power, noise, RAM ceiling) — written as soft targets with dates: “target ≤ 8 L (2026), ≤ 250 W sustained (2026)” | Updated only when a new reference rig proves the old numbers are wrong |
| /hardware/90-decisions.md & /software/90-decisions.md | Current concrete choices (e.g. “Strix Halo class APU for 2026 dev rigs”, “all-MiniLM-L6-v2 for prototype track 01”) — must have a date and a “valid until” | Anyone can propose a new decision with new numbers + proof (photo, measurement, benchmark) |
| goni-prototype-lab:prototype/** folders | Pin everything: exact model IDs, exact chunk sizes, exact seeds, exact eval datasets | These folders are allowed to be brutally specific because they are throw-away experiments |
| Everything else (goni-prototype-lab:software/kernel, goni-prototype-lab:config/, old docker-compose, etc.) | Treated as scratch paper — can be changed or deleted without ceremony | If something graduates from prototype → reference design, it moves out with a dated decision |
