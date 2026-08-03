# Local Sovereign Knowledge Runtime
DOC-ID: LSKR-01
Status: Specified only / roadmap
Maturity: Draft

> Your machine, your weights, your data, your rules.

The Local Sovereign Knowledge Runtime is Goni's owner-controlled posture for
local inference, evidence, derived knowledge, contradiction, and governed
action. It separates freedom of local computation and expression from authority
to change durable or external state.

This document is non-normative. It adds no plane, service, schema table, public
API, or executable policy format. The canonical Knowledge, Context, Control,
and Execution contracts remain authoritative. If this framing conflicts with a
canonical specification, the specification wins.

## 1. Core posture

Goni defaults to open-weight, offline-capable, account-free inference. The
owner controls the model bundle, keys, prompts, policy, memory, and data.
Mandatory remote moderation, entitlement checks, hidden telemetry,
provider-controlled policy prompts, or viewpoint filters that the owner cannot
remove are hostile dependencies, not sovereignty features.

Promoted model bundles SHOULD be hash-pinned and reproducible. Engine and
checkpoint licenses remain separate evidence and SHOULD be recorded
independently under MODEL-REG-01 rather than inferred from one another.

Local text is local expression. Generating, analyzing, criticizing, imagining,
or drafting controversial, heterodox, offensive, or politically sensitive
material does not itself move money, contact another person, publish content,
delete evidence, or control a machine. Goni does not treat private model output
as an effectful tool action merely because somebody dislikes the content.

"No filter" is a stack property: no mandatory external moderation or
application-layer suppression sits between the principal and local output. It
is not a false promise that every checkpoint will comply perfectly. Learned
weights may still hedge, refuse, omit, or moralize; model selection, prompting,
templates, adaptation, and evaluation remain owner-controlled remedies.

A deployment is sovereign when inference and canonical authority remain local
and the owner controls its keys, weights, policy, and data. Staffing, backup,
liability, and operational maturity affect reliability, but they are not
ideological admission tests for sovereignty.

## 2. Expression and effects are different boundaries

The model may emit text. Text alone grants no authority. Authority begins when
the system attempts to commit a durable change or impose an effect outside the
ephemeral reasoning context.

| Operation | TOOL-01 capability required? | Reason |
| --- | --- | --- |
| Generate, analyze, criticize, summarize, or draft locally | No | Private computation and expression are not tool effects. |
| Propose a memory, file, message, payment, or device action | No, until commit | A proposal carries no execution authority. |
| Commit durable memory or ontology changes | Yes | Canonical owner-controlled state changes. |
| Write or delete files, send or publish content, move money, call a network service, or actuate a device | Yes | Durable or external effects cross the kernel boundary. |

The effect path is deliberately simple:

1. The agent generates or proposes.
2. Policy determines whether the requested effect is admissible.
3. The principal authorizes directly or through an explicitly delegated
   capability where the owner's policy requires it.
4. The kernel executes through a scoped tool.
5. Receipts make the material chain reconstructible.

Content classification does not create or remove tool authority. A harmless
sentence cannot authorize a payment, and an offensive sentence does not justify
silently expanding surveillance or blocking unrelated local computation.

## 3. Sources remain sources

The runtime preserves an explicit derivation chain:

| Stage | Meaning | Authority posture |
| --- | --- | --- |
| Original evidence | File, message, record, media object, observation, or other captured source | Preserved with identity, integrity, permission, and retention metadata. |
| Technical representation | OCR, transcript, parsed table, normalized record, or extracted structure | Fallible representation linked to the source and parser. |
| Machine enrichment | Embedding, entity link, classification, summary, inferred claim, or graph edge | Derived and untrusted until used under policy. |
| Human interpretation | A person's explanation, judgment, dissent, or contextual reading | Attributable interpretation, not a rewrite of the source. |
| Principal or delegate assertion | A scoped rule or position selected for operational use | Carries only the authority explicitly delegated to that actor and scope. |

A representation of a source MUST NOT silently replace the source. Derived
artifacts retain source refs, derivation stage, parser or model identity,
confidence, permissions, validity, and applicable receipts. If retention,
deletion, or redaction removes the original, derivatives must expose the
source as unavailable, redacted, or tombstoned rather than pretending to be
original evidence.

An owner assertion is not metaphysical truth. It is a scoped authority decision
for the owner's system. Model output cannot promote itself into owner-approved
fact, policy, memory, or operational authority.

## 4. Time and contradiction stay visible

Knowledge changes over time. The runtime distinguishes when an event occurred,
when it was recorded, when a claim or rule applied, and when it was superseded
or ceased to apply. As-of queries use the requested validity window rather than
silently applying the newest available text to the past.

Contradiction is first-class data, not a database defect. It can reveal factual
disagreement, historical change, competing scope or jurisdiction, a category
mistake, or divergence between formal policy and actual practice.

Query behavior follows the owner's purpose:

- **Descriptive:** return the material competing claims and their provenance.
- **Historical:** return what applied during the requested time window and
  identify later changes separately.
