---
id: GONI-PROPOSAL-31F550706163
title: 5. Keep authority profiles explicit
type: proposal
status: draft
implementation_state: specified_only
proposition: '**Conformant Goni** makes the owner the root authority.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: 5. Keep authority profiles explicit
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# 5. Keep authority profiles explicit

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5. Keep authority profiles explicit

**Conformant Goni** makes the owner the root authority. The kernel enforces the
owner's declared capability scopes, risk corridors, egress policy, and receipt
requirements; it does not enforce a model vendor's worldview. Routine and
reversible work should run without paternalistic confirmation loops. Any limit
that remains must be attributable to an owner-selected policy, a concrete
resource boundary, or a documented legal/technical constraint.

A **sovereign local-expression profile** is conformant: it may remove
application-level output filters, remote moderation, and provider-authored
policy prompts for local text generation. This does not weaken file, network,
credential, financial, communications, or device-control permissions. Optional
owner-defined filters remain pluggable rather than mandatory.

An **unrestricted-execution research profile** may also remove action gates,
but only inside a disposable, credential-free, offline sandbox containing
synthetic or replaceable data. Because it bypasses kernel mediation, it is
non-conformant and must not be represented as a production mode. Containment
belongs outside that runtime: no personal vault, network route, reusable
credentials, mounted home directory, or real communications account.

The dividing line is therefore liberty of local computation and expression
versus authority to impose effects on other systems or people. The first is
owner-controlled by default; the second remains explicitly delegated and
receipted.
