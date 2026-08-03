---
id: GONI-SPEC-8E6F5412B93B
title: 5.2 Simple Node Lifecycle
type: specification
status: draft
implementation_state: specified_only
proposition: 'Joining a cluster: A new node must be joinable via a simple setup flow (e.g.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 5.2 Simple Node Lifecycle
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 5.2 Simple Node Lifecycle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.2 Simple Node Lifecycle

- Joining a cluster:
  - A new node must be joinable via a simple setup flow (e.g. pasting a join token or scanning a code).
- Leaving or failing:
  - If a node is shut down or fails, critical user data should not be lost.
  - The system should handle loss of a node gracefully, reducing capacity but not breaking basic functionality.

---
