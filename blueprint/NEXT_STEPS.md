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

## Validation checklist
- `python blueprint/scripts/validate_truth_map.py`
- `python blueprint/scripts/generate_agents.py`
- `bash blueprint/scripts/txt_lint.sh`
- `python blueprint/goni-lab/goni_lab.py bench --scenario blueprint/goni-lab/scenarios/mixed.json`
