# Scope Contract for goni (Blueprint Only)

Purpose: keep `goni/` as a pure blueprint repository (plans, specs, research, diagrams). All runnable artifacts live in `goni-prototype-lab/`.

## Allowed Content in goni
- `blueprint/00-map/**`
- `blueprint/10-product/**`
- `blueprint/20-system/**`
- `blueprint/30-specs/**`
- `blueprint/40-implementation/**`
- `blueprint/50-evidence/**`
- `blueprint/60-market/**`
- `blueprint/_archive/**`
- `blueprint/_inbox/**`
- `blueprint/api/**`
- `blueprint/asset/**`
- `blueprint/decisions/**`
- `blueprint/docs/**`
- `blueprint/hardware/**`
- `blueprint/runtime/**`
- `blueprint/schemas/**`
- `blueprint/security/**`
- `blueprint/software/**`
- `blueprint/spec/**`
- Root docs and governance: `README.md`, `LICENSE`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `SECURITY.md`, `CONTRIBUTING.md`
- Guardrails only under `.github/**` (no build/test workflows)

## Allowed File Types in goni
- `.md`, `.txt`
- `.png`, `.jpg`, `.jpeg`, `.svg`, `.drawio`
- `.xlsx` (hardware BOM evidence)
- Spec artifacts only under these paths:
- `blueprint/30-specs/**` (including `registry.yml`)
- `blueprint/schemas/**` (schema JSON)
- `blueprint/api/**` (`openapi.yaml`)
- `blueprint/docs/policy-templates/**`
- `blueprint/docs/agent-manifests/**`
- `blueprint/docs/assets/ai-2027/manifest.json.example`
- `blueprint/docs/meta/truth-map.json`

## Disallowed in goni
- Execution folders: `blueprint/benchmarks/**`, `blueprint/demo/**`, `blueprint/deploy/**`, `blueprint/eval/**`, `blueprint/examples/**`, `blueprint/tests/**`, `blueprint/tools/**`
- Scripts and runnable code: `*.py`, `*.sh`, `*.ps1`, `*.rs`, `*.ts`, `*.js`, `*.go`, `*.java`, `*.kt`, `*.rb`, `*.php`, `*.sql`, `Cargo.toml`, `package.json`
- Deployment manifests: `Dockerfile*`, `docker-compose*.yml`, `docker-compose*.yaml`, `kustomization.yaml`
- Runtime config and secrets: `.env`, `.env.*`
- Datasets or binary dumps not tied to research evidence

## Exceptions (Allowed Despite Patterns)
- `blueprint/scripts/blueprint_guard.py` (repo hygiene guardrail)

## Mapping to goni-prototype-lab
| If found in goni | Move to goni-prototype-lab |
|---|---|
| `blueprint/benchmarks/**` | `benchmarks/**` |
| `blueprint/demo/**` | `demo/**` |
| `blueprint/deploy/**` | `deploy/**` |
| `blueprint/eval/**` | `eval/**` |
| `blueprint/examples/**` | `examples/**` |
| `blueprint/tests/**` | `tests/**` |
| `blueprint/tools/**` | `tools/**` |
| `blueprint/scripts/**` (non-exception) | `scripts/**` |
| `blueprint/config/**` | `config/**` |
| `blueprint/software/kernel/**` | `software/kernel/**` |
| `.env.example` | `.env.example` |
| `.github/workflows/ci.yml` (build/test) | `.github/workflows/ci.yml` |

## Cross-repo references (policy)

This repository is blueprint-only. Any runnable artifact lives in `goni-prototype-lab`.

**Canonical reference format (guard-safe):**
- Use plain text: `goni-prototype-lab:<relative-path>`
  - Example: `goni-prototype-lab:goni-lab/STATUS.md`
  - Example: `goni-prototype-lab:deploy/k8s/`

**Do NOT:**
- Do not create markdown links that look like local paths for prototype-lab content
  (e.g., `[STATUS](/goni-prototype-lab:...)` or `(/blueprint/deploy/...)`).
  The blueprint guard treats these as in-repo paths and will flag them as broken.

**If you need clickability:**
- Use a full GitHub URL (allowed by the guard), e.g.
  `https://github.com/duracell04/goni-prototype-lab/blob/main/goni-lab/STATUS.md`
  `https://github.com/duracell04/goni-prototype-lab/tree/main/deploy/k8s`
