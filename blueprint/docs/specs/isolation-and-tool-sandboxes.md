# Isolation and Tool Sandboxes (SANDBOX-01)
DOC-ID: SANDBOX-01

Status: Specified only / roadmap

Tools run with minimal authority and no ambient network.

## Requirements
- tools execute in isolated processes or containers
- root filesystem is read-only by default
- outbound network denied unless via egress gate

## Upstream
- [Tool capability API](/blueprint/docs/specs/tool-capability-api.md)
- [Network gate and anonymity](/blueprint/docs/specs/network-gate-and-anonymity.md)
- [Privacy and text confinement](/blueprint/software/50-data/40-privacy-and-text-confinement.md)

## Downstream
- [OS and base image](/blueprint/software/30-components/os-and-base-image.md)
- [Sandbox README](/blueprint/runtime/sandbox/README.md)

## Adjacent
- [Receipts](/blueprint/docs/specs/receipts.md)
- [Threat model](/blueprint/docs/threat-model.md)

## Conformance tests
- direct socket use from tool sandbox must fail
- egress via gate succeeds with valid capability


