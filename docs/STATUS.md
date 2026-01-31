# Repo Reality Map (Spec vs Prototype vs Experiments)

## Definitions
- Spec: normative design intent; not necessarily implemented
- Prototype: runnable today; may be incomplete or unvalidated
- Experiment: exploratory; not a platform commitment

## Blessed Demo Path
- Entry point: `software/docker-compose.yml`
- Demo path excludes gateway; no gateway service is defined in compose or k8s overlays.
- Kernel-only demo scope (orchestrator + llm-local + optional vecdb):
  - Implemented (untested): `/v1/chat/completions` route in `software/kernel/goni-http/src/main.rs`
  - Implemented and tested: context selector budget and determinism test `selector_respects_budget_and_is_deterministic` in `software/kernel/goni-context/src/lib.rs`
  - Implemented and tested: TXT axiom guard (compile-time + runtime) in `software/kernel/goni-schema/src/macros.rs`; test in `software/kernel/goni-schema/tests/txt_axiom.rs`
  - Implemented and tested: agent manifest parsing tests in `software/kernel/goni-agent/src/lib.rs`
  - Implemented and tested: scheduler class preference test `interactive_preferred_over_background` in `software/kernel/goni-sched/src/lib.rs`
  - Implemented (untested): demo scripts `scripts/demo.sh` and `scripts/smoke_test.sh`

## Plane Status (Snapshot)
Use one status phrase per row: Implemented and tested / Implemented (untested) / Specified only / roadmap

| Plane | Status | Evidence |
|------|--------|----------|
| Data (spine, schemas, axioms) | Implemented (untested) | `software/kernel/goni-schema/src/lib.rs`; `software/kernel/goni-schema/src/macros.rs`; `software/kernel/goni-schema/tests/txt_axiom.rs`; `software/kernel/goni-store/src/lib.rs`; `software/kernel/goni-store/src/spine_mem.rs`; `software/kernel/goni-store/src/qdrant.rs` |
| Context selection | Implemented and tested | `software/kernel/goni-context/src/lib.rs` (`selector_respects_budget_and_is_deterministic`) |
| Control (scheduler/router) | Implemented (untested) | `software/kernel/goni-sched/src/lib.rs` (basic ordering test); `software/kernel/goni-router/src/lib.rs`; `docs/specs/scheduler-and-interrupts.md` |
| Execution (LLM runtime/tools) | Implemented (untested) | `software/kernel/goni-infer/src/http_vllm.rs`; `software/kernel/goni-tools/src/lib.rs`; `software/kernel/goni-http/src/main.rs` |

## Governance and receipts
- Receipts log: Implemented and tested (`software/kernel/goni-receipts/src/lib.rs`)
- Policy engine: Implemented and tested (`software/kernel/goni-policy/src/lib.rs`)
- Egress gate: Implemented (untested) (`software/kernel/goni-egress-gate/src/main.rs`)

## Demo Dependencies (Declare Truth)
### Gateway
- Status: Specified only / roadmap (not part of the demo path)
- Evidence:
  - Compose omits gateway: `software/docker-compose.yml`
  - K8s overlays omit gateway: `software/k8s/overlays/single-node/kustomization.yaml`; `software/k8s/overlays/cluster/kustomization.yaml`

### Egress gate
- Status: Implemented (untested)
- Evidence:
  - `software/kernel/goni-egress-gate/src/main.rs`
  - `software/docker-compose.yml` includes `egress-gate`

### Frontend
- Status: Specified only / roadmap (stub moved to prototype/)
- Evidence:
  - Present: `prototype/frontend-stub/`

### Goni Lab
- Status: Implemented (untested)
- Evidence:
  - `goni-lab/goni_lab.py`

## CI Reality (What Is Enforced)
- `.github/workflows/ci.yml`
  - guardrails job blocks pinned specs in `README.md`, `docs/goni-story.md`, `docs/goni-whitepaper.md`
  - rust job runs `cargo check`, `cargo test --workspace --all-features`, `cargo clippy -- -D warnings` under `software/kernel`
  - meta job runs `scripts/validate_truth_map.py` and `scripts/generate_agents.py`
  - bench_smoke job runs `goni-lab` synthetic benchmark
  - demo_smoke job runs `scripts/run_smoke_local.sh` with `LLM_STUB=1`
  - txt lint runs `scripts/txt_lint.sh`

## Known Risks / Open Decisions
- Zero-copy hot-path CI gates called for in D-003 are not implemented in CI: `software/90-decisions.md` vs `.github/workflows/ci.yml`
- Embeddings are a deterministic lexical baseline, not a neural model: `software/kernel/goni-embed/src/lib.rs`
- Gateway/UI not in demo path; reintroduction must be pinned and sourced or explicitly externalized.
- Prompt materialization and redaction enforcement are specified-only; policy checks exist but no runtime pipeline or gate is wired.
- MemoryEntries write gating is specified-only at runtime; policy checks exist but are not wired.
- Container-level non-bypass egress is not enforced in compose; policy gate only.
