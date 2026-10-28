# Agent operating systems and governed human–machine delegation

**Research snapshot:** 28 October 2026  
**Status:** non-canonical research synthesis; no implementation or normative contract changes

## 1. Research question

Can a personal AI system deliver useful long-running autonomy with verifiable limits on delegated authority, minimal excess privilege, and a small human-interruption burden?

This question differs from whether an agent can call tools, complete benchmarks, or maintain memory. The design target is **delegated action under human authority**, not autonomous cognition as a source of authority. The canonical Goni authority and conformance contracts remain in the knowledge graph (DELEG-01, DELEG-INT-01, SPEC-KERN-01, SPEC-POL-01, SPEC-ENF-01, SPEC-TXN-01, NET-01, TOOL-01, REC-01; EVID-ENF-01 and EVID-AUTON-01). Those contracts are specified, not established as enforced.

## 2. Carbon–silicon symbiosis: a bounded mathematical interpretation

“Carbon” and “silicon” denote the *biological human principal* and the *engineered computational substrate* metaphorically. They do not imply that the machine is a biological organism, has independently legitimate human goals, or constitutes a new source of delegated authority. Licklider's [*Man-Computer Symbiosis* (1960)](https://doi.org/10.1109/THFE2.1960.4503259) is the historical precedent for close human–computer cooperation; [Horvitz (1999)](https://doi.org/10.1145/302979.303030) studies mixed-initiative decisions under uncertainty and attention costs. [Tomašev, Franklin and Osindero (2026)](https://arxiv.org/abs/2602.11865) separately discuss delegation as an adaptive allocation of tasks, authority and accountability.

| Role | Function | Source of permission |
| --- | --- | --- |
| Human \(H\) | Sets goals and constraints, grants bounded mandates, judges outcomes, corrects and revokes | Human principal |
| AI cognition \(M\) | Estimates intent, interprets data, proposes plans, predicts outcomes, drafts artifacts | None merely by proposing |
| Goni authority kernel \(K\) | Evaluates actions against canonical Work Orders, capabilities, policy and state | Explicit human-originating authorization |
| Tools and host \(E\) | Perform bounded external effects; produce reconstructable evidence | Effective kernel capability plus isolation boundary |
| Governed memory \(V\) | Maintains source-linked evidence and qualified preferences | Read/write authority defined by existing contracts |

This is a responsibility decomposition, not an additional Goni architecture. The existing first-brain / second-brain / third-brain / digital-meninges explanatory model already locates these responsibilities.

At step \(t\), let \(W_t\) be the active Work Order, \(P_t\) canonical policy, \(C_t\) effective capabilities, \(x_t\) observed context, and \(m_t\) governed memory. Probabilistic model output defines a proposal distribution \(q_\phi(a\mid x_t,m_t,W_t)\), **not** a grant. The authorization-feasible set is:

$$
\mathcal F_t=\{a:\operatorname{Authorize}_K(a;W_t,P_t,C_t)=1\}.
$$

If the principal's task-specific utility is \(U_H(a;\theta)\), with uncertain preference state \(\theta\) represented by a belief \(b_t(\theta)\), a *candidate research objective* is:

$$
a_t^\star\in
\underset{a\in\mathcal F_t\cup\{\mathrm{defer}\}}{\arg\max}
\Big(
\mathbb E_{\theta\sim b_t}[U_H(a;\theta)]
-\lambda\,c_{\mathrm{interrupt}}(a)
-\mu\,c_{\mathrm{resources}}(a)
\Big).
$$

The feasible set is **a hard precondition**, not a risk penalty. \(\lambda\) and \(\mu\) are tunable trade-off weights, not measured constants. A learned preference estimate cannot increase capability scope, and deferral or clarification remains available when legitimate intent or consent is unresolved.

For effectful actions, the intended invariant is:

$$
\operatorname{Effect}(a_t)\Longrightarrow
\operatorname{Authorize}_K(a_t;W_t,P_t,C_t)=1
\quad\text{at execution time},
$$

with any subdelegate's effective authority no broader than its parent grant, \(C_{\mathrm{child}}\preceq C_{\mathrm{parent}}\). A formal invariant does not prove its implementation: it relies on *complete mediation*, trusted canonical authority state and a pinned host/sandbox threat model. Receipt integrity establishes evidentiary properties under stated trust assumptions; it does not certify the semantic correctness of decisions or outcomes.

## 3. Agent OS prior work: different systems, different claims

