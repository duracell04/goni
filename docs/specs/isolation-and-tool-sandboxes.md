# Isolation and Tool Sandboxes (SANDBOX-01)
DOC-ID: SANDBOX-01

Status: Specified only / roadmap

Tools run with minimal authority and no ambient network.

## Requirements
- tools execute in isolated processes or containers
- root filesystem is read-only by default
- outbound network denied unless via egress gate

## Upstream
- [System map](../00-system-map.md)
- [Tool capability API](./tool-capability-api.md)
- [Network gate and anonymity](./network-gate-and-anonymity.md)

## Downstream
- [Runtime overview](../../runtime/00-overview.md)
- [Security overview](../../security/00-overview.md)

## Adjacent
- [Receipts](./receipts.md)
- [Threat model](../threat-model.md)
- [Governance hub](../hubs/governance.md)

## Conformance tests
- direct socket use from tool sandbox must fail
- egress via gate succeeds with valid capability


