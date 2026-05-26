# GoniOS Research Neighbor Map
DOC-ID: REF-GONIOS-NEIGHBOR-01
Status: Specified only / roadmap

Purpose: map primary-source external projects onto GoniOS as a
resource-bounded local cognition system. This is a focused synthesis, not a
dependency shortlist and not a broad ecosystem inventory.

Source rule: use official repositories, official sites/docs, arXiv/OpenReview
or conference pages, standards drafts, or official benchmark sites only. Social
posts, third-party summaries, search snippets, and SEO pages are not evidence
for this document.

## Reading frame

GoniOS is framed here as:

```text
private event streams -> bounded memory/state -> selected context
-> policy-mediated action -> receipt -> memory update
```

The comparison layers are:

- memory OS and state,
- local-first runtime and agent OS substrate,
- receipts, governance, and identity,
- evaluation and benchmark environments,
- runtime verification and assurance.

Confidence labels:

- `primary-source verified`: official source found and used directly.
- `omitted pending primary source`: not used as a technical anchor here.

## Memory OS and state

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| MemGPT / Letta | https://arxiv.org/abs/2310.08560 | Tabulation, memoization, rolling state, virtual context | Treat context as managed memory with explicit movement between short and long-term stores. | Do not equate chat history management with Goni's full authority, receipt, and policy system. | `primary-source verified` |
| Letta | https://github.com/letta-ai/letta | Agent memory runtime | Study practical APIs for long-running agents with external memory. | Do not let an agent framework own canonical Goni memory or policy state. | `primary-source verified` |
| MemOS | https://arxiv.org/abs/2505.22101 | Memory as OS-level resource | Study memory lifecycle, memory scheduling, and memory service boundaries for LLM systems. | Do not claim Goni implements a memory OS unless the memory object contract and runtime wiring exist. | `primary-source verified` |
| MemPalace | https://github.com/mempalace/mempalace | Hierarchical local memory and retrieval | Compare memory topology, hierarchy, and retrieval organization against Vault and Context Gravity Graph designs. | Do not import benchmark claims without reproducing them in Goni Lab. | `primary-source verified` |
| Basic Memory | https://github.com/basicmachines-co/basic-memory | Local-first writable memory, graph-like notes | Study Markdown-backed local memory and MCP-facing workflows as a simple Vault substrate pattern. | Do not treat Markdown notes as sufficient for receipts, expiry, invalidation, or policy mediation. | `primary-source verified` |

## Local-first runtime and agent OS substrate

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| OpenJarvis | https://github.com/open-jarvis/OpenJarvis | Local-first runtime and resource evaluation | Use FLOPs, energy, latency, and cost as first-class local-agent metrics. | Do not collapse resource evaluation into a generic assistant UX claim. | `primary-source verified` |
| AIOS | https://arxiv.org/abs/2403.16971 | Agent OS scheduling, context, memory, storage, access control | Compare Goni's scheduler, context management, and tool governance against an agent-OS architecture. | Do not inherit "OS" language without Goni-specific mediation, receipts, and local-first boundaries. | `primary-source verified` |
| AIOS repository | https://github.com/agiresearch/AIOS | Agent SDK/runtime reference | Inspect implementation boundaries for agent scheduling and runtime services. | Do not treat the SDK as Goni's kernel or trusted computing base. | `primary-source verified` |
| OS-Copilot / FRIDAY | https://arxiv.org/abs/2402.07456 | Computer-use action execution and self-improvement loop | Study OS-level task execution across web, terminal, files, and applications. | Do not import autonomous action patterns without Goni capability tokens and receipts. | `primary-source verified` |
| OS-Copilot repository | https://github.com/OS-Copilot/OS-Copilot | Generalist computer agent implementation | Compare action surfaces and tool APIs. | Do not use it as evidence for safe delegated authority. | `primary-source verified` |

## Receipts, governance, and identity

