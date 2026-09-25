---
id: DELEG-AUTHORITY-01
title: Authority conservation and attenuation
type: principle
status: draft
implementation_state: specified_only
proposition: Delegated authority may remain equal to or narrower than the valid authority inherited from its upstream grant, while any expansion requires a fresh valid authorization path from an authority source permitted to grant it.
domains: [delegation, authority, security, kernel]
aliases:
- authority attenuation
- delegated authority conservation
relations:
- type: refines
  target: GONI-SYNTHESIS-DF627E719B9D
- type: depends_on
  target: DELEG-MODEL-01
sources:
- SRC-IBRAHIM2026-OVERLAY-GOVERNANCE
artifacts: []
uncertainty: Real systems may combine several independent grants; the subset relation applies per effective authorization derivation and requires an explicit composition rule when multiple principals or grants are involved.
legacy: []
---

# Authority conservation and attenuation

For a single valid delegation lineage, Goni should preserve:

[
A_{downstream}subseteq A_{upstream}subseteq A_{principal}
]

A downstream agent, subprocess, model, harness, tool, or delegated worker cannot gain broader authority merely because it is more capable, because a workflow decomposes, or because another component exposes additional tools.

Authority expansion requires a new valid authorization event from an authority source permitted to grant that expansion.

This principle separates **delegation** from **capability discovery**:

- discovering a tool does not grant permission to use it;
- retrieving information does not grant authority to act on it;
- a stronger model does not inherit a wider corridor;
- delegation to another worker does not silently amplify scope;
- retries and replanning do not renew expired or revoked authority.

Where multiple grants compose, the kernel must apply an explicit composition rule and preserve each contributing authority lineage.
