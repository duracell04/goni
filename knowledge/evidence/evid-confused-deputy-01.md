---
id: EVID-CONFUSED-DEPUTY-01
title: 'Source claim: authority confusion can cause a deputy to misuse ambient privilege'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Hardy's confused-deputy analysis shows how a program acting for multiple parties can misuse authority when request provenance and authority provenance are not cleanly bound.
domains: [research, security, authority]
aliases: []
relations:
- type: supports
  target: TRUST-INPUT-01
sources: [SRC-HARDY1988-CONFUSED-DEPUTY]
artifacts: []
uncertainty: Prompt injection is broader than the original confused-deputy problem; this source supports the authority-confusion analogy rather than equating all prompt injection with confused-deputy failures.
legacy: []
---

# Authority confusion can cause a deputy to misuse ambient privilege

The confused-deputy problem demonstrates that legitimate authority can be exercised for the wrong requester or purpose. Delegated AI therefore needs an explicit binding between request provenance and the authority under which an effect is performed.
