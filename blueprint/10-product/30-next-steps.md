# Next Steps (for devs/agents)

This file captures the **next concrete steps** to make the repo more complete,
more symmetric, and more interconnected without introducing dependency spaghetti.

## 1) Expand the truth map into a system graph
- Add entries for key docs, schemas, kernel crates, APIs, tests, and harnesses.
- Add typed edges: `implements`, `tested_by`, `depends_on`.
- Extend `blueprint/scripts/validate_truth_map.py` to enforce:
  - required edges for `role: spec` (unless explicitly exempted)
  - no orphan entries for must-connect roles

## 2) Turn invariants into real tests
- Replace placeholders in `blueprint/tests/invariants/` with real checks:
  - TXT boundary lint
  - receipt chain verification
  - default-deny policy behavior
  - egress allowlist enforcement (deny by default)
- Wire them into CI (fast gates on PRs).

## 2a) Harden sovereign-operator contracts
- Audit existing Work Order, Done Contract, receipt, CapabilityToken,
  AutonomyCorridor, memory retrieval, and model-registry specs for one
  end-to-end governed action path.
- Ensure the path preserves:
  - Work Order reference,
  - Done Contract hash,
  - capability token,
  - policy decision,
  - sandbox class,
  - receipt tier,
  - rollback reference where applicable.
- Add trace fixtures for at least one reversible write and one hard-gated
  external side effect.

## 2b) Add background autonomy slices
- Define fixtures for:
  - open-loop detection,
  - memory consolidation,
  - periodic audit,
  - model evaluation,
  - policy drift check,
  - scheduled brief.
- Each fixture should declare trigger, budget, corridor, receipt expectation,
  and interrupt rule.

## 2c) Add parser and memory-class hardening
- Add parser/ingestion fixtures with source hash, parser ID/version, chunk
  boundaries, confidence flags, and expected `parser_basis`.
- Add memory-class policy examples for `personal_private`, `project_shared`,
  `relationship`, `model_system`, `ephemeral`, and `quarantine`.
- Verify quarantine and expired memory are absent from normal retrieval.

## 2d) Add model provenance pipeline slices
- Add a model install fixture that produces a ModelManifest and InstallReceipt.
- Add a model eval fixture that produces an EvalReceipt.
- Add a rollback fixture that proves RollbackRef can restore the prior approved
  bundle or quarantine a failed candidate.

## 3) Expand the OpenAPI surface + stubs
- Add endpoints for:
  - receipts (list/get/verify)
  - policy (get/set)
  - action cards (list/approve/reject/snooze)
  - daily brief (get)
- Implement handler stubs in `goni-http` and add smoke tests.

## 4) Make the kernel pipeline canonical
- Move receipt emission into the kernel pipeline (not just HTTP handlers).
- Make policy checks unavoidable:
  - memory write gate
  - redaction/egress gate
- Ensure CLI/jobs/UI use the same pipeline.

## 5) Integrate goni-schema into runtime
- Replace stringly-typed table references with schema constants.
- Enforce plane/table pairing and TXT axiom at runtime boundaries.

## 6) Upgrade goni-lab from synthetic to live mode
- Add a live mode that hits the running orchestrator.
- Record TTFT and cancellation latency in JSON evidence files.
- Link evidence artifacts in TRACEABILITY.

## 7) Add a research-first long-context reading harness
- Add corpus fixtures and gold answers for long single-doc, multi-doc, span
  extraction, and needle-in-corpus tasks.
- Add a research harness that can compare:
  - full-context baseline,
  - current RAG/context assembly baseline,
  - programmatic corpus-reading baseline,
  - hybrid retrieval + corpus-reading baseline.
- Produce one operator-facing comparison report for quality, cost, latency, and
  failure modes.
- Keep stories INVEST-sized:
  - one fixture family,
  - one strategy comparison,
  - one measurable output per slice.

## 8) Prepare future hooks without changing active contracts
- Identify candidate receipt fields for scan/slice/subread accounting if the
  research lane graduates later.
- Identify candidate scheduler budget fields for recursion depth and bounded
  parallel subreads.
- Do not add those fields to public schemas in this phase.

## Validation checklist
- `python blueprint/scripts/validate_truth_map.py`
- `python blueprint/scripts/generate_agents.py`
- `bash blueprint/scripts/txt_lint.sh`
- `python blueprint/goni-lab/goni_lab.py bench --scenario blueprint/goni-lab/scenarios/mixed.json`
