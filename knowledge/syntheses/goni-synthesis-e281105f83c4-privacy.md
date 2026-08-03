---
id: GONI-SYNTHESIS-E281105F83C4
title: PRIVACY
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Status: specified only / roadmap Data classes: public: safe to share sensitive: requires redaction secret: must not leave the device by default Default: fail closed at egress for sensitive/secret classes.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/privacy.md
  heading: PRIVACY
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# PRIVACY

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# PRIVACY

Status: specified only / roadmap

Data classes:
- public: safe to share
- sensitive: requires redaction
- secret: must not leave the device by default

Default: fail closed at egress for sensitive/secret classes.
