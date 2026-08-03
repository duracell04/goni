---
id: GONI-SYNTHESIS-312D277D0C2E
title: 2. Digital meninges
type: synthesis
status: draft
implementation_state: specified_only
proposition: The meningeal names are mnemonic groupings, not separate runtime services.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/60-cognitive-exocortex-model.md
  heading: 2. Digital meninges
  revision: cdf162b26a4fe7d78e6daa6039696e89ee0ef17f
---

# 2. Digital meninges

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Digital meninges

The meningeal names are mnemonic groupings, not separate runtime services.

| Metaphor | Question | Existing controls |
| --- | --- | --- |
| **Dura** | Who or what may enter, execute, or control the system? | Host and device security, identity, secrets isolation, sandboxing, encrypted storage and backups, egress policy, and the [trust model](/blueprint/20-system/20-trust-model.md). |
| **Arachnoid** | How may context and commands move between components? | Work Orders, jobs, capability tokens, scheduling, budgets, context isolation, agent manifests, and the [tool capability](/blueprint/30-specs/tool-capability-api.md) and [scheduler](/blueprint/30-specs/scheduler-and-interrupts.md) contracts. |
| **Pia** | What is a claim, where did it come from, and how may it be used? | Source references, parser identity, hashes, confidence, validity, permissions, contradiction state, review status, redaction, receipts, and [governed memory retrieval](/blueprint/30-specs/memory-retrieval.md). |

External text, model output, parser output, and connector responses remain
untrusted. Provenance supports inspection and challenge; it does not turn a
claim into truth. Receipts provide operational evidence and tamper indication,
not epistemic certainty.
