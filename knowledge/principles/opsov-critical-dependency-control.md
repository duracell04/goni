---
id: OPSOV-PRINC-01
title: "Verifiable control of critical dependencies"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni's sovereign-delegation claims should be evaluated across the trusted dependency chain that preserves confidentiality, integrity, authorization, continuity and recoverability under specified adversaries."
domains:
- security
- governance
- system
aliases: []
relations:
- type: refines
  target: GONI-SYNTHESIS-8577DD9A5AD1
- type: depends_on
  target: GONI-GLOSSARY-C58A2FBFC264
- type: depends_on
  target: OPSOV-GLOSS-01
- type: derived_from
  target: OPSOV-EVID-SECURITY-01
sources:
- SRC-NIST-SP800-161R1-UPD1
- SRC-NIST-SP800-193
- SRC-NIST-SP800-160V2R1
- SRC-TORRES-ARIAS-IN-TOTO-2019
artifacts: []
uncertainty: "Proposed architecture principle. Claims of effective control require independently reviewed implementation and adversarial evidence."
legacy: []
---

# Verifiable control of critical dependencies

A principal's formally delegated authority is only as effective as the components enforcing it. Goni's kernel, memory, tool and network policies consequently depend on firmware, privileged operating-system controls, update mechanisms, key custodians, identity services, backups and external connectivity.

A sovereign deployment SHOULD identify the actors able to read, modify or disable critical functions; establish a trusted computing base and threat model; document prevention, detection and recovery controls; and publish residual dependencies with evidence.

This principle spans the existing four Goni architectural planes and adds **no new execution plane**. A privileged kernel compromise may undermine both authorization and the receipts intended to prove it. Detection claims require a verifier or evidence anchor outside the compromised boundary.

Self-hosting increases direct operator responsibility for patching, backup, key loss, hardware failures and continuity. Those trade-offs belong in the assurance assessment.
