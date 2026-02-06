# Move/Delete Plan (Deterministic)

This table is derived from `git ls-files` in `goni/` and maps every execution-shaped artifact to delete, move, or rename.

| current_path | action | destination_path | reason | docs_to_update |
|---|---|---|---|---|
| blueprint/benchmarks/README.md | delete | - | Placeholder; benchmarks live in prototype lab | - |
| blueprint/benchmarks/runner/README.md | delete | - | Placeholder; runner lives in prototype lab | - |
| blueprint/demo/data/README.md | delete | - | Placeholder; demo data lives in prototype lab | - |
| blueprint/deploy/compose/README.md | delete | - | Placeholder; deploy manifests live in prototype lab | - |
| blueprint/deploy/k8s/README.md | delete | - | Placeholder; deploy manifests live in prototype lab | - |
| blueprint/eval/README.md | delete | - | Placeholder; eval scenarios live in prototype lab | blueprint/docs/EVALUATION.md |
| blueprint/examples/README.md | delete | - | Placeholder; examples live in prototype lab | - |
| blueprint/tests/integration/nonbypass/README.md | rename | blueprint/docs/metrics/nonbypass-integration-metrics.md | Spec content should live under docs, not tests | - |
| blueprint/tests/invariants/README.md | delete | - | Placeholder | - |
| blueprint/tools/README.md | delete | - | Placeholder; tool manifests live in prototype lab | blueprint/docs/tools.md |
| blueprint/tools/sim/README.md | delete | - | Placeholder | - |
| blueprint/scripts/generate_agents.py | delete | - | Duplicate of prototype lab script | blueprint/AGENTS.md, blueprint/docs/meta/agents.root.template.md, blueprint/10-product/30-next-steps.md, blueprint/docs/STATUS.md |
| blueprint/scripts/validate_truth_map.py | delete | - | Duplicate of prototype lab script | blueprint/AGENTS.md, blueprint/docs/meta/agents.root.template.md, blueprint/10-product/30-next-steps.md, blueprint/docs/STATUS.md |
| blueprint/docs/STATUS.md | move | goni-prototype-lab/goni-lab/STATUS.md | Implementation status belongs in prototype lab | blueprint/README.md, blueprint/00-map/40-community.md, blueprint/docs/00-system-map.md, blueprint/docs/architecture/index.md, blueprint/30-specs/receipts.md, blueprint/docs/hubs/governance.md, blueprint/docs/hubs/product-surfaces.md |
| blueprint/docs/TRACEABILITY.md | move | goni-prototype-lab/goni-lab/TRACEABILITY.md | Spec-to-code matrix belongs in prototype lab | blueprint/docs/00-system-map.md, blueprint/docs/architecture/index.md, blueprint/30-specs/receipts.md, blueprint/docs/hubs/governance.md, blueprint/docs/hubs/product-surfaces.md, blueprint/00-map/40-community.md |
| blueprint/docs/quickstart.md | move | goni-prototype-lab/goni-lab/quickstart.md | Runnable quickstart belongs in prototype lab | CONTRIBUTING.md, blueprint/README.md, blueprint/docs/README.md |
| blueprint/docs/demo_expected.md | move | goni-prototype-lab/demo/demo_expected.md | Runnable demo outputs belong in prototype lab | blueprint/10-product/90-research-agenda.md |