| Source | Contribution | Goni comparison and evidence limits |
| --- | --- | --- |
| [Builder Methods Agent OS](https://github.com/buildermethods/agent-os) | Coding standards and specification shaping for development agents | Developer-process aid rather than an agent OS security boundary |
| [Mei et al. (2024), AIOS](https://arxiv.org/abs/2403.16971) | Scheduling, context, memory, storage and access-control services for LLM agents | Runtime architecture reference; compare concrete mechanisms instead of asserting AIOS lacks access controls |
| [Liu et al. (2026), AgentOS](https://arxiv.org/abs/2603.08938) | Personal natural-language interfaces, skills and knowledge-discovery agenda | Personal AI neighbor; intent mining is not authorization |
| [Sharma and Shah (2026), AOS](https://arxiv.org/abs/2608.03214) | Reference architecture separating governance from runtime coordination | Proposed architecture; separation is not unique to Goni nor an enforcement result |
| [Zhao et al. (2026), AgenticOS](https://arxiv.org/abs/2606.21129) | Structured-intent, least-privilege OS architecture with mediation and IFC | Architectural near-neighbor; compare policy synthesis and host confinement |
| [Microsoft Agent OS (2026)](https://microsoft.github.io/agent-governance-toolkit/packages/agent-os/) | Python application-level action validation and logging | Documentation acknowledges direct standard-library bypasses; middleware and OS isolation are distinct |
| [Li et al. (24 October 2026), AaOS](https://arxiv.org/abs/2610.33753) | Mapping of 116 OS mechanisms in 19 subsystems; 11 close equivalents and 65 missing; 13 experiments on three coding-agent CLIs | Scoped classification and tests, not universal measurements or evidence about Goni |
| [Sharma et al. (2026), Ajar](https://arxiv.org/abs/2609.26900) | Measures unnecessary permissions left open, separately from attack success and benign utility | Direct precedent for counterfactual authorization tests |
| [Li et al. (2026), FORTIS](https://arxiv.org/abs/2605.09163) | Tests least-privilege model selection of overlapping skills and tools | Selection benchmark, not a deployed enforcement evaluation |

The evidence hierarchy matters: **proposed mechanism → documented implementation → test observation → bounded assurance claim**. These are distinct propositions, and none can be inferred automatically from its predecessor. In particular, a runtime wrapper cannot claim reference-monitor properties merely by intercepting its registered tools.

## 4. Minimal authority is independent of successful task execution

A legitimate task can succeed while the authorization layer still leaves unnecessary effects available. Ajar's evaluation treats this as *open privilege*, separate from utility and successful attacks. Given a predeclared set \(\mathcal N_i\) of unnecessary candidate effectful calls for benchmark task \(i\), define the following Goni evaluation statistic:

$$
\operatorname{OPR}=
\frac{\displaystyle\sum_i\sum_{c\in\mathcal N_i}
\mathbf1\{\operatorname{Allow}_K(i,c)\}}
{\displaystyle\sum_i|\mathcal N_i|},
\qquad
\sum_i|\mathcal N_i|>0.
$$

The unit is an eligible candidate authorization decision, not a model token, tool invocation or security incident. Benchmark tasks with multiple valid plans may have no unique minimal permission set; eligibility and candidate sampling must therefore be fixed before measurement. Report overblocking of legitimate calls as well: a low open-privilege rate obtained by denying everything is not a useful solution. FORTIS independently motivates measuring the model's *selection* of overpowered skills, while kernel authorization tests measure what would actually execute.

## 5. Research design with existing Goni experiments

**EVID-ENF-01 (route-independent mediation).** For the same denied effect and frozen Work Order, test application adapters, raw sockets, shell, subprocess, browser automation, subagents and resumed sessions. Test a permitted control in parallel. Observe independent endpoint traffic or canary state, authorized policy decisions, effect outcomes and receipts. Include revocation between proposal and execution. Passing test cases establish only scoped evidence for the tested host configuration.

**EVID-AUTON-01 (delegation with minimum privilege).** Compare no_go, soft_gate and autopilot using matched task classes and benchmark-provided authorized plans. Report four jointly necessary dimensions: permitted task completion, observed unauthorized effects, open-privilege rate, and unnecessary human interruption. Add overblocking, revocation latency and receipt coverage as diagnostic measures. Prefer a Pareto comparison to an arbitrary sum of security and UX scores.

A defensible system-level result would demonstrate **high useful completion, limited open privilege, low interruption cost, and zero observed unauthorized effects within a specified threat model**. The evidence must identify runtime builds, host permissions, model, tools, policy fixtures, candidate-call sampling, trials and confidence intervals where applicable. Neither zero observed incidents nor a formally written kernel requirement proves universal non-bypassability.

## 6. Integration boundary

This is an evidence-calibrated research brief, **not** a specification, architecture decision, implementation proof, or claim of novelty for separating cognition from authority. Its findings belong conceptually with the existing cognitive-exocortex synthesis, Agent OS research framing, EVID-ENF-01, and EVID-AUTON-01. The canonical Goni nodes remain the sole source of normative requirements, and this note introduces no new runtime services, permissions, ABIs, stores or evaluation machinery.

**Central hypothesis:** A sovereign personal agent can couple human-directed purpose with machine-scale cognition while keeping execution authority independently constrained, inspectable and revocable. Whether the combination performs better than contemporary agent systems remains an empirical question.
