---
id: GTL-OBJECTION-01
title: "Serial-level traceability threatens privacy, fungibility and innocent holders"
type: objection
status: draft
implementation_state: not_applicable
proposition: "Public monetary serial histories risk undermining economic fungibility and privacy, while administrative recovery may transfer losses to innocent downstream holders without a justified legal process."
domains:
- network
- policy
- security
aliases: []
relations:
- type: objects_to
  target: GTL-01
sources: []
artifacts: []
uncertainty: "Alternative privacy mechanisms, legal property doctrines and dispute procedures have not been comparatively evaluated."
legacy: []
---

# GTL-OBJECTION-01 - Serial histories can impair money's usefulness

## Objection

Even where two units have the same nominal denomination, publicly linkable serial identities and transfer histories can make them economically unequal. Merchants, custodians and exchanges may discount or refuse units based on perceived "taint", past transfers or contested provenance. Persistent public lineage can reveal financial relationships, facilitate censorship and increase surveillance.

Tracking allegedly stolen units is different from proving theft, preventing their transfer or lawfully recovering them. Administrative recovery may transfer losses to an innocent downstream recipient. Dispute authorities introduce governance capture, false-claim, jurisdictional and private-key compromise risks. Repeated transfers can also fragment serial ranges and increase storage or proof costs.

## Possible responses for testing

Privacy-preserving or client-side validation, bounded selective disclosure, carefully defined authority and appeal rights, and separate bearer-like versus recoverable asset policies may mitigate particular risks. Each adds independent engineering and governance costs. The system cannot simultaneously assume globally observable transfer history and universal undisclosed private payments.

## Resolution boundary

Compare the economic and privacy consequences against ordinary fungible money and evaluate disputed claims involving honest recipients before proposing a live currency or confiscation authority. GTL-EVAL-01 specifies a possible comparative evaluation. This objection remains open.
