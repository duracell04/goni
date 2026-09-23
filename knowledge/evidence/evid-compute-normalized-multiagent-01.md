---
id: EVID-COMPUTE-NORMALIZED-MULTIAGENT-01
title: 'Source claim: multi-agent gains can be confounded by extra test-time compute'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Tran and Kiela show on multi-hop reasoning tasks that under matched reasoning-token budgets single-agent systems can match or outperform multi-agent systems, and that uncontrolled compute and context effects can inflate apparent multi-agent gains.
domains:
- research
- evaluation
- agents
aliases: []
relations:
- type: supports
  target: MULTIAGENT-EVAL-01
sources:
- SRC-TRAN2026-COMPUTE-NORMALIZED-MULTIAGENT
artifacts: []
uncertainty: The result is task- and setup-dependent and should not be generalized into a universal claim that single-agent architectures are superior.
legacy: []
---

# Source claim: multi-agent gains can be confounded by extra test-time compute

Tran and Kiela provide a methodological warning: architecture comparisons are invalid when one system receives materially more reasoning tokens or other test-time resources.

For Goni, Council and multi-agent experiments should therefore compare against strong single-agent baselines under matched resource budgets.
