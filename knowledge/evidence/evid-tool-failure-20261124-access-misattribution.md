---
id: EVID-TOOL-FAILURE-20261124
title: "Case report: execution-path failure misattributed to user browser"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "A user-supplied screenshot and subsequent assistant self-assessment describe conflated execution boundaries, unchanged failed retries, unverified promises to send, and unnecessary user troubleshooting during an unfinished delegated messaging task."
domains:
- agent
- evaluation
- system
aliases: []
relations:
- type: supports
  target: SPEC-TOOLFAIL-01
  note: "Motivates the control but does not demonstrate its effectiveness."
sources:
- SRC-CASE20261124-TOOL-FAILURE
artifacts: []
uncertainty: "Based on the user's screenshot and a later assistant self-report, not authenticated executor logs, shell traces, browser state, or destination records. The agent reported checking a shell where CODEX_CLI_PATH was unset and Codex CLI ran, but whether the failing managed executor inherited that environment remains unknown. Earlier browser access and final external effects were not independently verified. No private screenshot or message contents are published."
legacy: []
---

# Case report: execution-path failure misattributed to user browser

## Reported observation

A user-supplied screenshot of an assistant conversation shows a delegated browser-mediated messaging task interrupted by a reported execution error:

```text
Managed execution requires a usable CODEX_CLI_PATH
```

The assistant first implied the user needed to restore browser access. In the same displayed exchange, it subsequently acknowledged that an earlier browser/computer-control path and the failing managed-execution path were distinct, and that the proposed browser-repair instruction was not supported by the observed error. The displayed assistant also reported that the message **had not been sent**.

## Subsequent assistant self-assessment (reported, not independently checked)

The user subsequently supplied the assistant's more specific post-mortem. The assistant reported the following checks:

| Reported check | Reported outcome | Evidentiary boundary |
| --- | --- | --- |
| `CODEX_CLI_PATH` in the shell available to the assistant | Unset | Establishes only that shell's variable state; the failing managed executor may use another environment |
| Codex binary on the user's Mac | Present; command returned `codex-cli 0.169.0` | Does not establish that the managed execution tool is configured to use that binary |
| Browser-control tool exposed to that conversation | None found | A current-session tool-availability observation, not evidence the browser is uninstalled |
| Earlier browser access | Reportedly succeeded, with user-provided screenshot | Earlier access does not prove current-session tool availability |

The assistant also acknowledged repeatedly invoking an unchanged failed route, promising to send and verify before establishing an executable route, and suggesting browser remediation without sufficient evidence. At the task-quality layer, it acknowledged drafting an outbound reply without first obtaining the counterparty's exact latest message, then proposing multiple high-friction diagnostic questions and a vague discussion. Its final status was that a particular outbound message had **not been sent by that session**. These are self-reported findings; the underlying tool traces, destination account, and local shell were not independently inspected for this case report.

Crucially, `CODEX_CLI_PATH` being unset in the inspected shell is **consistent with** an execution-configuration hypothesis, but does not prove the failed tool uses that shell or establish its root cause.

## Interpretation

The demonstrated failure is **diagnostic boundary confusion**: an error surfaced at one tool/execution boundary was treated as evidence about a different boundary (the user's browser environment). That unsupported causal leap risked transferring a platform or assistant capability problem to the user and distracted from the original delegated outcome. The further report identifies distinct process failures: **retrying without a changed hypothesis**, **claiming execution capability before preflight**, and **drafting a consequential communication from insufficient source context**.

This record does **not** establish the infrastructure root cause, the current state of the browser or earlier sessions, or the external messaging service's independently confirmed state. In particular, an assistant's statement that no message was sent is a reported status, not proof from the destination system.

## Generalizable failure mode

```text
Observed tool error → repeated unchanged route
→ unverified cross-boundary diagnosis and premature delivery promise
→ unsupported user remediation / insufficiently grounded draft
→ delegated outcome remains unresolved
```

A later apology or corrected explanation can improve transparency but does not itself restore task completion. The architectural lesson is to preserve the distinction among **what failed**, **where it failed**, **what actually happened**, and **what the user must do, if anything**.

## Privacy and evidence boundary

This is a minimal, redacted case report grounded in a user-shared screenshot and later assistant self-assessment. Neither original transcript nor screenshot is embedded in the public blueprint. No recipient identity, message contents, browser profile, session token, or private account details are retained here.

The proposed prevention contract is `SPEC-TOOLFAIL-01`. Its effectiveness remains to be evaluated.
