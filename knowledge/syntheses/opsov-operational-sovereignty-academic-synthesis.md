---
id: OPSOV-SYN-01
title: "Operational sovereignty and dependency assurance in Goni"
type: synthesis
status: draft
implementation_state: specified_only
proposition: "Operational sovereignty operationalizes effective control over the dependencies that uphold Goni's delegated human authority, subject to specified threats and independently verifiable evidence."
domains:
- governance
- research
- security
- system
aliases: []
relations:
- type: synthesizes
  target: OPSOV-GLOSS-01
- type: synthesizes
  target: OPSOV-GLOSS-JURIS-01
- type: synthesizes
  target: OPSOV-PRINC-01
- type: synthesizes
  target: OPSOV-SPEC-01
- type: synthesizes
  target: OPSOV-EXP-01
- type: synthesizes
  target: OPSOV-OBJ-01
- type: synthesizes
  target: OPSOV-EVID-CONCEPT-01
- type: synthesizes
  target: OPSOV-EVID-SECURITY-01
- type: synthesizes
  target: OPSOV-EVID-CRYPTO-01
- type: synthesizes
  target: OPSOV-EVID-ICC-01
- type: synthesizes
  target: OPSOV-EVID-SALT-01
- type: refines
  target: GONI-SYNTHESIS-A943990C6B33
sources:
- SRC-COUTURE-TOUPIN-2019
- SRC-POHLE-THIEL-2020
- SRC-KLEPPMANN2019-LOCAL-FIRST
- SRC-NIST-SP800-161R1-UPD1
- SRC-NIST-SP800-193
- SRC-NIST-SP800-160V2R1
- SRC-TORRES-ARIAS-IN-TOTO-2019
artifacts: []
uncertainty: "Literature-grounded architecture proposal, not an implemented sovereignty guarantee or demonstrated security benchmark."
legacy: []
---

# Operational sovereignty and dependency assurance in Goni

## Abstract

Goni separates AI-model cognition from the legitimate authority to act. This synthesis extends the analysis from the delegation boundary to the privileged infrastructure sustaining that boundary. It proposes *operational sovereignty* as a threat-relative, independently assessable assurance property involving data control, authorization, service continuity and recovery.

## Theory and prior literature

Digital sovereignty is not a single agreed technical criterion. Couture and Toupin (2019) document diverse groups and interpretations; Pohle and Thiel (2020) study a contested policy concept. Goni distinguishes **human sovereignty** (the legitimacy of principal goals and delegated authority), **operational sovereignty** (the practical persistence of that authority under external interference), and **jurisdictional exposure** (legal authorities and obligations affecting dependencies).

Local-first research emphasizes offline functionality, user agency and durable data ownership (Kleppmann et al., 2019). Goni accordingly treats the kernel as the authority mediator and network access as an explicitly mediated effect. Nevertheless, the kernel's enforcement is conditional upon firmware, OS privilege, boot and update roots, cryptographic keys, external identity services and recovery facilities.

NIST SP 800-161 Rev. 1 Update 1 provides a supply-chain risk framework; SP 800-193 addresses firmware integrity and recovery; SP 800-160 Vol. 2 Rev. 1 formalizes cyber-resilience engineering. The `in-toto` work by Torres-Arias et al. (2019) illustrates verifiable software-supply-chain steps. These sources establish relevant classes of mechanisms and risks, **not** Goni-specific assurance. A privileged compromised component can also alter the very receipts used to document its behavior.

## Case evidence and epistemic boundaries

The Crypto AG parliamentary investigation motivates a supplier-control scenario. The FBI's Salt Typhoon announcement concerns compromised telecommunications infrastructure. The 2025 ICC email controversy is distinct: a UK parliamentary answer documented media allegations of a Microsoft account suspension **and Microsoft's strong denial**. The alleged cutoff remains disputed and must not be represented as established fact. Each case informs a different threat hypothesis; none quantifies Goni risk or proves a Goni security failure.

## Design and test program

OPSOV-SPEC-01 requires profiles of critical functions, operators able to read/alter/disable them, trusted components, jurisdictional exposure, recovery, evidence and residual dependencies. OPSOV-EXP-01 proposes matched experiments comparing a local/mediated baseline with additional independent control over provenance, identity, tamper evidence and recovery.

**H1 (unverified):** local inference plus kernel authorization leaves measurable failure modes where other critical dependencies remain externally controlled. Report results by scenario with explicit boundaries, correctness, recovery, operator effort and cost. The system's four-plane architecture remains unchanged.

## Contribution and limitation

Operational sovereignty is an engineering hypothesis and assurance objective: the authority of the human principal should remain effective under specified adverse conditions, with evidence proving exactly which properties hold. Absolute independence, universal security and exemption from applicable law are outside this claim. All design-bearing nodes remain `specified_only`.

**Research question:** Under what adversarial and failure conditions can a principal continue to access, govern, verify and recover critical delegated-AI functions despite third-party dependency loss or interference?
