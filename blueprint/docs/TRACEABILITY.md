# Spec -> Code Traceability Matrix

## Legend
- Status: Implemented and tested / Implemented (untested) / Specified only / roadmap
- CI reference: `/.github/workflows/ci.yml` (rust job runs `cargo test --workspace --all-features` under `blueprint/software/kernel`)

| Invariant / Claim | Specified In | Implemented In | Tested By | Status | Notes |
|---|---|---|---|---|---|
| TXT (forbid LargeUtf8 in Control/Execution) | `blueprint/software/50-data/40-privacy-and-text-confinement.md` | `blueprint/software/kernel/goni-schema/src/macros.rs`; `blueprint/software/kernel/goni-schema/src/lib.rs` | `blueprint/software/kernel/goni-schema/tests/txt_axiom.rs` | Implemented and tested | Compile-time guard and runtime check; test only constructs schemas |
| Schema DSL/macros conformance | `blueprint/software/50-data/53-schema-dsl-and-macros.md` | `blueprint/software/kernel/goni-schema/src/macros.rs` | None | Implemented (untested) | Add explicit DSL conformance tests if needed |
| Agent manifest format | `blueprint/30-specs/agent-manifest.md` | `blueprint/software/kernel/goni-agent/src/lib.rs` | `parses_legacy_manifest_without_new_fields`; `parses_manifest_with_new_fields` (same file) | Implemented and tested | Parser only; enforcement lives elsewhere |
| Context selection (facility-location greedy) | `blueprint/software/30-conformance.md`; `blueprint/software/90-decisions.md` (D-007) | `blueprint/software/kernel/goni-context/src/lib.rs` | `selector_respects_budget_and_is_deterministic` (tokio test) | Implemented and tested | No bound or approximation tests yet |
| Zero-copy hot-path objective (ZCO) | `blueprint/software/50-data/52-zero-copy-mechanics.md`; `blueprint/software/90-decisions.md` (D-003) | `blueprint/software/kernel/goni-context/src/lib.rs` (`record_batch_to_candidate_chunks`) | None | Implemented (untested) | D-003 expects CI property tests; not present |
| Spine IDs (row_id, tenant_id, plane, kind, schema_version, timestamps) | `blueprint/software/50-data/20-spine-and-ids.md` | `blueprint/software/kernel/goni-schema/src/macros.rs` | None | Implemented (untested) | Consider schema snapshot tests |
| Scheduler class preference (basic) | `blueprint/30-specs/scheduler-and-interrupts.md` | `blueprint/software/kernel/goni-sched/src/lib.rs` | `interactive_preferred_over_background` (tokio test) | Implemented and tested | Basic ordering only |
| Scheduler admission control (WIP) | `blueprint/30-specs/scheduler-and-interrupts.md` | `blueprint/software/kernel/goni-sched/src/lib.rs` (`QoSScheduler`) | `background_limit_enforced` (same file) | Implemented and tested | Simple limits only |
| Scheduler MaxWeight and Lyapunov stability (K1) | `blueprint/software/30-conformance.md`; `blueprint/software/90-decisions.md` (D-008) | `blueprint/software/kernel/goni-sched/src/lib.rs` | None | Implemented (untested) | Needs load simulation tests |
| Router regret bound (K2) | `blueprint/software/30-conformance.md`; `blueprint/software/90-decisions.md` (D-009) | `blueprint/software/kernel/goni-router/src/lib.rs` (NullRouter) | None | Specified only / roadmap | NullRouter does not implement regret policy |
| Tool capability API | `blueprint/30-specs/tool-capability-api.md` | `blueprint/software/kernel/goni-blueprint/tools/src/lib.rs` | None | Implemented (untested) | Executor is an MVP stub |
| Receipts v1 (hash chain) | `blueprint/30-specs/receipts.md` | `blueprint/software/kernel/goni-receipts/src/lib.rs` | `receipt_chain_verifies` (same file) | Implemented and tested | JSONL log with hash chaining |
| Policy engine (capabilities + gates) | `blueprint/30-specs/tool-capability-api.md`; `blueprint/30-specs/receipts.md` | `blueprint/software/kernel/goni-policy/src/lib.rs` | `initiative_is_deterministic`, `memory_write_requires_evidence`, `redaction_requires_profile` (same file) | Implemented and tested | Minimal allow/deny rules |
| Memory write gate (policy checks) | `blueprint/30-specs/latent-state-contract.md` | `blueprint/software/kernel/goni-policy/src/lib.rs` (`evaluate_memory_write`) | `memory_write_requires_evidence` (same file) | Implemented and tested | Not wired into runtime yet |
| Redaction gate (policy checks) | `blueprint/30-specs/network-gate-and-anonymity.md` | `blueprint/software/kernel/goni-policy/src/lib.rs` (`evaluate_redaction`) | `redaction_requires_profile` (same file) | Implemented and tested | Not wired into runtime yet |
| Egress gate service | `blueprint/30-specs/network-gate-and-anonymity.md` | `blueprint/software/kernel/goni-egress-gate/src/main.rs` | None | Implemented (untested) | HTTP proxy with allowlist |
| ITCR signals and knobs | `blueprint/30-specs/itcr.md` | None found in kernel | N/A | Specified only / roadmap | No code-level contract yet |
| SMA (Symbolic Memory Axiom) | `blueprint/software/50-data/10-axioms-and-planes.md`; `blueprint/30-specs/symbolic-substrate.md` | None found in kernel | N/A | Specified only / roadmap | Keep distinct from TXT enforcement |

## Evidence keys used by contracts

Annotated citations live in `blueprint/docs/references/bibliography.md`. Keys currently
referenced by normative sections include:
- `[[liu2023-lost-middle]]`
- `[[greshake2023-indirect-prompt-injection]]`

