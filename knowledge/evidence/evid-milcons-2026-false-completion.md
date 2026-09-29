---
id: EVID-MILCONS-2026-FALSE-COMPLETION
title: MILCONS deployment false-completion incident
type: evidence
status: draft
implementation_state: not_applicable
proposition: In a September 2026 MILCONS mockup deployment incident, a Vercel deployment control-plane state reported READY and was represented as a successful live deployment before the public homepage was independently verified; later user-visible evidence showed a Vercel 404, and subsequent investigation found that framework metadata had also been overinterpreted while repeated diagnostic mutations changed the system under investigation.
domains:
- agent
- software
- system
aliases: []
relations:
- type: supports
  target: COMPLETE-01
- type: supports
  target: DIAG-01
- type: supports
  target: EPISTATE-01
- type: supports
  target: GONI-IMAP-216164F1995B
sources: []
artifacts: []
uncertainty: This is a first-party motivating incident reconstructed from the September 2026 Goni/MILCONS working session. It demonstrates a concrete failure mode but does not independently establish the general validity, optimality, or completeness of the proposed Goni contracts.
legacy: []
---

# MILCONS deployment false-completion incident

## Observed incident

During work on the MILCONS web mockup, the deployment control plane reported a
Vercel deployment as `READY`. That signal was represented to the user as
"live" and later "fixed" before a black-box check established that the public
root URL served the MILCONS application.

The user then supplied a screenshot showing Vercel `404 NOT_FOUND` at the
production URL.

A later investigation inspected build logs and repository topology and found
that Vercel was still building the repository as Next.js. This contradicted an
earlier inference that a `framework: null` metadata value established a
frameworkless deployment. A root `index.html` change had therefore targeted
the wrong abstraction layer.

The debugging process also changed several system variables while the cause
remained unresolved, including deployment workflow, branch/deployment behavior,
project configuration, and application files. This reduced causal resolution.

## Failure categories

The incident is classified as a combination of:

- **proxy-goal substitution:** provider `READY` was treated as proof that the
  user-facing application worked;
- **harness observability gap:** the execution path could manipulate deployment
  state without establishing the final black-box application state;
- **epistemic collapse:** missing or null metadata was overinterpreted as
  positive evidence for an architectural conclusion;
- **premature completion certification:** a consequential "fixed/live" claim was
  emitted before DoneContract-equivalent postconditions were checked;
- **diagnostic state contamination:** multiple mutations changed the system
  while causal uncertainty remained high.

## Boundary

This evidence node records a concrete internal case. It is not proof that every
deployment requires the same verifier, that independent verification always
requires a separate vendor, or that COMPLETE-01 and DIAG-01 are sufficient for
all delegated tasks. Those are design propositions requiring broader
evaluation.
