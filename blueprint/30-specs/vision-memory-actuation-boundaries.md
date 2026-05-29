---
id: BOUND-01
type: SPEC
status: specified_only
---
# BOUND-01 - Vision, Memory, and Actuation Boundaries
DOC-ID: BOUND-01
Status: Specified only / roadmap

Goni treats observation, context extraction, memory, and actuation as separate
governed capabilities. Desktop and browser agents often collapse these powers
into one session grant. Goni does not.

The Desktop Agent Firewall is the kernel-mediated boundary that prevents a
visible screen fact from becoming extracted context, durable memory, model
input, synthetic input, external egress, or a side effect without explicit
policy authority.

## 1. Scope

This spec applies to:

- local GUI agents using screenshot or framebuffer loops,
- cloud computer-use systems that request screenshots and return actions,
- browser automation and browser-isolated tools,
- embodied robot sensor and actuator systems,
- OS memory layers that record screen, audio, accessibility, or OCR history,
- desktop integrations that read app context through accessibility APIs,
- permissioned-view assistants that observe and annotate only,
- future OS-native vision, capture, and desktop-agent APIs.

The same boundary model applies whether inference is local, remote, hybrid, or
absent.

## 2. Capability powers

Goni separates four powers:

| Power | Meaning | Boundary question |
| --- | --- | --- |
| `observation` | The agent may see a screen, window, tab, app, frame, stream, accessibility tree, or event. | What can be observed, for how long, and under which user/session scope? |
| `extraction` | The agent may parse, OCR, summarize, classify, embed, transform, or send observed content to a model. | What can be derived from observation, and may it leave the local node? |
| `memory` | The agent may store, index, consolidate, retrieve, sync, or reuse extracted facts or artifacts. | What memory class may be written or read, and with what expiry/review policy? |
| `actuation` | The agent may click, type, scroll, submit, delete, move, publish, run commands, or call tools. | What side effects are allowed, under which tool token, sandbox, corridor, and receipt? |

Granting one power MUST NOT imply any other power.

## 3. Desktop Agent Firewall flow

Every desktop, browser, or vision-mediated action follows this logical flow:

```text
desktop/window/app/event
-> observation permission
-> extraction permission
-> memory permission
-> tool/action permission
-> autonomy corridor
-> receipt
```

The flow may stop at any stage. A permissioned-view assistant may stop after
observation and annotation. A memory layer may stop after memory write. An
agentic operator may continue to act only when actuation is separately granted.

Denied boundary transitions MUST fail closed and remain auditable.

## 4. Boundary objects

Every mediated transition SHOULD preserve compact refs for the following
objects. Raw private content is not stored in Control-plane fields by default.

- `observation_scope`: permitted app, window, tab, screen, monitor, event
  stream, accessibility tree, frame rate class, time window, and revocation
  rule.
- `extraction_profile`: permitted extraction modes such as OCR, accessibility
  parse, layout parse, summarization, embedding, remote model submission,
  redaction profile, and output shape.
- `memory_grant`: permitted memory class, source refs, retention/expiry,
  indexing rule, sync posture, review status, and tombstone behavior.
- `actuation_grant`: permitted tool IDs, synthetic input classes, filesystem or
  network scopes, irreversible-action rules, idempotency rule, and allowed side
  effects.
- `sandbox_profile`: required process, container, browser, microVM, or OS
  isolation boundary for the current action class.
- `approval_requirement`: no approval, queued review, soft gate, hard gate,
  two-phase commit, or explicit human confirmation.
- `receipt_requirement`: receipt tier and required basis fields for the
  boundary transition.
- `rollback_or_repair_ref`: snapshot, undo strategy, compensation path, repair
  workflow, or explicit statement that no rollback exists.

## 5. Policy mediation

The Desktop Agent Firewall is a Policy Decision Point and Policy Enforcement
Point for boundary transitions. Policy MUST decide at least:

- default observation scope,
- allowed extraction modes,
- memory classes allowed per task class,
- actuation classes allowed per autonomy corridor,
- remote extraction and egress rules,
- sandbox profile by action class,
- approval requirements,
- receipt tier and retention posture,
- default denial for observation-to-actuation escalation.

If policy is missing or cannot be loaded, the boundary transition is denied.

## 6. Synthetic input and tools

Synthetic input is a tool syscall. Mouse, keyboard, scroll, drag, browser DOM
mutation, shell command, filesystem write, external API mutation, and publish
actions are actuation events. They require:

