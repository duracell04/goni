---
id: DOCTRINE-DELEG-01
type: DOCTRINE
status: specified_only
---
# Delegation Doctrine
DOC-ID: DOCTRINE-DELEG-01
Status: Specified only / roadmap

This document states product-level doctrine. Normative behavior lives in
`blueprint/30-specs/`.

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

## Desktop Agent Firewall

The Desktop Agent Firewall is a kernel concept, not a preference panel. It
mediates whether an observed desktop, browser, app, or event can become
extracted context, durable memory, model input, synthetic input, external
egress, or a user-visible side effect.

Canonical flow:

```text
desktop/window/app/event
-> observation permission
-> extraction permission
-> memory permission
-> tool/action permission
-> autonomy corridor
-> receipt
```

The same model applies whether the upstream system is read-only, memory-only,
or fully agentic. The decisive boundary is not whether an assistant is local or
cloud-hosted; it is whether each power is mediated separately by the Goni
kernel.
