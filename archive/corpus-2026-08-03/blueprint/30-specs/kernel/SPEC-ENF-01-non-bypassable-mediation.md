---
id: SPEC-ENF-01
type: SPEC
status: specified_only
---
# SPEC-ENF-01 - Non-bypassable Mediation on Linux
DOC-ID: SPEC-ENF-01
Status: Specified only / roadmap

This spec defines enforcement placement required to realize complete mediation
on commodity Linux deployments.

## 1. Enforcement objective

Guarantee that untrusted tool/runtime components cannot bypass policy and
receipt mediation for:
- network egress,
- filesystem mutation,
- external connector side effects.

## 2. TCB definition

Minimum trusted computing base for mediation:
- policy engine and capability issuer,
- mediation gateway (agent syscall boundary),
- receipt root and integrity chain,
- egress gateway/proxy and decision logger.

Untrusted by default:
- model runtimes,
- tool plugin code,
- connector adapters,
- user-defined automation scripts.

## 3. Placement model

### 3.1 Tool runner boundary

Tool runners execute in constrained sandboxes:
- separate UID and mount namespace,
- restricted syscall profile,
- no direct credential material,
- no ambient privileged file descriptors.

### 3.2 Egress architecture

Only mediation gateway and egress gateway are allowed outbound routes.
Tool runners have:
- no direct outbound socket permission,
- explicit proxy path for permitted calls,
- trace/capability metadata attached per request.

### 3.3 Filesystem mutation path

Mutating writes occur through mediated broker path:
- write roots constrained by capability scope,
- direct writes outside approved roots denied,
- write intent linked to transaction and receipt chain.

## 4. Supported isolation backends

Conformant implementations may use one or more:
- namespace/cgroup/seccomp profile sets,
- microVM isolation,
- sandbox runtimes with syscall mediation,
- WASI-style execution for selected tools.

Regardless of backend, mediation invariants are identical.

## 5. Credential and secret handling

- Tool runners must not receive long-lived cloud credentials by default.
- Secrets are requested via scoped handles and short-lived grants.
- Any secret-bound external call must be policy-mediated and receipt-linked.

## 6. Attribution requirements

All egress and mutating writes must be attributable to:
- `trace_id`
- `span_id`
- `capability_id`
- `policy_hash`
- `job_id`

Attribution metadata is mandatory in receipts.

## 7. Invariants

- No direct socket egress from tool runner context.
- No filesystem mutation without capability-scoped mediated path.
- No side effect without policy decision artifact.
- No mediated side effect without receipt emission.

## 8. Upstream
- [Agent kernel ABI](/blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md)
- [Policy language](/blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md)
- [Transactional tools](/blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md)

## 9. Downstream
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## 10. Adjacent
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Evidence: egress non-bypass](/blueprint/50-evidence/validation/EVID-ENF-01-egress-nonbypass.md)

## Conformance tests
- Direct socket syscalls from tool runners fail in baseline policy profile.
- Tool runners cannot write outside capability-authorized roots.
- All allowed egress events include trace and capability attribution fields.
- Forced policy-engine outage fails closed for effectful operations.
