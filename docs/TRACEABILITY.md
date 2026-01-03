# Spec -> Code Traceability Matrix

## Legend
- Status: Implemented and tested / Implemented (untested) / Specified only / roadmap
- CI reference: `.github/workflows/ci.yml` (rust job runs `cargo test --workspace --all-features` under `software/kernel`)

| Invariant / Claim | Specified In | Implemented In | Tested By | Status | Notes |
|---|---|---|---|---|---|
| TXT (forbid LargeUtf8 in Control/Execution) | `software/50-data/40-privacy-and-text-confinement.md` | `software/kernel/goni-schema/src/macros.rs`; `software/kernel/goni-schema/src/lib.rs` | `software/kernel/goni-schema/tests/txt_axiom.rs` | Implemented and tested | Compile-time guard and runtime check; test only constructs schemas |
| Schema DSL/macros conformance | `software/50-data/53-schema-dsl-and-macros.md` | `software/kernel/goni-schema/src/macros.rs` | None | Implemented (untested) | Add explicit DSL conformance tests if needed |
| Agent manifest format | `docs/specs/agent-manifest.md` | `software/kernel/goni-agent/src/lib.rs` | `parses_legacy_manifest_without_new_fields`; `parses_manifest_with_new_fields` (same file) | Implemented and tested | Parser only; enforcement lives elsewhere |
| Context selection (facility-location greedy) | `software/30-conformance.md`; `software/90-decisions.md` (D-007) | `software/kernel/goni-context/src/lib.rs` | `selector_respects_budget_and_is_deterministic` (tokio test) | Implemented and tested | No bound or approximation tests yet |
| Zero-copy hot-path objective (ZCO) | `software/50-data/52-zero-copy-mechanics.md`; `software/90-decisions.md` (D-003) | `software/kernel/goni-context/src/lib.rs` (`record_batch_to_candidate_chunks`) | None | Implemented (untested) | D-003 expects CI property tests; not present |
| Spine IDs (row_id, tenant_id, plane, kind, schema_version, timestamps) | `software/50-data/20-spine-and-ids.md` | `software/kernel/goni-schema/src/macros.rs` | None | Implemented (untested) | Consider schema snapshot tests |
| Scheduler class preference (basic) | `docs/specs/scheduler-and-interrupts.md` | `software/kernel/goni-sched/src/lib.rs` | `interactive_preferred_over_background` (tokio test) | Implemented and tested | Basic ordering only |
| Scheduler MaxWeight and Lyapunov stability (K1) | `software/30-conformance.md`; `software/90-decisions.md` (D-008) | `software/kernel/goni-sched/src/lib.rs` | None | Implemented (untested) | Needs load simulation tests |
| Router regret bound (K2) | `software/30-conformance.md`; `software/90-decisions.md` (D-009) | `software/kernel/goni-router/src/lib.rs` (NullRouter) | None | Specified only / roadmap | NullRouter does not implement regret policy |
| Tool capability API | `docs/specs/tool-capability-api.md` | `software/kernel/goni-tools/src/lib.rs` | None | Implemented (untested) | Executor is an MVP stub |
| ITCR signals and knobs | `docs/specs/itcr.md` | None found in kernel | N/A | Specified only / roadmap | No code-level contract yet |
| SMA (Symbolic Memory Axiom) | `software/50-data/10-axioms-and-planes.md`; `docs/specs/symbolic-substrate.md` | None found in kernel | N/A | Specified only / roadmap | Keep distinct from TXT enforcement |
