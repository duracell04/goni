---
id: REC-01
title: Receipts (REC-01)
type: specification
status: draft
implementation_state: specified_only
proposition: Receipts are immutable, minimal records of mediated actions and their governance evidence; when completion certification is material, the receipt chain must distinguish action outcome from observation, verification, and DoneContract completion state.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: The completion-evidence extension is specified-only and does not introduce a new shipping receipt schema. Concrete field layouts and retention policies require implementation and validation.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Receipts (REC-01)
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Receipts (REC-01)

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

# Receipts (REC-01)
DOC-ID: REC-01

Status: Specified only / roadmap

Receipts are immutable records of mediated actions. They must be minimal by
default and verifiable via hash chaining.

Receipts are a Goni-kernel primitive. Third-party gateways, tool hosts, or
assistant frameworks may emit their own logs, but those logs do not substitute
for a canonical Goni receipt.

## Completion evidence

REC-01 remains one receipt primitive. COMPLETE-01 does not create separate
ActionReceipt, ObservationReceipt, VerificationReceipt, or AcceptanceReceipt
objects.

When completion certification is material, the logical receipt chain SHOULD
preserve, directly or by stable reference:

- **action outcome:** what operation was attempted and what the action interface
  reported;
- **observation refs:** the post-action evidence collected about resulting
  state;
- **verification results:** which DoneContract predicates were tested, by which
  observation path, and with what result;
- **completion basis:** the `done_contract_hash` or equivalent stable reference
  and the resulting completion state;
- **epistemic state:** whether material claims are observed, inferred, verified,
  or certified where EPISTATE-01 requires the distinction;
- **verification limits:** missing evidence, unavailable observers, or
  independence limits that prevent stronger certification.

An action may therefore have:

```text
action_outcome = success
completion_state = verification_incomplete
```

without contradiction.

Provider logs are evidence about the provider's reported state. They do not by
themselves establish higher-level DoneContract satisfaction unless the contract
explicitly defines that provider state as its terminal predicate.

Receipts remain evidence, not truth. Completion certification is a kernel
judgment over the DoneContract and qualifying evidence recorded or referenced
by the receipt chain.