- **Operational:** apply a controlling rule only when the principal or an
  explicitly delegated role supplied one; preserve material dissent and
  conflicting practice as context.

When no controlling rule exists, the runtime surfaces the conflict or asks for
an authority decision. It does not manufacture consensus. Resolving a conflict
may change operational selection, but it does not erase the losing claim or
its provenance.

## 5. Minimum necessary ontology

Ontologies are tools, not reality. The runtime creates only enough structure to
support retrieval, permissions, temporal reasoning, and owner-directed action.
It does not claim that its categories exhaust the world.

- Category, identity, and relationship changes are versioned.
- Merge and split operations preserve source identities, dissent, rationale,
  receipts, and undo references.
- Inferred relationships remain distinguishable from explicit owner-set links.
- Formal rules and de facto practice remain separate even when they concern the
  same subject.
- No graph score, majority count, or model confidence silently becomes truth or
  authority.

The map is not the territory. Reversibility is the defense against a useful map
hardening into compulsory doctrine.

## 6. Receipts serve the owner

Receipts reconstruct meaningful boundary decisions, durable knowledge changes,
and external effects. They help the owner inspect what source, model, parser,
policy, capability, and approval affected a result.

Receipts are evidence, not truth. More logs do not automatically create
accountability. REC-01 therefore minimizes raw content, records bounded refs and
hashes, and distinguishes receipts from ordinary telemetry.

Nothing in this model authorizes ambient monitoring of workers, continuous
capture of private activity, or collection unrelated to a governed system
transition. Receipt access, retention, and disclosure remain owner-controlled
and permission-scoped.

## 7. Existing Goni mapping

The runtime is a cross-contract posture over the existing architecture:

| Existing responsibility | Local sovereign use |
| --- | --- |
| Knowledge Plane | Preserve sources, derived memory, provenance, validity, permissions, and conflict state. |
| Context Plane | Materialize bounded, source-linked evidence without granting it authority. |
| Control Plane | Apply principal-owned policy, delegation, budgets, scheduling, and stopping decisions. |
| Execution substrate | Run replaceable local models and capability-scoped tools. |
| Harness | Keep prompts, retrieval, routing, proposals, and commits inspectable and separable. |
| Receipts | Reconstruct meaningful knowledge changes and mediated effects for the owner. |

The primary contract mappings are:

- [Governed Memory Retrieval](/blueprint/30-specs/memory-retrieval.md) for source
  and derived-artifact separation.
- [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md) for
  temporal, contradiction, identity, and ontology behavior.
- [Receipts](/blueprint/30-specs/receipts.md) for purpose-limited
  reconstruction.
- [Tool Capability API](/blueprint/30-specs/tool-capability-api.md) for the hard
  expression/effects boundary.

## 8. Invariants

- Local expression does not require a tool capability.
- Model output cannot grant itself memory, policy, identity, approval, or tool
  authority.
- No durable or external effect bypasses capability, policy, and receipt
  mediation applicable to that effect.
- Original evidence and every derived stage remain distinguishable.
- AI-derived claims cannot impersonate source evidence or owner assertions.
- Contradictions remain retrievable until owner policy removes or redacts the
  underlying material.
- Conflict resolution changes operational selection, not historical evidence.
- Ontology changes remain versioned and reversible.
- Receipt collection remains scoped to governed system transitions, not
  ambient human activity.
- No third-party runtime, model provider, or moderation service owns canonical
  Goni authority.

## 9. Acceptance scenarios

- An offline open-weight bundle runs without a provider account, entitlement
  check, remote moderation call, hidden telemetry requirement, or mandatory
  non-overridable provider prompt. Its bundle hash and separate engine and
  checkpoint license evidence are inspectable.
- A model drafts controversial local text without invoking TOOL-01. Sending or
  publishing the draft requires the appropriate capability and receipt.
- A scanned document, its OCR, a model summary, and a human interpretation are
  individually addressable and trace to the same source without collapsing.
- A historical query returns the rule valid at the requested time and names a
  later amendment separately.
- Conflicting formal policy and observed practice are both retrieved. Only an
  explicit principal or delegated rule controls an effectful decision.
- An ontology merge can be undone without losing source identities or dissent.
- A receipt explains a durable memory change without storing the source text or
  an unrelated person's activity log.

## 10. Contract relationships

Upstream:

- [Sovereign Delegation OS Thesis](/blueprint/10-product/05-sovereign-delegation-os-thesis.md)
- [Axioms and Planes](/blueprint/software/50-data/10-axioms-and-planes.md)
- [Agentic Kernel Foundations](/blueprint/20-system/40-agentic-kernel-foundations.md)

Downstream:

- [Goni Whitepaper](/blueprint/docs/goni-whitepaper.md)
- [Governance Hub](/blueprint/docs/hubs/governance.md)

Adjacent:

- [Cognitive Exocortex Model](/blueprint/20-system/60-cognitive-exocortex-model.md)
- [Model Bundle Registry Governance](/blueprint/30-specs/model-registry.md)
- [Network Gate and Anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
