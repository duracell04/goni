---
id: COOP-PRIVACY-01
title: Compute exports can reveal information through composition
type: objection
status: draft
implementation_state: specified_only
proposition: Payload minimization alone leaves confidentiality unresolved when outputs,
  commitments, timing, receipts, or repeated requests permit inference about private
  state.
domains:
- privacy
- network
aliases: []
relations:
- type: objects_to
  target: NET-COMP-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: This is a design objection; the exploitability of each channel depends
  on a selected implementation and adversary.
legacy: []
---

# Compute exports can reveal information through composition

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

This objection challenges a general privacy guarantee based only on a sanitized
job payload. A worker may observe its input; a verifier may observe the public
statement; an operator may observe timing and size; repeated individually
permitted jobs may reveal more together. Shared evidence may also disclose a
personal context indirectly.

NET-COMP-01 responds by requiring explicit observer scopes and composition
assumptions. Whether a particular profile meets its disclosure bound remains an
experiment question. Local execution is a permitted alternative for a job whose
remote confidentiality requirements remain unresolved.
