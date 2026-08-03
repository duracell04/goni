---
id: GONI-SYNTHESIS-DF627E719B9D
title: 3.2 Capability-based authority, not ambient authority
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Authority should be explicit and attenuable: capability tokens encode scope, constraints, and expiry, delegation attenuates rather than amplifies authority, revocation and expiry are first-class operational controls.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 3.2 Capability-based authority, not ambient authority
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.2 Capability-based authority, not ambient authority

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Capability-based authority, not ambient authority

Authority should be explicit and attenuable:
- capability tokens encode scope, constraints, and expiry,
- delegation attenuates rather than amplifies authority,
- revocation and expiry are first-class operational controls.

Related foundations:
- least privilege and protection principles [[saltzer1975-protection]]
- practical capability mode in UNIX context [[watson2010-capsicum]]
- object-capability delegation reasoning [[miller2003-capability-myths]]
- hardware capability line for future hardening [[watson2014-cheri]].
