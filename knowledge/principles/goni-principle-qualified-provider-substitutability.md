---
id: GONI-PRINCIPLE-QUALIFIED-PROVIDER-SUBSTITUTABILITY
title: Standardized contracts enable qualified provider substitutability
type: principle
status: draft
implementation_state: specified_only
proposition: A common computation contract can make providers substitutable only within service classes that satisfy the same semantic, capability, privacy, evidence, deadline, locality, and budget constraints; computation is not treated as globally fungible.
domains:
- compute
- scheduler
- market
aliases:
- provider-substitutability
relations:
- type: depends_on
  target: COMP-01
- type: depends_on
  target: EXEC-01
- type: refines
  target: GONI-DECISION-0532C27DA3CC
sources:
- SRC-RAY-SCHEDULING-LOCALITY
artifacts: []
uncertainty: Provider discovery, pricing, reputation, and market-clearing mechanisms remain outside the current GONI MVP.
legacy: []
---

# Standardized contracts enable qualified provider substitutability

A common interface standardizes procurement and acceptance. It does not make every computation or every provider interchangeable.

Two providers are substitutable for an execution contract only if each can satisfy the relevant:

- computation semantics;
- required capability set;
- memory and data-locality constraints;
- privacy policy;
- evidence policy;
- deadline or latency objective;
- trust or sovereignty boundary;
- budget or accepted price.

The useful economic abstraction is therefore a service class of qualified providers rather than a single universal unit of fungible compute.

This principle keeps the GONI scheduler free to choose among local engines, remote providers, and future specialized hardware without erasing differences that materially affect correctness or total cost.
