# Personal Twin Autonomy Map

Purpose: map academic claims to enforceable Goni contracts so autonomy design
is traceable from literature to requirements, specs, conformance, and evidence.

## Claim map

### C-AUTON-01 Mixed-initiative decisions should be utility/risk aware
- Sources:
  - [[horvitz1999-mixed-initiative]]
- Mapped contracts:
  - `blueprint/software/10-requirements.md` (AUTON-02)
  - `blueprint/30-specs/delegation-and-autonomy.md` (risk-bounded default)
  - `blueprint/30-specs/tool-capability-api.md` (risk fields)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-02-risk-threshold-calibration.md`

### C-AUTON-02 Adjustable autonomy should be policy-constrained
- Sources:
  - [[bradshaw2004-adjustable-autonomy]]
- Mapped contracts:
  - `blueprint/software/10-requirements.md` (AUTON-01, AUTON-03)
  - `blueprint/30-specs/delegation-and-autonomy.md` (corridors + escalation)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-01-autonomy-corridors.md`

### C-AUTON-03 High automation and high human control can coexist
- Sources:
  - [[shneiderman2020-hcai-thci]]
  - [[shneiderman2020-hcai-ijhci]]
- Mapped contracts:
  - `blueprint/software/90-decisions.md` (D-022)
  - `blueprint/software/10-requirements.md` (AUTON-03, AUTON-04)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-03-policy-anomaly-ux.md`

### C-AUTON-04 Cognitive offloading has both benefits and deskilling risk
- Sources:
  - [[clark1998-extended-mind]]
  - [[risko2016-cognitive-offloading]]
- Mapped contracts:
  - `blueprint/software/10-requirements.md` (AUTON-04 safeguards)
  - `blueprint/docs/metrics.md` (oversight, unsafe autonomy, rollback metrics)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-04-offloading-safety-longitudinal.md`

### C-AUTON-05 Routine digital process work is highly automatable
- Sources:
  - [[haegner2020-rpa-slr]]
  - [[smagul2023-rpa-review]]
  - [[guner2020-rpa-capability]]
- Mapped contracts:
  - `blueprint/30-specs/delegation-and-autonomy.md` (SOP lifecycle)
  - `blueprint/software/10-requirements.md` (SOP lifecycle and packs)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-01-autonomy-corridors.md`

### C-AUTON-06 Human digital twin concepts can be localized and user-owned
- Sources:
  - [[gaffinet2025-human-digital-twin]]
  - [[zafar2024-hdt-business-review]]
- Mapped contracts:
  - `blueprint/software/90-decisions.md` (D-021, D-022)
  - `blueprint/software/10-requirements.md` (local-first delegation)
- Evidence lane:
  - `blueprint/50-evidence/eval/EVID-AUTON-04-offloading-safety-longitudinal.md`
