---
id: GONI-SYNTHESIS-AE2511CE8769
title: 3. Trust boundaries
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Untrusted by default: model output text, retrieved external text, third-party tools and extensions, connector responses from remote services.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/20-trust-model.md
  heading: 3. Trust boundaries
  revision: 628398028a2ae5fe5696b6b3ec004da2314ddd96
---

# 3. Trust boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Trust boundaries

Untrusted by default:
- model output text,
- retrieved external text,
- third-party tools and extensions,
- connector responses from remote services.

Trusted only through mediation:
- authority tokens/capabilities,
- policy decisions and budget checks,
- egress approval outcomes,
- receipt emission and verification.
