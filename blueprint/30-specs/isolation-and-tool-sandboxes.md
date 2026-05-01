---
id: SANDBOX-01
type: SPEC
status: specified_only
---
# Isolation and Tool Sandboxes (SANDBOX-01)
DOC-ID: SANDBOX-01

Status: Specified only / roadmap

Tools run with minimal authority and no ambient network.

## Requirements
- tools execute in isolated processes or containers
- root filesystem is read-only by default
- outbound network denied unless via egress gate

## Action-risk isolation matrix

Sandboxing is a hard trust boundary, not a deployment preference. Minimum
isolation follows the action class:

| Action class | Examples | Minimum isolation | Gate type |
| --- | --- | --- | --- |
| `read_only_retrieval` | local search, metadata read, vector lookup | process isolation or equivalent capability sandbox | capability check |
| `reversible_local_write` | draft file, temporary index update, local note edit with snapshot | container/gVisor-style sandbox plus rollback reference | soft gate or autopilot if policy allows |
| `code_execution` | shell, Python, browser automation with local credentials, package install | microVM/Firecracker-class isolation or stronger; no ambient host credentials | hard gate unless explicitly pre-approved |
| `external_side_effect` | email send, calendar mutation, API write, publish action | isolated executor plus egress gate and idempotency/rollback plan where possible | soft or hard gate by risk |
| `irreversible_high_risk` | financial, legal, account deletion, regulated data release | strongest available isolation, dual receipt, explicit human confirmation | hard gate |

The matrix defines minimums. Policy may require stronger isolation or block the
action. If the required isolation is unavailable, execution fails closed.

## Upstream
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Privacy and text confinement](/blueprint/software/50-data/40-privacy-and-text-confinement.md)

## Downstream
- [OS and base image](/blueprint/software/30-components/os-and-base-image.md)
- [Sandbox README](/blueprint/runtime/sandbox/README.md)

## Adjacent
- [Receipts](/blueprint/30-specs/receipts.md)
- [Threat model](/blueprint/docs/threat-model.md)

## Conformance tests
- direct socket use from tool sandbox must fail
- egress via gate succeeds with valid capability
- unavailable required isolation fails closed
- irreversible high-risk actions require explicit approval evidence and dual
  receipts




