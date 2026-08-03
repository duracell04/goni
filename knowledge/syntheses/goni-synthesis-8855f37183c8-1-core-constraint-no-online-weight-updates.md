---
id: GONI-SYNTHESIS-8855F37183C8
title: 1. Core constraint (no online weight updates)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni must not assume that a deployed model will learn its way out of failures.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 1. Core constraint (no online weight updates)
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 1. Core constraint (no online weight updates)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Core constraint (no online weight updates)
Goni must not assume that a deployed model will learn its way out of failures.
Runtime improvement is achieved by system-managed artifacts (memory, playbooks,
checkers, and policies), not by changing model weights in production.
[[tworek2026-decoder]]
