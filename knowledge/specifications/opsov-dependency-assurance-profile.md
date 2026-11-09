---
id: OPSOV-SPEC-01
title: "Operational sovereignty assurance profile"
type: specification
status: draft
implementation_state: specified_only
proposition: "Every critical Goni function should have a threat-scoped dependency profile, recovery requirements, verifiable test oracles and explicitly stated assurance limitations."
domains:
- security
- system
- specs
aliases: []
relations:
- type: depends_on
  target: OPSOV-PRINC-01
- type: refines
  target: GONI-SYNTHESIS-A943990C6B33
- type: refines
  target: THREAT-MODEL
- type: refines
  target: GONI-THESIS-D2E69CC012F4
sources:
- SRC-NIST-SP800-161R1-UPD1
- SRC-NIST-SP800-193
- SRC-NIST-SP800-160V2R1
- SRC-TORRES-ARIAS-IN-TOTO-2019
- SRC-KLEPPMANN2019-LOCAL-FIRST
artifacts: []
uncertainty: "Specified only. Tests, trust roots, monitored egress boundaries and RTO/RPO are deployment-specific and remain unverified."
legacy: []
---

# Operational sovereignty assurance profile

## Dependency register

For each critical function record: human principal; data classification; offline requirements; privileged operators; third-party read/alter/disable mechanisms; software, firmware and boot roots; model and update suppliers; identity and key custody; network and storage dependencies; jurisdictional exposure; substitutions; recovery time objective (RTO), recovery point objective (RPO), evidence pointers and residual risk.

## Candidate acceptance criteria

| ID | Scenario | Test oracle |
| --- | --- | --- |
| OPSOV-A01 | Remote inference or cloud API outage | Declared offline critical tasks complete correctly; degraded or unavailable capabilities are documented. |
| OPSOV-A02 | Network denied or hostile | Monitored egress respects policy, with coverage limits and residual channels stated. |
| OPSOV-A03 | Malicious or unauthorized update | Unauthorized provenance or update authority is rejected at a stated trust boundary; recovery or rollback is tested. |
| OPSOV-A04 | External identity/provider revocation | Declared local identity, information access and export continue within the defined offline scope. |
| OPSOV-A05 | Key loss or corrupted backup | An authorized restoration satisfies measured RTO/RPO and confidentiality constraints. |
| OPSOV-A06 | Privileged OS/kernel/firmware compromise | Document invalidated guarantees; test only separately trusted detection or recovery components. |
| OPSOV-A07 | Device failure or replacement | Restore critical data and functions on replacement equipment within declared bounds. |
| OPSOV-A08 | Receipt tampering | Detect only where independent reference evidence is protected outside the attacker-controlled boundary. |

**Evidence rule:** Each `implemented_tested`, `enforced`, `verified` or `non-bypassable` claim requires pinned code and configuration, reproducible tests, observed result, adversary assumptions and residual limitations. Signed updates rely on key-custody assumptions; local receipts without independent integrity protection can be altered by a privileged attacker.

Prioritize synthetic and fault-injection tests before expensive hardware validation. Include isolation's operational and resource costs.
