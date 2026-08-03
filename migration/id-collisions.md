# Legacy ID collision decisions

Legacy files reused several IDs for unrelated propositions. A stable ID can resolve to only one canonical node, so the coordinator retained it on the strongest domain document and assigned deterministic IDs to the other propositions. No legacy path or content was discarded.

- `OS-AND-BASE-IMAGE` remains owned by `blueprint/hardware/os-and-base-image.md`.
  - Ambiguous reuse in `blueprint/software/30-components/os-and-base-image.md` became `GONI-IMAP-4579FCFB285C`; the legacy pointer preserves the collision boundary.
- `SYS-02` remains owned by `blueprint/20-system/30-performance.md`.
  - Ambiguous reuse in `blueprint/docs/02-taxonomy-and-ids.md` became `GONI-SYNTHESIS-610F664BC30C`; the legacy pointer preserves the collision boundary.
- `SYS-03` remains owned by `blueprint/20-system/50-learning-loop.md`.
  - Ambiguous reuse in `blueprint/docs/hubs/00-index.md` became `GONI-SYNTHESIS-8C6D25C472B7`; the legacy pointer preserves the collision boundary.
