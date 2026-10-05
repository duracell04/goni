---
id: DIST-01
title: Validity, availability, and canonicality are distinct distributed-state properties
type: principle
status: draft
implementation_state: specified_only
proposition: In distributed execution, evidence that a state transition is valid does not by itself make the resulting data available or the transition canonical; GONI should model validity, availability, and canonicality as separate properties whenever authority spans multiple nodes or competing histories.
domains:
- distributed
- compute
- kernel
aliases:
- validity-availability-canonicality
relations:
- type: depends_on
  target: EVID-01
- type: refines
  target: GONI-IMAP-ED288DB4EAF5
sources:
- SRC-ETHEREUM-CONSENSUS-VALIDATOR
- SRC-ETHEREUM-DATA-AVAILABILITY
artifacts: []
uncertainty: This principle is future-facing for distributed GONI deployments; the local-first MVP can rely on a single kernel authority and does not require blockchain consensus.
legacy: []
---

# Validity, availability, and canonicality are distinct

For a local single-authority GONI node, the kernel can ordinarily decide which state is authoritative.

For distributed execution, three questions must remain separate.

## Validity

Does the proposed transition satisfy the accepted computation and policy relation?

A proof, replay, or other evidence mechanism may establish this under its declared assumptions.

## Availability

Can authorized participants obtain the output, state delta, or other data required to use, reconstruct, audit, or continue from the transition?

A commitment to data does not itself deliver the data.

## Canonicality

When multiple individually valid transitions or histories compete, which one is authoritative for subsequent work?

Canonicality is a coordination property. It may be supplied by a single sovereign kernel, a replicated protocol, an external ledger, a consensus mechanism, or another explicit authority model.

The distinction can be summarized as:

[
valid \neq available \neq canonical
]

GONI should introduce consensus or blockchain machinery only when the deployment actually requires distributed agreement over canonical state.
