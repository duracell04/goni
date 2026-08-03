---
id: GONI-EVIDENCE-52DCF407A2C5
title: 'Source claim: cheng2026-dspark'
type: evidence
status: draft
implementation_state: not_applicable
proposition: DSpark is a speculative decoding framework that combines semi-autoregressive drafting, confidence-scheduled verification, and hardware-aware scheduling. Its reported live DeepSeek-V4 deployment gains are 60%-85% per-user generation speed for V4-Flash and 57%-78% for V4-Pro against the prior MTP-1 baseline. The nominal +661% aggregate throughput figure is a strict-SLA frontier result, not a representative general 7x throughput claim; moderate aggregate throughput gains are closer to about 50%.
domains:
- research
aliases: []
relations:
- type: supports
  target: GONI-IMAP-45DA8323C140
- type: supports
  target: LLM-RUNTIME
sources:
- SRC-CHENG2026-DSPARK
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[cheng2026-dspark]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: cheng2026-dspark

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[cheng2026-dspark]]
Claim: DSpark is a speculative decoding framework that combines
semi-autoregressive drafting, confidence-scheduled verification, and
hardware-aware scheduling. Its reported live DeepSeek-V4 deployment gains are
60%-85% per-user generation speed for V4-Flash and 57%-78% for V4-Pro against
the prior MTP-1 baseline. The nominal +661% aggregate throughput figure is a
strict-SLA frontier result, not a representative general 7x throughput claim;
moderate aggregate throughput gains are closer to about 50%.
Relevance:
- Supports Goni's adaptive-inference doctrine: draft cheaply, verify with
  calibrated confidence, and schedule verifier work according to real hardware
  load.
Used in:
- `blueprint/software/20-architecture.md`
- `blueprint/software/30-components/llm-runtime.md`
Source:
- https://www.alphaxiv.org/abs/2026.dspark
