---
id: GONI-SPEC-80D74057B99C
title: 5.1 Egress modes and guarantees
type: specification
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: Egress modes are configured via goni-prototype-lab:config/council.yaml (or env) and enforced by the Gate for all Council traffic: Mode 0: no egress (deny all remote calls).'
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 5.1 Egress modes and guarantees
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 5.1 Egress modes and guarantees

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5.1 Egress modes and guarantees

Egress modes are configured via `goni-prototype-lab:config/council.yaml` (or env) and enforced by
the Gate for all Council traffic:

- Mode 0: no egress (deny all remote calls).
- Mode 1: structured-only (no raw chunk text; summaries and structured fields only).
- Mode 2: redacted text allowed (apply the active redaction profile).
- Mode 3: user-approved full context (explicit user acknowledgement required).

The Gate MUST block any payload that does not match the configured mode and MUST
emit a receipt that records the mode, profile, and enforcement decision.
