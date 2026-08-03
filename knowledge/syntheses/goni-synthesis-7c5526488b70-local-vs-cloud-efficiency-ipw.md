---
id: GONI-SYNTHESIS-7C5526488B70
title: Local vs cloud efficiency (IPW)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Saad-Falcon et al. (2025) define **Intelligence per Watt (IPW)** and publish a harness that measures accuracy, energy, and latency per query across model × hardware pairs. We adopt IPW as the way to argue "local-first": for any workload we compare local IPW to cloud baselines and treat hybrid routing savings as a'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Local vs cloud efficiency (IPW)
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Local vs cloud efficiency (IPW)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Local vs cloud efficiency (IPW)

Saad-Falcon et al. (2025) define **Intelligence per Watt (IPW)** and publish a harness that measures accuracy, energy, and latency per query across model × hardware pairs. We adopt IPW as the way to argue "local-first": for any workload we compare local IPW to cloud baselines and treat hybrid routing savings as a design constraint, not an optimisation afterthought.
