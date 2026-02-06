# Broken Link Fixes (Exact Replacements)

Convention: cross-repo references use `goni-prototype-lab:<relative-path>`.

## Internal Path Corrections (within goni)
| file | exact string to replace | replacement string | rationale |
|---|---|---|---|
| blueprint/security/00-overview.md | `[Threat model](/blueprint/security/docs/threat-model.md)` | `[Threat model](/blueprint/docs/threat-model.md)` | Security docs live under `blueprint/docs` |
| blueprint/security/00-overview.md | `[Privacy](/blueprint/security/docs/privacy.md)` | `[Privacy](/blueprint/docs/privacy.md)` | Security docs live under `blueprint/docs` |
| blueprint/security/00-overview.md | `[TCB](/blueprint/security/docs/tcb.md)` | `[TCB](/blueprint/docs/tcb.md)` | Security docs live under `blueprint/docs` |
| blueprint/security/10-threats.md | `[Threat model](/blueprint/security/docs/threat-model.md)` | `[Threat model](/blueprint/docs/threat-model.md)` | Replace all occurrences |
| blueprint/runtime/00-overview.md | `[Threat model](/blueprint/runtime/docs/threat-model.md)` | `[Threat model](/blueprint/docs/threat-model.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/runtime/00-overview.md | `[Privacy](/blueprint/runtime/docs/privacy.md)` | `[Privacy](/blueprint/docs/privacy.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/runtime/00-overview.md | `[TCB](/blueprint/runtime/docs/tcb.md)` | `[TCB](/blueprint/docs/tcb.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/runtime/sandbox/00-index.md | `[Threat model](/blueprint/runtime/docs/threat-model.md)` | `[Threat model](/blueprint/docs/threat-model.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/runtime/sandbox/00-index.md | `[Privacy](/blueprint/runtime/docs/privacy.md)` | `[Privacy](/blueprint/docs/privacy.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/runtime/sandbox/00-index.md | `[TCB](/blueprint/runtime/docs/tcb.md)` | `[TCB](/blueprint/docs/tcb.md)` | Runtime governance docs live under `blueprint/docs` |
| blueprint/software/20-architecture.md | `[LLM Council](/blueprint/software/docs/llm-council.md)` | `[LLM Council](/blueprint/docs/llm-council.md)` | Docs live under `blueprint/docs` |
| blueprint/software/30-components/mesh-and-wireguard.md | `[Remote LLM Architecture](/blueprint/software/docs/remote-llm-architecture.md)` | `[Remote LLM Architecture](/blueprint/docs/remote-llm-architecture.md)` | Docs live under `blueprint/docs` |
| blueprint/software/30-components/00-index.md | `[Planes hub](/blueprint/software/docs/hubs/planes.md)` | `[Planes hub](/blueprint/docs/hubs/planes.md)` | Docs live under `blueprint/docs` |
| blueprint/software/30-components/00-index.md | `[Contracts hub](/blueprint/software/docs/hubs/contracts.md)` | `[Contracts hub](/blueprint/docs/hubs/contracts.md)` | Docs live under `blueprint/docs` |
| blueprint/software/30-components/llm-runtime.md | `[Hardware requirements](/blueprint/software/hardware/10-requirements.md)` | `[Hardware requirements](/blueprint/hardware/10-requirements.md)` | Hardware docs live under `blueprint/hardware` |
| blueprint/software/30-components/os-and-base-image.md | `[Hardware requirements](/blueprint/software/hardware/10-requirements.md)` | `[Hardware requirements](/blueprint/hardware/10-requirements.md)` | Hardware docs live under `blueprint/hardware` |
| blueprint/software/30-components/orchestrator.md | `[API README](/blueprint/software/api/README.md)` | `[API README](/blueprint/api/README.md)` | API docs live under `blueprint/api` |
| blueprint/software/30-components/orchestrator.md | `[OpenAPI spec](/blueprint/software/api/openapi.yaml)` | `[OpenAPI spec](/blueprint/api/openapi.yaml)` | API docs live under `blueprint/api` |

## Redirects To goni-prototype-lab (runtime artifacts)
| file | exact string to replace | replacement string | rationale |
|---|---|---|---|
| blueprint/software/00-overview.md | `[docker-compose.yml](/blueprint/software/docker-compose.yml)` | `goni-prototype-lab:deploy/docker-compose.yml` | Compose lives in prototype lab |
| blueprint/software/00-overview.md | `[k8s/](/blueprint/software/k8s)` | `goni-prototype-lab:deploy/k8s/` | K8s manifests live in prototype lab |
| blueprint/docs/tools.md | `blueprint/tools/manifests/` | `goni-prototype-lab:tools/manifests/` | Tool manifests live in prototype lab |
| blueprint/30-specs/network-gate-and-anonymity.md | `blueprint/config/council.yaml` | `goni-prototype-lab:config/council.yaml` | Config lives in prototype lab |
| blueprint/docs/goni-lab.md | `blueprint/config/council.yaml` | `goni-prototype-lab:config/council.yaml` | Replace all occurrences |
| blueprint/docs/llm-council.md | `blueprint/config/council.yaml` | `goni-prototype-lab:config/council.yaml` | Replace all occurrences |
| blueprint/docs/remote-llm-architecture.md | `blueprint/config/council.yaml` | `goni-prototype-lab:config/council.yaml` | Config lives in prototype lab |
| blueprint/docs/remote-llm-architecture.md | `blueprint/config/env` | `goni-prototype-lab:config` | Config lives in prototype lab |
| blueprint/docs/remote-llm-architecture.md | `blueprint/config/council.env.example` | `goni-prototype-lab:config/council.env.example` | Env defaults live in prototype lab |
| blueprint/docs/goni-agility-rules.md | `/blueprint/software/kernel` | `goni-prototype-lab:software/kernel` | Kernel lives in prototype lab |
| blueprint/docs/goni-agility-rules.md | `blueprint/config/` | `goni-prototype-lab:config/` | Config lives in prototype lab |
| blueprint/docs/goni-agility-rules.md | `blueprint/prototype/**` | `goni-prototype-lab:prototype/**` | Prototype lives in prototype lab |
| .github/CONTRIBUTING.md | `blueprint/prototype/**` | `goni-prototype-lab:prototype/**` | Prototype lives in prototype lab |
| blueprint/docs/00-system-map.md | `[Prototype catalog](/blueprint/prototype/00-index.md)` | `Prototype catalog: goni-prototype-lab:prototype/00-index.md` | Prototype catalog lives in prototype lab |
| blueprint/hardware/25-hardware-layers-and-supplier-map.md | `blueprint/software/kernel/goni-infer` | `goni-prototype-lab:software/kernel/goni-infer` | Kernel implementation lives in prototype lab |
| blueprint/hardware/90-decisions.md | `blueprint/software/kernel/goni-infer` | `goni-prototype-lab:software/kernel/goni-infer` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-receipts/` | `goni-prototype-lab:software/kernel/goni-receipts/` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-receipts/src/lib.rs` | `goni-prototype-lab:software/kernel/goni-receipts/src/lib.rs` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-policy/` | `goni-prototype-lab:software/kernel/goni-policy/` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-policy/src/lib.rs` | `goni-prototype-lab:software/kernel/goni-policy/src/lib.rs` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-sched/` | `goni-prototype-lab:software/kernel/goni-sched/` | Kernel implementation lives in prototype lab |
| blueprint/docs/architecture/index.md | `blueprint/software/kernel/goni-sched/src/lib.rs` | `goni-prototype-lab:software/kernel/goni-sched/src/lib.rs` | Kernel implementation lives in prototype lab |
| blueprint/software/50-data/51-schemas-mvp.md | `blueprint/software/kernel/goni-schema` | `goni-prototype-lab:software/kernel/goni-schema` | Executable schema lives in prototype lab |
| blueprint/software/50-data/schemas.md | `blueprint/software/kernel/goni-schema` | `goni-prototype-lab:software/kernel/goni-schema` | Executable schema lives in prototype lab |
| blueprint/software/50-data/80-validation-and-ci.md | `/.github/workflows/ci.yml` | `goni-prototype-lab:.github/workflows/ci.yml` | CI lives in prototype lab |
| blueprint/software/50-data/80-validation-and-ci.md | `blueprint/software/kernel` | `goni-prototype-lab:software/kernel` | Kernel lives in prototype lab |
| blueprint/AGENTS.md | `blueprint/software/kernel/goni-schema/tests/` | `goni-prototype-lab:software/kernel/goni-schema/tests/` | Kernel tests live in prototype lab |
| blueprint/docs/meta/agents.root.template.md | `blueprint/software/kernel/goni-schema/tests/` | `goni-prototype-lab:software/kernel/goni-schema/tests/` | Kernel tests live in prototype lab |
| blueprint/docs/meta/agents.software.template.md | `blueprint/software/kernel/goni-schema/tests/` | `goni-prototype-lab:software/kernel/goni-schema/tests/` | Kernel tests live in prototype lab |
| blueprint/software/AGENTS.md | `blueprint/software/kernel/goni-schema/tests/` | `goni-prototype-lab:software/kernel/goni-schema/tests/` | Kernel tests live in prototype lab |
| blueprint/docs/meta/agents.kernel.template.md | `/blueprint/software/kernel` | `goni-prototype-lab:software/kernel` | Kernel lives in prototype lab |
| blueprint/docs/EVALUATION.md | `blueprint/eval/scenarios` | `goni-prototype-lab:eval/scenarios` | Eval scenarios live in prototype lab |

## Moved Docs (STATUS, TRACEABILITY, quickstart, demo_expected)
| file | exact string to replace | replacement string | rationale |
|---|---|---|---|
| blueprint/README.md | `blueprint/docs/STATUS.md` | `goni-prototype-lab:goni-lab/STATUS.md` | STATUS moved to prototype lab |
| blueprint/README.md | `[blueprint/docs/STATUS.md](/blueprint/docs/STATUS.md)` | `goni-prototype-lab:goni-lab/STATUS.md` | STATUS moved to prototype lab |
| blueprint/README.md | `[blueprint/docs/quickstart.md](/blueprint/docs/quickstart.md)` | `goni-prototype-lab:goni-lab/quickstart.md` | Quickstart moved to prototype lab |
| blueprint/00-map/40-community.md | `Read blueprint/docs/STATUS.md` | `Read goni-prototype-lab:goni-lab/STATUS.md` | STATUS moved to prototype lab |
| blueprint/00-map/40-community.md | `Read blueprint/docs/TRACEABILITY.md` | `Read goni-prototype-lab:goni-lab/TRACEABILITY.md` | TRACEABILITY moved to prototype lab |
| blueprint/docs/00-system-map.md | `[TRACEABILITY](/blueprint/docs/TRACEABILITY.md)` | `goni-prototype-lab:goni-lab/TRACEABILITY.md` | TRACEABILITY moved to prototype lab |
| blueprint/docs/hubs/governance.md | `[Traceability](/blueprint/docs/TRACEABILITY.md)` | `goni-prototype-lab:goni-lab/TRACEABILITY.md` | TRACEABILITY moved to prototype lab |
| blueprint/docs/hubs/product-surfaces.md | `[Traceability](/blueprint/docs/TRACEABILITY.md)` | `goni-prototype-lab:goni-lab/TRACEABILITY.md` | TRACEABILITY moved to prototype lab |
| blueprint/30-specs/receipts.md | `[Traceability](/blueprint/docs/TRACEABILITY.md)` | `goni-prototype-lab:goni-lab/TRACEABILITY.md` | TRACEABILITY moved to prototype lab |
| blueprint/docs/architecture/index.md | `For detailed status, see blueprint/docs/TRACEABILITY.md.` | `For detailed status, see goni-prototype-lab:goni-lab/TRACEABILITY.md.` | TRACEABILITY moved to prototype lab |
| blueprint/docs/README.md | `[Quickstart](/blueprint/docs/quickstart.md)` | `Quickstart: goni-prototype-lab:goni-lab/quickstart.md` | Quickstart moved to prototype lab |
| CONTRIBUTING.md | `blueprint/docs/quickstart.md` | `goni-prototype-lab:goni-lab/quickstart.md` | Quickstart moved to prototype lab |
| blueprint/10-product/90-research-agenda.md | `demo_expected.md` | `goni-prototype-lab:demo/demo_expected.md` | Demo expected outputs moved to prototype lab |

## Block Replacement (demo_expected)
File: `blueprint/docs/demo_expected.md` (after moving to `goni-prototype-lab/demo/demo_expected.md`)

Replace block:
```text
- blueprint/demo/output/bench.json
- blueprint/demo/output/action_card.json
- blueprint/demo/output/receipts.jsonl
```

With:
```text
- goni-prototype-lab:demo/expected/demo_output.json
```

## Post-Move Edits in goni-prototype-lab (apply after moving STATUS/TRACEABILITY/quickstart)
| file | exact string to replace | replacement string | rationale |
|---|---|---|---|
| goni-prototype-lab/goni-lab/STATUS.md | `blueprint/software/kernel/` | `software/kernel/` | Paths are no longer under `blueprint/` |
| goni-prototype-lab/goni-lab/STATUS.md | `blueprint/software/docker-compose.yml` | `deploy/docker-compose.yml` | Deploy manifests live in `deploy/` |
| goni-prototype-lab/goni-lab/STATUS.md | `blueprint/software/k8s/` | `deploy/k8s/` | Deploy manifests live in `deploy/` |
| goni-prototype-lab/goni-lab/STATUS.md | `/.github/workflows/ci.yml` | `.github/workflows/ci.yml` | CI path is repo-local |
| goni-prototype-lab/goni-lab/STATUS.md | `blueprint/scripts/` | `scripts/` | Scripts live at repo root |
| goni-prototype-lab/goni-lab/TRACEABILITY.md | `blueprint/software/kernel/` | `software/kernel/` | Paths are no longer under `blueprint/` |
| goni-prototype-lab/goni-lab/TRACEABILITY.md | `/.github/workflows/ci.yml` | `.github/workflows/ci.yml` | CI path is repo-local |
| goni-prototype-lab/goni-lab/TRACEABILITY.md | `blueprint/scripts/` | `scripts/` | Scripts live at repo root |

## Script References (move to prototype lab)
| file | exact string to replace | replacement string | rationale |
|---|---|---|---|
| blueprint/AGENTS.md | `blueprint/scripts/generate_agents.py` | `goni-prototype-lab:scripts/generate_agents.py` | Scripts live in prototype lab |
| blueprint/AGENTS.md | `blueprint/scripts/validate_truth_map.py` | `goni-prototype-lab:scripts/validate_truth_map.py` | Scripts live in prototype lab |
| blueprint/docs/meta/agents.root.template.md | `blueprint/scripts/generate_agents.py` | `goni-prototype-lab:scripts/generate_agents.py` | Scripts live in prototype lab |
| blueprint/docs/meta/agents.root.template.md | `blueprint/scripts/validate_truth_map.py` | `goni-prototype-lab:scripts/validate_truth_map.py` | Scripts live in prototype lab |
| blueprint/10-product/30-next-steps.md | `blueprint/scripts/validate_truth_map.py` | `goni-prototype-lab:scripts/validate_truth_map.py` | Scripts live in prototype lab |
| blueprint/10-product/30-next-steps.md | `python blueprint/scripts/validate_truth_map.py` | `python goni-prototype-lab:scripts/validate_truth_map.py` | Scripts live in prototype lab |
| blueprint/10-product/30-next-steps.md | `python blueprint/scripts/generate_agents.py` | `python goni-prototype-lab:scripts/generate_agents.py` | Scripts live in prototype lab |
