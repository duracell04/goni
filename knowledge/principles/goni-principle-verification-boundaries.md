---
id: GONI-PRINCIPLE-VERIFICATION-BOUNDARIES
title: Verification does not cross execution boundaries automatically
type: principle
status: draft
implementation_state: specified_only
proposition: Evidence that one execution layer behaved correctly does not automatically establish correctness of delegated work performed outside that layer; compiler, accelerator, remote-node, and external-tool boundaries require explicit trust or verification semantics.
domains:
- compute
- security
- kernel
aliases:
- verification-boundaries
relations:
- type: depends_on
  target: COMP-01
- type: supports
  target: EVID-01
sources:
- SRC-RISC0-ZKVM
- SRC-SP1-SECURITY
artifacts: []
uncertainty: Concrete proof composition, compiler-validation, accelerator-checking, and remote attestation mechanisms remain workload- and backend-specific.
legacy: []
---

# Verification does not cross execution boundaries automatically

A proof or replay of one layer does not automatically authenticate computation delegated beyond that layer.

Examples include:

- source program to compiler or lowering pipeline;
- intermediate representation to target executable;
- CPU host to GPU or NPU accelerator;
- local scheduler to remote compute node;
- model runtime to side-effectful external tool.

If a CPU program receives a tensor produced by an accelerator, proving only the host program's control flow is insufficient unless the accepted claim also constrains the tensor's relation to the accelerator inputs or the accelerator result is trusted through an explicitly accepted mechanism.

Likewise, proving execution of an approved binary does not independently establish semantic equivalence between that binary and an unverified source program.

For each boundary, GONI should identify:

1. the artifact or claim being bound;
2. the component that is trusted;
3. the evidence mechanism;
4. the verifier or trust anchor;
5. the residual assumptions;
6. the failure and downgrade behavior.

The system may choose trust, replay, attestation, cryptographic proof, independent re-execution, or another mechanism. The architectural requirement is that the boundary remain explicit.
