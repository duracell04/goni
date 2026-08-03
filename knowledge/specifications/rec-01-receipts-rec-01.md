---
id: REC-01
title: Receipts (REC-01)
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: REC-01 Status: Specified only / roadmap Receipts are immutable records of mediated actions.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Receipts (REC-01)
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Receipts (REC-01)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Receipts (REC-01)
DOC-ID: REC-01

Status: Specified only / roadmap

Receipts are immutable records of mediated actions. They must be minimal by
default and verifiable via hash chaining.

Receipts are a Goni-kernel primitive. Third-party gateways, tool hosts, or
assistant frameworks may emit their own logs, but those logs do not substitute
for a canonical Goni receipt.
