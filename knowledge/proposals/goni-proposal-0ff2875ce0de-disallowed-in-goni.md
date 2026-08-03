---
id: GONI-PROPOSAL-0FF2875CE0DE
title: Disallowed in goni
type: proposal
status: draft
implementation_state: specified_only
proposition: 'Execution folders: blueprint/benchmarks/**, blueprint/demo/**, blueprint/deploy/**, blueprint/eval/**, blueprint/examples/**, blueprint/tests/**, blueprint/tools/** Scripts and runnable code: *.py, *.sh, *.ps1, *.rs, *.ts, *.js, *.go, *.java, *.kt, *.rb, *.php, *.sql, Cargo.toml, package.json Deployment manifests: Dockerfile*, docker-compose*.yml, docker-compose*.yaml, kustomization.yaml'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: SCOPE-CONTRACT.md
  heading: Disallowed in goni
  revision: 86d76976933ac0bfb8fe18839e67f17a8fc63531
---

# Disallowed in goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Disallowed in goni
- Execution folders: `blueprint/benchmarks/**`, `blueprint/demo/**`, `blueprint/deploy/**`, `blueprint/eval/**`, `blueprint/examples/**`, `blueprint/tests/**`, `blueprint/tools/**`
- Scripts and runnable code: `*.py`, `*.sh`, `*.ps1`, `*.rs`, `*.ts`, `*.js`, `*.go`, `*.java`, `*.kt`, `*.rb`, `*.php`, `*.sql`, `Cargo.toml`, `package.json`
- Deployment manifests: `Dockerfile*`, `docker-compose*.yml`, `docker-compose*.yaml`, `kustomization.yaml`
- Runtime config and secrets: `.env`, `.env.*`
- Datasets or binary dumps not tied to research evidence
