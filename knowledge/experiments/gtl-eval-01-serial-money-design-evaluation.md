---
id: GTL-EVAL-01
title: "Evaluation of serial-identified digital money and recovery"
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Compare serial-identified lots with conventional fungible ledgers and private variants for conservation, double-spend resistance, performance, privacy and disputed-ownership outcomes."
domains:
- network
- research
- security
- system
aliases: []
relations:
- type: tests
  target: GTL-01
- type: tests
  target: GTL-OBJECTION-01
sources: []
artifacts: []
uncertainty: "Research plan only; no executable tests, measured results, legal review, production protocol or financial implementation exist."
legacy: []
---

# GTL-EVAL-01 - Test serial-identified settlement designs

## Research question

Do persistent unit identities and lineage provide measurable accountability or loss-response value sufficient to outweigh additional privacy, fragmentation, verification, liquidity, governance and user-experience costs?

This is an experiment **plan only**, not test results, implementation evidence or regulatory validation.

## Comparable alternatives

Compare (A) ordinary account or UTXO payment rails, (B) public serial-identified lots and (C) a feasible shielded or client-validated design. Preserve identical issuance volume, synthetic payment patterns, threat model, and finality assumptions where feasible; record any incomparable features.

## Synthetic scenarios

1. Issue 100 distinct smallest units, transfer 35, retain 65, then merge, split, and transfer noncontiguous lots.
2. Test micropayment volume, high fragmentation, long ownership chains and mixed inputs.
3. Attempt unauthorized minting, duplicate serial creation, double spending, replay, unauthorized transfer and false recovery.
4. Simulate a stolen signing key, an honest intermediary and merchant, a false accusation, freeze, appeal, recovery, and compensation.
5. Test offline preparation and later settlement, network partitions, reorganizations and delayed confirmation.
6. Model Goni compute escrow with accepted evidence, failed evidence, disputed quality, timeouts and linked independent receipts.
7. Measure what an observer can infer about user identity and transaction relationships under each privacy design.

## Measurements and pass criteria

Measure supply and serial conservation, deterministic state reconstruction, unauthorized action rejection, finality correctness, proof size, verifier cost, transaction bytes, state growth, compression, fragmentation, settlement latency, fee impact, privacy linkability, dispute errors, honest-party losses and human intervention cost.

Safety failures such as duplicate spend, supply violation or unauthorized reassignment are blocking defects regardless of throughput. Claims of recovery effectiveness require legal/process as well as cryptographic evaluation. Define practical performance and privacy thresholds before running benchmarks.

Prototype implementations and pinned measurements belong in an experimental workspace; results are to be incorporated as separately supported evidence nodes.
