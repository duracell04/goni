# Goni Agility Rules

This keeps Goni flexible while letting contributors be specific in the right places.

## What stays pinned vs loose

| Where | What is allowed to be specific / pinned | What must stay loose or move out |
|-------|------------------------------------------|----------------------------------|
| README top level | Only the gut-punch feeling we are chasing right now (currently: perfect offline memory of the repo) + the 60-second run command | Never model sizes, never exact hardware specs, never timelines, never “30–40B”, never “silent”, never “6–8 L” |
| docs/goni-story.md & docs/goni-whitepaper.md | Vision, invariants, planes, math — these can be bold and long-term | Can say “we believe future APUs will…” but never “Goni will ship with Ryzen AI Max+ 395” |
| /hardware/10-requirements.md | Current reference envelope only (volume, power, noise, RAM ceiling) — written as soft targets with dates: “target ≤ 8 L (2026), ≤ 250 W sustained (2026)” | Updated only when a new reference rig proves the old numbers are wrong |
| /hardware/90-decisions.md & /software/90-decisions.md | Current concrete choices (e.g. “Strix Halo class APU for 2026 dev rigs”, “all-MiniLM-L6-v2 for prototype track 01”) — must have a date and a “valid until” | Anyone can propose a new decision with new numbers + proof (photo, measurement, benchmark) |
| prototype/** folders | Pin everything: exact model IDs, exact chunk sizes, exact seeds, exact eval datasets | These folders are allowed to be brutally specific because they are throw-away experiments |
| Everything else (software/kernel, config/, old docker-compose, etc.) | Treated as scratch paper — can be changed or deleted without ceremony | If something graduates from prototype → reference design, it moves out with a dated decision |

## Enforcement

1. CI blocks any commit that adds a concrete model size, exact dBA number, or specific CPU/GPU part name to `README.md`, `docs/goni-story.md`, or `docs/goni-whitepaper.md` (simple grep check in `.github/workflows/ci.yml`).
2. Any PR that wants to tighten a constraint (e.g. “change target volume from ≤ 10 L to ≤ 8 L”) must:
   - update `hardware/10-requirements.md` or `software/90-decisions.md` only,
   - include a photo / measurement / benchmark proving the new number is real today,
   - keep a one-line changelog with date and a **valid until** line.

## How to contribute within the guardrails

- Put hard numbers in `hardware/90-decisions.md`, `software/90-decisions.md`, or `prototype/**` with dates, seeds, and **valid until**.
- Leave `README.md`, `docs/goni-story.md`, and `docs/goni-whitepaper.md` inspirational and timeless; point to decision files for specifics.
- Prototype freely, but keep the exact knobs (models, chunk sizes, eval datasets) inside the prototype track.
