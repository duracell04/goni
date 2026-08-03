---
id: GONI-SYNTHESIS-0FEC67AFFE7C
title: 2.1 Formal state tuple and OS transition loop (normative)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Operationally, the runtime is modeled as a partially observable control loop.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 2.1 Formal state tuple and OS transition loop (normative)
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 2.1 Formal state tuple and OS transition loop (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1 Formal state tuple and OS transition loop (normative)
Operationally, the runtime is modeled as a partially observable control loop.
Let the kernel-visible state at step `t` be:

`X_t = (S_core_t, F_sparse_t, M_t, C_t, B_t, P_t, H_t)`

Where:
- `S_core_t`: dense working latent state.
- `F_sparse_t`: symbolic facts/flags.
- `M_t`: memory index references.
- `C_t`: active capability token set.
- `B_t`: budget ledger state.
- `P_t`: active policy hash/version.
- `H_t`: current receipt-chain head hash.

The system treats hidden world factors as unobserved variables and assumes
state transitions are Markov with respect to the kernel state:

`Pr(X_{t+1} | X_{0:t}, a_t) = Pr(X_{t+1} | X_t, a_t)`

Kernel loop per step:
1. Ingest observation/event and snapshot `X_t`.
2. Select action under policy + capability constraints.
3. Execute action/tool in a mediated transaction.
4. Commit delta + receipt on success, or rollback + failure receipt on reject.
5. Emit experience packet for P0/P1/P2 promotion gates.