| Project / standard | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| AEOESS / Agent Passport System | https://datatracker.ietf.org/doc/draft-pidlisnyi-aps/ | Agent identity, scoped delegation, revocation | Study agent passports, signed authorization, and revocation chains for sovereign agent identity. | Do not bind Goni to a draft protocol before local identity requirements are specified. | `primary-source verified` |
| AgentHook | https://agenthook.org/ | Runtime evidence and lifecycle events | Study common event shapes for tool calls, approvals, denials, policy decisions, and evidence bundles. | Do not replace Goni receipts with generic observability events. | `primary-source verified` |
| Microsoft Agent Governance Toolkit | https://github.com/microsoft/agent-governance-toolkit | Runtime policy, sandboxing, governance | Compare zero-trust identity, policy checks, and runtime security controls. | Do not import enterprise cloud assumptions into Goni's personal local-first trust model. | `primary-source verified` |
| AgentMint | https://github.com/aniketh-maddipati/agentmint | Signed receipts and runtime authorization | Study minimal Ed25519-style action proof patterns. | Do not treat signature presence as receipt completeness. | `primary-source verified` |
| Sanna | https://sanna.dev/ | Constitution-as-code and governance receipts | Study policy-before-action and cryptographic governance receipt framing. | Do not import policy branding without a Goni policy language and conformance tests. | `primary-source verified` |
| Delegation Receipt Protocol | https://datatracker.ietf.org/doc/draft-nelson-agent-delegation-receipts/ | Signed delegation receipts | Study authorization object fields, scope boundaries, time windows, and model-state commitments. | Do not claim standards compliance while the draft is unstable. | `primary-source verified` |
| Authenticated Delegation and Authorized AI Agents | https://arxiv.org/abs/2501.09674 | Authenticated delegated authority | Use IAM-style authenticated delegation as an academic anchor for bounded authority. | Do not treat legal authority as solved by authentication alone. | `primary-source verified` |

## Evaluation and benchmark environments

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| OSWorld | https://os-world.github.io/ | Desktop/computer-use benchmark | Study reproducible multimodal desktop tasks for future LifeBench-style evaluation. | Do not treat OSWorld task success as sufficient for personal-memory delegation quality. | `primary-source verified` |
| OSWorld repository | https://github.com/xlang-ai/OSWorld | Benchmark harness | Inspect environment construction and task packaging. | Do not import real-user traces into public fixtures. | `primary-source verified` |
| WebArena | https://github.com/web-arena-x/webarena | Self-hostable web-agent environment | Use realistic reproducible websites for delegated web-action tests. | Do not expose private accounts or real browsing workflows in public tests. | `primary-source verified` |
| WorkArena | https://github.com/ServiceNow/WorkArena | Enterprise knowledge-work benchmark | Study office-work task decomposition for Action Cards and Daily Briefs. | Do not assume enterprise SaaS tasks represent personal sovereign workflows. | `primary-source verified` |
| Mind2Web | https://osu-nlp-group.github.io/Mind2Web/ | Web-agent dataset and generalization benchmark | Study task representation and website generalization labels. | Do not import non-synthetic personal workflows into Goni fixtures. | `primary-source verified` |
| AgentBench | https://openreview.net/forum?id=zAdUB0aCTQ | Multi-environment agent benchmark | Study multi-domain task coverage and agent evaluation reporting. | Do not use broad agent scores as a proxy for Goni memory, receipt, or privacy quality. | `primary-source verified` |

## Runtime verification and assurance

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| AgentGuard | https://arxiv.org/abs/2509.23864 | Runtime verification and agent assurance | Study formal events, monitorable safety properties, and MDP-style assurance framing. | Do not call Goni a POMDP or MDP implementation before states, actions, transitions, rewards, and evaluations are formalized. | `primary-source verified` |
| SWE-agent | https://github.com/SWE-agent/SWE-agent | Agent-computer interface design | Study how interface design changes agent task success and repair loops. | Do not generalize software-engineering agent results to personal delegation without LifeBench evidence. | `primary-source verified` |
| OpenHands | https://github.com/All-Hands-AI/OpenHands | Mature open-source software-agent platform | Study sandbox/workspace patterns and action trace ergonomics. | Do not treat coding-agent infrastructure as a personal memory OS. | `primary-source verified` |

## GoniOS owned gap

No surveyed project cleanly unifies all of the following into one disciplined
architecture:

- local-first personal memory,
- resource-bounded context selection,
- rolling state,
- auditable Action Cards,
- cryptographic receipts,
- sovereign portability.

That is the academic lane for GoniOS. The repo should use these neighbors to
sharpen contracts and evidence, not to dilute Goni into a generic agent
framework.

## Needs primary source or out-of-scope for this map

These entries are not used as technical anchors in this document:

| Entry | Reason |
| --- | --- |
| OpenClaw-style gateway references | Covered in the broader adjacent-project inventory; useful as gateway prior art, not a primary academic anchor for resource-bounded cognition. |
| Cognithor | Omitted until a stable primary source is reviewed under this document's source rule. |
| Third-party AI OS summaries | Omitted because this map accepts only primary sources. |

## Related Goni documents

- [Adjacent projects](/blueprint/docs/adjacent-projects.md)
- [Related projects](/blueprint/docs/related-projects.md)
- [Bibliography](/blueprint/docs/references/bibliography.md)
- [Memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
