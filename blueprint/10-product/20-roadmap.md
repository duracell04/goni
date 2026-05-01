# ROADMAP
Status: Specified only / roadmap

This roadmap tracks product-facing milestones.
Each milestone must remain aligned with normative contracts in `blueprint/30-specs/`.

## Milestone v0.1 - Trustable mediation baseline

- Capability checks at every mediated tool action (default deny by policy).
- Receipts v1 with chain verification and minimal disclosure defaults.
- Egress gate path wired for mediated external calls only.
- Smoke path proving "propose -> approve -> execute -> receipt."
- Kernel contract path proving Work Order -> Done Contract -> capability token
  -> policy decision -> sandbox class -> receipt tier.
- Parser and memory-write receipts for governed Knowledge & Context
  Engineering.

## Milestone v0.2 - Non-bypass and isolation proof

- Tool sandbox profile with negative tests for bypass attempts.
- Integration tests for direct egress denial outside the gate.
- Budget and policy outcomes recorded in receipts for replayability.
- Mixed-load harness that reports interactive vs background behavior.
- Action-risk isolation matrix tests for reversible writes, code execution,
  external side effects, and irreversible high-risk actions.
- Background autonomy taxonomy fixtures for open-loop detection, memory
  consolidation, periodic audits, model evaluation, policy drift checks, and
  scheduled briefs.
- Model provenance path covering ModelManifest, InstallReceipt, EvalReceipt,
  and RollbackRef before model promotion.

## Milestone v0.3 - Safe extension and operations layer

- Agent/skill manifest validation with scoped permissions and budgets.
- Signed extension bundle flow and revocation/quarantine path.
- Vault v0 as local-first durable memory plus provenance pointers.
- Upgrade and rollback playbook linked to incident evidence artifacts.

## Milestone v0.4 - Managed distribution posture

- Fleet-oriented policy templates and drift detection basics.
- Security update channel with signed artifact requirements.
- Compliance-oriented export bundle (receipts, policy decisions, audit trail).
- Reference "secure updates for long support windows" contract draft.

## Roadmap principles

- Do not position "Linux distro" as the milestone outcome.
- Differentiate on governance, verification, and accountable execution.
- Keep status language honest: specified vs implemented must stay explicit.
