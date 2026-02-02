# Isolation and Tool Sandboxes (SANDBOX-01)
DOC-ID: SANDBOX-01

Status: Specified only / roadmap

Tools run with minimal authority and no ambient network.

## Requirements
- tools execute in isolated processes or containers
- root filesystem is read-only by default
- outbound network denied unless via egress gate

## Conformance tests
- direct socket use from tool sandbox must fail
- egress via gate succeeds with valid capability

