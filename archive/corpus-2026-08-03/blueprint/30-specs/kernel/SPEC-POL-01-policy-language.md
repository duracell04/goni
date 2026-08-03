---
id: SPEC-POL-01
type: SPEC
status: specified_only
---
# SPEC-POL-01 - Policy Language (Capabilities + IFC)
DOC-ID: SPEC-POL-01
Status: Specified only / roadmap

This spec defines the policy language and decision model that composes
capability authorization, budget constraints, and information-flow controls.

## 1. Policy posture

Policy is default-deny:
- all effectful operations are denied unless explicitly authorized,
- all data downgrades require explicit declassification rules,
- all policy decisions are machine-readable and receipt-linked.

## 2. Capability schema

Each capability grant includes:
- `capability_id`
- `subject` (agent/tool identity)
- `resource_scope` (fs/net/api/object set)
- `action_set` (read/write/execute/send/etc.)
- `budget_caps` (calls, bytes, time, spend units)
- `valid_from`, `expires_at`
- `revocation_ref`

Capabilities are non-transferrable unless explicit delegation rule exists.

## 3. Resource scopes

Minimum scope types:
- `fs_scope`: path-prefix allowlist with mode constraints,
- `net_scope`: domain/method/port allowlist,
- `api_scope`: connector method allowlist,
- `state_scope`: mutable table/object IDs.

Scope matching is exact or prefix-bounded; wildcards require explicit policy
approval and must be highlighted in decision output.

## 4. Budget capabilities

Budgets are first-class policy constraints:
- `max_calls`
- `max_bytes_out`
- `max_cpu_ms`
- `max_wall_ms`
- `max_cost_units`

Budget exhaustion is a deny condition with structured decision output.

## 5. Data labels and IFC model

The policy language defines labels for protected data classes, for example:
- `public`
- `internal`
- `user_private`
- `sensitive_pii`
- `secret`

Every mediated operation evaluates:
- source label set,
- destination label constraints,
- declassification requirements.

## 6. Declassification rules

A declassification rule must include:
- authorized actor class,
- allowed downgrade path (e.g., `secret -> internal`),
- justification code,
- optional approval requirement.

Any declassification emits a dedicated receipt entry with justification.

## 7. Decision output contract

Every policy decision returns:
- `decision` (`allow` | `deny` | `allow_with_constraints`)
- `matched_rules`
- `capability_id`
- `budget_delta`
- `label_flow_decision`
- `declassification_ref` (if applied)
- `policy_hash`

Decision output is mandatory input to receipt emission.

## 8. Invariants

- Policy completeness: default-deny with explicit allow rules only.
- No hidden downgrade: all label downgrades are explicit and logged.
- No ambient budget use: every resource spend maps to a capability budget.
- Deterministic decision encoding for same policy + input context.

## 9. Upstream
- [Symbolic substrate](/blueprint/30-specs/symbolic-substrate.md)
- [Agent definition](/blueprint/30-specs/agent-definition.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)

## 10. Downstream
- [Agent kernel ABI](/blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md)
- [Transactional tools](/blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 11. Adjacent
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)

## Conformance tests
- Any request lacking an allow rule is denied.
- Label downgrade without matching declassification rule is denied.
- Budget exhaustion yields deny with machine-readable cause.
- Every allow/deny decision is linked to a receipt via `policy_hash` and
  decision reference.
