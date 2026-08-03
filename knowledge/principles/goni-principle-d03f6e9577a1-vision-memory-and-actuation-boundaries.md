---
id: GONI-PRINCIPLE-D03F6E9577A1
title: Vision, Memory, and Actuation Boundaries
type: principle
status: draft
implementation_state: specified_only
proposition: 'Goni separates four capabilities that desktop and browser agents often collapse: Observation: what the agent may see.'
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/15-delegation-doctrine.md
  heading: Vision, Memory, and Actuation Boundaries
  revision: 8eca78baa6e9fe022fe69ba0f6249f53ea9fa79b
---

# Vision, Memory, and Actuation Boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Vision, Memory, and Actuation Boundaries

Goni separates four capabilities that desktop and browser agents often collapse:

1. Observation: what the agent may see.
2. Context extraction: what the agent may parse, summarize, OCR, classify, or
   send to a model.
3. Memory: what the agent may store, index, consolidate, and reuse later.
4. Actuation: what the agent may click, type, submit, delete, move, publish, or
   send.

These powers must never be granted as one blanket permission.

Every desktop, browser, or vision-mediated agent action must pass through a
Goni-controlled boundary:

- app, window, tab, screen, or event scope,
- memory class,
- tool capability,
- sandbox profile,
- autonomy corridor,
- approval requirement,
- receipt,
- rollback or repair path where possible.

Permissioned-view systems may observe without acting. OS memory layers may
store without acting. Agentic desktop operators may act, but only through Goni
capability tokens, sandboxes, autonomy corridors, and receipts.

The Goni principle is:

> Seeing is not remembering.
> Remembering is not acting.
> Acting is not authority.
> Authority belongs to the principal and is mediated by the Goni kernel.

No agent may turn seeing into acting without a Work Order, permission boundary,
capability token, approval corridor, and receipt.
