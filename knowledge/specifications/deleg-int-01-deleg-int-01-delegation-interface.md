---
id: DELEG-INT-01
title: DELEG-INT-01 - Delegation Interface
type: specification
status: draft
implementation_state: specified_only
proposition: The pre-execution control plane compiles principal intent into model-specific, authority-bounded, verifiable delegations by resolving machine-answerable gaps, minimizing principal interruption, selecting task-specific context and agents, and escalating deliberation only when its expected value justifies its cost.
domains:
- specs
- delegation
- orchestration
aliases:
- DELEGATION-INTERFACE
relations:
- type: refines
  target: DELEG-01
  note: Makes prompt reconstruction, clarification, model routing, and verification explicit pre-execution control-plane responsibilities.
- type: depends_on
  target: MODEL-REG-01
  note: Model-relative delegation requires governed knowledge of available model bundles and their effective capabilities.
- type: depends_on
  target: MSC-01
  note: Delegation compilation allocates context, scaffolding, verification, and interruption according to minimum sufficient control.
- type: depends_on
  target: LLM-COUNCIL
  note: Council deliberation is an optional escalation path available to the orchestrator when additional independent reasoning has positive expected value.
sources: []
artifacts: []
uncertainty: This specifies target orchestration behavior. Gap-value estimation, model capability profiles, interruption costs, council thresholds, and compiler policies remain uncalibrated and unimplemented unless separately evidenced.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: DELEG-INT-01 - Delegation Interface
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# DELEG-INT-01 - Delegation Interface

> Status boundary: this is a `draft` / `specified_only` architecture contract. It defines intended pre-execution behavior, not an implemented or empirically validated orchestrator.

## 1. Purpose

Goni treats delegation as **intent compilation**, not prompt relay. The principal supplies intent, preferences, corrections, relevant private knowledge, and authority. The orchestrator reconstructs that input into a stable, reviewable task contract before execution crosses the kernel boundary.

The target transformation is:

`principal intent -> gap detection -> context acquisition -> specification -> routing -> execution -> verification -> integration`

In compact form:

> **Goni compiles human intent into model-specific, authority-bounded, verifiable delegations.**

The orchestrator may use one model, several specialist agents, deterministic software, or a council of models to perform the cognitive work. None of those components gains authority merely by being more capable. The kernel remains the authority boundary for tools, capabilities, side effects, approvals, and receipts.

## 2. Principal-orchestrator boundary

The principal should not need to diagnose the task perfectly or write an expert prompt before useful work can begin. The orchestrator is responsible for identifying which information is missing, whether that missing information matters, and the least costly reliable way to obtain it.

Missing information should be classified by source and decision ownership. The orchestrator should prefer, in policy-permitted order, evidence already available from:

- active policy and mandates;
- stable principal preferences and prior approvals;
- current task and repository state;
- local memory and retrieval;
- connected tools and authorized external sources;
- deterministic constraints and validation systems;
- specialist agents or model deliberation; and
- the principal when the uncertainty is irreducibly principal-owned.

Principal interruption is justified when the unresolved variable materially concerns:

- goal identity or preference;
- a value judgment or trade-off that the principal has not delegated;
- authority to cross a capability or policy boundary;
- inaccessible private knowledge that cannot be recovered elsewhere;
- acceptance of an irreversible or materially consequential choice; or
- ambiguity whose expected cost exceeds the cost of interruption.

Human attention is therefore a governed resource. The orchestrator should minimize unnecessary interruption while preserving informed principal control.

## 3. Gap-resolution loop

Before compiling an executable Work Order, the orchestrator should maintain a bounded epistemic loop:

1. **Observe:** extract atomic facts, current state, explicit requests, constraints, and authority already granted.
2. **Hypothesize:** identify plausible interpretations of the objective and the missing variables that could change execution.
3. **Value the gaps:** estimate whether resolving each unknown can materially change correctness, risk, cost, or authority.
4. **Acquire:** resolve high-value gaps from the cheapest sufficiently reliable authorized source.
5. **Escalate:** ask the principal or invoke additional deliberation only when local resolution is insufficient and the expected value justifies the cost.
6. **Compile:** produce the Work Order, Done Contract, model/tool allocation, verification plan, and stop condition.
7. **Execute and measure:** delegate bounded work, collect outputs and receipts, and compare them with the contract.
8. **Integrate or reopen:** accept verified work, repair a bounded failure, or reopen the relevant gap if new evidence changes the task model.

This loop should preserve the distinction between observations, orchestrator inferences, principal-endorsed objectives, and granted authority.

## 4. Delegation compilation

An executable delegation should be compiled from a model-independent task contract and a model-specific execution scaffold.

### 4.1 Canonical task contract

The stable contract should identify at least:

- the objective and intended outcome;
- authoritative context and source hierarchy;
- hard invariants and constraints;
- assumptions and unresolved uncertainty;
- authority boundaries and permitted effects;
- deliverable and output contract;
- acceptance criteria and required evidence;
- verification and rollback requirements; and
- the stop or escalation condition.

This contract should remain semantically stable when the execution model changes.

### 4.2 Model-specific execution scaffold

The orchestrator should adapt the execution scaffold to the effective agent system, including where relevant:

- model family and checkpoint;
- observed task competence and instruction reliability;
- reasoning mode or budget;
- context-window characteristics;
- local versus remote execution;
- quantization or other capability-affecting deployment choices;
- tool and retrieval support;
- structured-output reliability;
- task horizon and constraint coupling; and
- available deterministic verification.

