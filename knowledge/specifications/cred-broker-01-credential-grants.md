---
id: CRED-BROKER-01
title: Credential Grant Broker
type: specification
status: draft
implementation_state: specified_only
proposition: Models and general tool runners should receive opaque credential handles rather than raw long-lived secrets; a broker should resolve authorized short-lived grants only at the execution boundary for the permitted tool, operation, account, audience, WorkOrder, scope, and lifetime.
domains:
- kernel
- security
- tools
aliases:
- credential broker
- scoped credential grant
relations:
- type: refines
  target: GONI-SPEC-31F371FDA554
- type: depends_on
  target: TOOL-01
- type: depends_on
  target: SPEC-ENF-01
sources: []
artifacts: []
uncertainty: Concrete credential mechanisms depend on target services and operating-system facilities. This specifies the authority and information-flow contract, not one secret-store implementation.
legacy: []
---

# Credential Grant Broker

Goni should keep credential material outside ordinary model context and outside long-lived ambient tool-runner state.

The intended flow is:

```text
Model / harness
  -> credential_handle
Kernel authorization
  -> CredentialGrant
Broker / secret store
  -> short-lived usable credential
Bound tool execution
  -> external service
```

## CredentialGrant

A logical grant should identify:

- `grant_id`;
- opaque `credential_handle`;
- account or principal ref;
- tool and operation IDs;
- permitted scopes;
- WorkOrder ref;
- capability-token ref;
- intended audience/service;
- permitted egress destination where relevant;
- issuance and expiry;
- one-shot or reuse semantics;
- sandbox/runtime binding;
- revocation ref;
- policy-decision ref; and
- resulting receipt ref.

The grant should contain or reference only the minimum authority required for the current operation.

## Model boundary

The model may reason over:

- account aliases;
- scope names;
- whether authorization is available;
- whether approval is required; and
- opaque credential handles.

The model should not receive raw passwords, API keys, refresh tokens, signing secrets, private keys, or unrestricted bearer tokens merely because it selected a tool.

## Tool-runner boundary

A tool runner receives usable credential material only after:

1. WorkOrder and tool proposal validation;
2. kernel policy and capability authorization;
3. tool/operation/scope matching;
4. audience and egress validation;
5. sandbox binding where required; and
6. expiry/revocation checks.

Usable secrets should be released as late as possible, retained for the shortest practical lifetime, and excluded from ordinary receipts, prompts, logs, and error messages.

## Failure and revocation

Expired, mismatched, revoked, or over-scoped grants fail closed. Retry policy must not silently widen credential scope. Revocation should prevent future resolution of the credential handle even when a model or stale task state still references it.

This contract extends the existing rule that tool runners should not receive long-lived cloud credentials by default into an explicit runtime object and lifecycle.
