---
id: GONI-SPEC-1341BABBEDC2
title: 9. Threat model hooks
type: specification
status: draft
implementation_state: specified_only
proposition: 'Desktop and browser agents add threat surfaces that ordinary chat agents do not: screen prompt injection that tries to convert observed text into authority, poisoned screen content that tries to escalate from extraction to actuation, memory poisoning through OCR/accessibility/history stores, local unsandboxed execution through synthetic input or shell tools,'
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 9. Threat model hooks
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 9. Threat model hooks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Threat model hooks

Desktop and browser agents add threat surfaces that ordinary chat agents do
not:

- screen prompt injection that tries to convert observed text into authority,
- poisoned screen content that tries to escalate from extraction to actuation,
- memory poisoning through OCR/accessibility/history stores,
- local unsandboxed execution through synthetic input or shell tools,
- privacy leakage through remote extraction of screen context,
- over-retention of private screenshots, OCR, or accessibility text.

The firewall mitigates these by separating powers, requiring policy mediation,
and forcing receipts for allowed and denied transitions.
