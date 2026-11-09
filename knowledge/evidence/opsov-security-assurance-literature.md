---
id: OPSOV-EVID-SECURITY-01
title: "Evidence: supply-chain and cyber-resilience trust boundaries"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "NIST guidance and verifiable supply-chain research establish the relevance of privileged firmware, update provenance and recovery to system-wide integrity and continuity."
domains:
- research
- security
aliases: []
relations:
- type: supports
  target: OPSOV-PRINC-01
- type: supports
  target: OPSOV-SPEC-01
sources:
- SRC-NIST-SP800-161R1-UPD1
- SRC-NIST-SP800-193
- SRC-NIST-SP800-160V2R1
- SRC-TORRES-ARIAS-IN-TOTO-2019
artifacts: []
uncertainty: "Standards and research motivate candidate controls; they do not document Goni implementation."
legacy: []
---

# Evidence: supply-chain and cyber-resilience trust boundaries

NIST SP 800-161 Rev. 1 Update 1 addresses supply-chain risk, SP 800-193 firmware protection and recovery, and SP 800-160 Vol. 2 Rev. 1 cyber-resilience engineering. Torres-Arias et al. (2019) propose the `in-toto` framework for cryptographically verifiable supply-chain steps.

**Goni inference:** authority mediation relies on trustworthy privileged components, provenance and independent recovery paths.

**Limit:** security control descriptions are not proof of their use or adequacy in Goni; all claims retain their own assumptions and trust roots.
