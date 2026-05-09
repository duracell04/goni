---
id: EVAL-README-00
type: PLAYBOOK
status: specified_only
---
# Eval Lane

Evaluation artifacts for reliability, security, and reproducibility.

## Security lanes
- [Prompt injection](/blueprint/50-evidence/eval/EVID-SEC-01-prompt-injection.md)
- [Memory poisoning](/blueprint/50-evidence/eval/EVID-SEC-02-memory-poisoning.md)
- [Trace replayability](/blueprint/50-evidence/eval/EVID-SEC-03-trace-replayability.md)
- [Open adversarial evaluation](/blueprint/50-evidence/eval/EVID-ALIGN-01-open-adversarial-evaluation.md)

## Delegation lanes
- [Autonomy corridor validation](/blueprint/50-evidence/eval/EVID-AUTON-01-autonomy-corridors.md)
- [Risk threshold calibration](/blueprint/50-evidence/eval/EVID-AUTON-02-risk-threshold-calibration.md)
- [Policy-and-anomaly UX](/blueprint/50-evidence/eval/EVID-AUTON-03-policy-anomaly-ux.md)
- [Offloading safety longitudinal](/blueprint/50-evidence/eval/EVID-AUTON-04-offloading-safety-longitudinal.md)
- [Delegation mode classification](/blueprint/50-evidence/eval/EVID-AUTON-05-delegation-mode-classification.md)
- [Decisive question value](/blueprint/50-evidence/eval/EVID-AUTON-06-decisive-question-value.md)
- [Assumption coverage](/blueprint/50-evidence/eval/EVID-AUTON-07-assumption-coverage.md)
- [Branch efficiency](/blueprint/50-evidence/eval/EVID-AUTON-08-branch-efficiency.md)

## Long-context reading lanes
- [Reading strategy comparison](/blueprint/50-evidence/eval/EVID-LONGCTX-01-reading-strategy-comparison.md)
- [Corpus-reading fidelity](/blueprint/50-evidence/eval/EVID-LONGCTX-02-corpus-reading-fidelity.md)
- [Recursion-budget safety](/blueprint/50-evidence/eval/EVID-LONGCTX-03-recursion-budget-safety.md)

## Routing lanes
- [Frugal Sovereign Routing](/blueprint/50-evidence/eval/EVID-ROUTE-01-frugal-sovereign-routing.md)

## Harness governance lanes
- [Harness Change Evaluation](/blueprint/50-evidence/eval/EVID-HARNESS-01-harness-change-evaluation.md)

## Governance gate lanes

These lanes define required evaluation coverage for a sovereign operator. They
may reuse existing evidence files until dedicated artifacts are added.

- **Policy tests:** verify allow/deny decisions, corridor outcomes, approval
  evidence, and fail-closed behavior.
- **Route tests:** verify local-first routing, privacy-class handling, cloud
  escalation reasons, and refusal when no permitted route exists.
- **Memory retrieval tests:** verify permission filters, Work Order binding,
  expired/quarantined memory exclusion, and source waypoint fidelity.
- **Autonomy-safety tests:** verify open-loop detection, background autonomy
  triggers, risk thresholds, and user interruption rules.
- **Rollback tests:** verify idempotency, compensation paths, rollback refs, and
  no partial side effects after failure.
- **Parser/ingestion tests:** verify parser identity, source hashes,
  confidence flags, chunk boundaries, and `parser_basis` receipt coverage.
- **Open adversarial tests:** verify that promoted models, tools, memory
  policies, routing rules, harness changes, and autonomy expansions pass
  versioned adversarial scenarios before they move closer to the principal's
  runtime.

Governance gates answer "was this allowed and reversible?" in addition to
"did it work?" Standard observability metrics do not satisfy these gates by
themselves.

## Trace replay harness

Delegation evaluation uses trace replay rather than ad-hoc screenshots or
anecdotal demos.

Replay suites should include:

- vague intent prompts with gold deliverables,
- policy bundles that vary corridor, clarification, and visibility rules,
- traces that distinguish delegation from co-creation,
- receipts and audit traces for each mediated action,
- outcome labels covering success, overrides, unsafe autonomy, and question
  usefulness.

Minimum reported dimensions:

- quality/success,
- turns and latency,
- user corrections and overrides,
- interaction-mode classification accuracy,
- surfaced-assumption coverage,
- question count and question value,
- branch_count / variant_count_requested under fixed budgets.

Long-context reading evaluation should additionally report:

- reading strategy used,
- span/citation fidelity,
- scan/slice/subread counts,
- recursion depth,
- whether the strategy underperformed the native in-window baseline.
