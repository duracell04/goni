---
id: GONI-SPEC-4320274E579A
title: 4.2 No Hidden Data Exfiltration
type: specification
status: draft
implementation_state: specified_only
proposition: 'No user data or model outputs should be sent externally without: clear policy, explicit user consent, and a record that can be inspected.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 4.2 No Hidden Data Exfiltration
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 4.2 No Hidden Data Exfiltration

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.2 No Hidden Data Exfiltration

- No user data or model outputs should be sent externally without:
  - clear policy,
  - explicit user consent, and
  - a record that can be inspected.
- External gateways or assistant frameworks must not substitute their own audit
  or permission model for Goni's kernel mediation. Any external effect still
  terminates in Goni capability checks and Goni receipts.

---
