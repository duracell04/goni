# Repo Reality Map (Spec vs Prototype vs Experiments)

## Definitions
- Spec: normative design intent; not necessarily implemented
- Prototype: runnable today; may be incomplete or unvalidated
- Experiment: exploratory; not a platform commitment

## Blessed Demo Path
- Entry point: `blueprint/software/docker-compose.yml`
- Demo path excludes gateway; no gateway service is defined in compose or k8s overlays.
- Kernel-only demo scope (orchestrator + llm-local + optional vecdb):
  - Implemented (untested): `/v1/chat/completions` route in `blueprint/software/kernel/goni-http/src/main.rs`
  - Implemented and tested: context selector budget and determinism test `selector_respects_budget_and_is_deterministic` in `blueprint/software/kernel/goni-context/src/lib.rs`
  - Implemented and tested: TXT axiom guard (compile-time + runtime) in `blueprint/software/kernel/goni-schema/src/macros.rs`; test in `blueprint/software/kernel/goni-schema/tests/txt_axiom.rs`
  - Implemented and tested: agent manifest parsing tests in `blueprint/software/kernel/goni-agent/src/lib.rs`
  - Implemented and tested: scheduler class preference test `interactive_preferred_over_background` in `blueprint/software/kernel/goni-sched/src/lib.rs`
  - Implemented (untested): demo scripts `blueprint/scripts/demo.sh` and `blueprint/scripts/smoke_test.sh`

## Plane Status (Snapshot)
Use one status phrase per row: Implemented and tested / Implemented (untested) / Specified only / roadmap

| Plane | Status | Evidence |
|------|--------|----------|
| Data (spine, schemas, axioms) | Implemented (untested) | `blueprint/software/kernel/goni-schema/src/lib.rs`; `blueprint/software/kernel/goni-schema/src/macros.rs`; `blueprint/software/kernel/goni-schema/tests/txt_axiom.rs`; `blueprint/software/kernel/goni-store/src/lib.rs`; `blueprint/software/kernel/goni-store/src/spine_mem.rs`; `blueprint/software/kernel/goni-store/src/qdrant.rs` |
| Context selection | Implemented and tested | `blueprint/software/kernel/goni-context/src/lib.rs` (`selector_respects_budget_and_is_deterministic`) |
| Control (scheduler/router) | Implemented (untested) | `blueprint/software/kernel/goni-sched/src/lib.rs` (basic ordering test); `blueprint/software/kernel/goni-router/src/lib.rs`; `blueprint/docs/specs/scheduler-and-interrupts.md` |
| Execution (LLM blueprint/runtime/tools) | Implemented (untested) | `blueprint/software/kernel/goni-infer/src/http_vllm.rs`; `blueprint/software/kernel/goni-blueprint/tools/src/lib.rs`; `blueprint/software/kernel/goni-http/src/main.rs` |

## Governance and receipts
- Receipts log: Implemented and tested (`blueprint/software/kernel/goni-receipts/src/lib.rs`)
- Policy engine: Implemented and tested (`blueprint/software/kernel/goni-policy/src/lib.rs`)
- Egress gate: Implemented (untested) (`blueprint/software/kernel/goni-egress-gate/src/main.rs`)

## Demo Dependencies (Declare Truth)
### Gateway
- Status: Specified only / roadmap (not part of the demo path)
- Evidence:
  - Compose omits gateway: `blueprint/software/docker-compose.yml`
  - K8s overlays omit gateway: `blueprint/software/k8s/overlays/single-node/kustomization.yaml`; `blueprint/software/k8s/overlays/cluster/kustomization.yaml`

### Egress gate
- Status: Implemented (untested)
- Evidence:
  - `blueprint/software/kernel/goni-egress-gate/src/main.rs`
  - `blueprint/software/docker-compose.yml` includes `egress-gate`

### Frontend
- Status: Specified only / roadmap (stub moved to blueprint/prototype/)
- Evidence:
  - Present: `blueprint/prototype/frontend-stub/`

### Goni Lab
- Status: Implemented (untested)
- Evidence:
  - `blueprint/goni-lab/goni_lab.py`

## CI Reality (What Is Enforced)
- `/.github/workflows/ci.yml`
  - guardrails job blocks pinned specs in `README.md`, `blueprint/docs/goni-story.md`, `blueprint/docs/goni-whitepaper.md`
  - rust job runs `cargo check`, `cargo test --workspace --all-features`, `cargo clippy -- -D warnings` under `blueprint/software/kernel`
  - meta job runs `blueprint/scripts/validate_truth_map.py` and `blueprint/scripts/generate_agents.py`
  - bench_smoke job runs `goni-lab` synthetic benchmark
  - demo_smoke job runs `blueprint/scripts/run_smoke_local.sh` with `LLM_STUB=1`
  - txt lint runs `blueprint/scripts/txt_lint.sh`

## Known Risks / Open Decisions
- Zero-copy hot-path CI gates called for in D-003 are not implemented in CI: `blueprint/software/90-decisions.md` vs `/.github/workflows/ci.yml`
- Embeddings are a deterministic lexical baseline, not a neural model: `blueprint/software/kernel/goni-embed/src/lib.rs`
- Gateway/UI not in demo path; reintroduction must be pinned and sourced or explicitly externalized.
- Prompt materialization and redaction enforcement are specified-only; policy checks exist but no runtime pipeline or gate is wired.
- MemoryEntries write gating is specified-only at runtime; policy checks exist but are not wired.
- Container-level non-bypass egress is not enforced in compose; policy gate only.
