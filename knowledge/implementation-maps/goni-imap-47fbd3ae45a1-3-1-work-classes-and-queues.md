---
id: GONI-IMAP-47FBD3AE45A1
title: 3.1 Work classes and queues
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'We model the node as a discrete-time (or fluid-limit) queueing network with \(n=3\) **classes**: Class 1 â€“ interactive (chat, IDE, UI).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.1 Work classes and queues
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.1 Work classes and queues

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Work classes and queues

We model the node as a discrete-time (or fluid-limit) queueing network with \(n=3\) **classes**:

1. Class 1 â€“ interactive (chat, IDE, UI).  
2. Class 2 â€“ background (indexing, batch tools, fine-tuning).  
3. Class 3 â€“ maintenance (compaction, vacuum, WAL rotation).

Let:

- \(Q_i(t)\) = queue length of class \(i\) at time \(t\).  
- \(\lambda_i\) = average arrival rate (jobs / second).  
- \(\mu_i^{\max}\) = maximum service rate (jobs / second) when fully scheduled.  
- \(\rho_i = \lambda_i/\mu_i^{\max}\) = nominal load.  
- \(w_i > 0\) = priority weight.

We collect in vector form: \(\mathbf{Q}(t) = (Q_1(t),Q_2(t),Q_3(t))^\top\).
