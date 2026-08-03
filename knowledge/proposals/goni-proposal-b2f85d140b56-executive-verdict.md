---
id: GONI-PROPOSAL-B2F85D140B56
title: Executive verdict
type: proposal
status: draft
implementation_state: specified_only
proposition: A highly personalized local secretary is practical because the owner can control the inference runtime, prompts, memory, tools, data retention, and network boundary.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: Executive verdict
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# Executive verdict

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Executive verdict

A highly personalized local secretary is practical because the owner can
control the inference runtime, prompts, memory, tools, data retention, and
network boundary. Downloadable model weights remove many controls imposed by a
hosted product: remote system instructions, provider-side input/output
classifiers, account enforcement, service availability, and provider-owned
tool restrictions no longer have to sit on the live inference path.

Goni's preferred posture is owner-sovereign and open-weight: no mandatory
provider account, remote policy prompt, server-side classifier, silent
telemetry, revocable API entitlement, or vendor-controlled behavior update is
allowed on the default inference path. The owner chooses the checkpoint, chat
template, system prompt, decoding settings, memory, and optional content
controls. No mandatory viewpoint or content filter should sit between the
owner and locally generated text.

This is an architectural allocation of authority, not a promise of polite or
approved opinions. A sovereign assistant should answer owner-requested text
queries candidly, including controversial, heterodox, or offensive subjects,
without adding a third party's moral or political policy layer. Goni should
govern consequential actions at the capability boundary instead of treating
the model's speech as the action: generating text is not the same operation as
sending a message, spending money, deleting data, or controlling a machine.

That does **not** create absolute obedience or remove every constraint:

- model behavior remains learned and probabilistic;
- pretrained and instruction-tuned weights can retain refusal, bias, and
  uncertainty patterns;
- model and software licenses still apply;
- local integrations can still send data to cloud services or telemetry
  endpoints;
- a system prompt cannot guarantee a permanent personality or perfect
  instruction adherence; and
- tool execution creates ordinary computer-security and accidental-action
  risks regardless of how the model was trained.

There is no single refusal switch to disable. Refusal and helpfulness behavior
is distributed across model weights, prompting, chat templates, decoding,
optional guard models, and the surrounding application. Local ownership makes
those layers configurable; it does not make the model deterministic or
infallible. "No filter" can therefore be a stack policy: no mandatory external
moderation or application-layer suppression--but cannot honestly guarantee
that learned weights will never hedge, refuse, omit, or moralize.

For a secretary, a capable instruction- and tool-tuned model is usually a
better starting point than a raw base model. A base model primarily predicts
continuations and may require extensive prompting or tuning to converse,
follow tool schemas, stop correctly, and maintain a role. Goni should select a
model through behavior and systems evaluation rather than assuming that
"base" means more loyal or useful.
