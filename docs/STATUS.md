# Repo Reality Map (Spec vs Prototype vs Experiments)

## Definitions
- Spec: normative design intent; not necessarily implemented
- Prototype: runnable today; may be incomplete or unvalidated
- Experiment: exploratory; not a platform commitment

## Blessed Demo Path
- Entry point: `software/docker-compose.yml`
- Known break: `gateway` service references `./gateway` but `software/gateway/` is missing.
- Kernel-only demo scope (orchestrator + llm-local + optional vecdb):
  - Implemented (untested): `/v1/chat/completions` route in `software/kernel/goni-http/src/main.rs`
  - Implemented and tested: context selector budget and determinism test `selector_respects_budget_and_is_deterministic` in `software/kernel/goni-context/src/lib.rs`
  - Implemented and tested: TXT axiom guard (compile-time + runtime) in `software/kernel/goni-schema/src/macros.rs`; test in `software/kernel/goni-schema/tests/txt_axiom.rs`
  - Implemented and tested: agent manifest parsing tests in `software/kernel/goni-agent/src/lib.rs`
  - Implemented and tested: scheduler class preference test `interactive_preferred_over_background` in `software/kernel/goni-sched/src/lib.rs`

## Plane Status (Snapshot)
Use one status phrase per row: Implemented and tested / Implemented (untested) / Specified only / roadmap

| Plane | Status | Evidence |
|------|--------|----------|
| Data (spine, schemas, axioms) | Implemented (untested) | `software/kernel/goni-schema/src/lib.rs`; `software/kernel/goni-schema/src/macros.rs`; `software/kernel/goni-schema/tests/txt_axiom.rs`; `software/kernel/goni-store/src/lib.rs`; `software/kernel/goni-store/src/spine_mem.rs`; `software/kernel/goni-store/src/qdrant.rs` |
| Context selection | Implemented and tested | `software/kernel/goni-context/src/lib.rs` (`selector_respects_budget_and_is_deterministic`) |
| Control (scheduler/router) | Implemented (untested) | `software/kernel/goni-sched/src/lib.rs` (basic ordering test); `software/kernel/goni-router/src/lib.rs`; `docs/specs/scheduler-and-interrupts.md` |
| Execution (LLM runtime/tools) | Implemented (untested) | `software/kernel/goni-infer/src/http_vllm.rs`; `software/kernel/goni-tools/src/lib.rs`; `software/kernel/goni-http/src/main.rs` |

## Demo Dependencies (Declare Truth)
### Gateway
- Status: Specified only / roadmap
- Evidence:
  - Compose reference: `software/docker-compose.yml` (build context `./gateway` missing)
  - K8s reference: `software/k8s/base/gateway.yaml` (image `ghcr.io/duracell04/goni-gateway:latest`)

### Frontend
- Status: Specified only / roadmap (code stub, not scaffolded)
- Evidence:
  - Present: `frontend/app/page.tsx`; `frontend/components/MarkdownPage.tsx`
  - Missing: `frontend/package.json` (not found in tree)

## CI Reality (What Is Enforced)
- `.github/workflows/ci.yml`
  - guardrails job blocks pinned specs in `README.md`, `docs/goni-story.md`, `docs/goni-whitepaper.md`
  - rust job runs `cargo check`, `cargo test --workspace --all-features`, `cargo clippy -- -D warnings` under `software/kernel`

## Known Risks / Open Decisions
- Gateway reproducibility gap: `software/docker-compose.yml` and `software/k8s/base/gateway.yaml`
- Zero-copy hot-path CI gates called for in D-003 are not implemented in CI: `software/90-decisions.md` vs `.github/workflows/ci.yml`
- Qdrant embeddings are placeholder hash vectors, not model embeddings: `software/kernel/goni-store/src/qdrant.rs`