- a Work Order and Done Contract when delegated,
- a capability token with an actuation grant,
- a sandbox profile that meets or exceeds the action class,
- autonomy corridor evaluation,
- approval evidence when required,
- idempotency and rollback/repair metadata where possible,
- a canonical Goni receipt.

External assistant logs, browser extension logs, operating-system event logs, or
third-party computer-use traces do not replace Goni receipts.

## 7. Remote extraction and egress

Sending observed or extracted screen context to a remote model, service,
connector, or API is egress. It requires both:

- an extraction grant that permits remote submission for the observed content
  class, and
- a Network Gate capability that permits the destination, purpose, payload
  classification, budget, and redaction mode.

If either grant is absent, remote extraction is denied.

## 8. Memory separation

Observation and extraction do not imply memory. Screen frames, OCR text,
accessibility trees, summaries, embeddings, layout facts, screenshots, and audio
transcripts may enter durable memory only through a memory grant.

Memory writes MUST preserve source refs, permission scope, memory class,
retention policy, parser/extraction basis, and receipt refs. Memory layers that
record continuously are still governed memory writers; being passive does not
make storage authority ambient.

## 9. Threat model hooks

Desktop and browser agents add threat surfaces that ordinary chat agents do
not:

- screen prompt injection that tries to convert observed text into authority,
- poisoned screen content that tries to escalate from extraction to actuation,
- memory poisoning through OCR/accessibility/history stores,
- local unsandboxed execution through synthetic input or shell tools,
- privacy leakage through remote extraction of screen context,
- over-retention of private screenshots, OCR, or accessibility text.

The firewall mitigates these by separating powers, requiring policy mediation,
and forcing receipts for allowed and denied transitions.

## 10. Evaluation dimensions

Goni evaluates Desktop Agent Firewall behavior as separation of powers, not as
vendor comparison. Evaluation SHOULD measure:

- latency and throughput of mediated capture/extraction/action paths,
- offline capability,
- privacy leakage rate,
- prompt-injection recovery,
- GPU/VRAM and memory pressure,
- rollback or repair success,
- receipt completeness,
- denied-transition fail-closed behavior.

These are measurement dimensions. This spec does not assert benchmark values.

## 11. Invariants

- Observation does not imply extraction.
- Extraction does not imply memory.
- Memory does not imply actuation.
- Actuation does not imply authority.
- Synthetic input requires a capability token and tool mediation.
- Remote extraction routes through the Network Gate.
- Continuous memory capture requires a memory grant.
- Every allowed or denied boundary transition emits a receipt or receipt-linked
  audit record.
- Raw private screenshots, full OCR text, accessibility dumps, audio
  transcripts, and unbounded prompts are not stored in receipts by default.
- If required boundary policy, sandbox, approval, or receipt support is
  unavailable, execution fails closed.

## 12. Related specs

- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Visual Intelligence Plane](/blueprint/30-specs/visual-intelligence-plane.md)
- [Embodied Robot Control Plane](/blueprint/30-specs/embodied-robot-control-plane.md)
- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## 13. Upstream

- [Delegation doctrine](/blueprint/10-product/15-delegation-doctrine.md)
- [Agentic kernel foundations](/blueprint/20-system/40-agentic-kernel-foundations.md)

## 14. Downstream

- [Receipt schema](/blueprint/schemas/receipts/receipt.schema.json)
- [Policy schema](/blueprint/schemas/policy/policy.schema.json)
- [Desktop Agent Firewall eval lane](/blueprint/50-evidence/eval/EVID-DESK-01-desktop-agent-firewall.md)

## 15. Adjacent

- [Governance hub](/blueprint/docs/hubs/governance.md)
- [Contracts hub](/blueprint/docs/hubs/contracts.md)
- [Planes hub](/blueprint/docs/hubs/planes.md)

## Conformance tests

- observe-only assistants cannot write memory or act.
- memory-only layers cannot actuate.
- extraction-to-remote-model attempts require egress permission.
- local screen prompt injection cannot escalate to shell, synthetic input, or
  browser mutation without an actuation grant.
- screen capture and accessibility extraction require separate grants.
- synthetic input requires a capability token and sandbox profile.
- denied boundary transitions fail closed and are auditable.
- actuation attempts emit receipts with Work Order, policy hash, sandbox
  profile, boundary basis, and rollback/repair ref where available.
- receipts omit raw private screen content by default.
