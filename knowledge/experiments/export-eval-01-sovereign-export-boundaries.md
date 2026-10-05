---
id: EXPORT-EVAL-01
title: Sovereign export boundary evaluation
type: experiment
status: draft
implementation_state: specified_only
proposition: An adversarial evaluation should test whether each declared compute-export
  profile confines recipient views and state changes to current principal authorization.
domains:
- evaluation
- privacy
aliases: []
relations:
- type: tests
  target: D-025
- type: tests
  target: NET-COMP-01
- type: tests
  target: COOP-PRIVACY-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Observer models, leakage measures, and adversarial fixtures must be selected
  and pinned before execution.
legacy: []
---

# Sovereign export boundary evaluation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

Compare local-only, enrolled owner-mesh, and independent-provider profiles under
identical authorized tasks. Name each observer and its allowed view before the
run. Exercise forbidden private fields, low-entropy commitments, output leakage,
receipt/telemetry disclosure, repeated-query composition, unenrolled peers,
recipient substitution, expired/revoked authority, and stale returned state.

Measure allowed and disallowed disclosures, violations of declared observer
views, cumulative leakage where a defined metric exists, and unauthorized state
acceptance. Record policy/runtime/full implementation revisions and redacted
receipts sufficient to reproduce the boundary check. The conformance target is
zero unauthorized disclosures or commits within the declared test boundary;
unmeasured side channels remain explicit limitations. Planned tests establish
neither an implementation nor a general confidentiality guarantee.
