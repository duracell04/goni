# NET-01 - Network Gate and Anonymity
DOC-ID: NET-01
Status: Specified only / roadmap
Conformance: TBD (goni-lab harness)

This spec defines network egress control for Goni OS. Networking is treated as
a capability-scoped syscall mediated by a reference monitor (Network Gate).
It formalizes two policy bundles ("Sovereign Mode" and "Anonymous Mode"),
adversary models, audit receipts, and failure modes.

## 1. Purpose

- "Two network personalities" is an information-flow control problem, not a UI
  toggle. The system must enforce where bytes may go under a declared policy.
- Networking is a governed side effect, analogous to Tool Capability API (TOOL-01).

## 2. Adversary models

This spec uses the following threat categories:

- Local attacker: untrusted local process/container attempting to exfiltrate
  data or bypass policy.
- Network observer: ISP or on-path observer correlating traffic and destinations.
- Provider adversary: cloud provider or API endpoint that sees request content.
- Global passive adversary: state-level observer with broad traffic visibility.

Out of scope by default: active hardware compromise, physical tamper, or a
global active adversary that can modify traffic without detection.

## 3. Components and trust domains

### 3.1 Network Gate (reference monitor)

The Network Gate is both:

- Policy Decision Point (PDP): decides allow/deny and route selection.
- Policy Enforcement Point (PEP): enforces budgets, timeouts, redactions,
  and route choice.

Complete mediation is required: no external egress may bypass the Gate.

### 3.2 Overlay Capsule (anonymity compartment)

The Overlay Capsule is a separate trust domain that hosts anonymity machinery
and exposes a single local proxy endpoint to the Gate. It is opt-in and
isolated from the rest of the runtime to keep the Trusted Computing Base (TCB)
small for Anonymous Mode.

### 3.3 Council mediation

Remote LLM traffic is just another egress class. The Gate routes Council calls
either DIRECT or OVERLAY according to policy (see docs/remote-llm-architecture.md).

## 4. Network egress syscall

Networking is a capability-scoped syscall:

```
net.egress(
  route,
  purpose,
  classification,
  budgets,
  retention_mode,
  payload_ref | stream_ref
) -> receipt_id
```

Requirements:

- Caller MUST present a network capability token (TOOL-01).
- Gate MUST validate route/purpose/budget/retention constraints.
- Gate MUST emit an audit receipt for every external transfer.

## 5. Policy model (Control Plane)

Network policies are declared and bound to agents via capability tokens and
policy_hash. At minimum:

- Route constraints: DIRECT vs OVERLAY.
- Purpose constraints: REMOTE_LLM, WEB_SEARCH, MESSAGING, UPDATE, TIME_SYNC, etc.
- Budgets: bytes, time, concurrency, retries, daily quota.
- Retention constraints: minimal vs verbose receipt logging.

Default policy is deny-by-default for external egress; explicit capability is
required for any route outside the local node.

## 5.1 Egress modes and guarantees

Egress modes are configured via `config/council.yaml` (or env) and enforced by
the Gate for all Council traffic:

- Mode 0: no egress (deny all remote calls).
- Mode 1: structured-only (no raw chunk text; summaries and structured fields only).
- Mode 2: redacted text allowed (apply the active redaction profile).
- Mode 3: user-approved full context (explicit user acknowledgement required).

The Gate MUST block any payload that does not match the configured mode and MUST
emit a receipt that records the mode, profile, and enforcement decision.

## 6. Security invariants

1. No external egress without Gate mediation.
2. No ambient internet for agents/tools; only the Gate has outbound privileges.
3. Overlay use requires explicit capability and policy approval.
4. Every egress emits an audit receipt tied to agent_id, policy_hash, and
   state_snapshot_id.
5. Anonymous Mode receipts are non-deanonymizing by default; logging is treated
   as a side channel and minimized unless explicitly enabled.

## 7. Two network personalities (policy bundles)

### 7.1 Sovereign Mode (default)

- Route default: DIRECT.
- Overlay default: DENY unless explicitly allowed per purpose.
- Reliability-first; normal DNS/network stack.
- Receipts emitted with minimal metadata by default.

### 7.2 Anonymous Mode (opt-in)

- Route default: OVERLAY for selected purposes or all external flows.
- Direct egress denied unless explicitly allowed.
- Strict logging discipline; receipts avoid destinations/URLs by default.
- Web/search seats, if used, run through the Overlay Capsule.

These are policy bundles, not bypass switches; the Gate remains the only egress
chokepoint in all modes.

## 8. TCB definition

### 8.1 Sovereign Mode TCB

Trust required for policy enforcement and audit:

- Goni kernel policy engine and capability token store.
- Network Gate (PDP/PEP).
- Local OS networking stack and firewall rules that enforce default-deny.
- Audit log writer (Control plane).

### 8.2 Anonymous Mode TCB

All Sovereign TCB components, plus:

- Overlay Capsule (overlay client + dependencies).
- Gate-to-capsule proxy interface.

Anonymous Mode does not expand trust in the rest of the runtime; anonymity is
opt-in and compartmentalized.

## 9. Auditing semantics (receipts)

Receipts prove policy application, not truth of content or external identity.
They are evidence that the Gate enforced the declared constraints at time of
egress. Receipts intentionally do not store raw payloads by default.

Recommended receipt fields (stored in AuditRecords.provenance):

- timestamp, duration_ms, bytes_up/down
- agent_id, capability_token_id
- policy_hash, state_snapshot_id
- purpose, classification, route_used
- budgets applied and budget exhaustion flags
- retention_mode and redaction mode applied

Anonymous Mode defaults to minimal retention and omits destinations/URLs unless
the user explicitly opts into verbose logging.

## 10. Failure modes (status-honest)

- Overlay unavailable:
  - DENY (strict anonymity policy)
  - PROMPT user to allow a one-off DIRECT route
  - DEGRADE to Sovereign Mode only if policy allows
- Network Gate unavailable: fail closed; deny all egress and surface error.
- Budget exhausted: deny and emit receipt with exhaustion flags.
- Council unreachable: fall back to local-only execution per
  docs/remote-llm-architecture.md.

## 11. Related specs

- Tool capability API: docs/specs/tool-capability-api.md
- Scheduler and interrupts: docs/specs/scheduler-and-interrupts.md
- Latent State Contract: docs/specs/latent-state-contract.md
- Remote LLM Architecture: docs/remote-llm-architecture.md

## Conformance tests
- TBD: add tests for this spec.



