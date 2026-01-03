# AGENTS.md (software/kernel) - Overrides

## Overrides
- Kernel crates must respect plane boundaries (Data / Context / Control / Execution). Do not bypass contracts for convenience.
- Any change that impacts routing, scheduling, or tool execution MUST update the corresponding spec:
  - routing/scheduling: `docs/specs/scheduler-and-interrupts.md`
  - tool execution/permissions: `docs/specs/tool-capability-api.md`

## Status honesty
- If an invariant is described as enforced, you must point to a test (or explicitly downgrade the wording to "intended / planned").