Stronger agents can receive broader coherent objectives and more solution-space freedom when the task is observable and verifiable. Smaller, weaker, quantized, or locally constrained models can receive narrower work units, clearer schemas, lower simultaneous constraint coupling, examples where they add information, and more frequent deterministic checkpoints.

The orchestrator should move complexity from prose into orchestration, state, tools, tests, schemas, and feedback loops whenever those mechanisms provide stronger guarantees at lower cognitive cost.

## 5. Context allocation

The orchestrator may maintain a much larger global task and principal context than any individual worker needs. Worker context should be selected for task relevance rather than broadcast indiscriminately.

The intended pattern is:

`global context -> relevance selection -> agent-specific working context`

Each sub-agent should receive the minimum sufficient context required to understand its objective, constraints, interfaces, evidence boundary, and acceptance criteria. Additional context should be included when its expected contribution exceeds its attention, latency, privacy, or contradiction cost.

This allows global coherence without forcing every worker to reason over the entire principal state.

## 6. Agent and mechanism routing

The orchestrator should choose the cheapest sufficiently capable mechanism for each subproblem. Candidate mechanisms include:

- deterministic software for exact state, arithmetic, schemas, permissions, validation, and other must-be-exact operations;
- a local model for bounded semantic work where privacy, latency, or sovereignty dominates;
- a stronger remote model when policy permits and expected capability gain justifies egress and cost;
- specialist agents for separable domain or tool tasks; and
- an LLM Council for selected uncertainty or review boundaries.

Routing decisions should consider task difficulty, consequence, reversibility, verifiability, privacy, expected regret, latency, cost, and current model evidence rather than model prestige alone.

## 7. Council escalation

The LLM Council is a cognitive escalation mechanism, not a standing committee for routine work. The orchestrator should invoke it when the expected reduction in decision error, uncertainty, or blind spots exceeds the added cost, latency, privacy exposure, and integration burden.

Council escalation is especially relevant when:

- several materially plausible interpretations or strategies remain after ordinary retrieval and reasoning;
- consequences are high and independent review has meaningful expected value;
- evidence conflicts or the orchestrator detects fragile assumptions;
- the decision is difficult to reverse;
- specialist perspectives are genuinely complementary; or
- a single model's confidence is poorly calibrated for the task.

Council quality depends on error diversity more than headcount. Where feasible, the orchestrator should prefer independent first-pass reasoning, heterogeneous model or specialist roles, explicit criticism or falsification, and a separate synthesis or adjudication step. Repeated copies of the same model and context should not be treated as equivalent to independent evidence.

Council outputs remain proposals and evidence for the orchestrator. They do not expand the principal's mandate, bypass the kernel, or acquire execution authority.

## 8. Verification and stopping

The orchestrator should compile verification independently from model intelligence. More capable reasoning can justify broader execution freedom inside an authorized corridor, while verification depth remains proportional to consequence, irreversibility, observability, and error loss.

Preferred closed-loop execution is:

`understand -> act -> measure -> correct -> verify`

The system should stop when the Done Contract and required evidence are satisfied. Additional work requires a substantive new objective, unresolved acceptance criterion, detected failure, or justified optimization opportunity whose expected value exceeds its cost.

## 9. Clarification policy

The existing clarification outcomes remain:

`ClarificationDecision = assume | ask_decisive | propose_objectives | block`

The orchestrator should:

- `assume` when policy, context, and reversibility make surfaced assumptions sufficient;
- `ask_decisive` when one principal-owned answer materially changes the objective, authority, risk, or irreversible path;
- `propose_objectives` when goal identity itself remains materially ambiguous; and
- `block` when no authorized path can make safe progress.

Questions that can be answered reliably from authorized system context should be resolved by the orchestrator instead of transferred to the principal.

## 10. Invariants

- **I1 - Principal sovereignty:** the orchestrator may infer and propose; principal-owned goals, values, and undelegated trade-offs remain principal decisions.
- **I2 - Authority independence:** model capability, confidence, council agreement, or reasoning effort cannot enlarge the authority corridor.
- **I3 - Kernel mediation:** mutating or externally visible effects remain subject to kernel policy, capability checks, approval requirements, and receipts.
- **I4 - Work-order first:** consequential delegated execution requires a stable Work Order and Done Contract.
- **I5 - Machine-resolvable gaps first:** authorized context and tools should resolve material factual gaps before the principal is interrupted.
- **I6 - Minimum sufficient context:** sub-agents receive task-relevant context sufficient for their contracts without default full-context broadcast.
- **I7 - Model-relative scaffolding:** decomposition, prompt detail, context, and checkpoints adapt to effective model capability and environment.
- **I8 - Verification independence:** reasoning capability and execution authority do not substitute for required evidence or deterministic checks.
- **I9 - Council proportionality:** council deliberation is invoked when its expected epistemic value justifies its total cost.
- **I10 - Corrigibility:** assumptions, inferred objectives, routing decisions, and material uncertainty remain reviewable and correctable.

## 11. Architectural consequence

Prompt engineering becomes an internal compilation problem. The principal interacts primarily through intent, preferences, corrections, and authority. Goni is responsible for producing the smallest sufficient specification and execution scaffold for the selected agent system, then verifying the resulting state under the kernel's authority boundary.

This preserves the core system doctrine:

> Models reason. The kernel authorizes. Tools act. Receipts prove.
