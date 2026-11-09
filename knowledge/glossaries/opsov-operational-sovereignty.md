---
id: OPSOV-GLOSS-01
title: "Operational sovereignty"
type: glossary
status: draft
implementation_state: not_applicable
proposition: "Operational sovereignty is the threat-relative and independently assessable degree of effective principal control over critical information, authorization, essential functions and recovery when specified external dependencies fail or are interfered with."
domains:
- governance
- security
- system
aliases: []
relations:
- type: refines
  target: GONI-GLOSSARY-C58A2FBFC264
- type: derived_from
  target: OPSOV-EVID-CONCEPT-01
sources:
- SRC-COUTURE-TOUPIN-2019
- SRC-POHLE-THIEL-2020
- SRC-KLEPPMANN2019-LOCAL-FIRST
artifacts: []
uncertainty: "This is a proposed Goni-specific operational definition, not a universally agreed meaning or an implemented security guarantee."
legacy: []
---

# Operational sovereignty

For a specified principal, protected asset and critical-function set, time horizon and threat model, **operational sovereignty** denotes the degree to which the principal can retain effective and independently assessable control over confidentiality, integrity, authorized operations, continuity and recovery despite declared dependency failures or interference.

**Human sovereignty** concerns who legitimately sets objectives, delegates and revokes authority. **Operational sovereignty** concerns whether that authority remains technically effective. **Jurisdictional exposure** concerns legal authorities and obligations affecting critical dependencies. The terms address distinct questions.

A claim requires a documented trust boundary, adversary class, critical-function profile, trusted computing base and evidence. Local hardware, encryption, air-gapping, local weights and owner-held keys are candidate control mechanisms; none individually establishes this system-level property.

The working definition is a Goni research proposal informed by the contested conceptual literature (Couture and Toupin, 2019; Pohle and Thiel, 2020) and local-first software research (Kleppmann et al., 2019).
