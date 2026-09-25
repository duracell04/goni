---
id: GONI-SYNTHESIS-20260916-EPISTEMIC-DELIBERATION
title: Epistemic Deliberation Architecture for Multi-Agent AI
type: synthesis
status: draft
implementation_state: not_applicable
proposition: For high-reliability multi-agent AI, the primary design objective is to preserve and aggregate independent evidence while controlling social coupling; consensus, agent count, confidence, and recursive review are only useful insofar as they add discriminative information about the underlying task.
domains:
- epistemics
- multi-agent
- orchestration
- research
aliases:
- EPISTEMIC-DELIBERATION-ARCHITECTURE
relations:
- type: synthesizes
  target: GONI-EVIDENCE-20260916-MAD-CONFORMITY
- type: synthesizes
  target: GONI-EVIDENCE-20260916-DIVERSITY-CALIBRATION
- type: synthesizes
  target: GONI-EVIDENCE-20260916-EVALUATOR-BIAS
- type: synthesizes
  target: GONI-EVIDENCE-20260916-INCENTIVE-AGGREGATION
- type: synthesizes
  target: GONI-PROPOSAL-20260916-STAGED-EPISTEMIC-DELIBERATION
- type: synthesizes
  target: GONI-EXPERIMENT-20260916-EPISTEMIC-DELIBERATION
sources:
- SRC-GONI20260916-EPISTEMIC-DELIBERATION-NOTE
- SRC-KRAIDIA2026-ADVERSARIAL-MAD
- SRC-CUI2026-FREE-MAD
- SRC-SMIT2024-MAD
- SRC-ZHU2026-MAD-DIVERSITY
- SRC-OKAWA2026-BIASED-CONSENSUS
- SRC-CHOI2025-GROUP-CONFORMITY
- SRC-PANICKSSERY2024-SELF-PREFERENCE
- SRC-SHI2025-POSITION-BIAS
- SRC-BAND2024-LINGUISTIC-CALIBRATION
- SRC-LADHA1995-CORRELATED-VOTES
- SRC-GNEITING2007-PROPER-SCORING
- SRC-PRELEC2004-BAYESIAN-TRUTH-SERUM
artifacts: []
uncertainty: This synthesis combines peer-reviewed results, a recent preprint, classical aggregation theory, and Goni-specific architectural deductions. Several mathematical expressions are explanatory models or heuristics rather than validated quantitative estimators for LLM collectives.
legacy: []
---

# Epistemic Deliberation Architecture for Multi-Agent AI

> **Research snapshot:** 2026-09-16  
> **Scope:** conceptual and architectural synthesis  
> **Implementation boundary:** this document does not claim that Goni currently implements the proposed protocol.

## Abstract

Multi-agent language-model systems are often motivated by an intuitive analogy to human deliberation: multiple reasoners exchange arguments and thereby converge toward better answers. Current evidence makes that intuition conditional. Interaction can improve error correction, but it can also propagate correlated mistakes, amplify persuasion, induce conformity, and create false confidence through repeated agreement. The central resource is therefore not the nominal number of agents but the quantity and quality of **independent epistemic signal** available to the system.

This synthesis develops a high-level architecture for reliable multi-agent reasoning in Goni. The proposed design separates solution generation, critique, meta-review, external verification, and aggregation into staged information-flow regimes. It treats claims rather than agents as the principal objects of evaluation; preserves provenance while selectively blinding evaluators; weights independent evidence lineages more heavily than repeated correlated agreement; and terminates recursive model review by grounding verifiable claims in external evidence. The architecture is best understood as **epistemic mechanism design**: the system controls information exposure, role incentives, evaluation procedures, and verification channels so that useful dissent and falsification survive longer than rhetorical consensus.

## 1. Problem formulation: collective reasoning is not automatically collective intelligence

A naive deliberation model assumes that additional interaction tends to reveal errors and improve collective accuracy. Multi-agent LLM research increasingly shows that this implication is not generally valid. Persuasive adversaries can degrade collective accuracy; initially correct agents can move toward incorrect group positions; and debate protocols do not reliably dominate simpler independent ensembling without careful design. At the same time, recent work suggests that initial diversity and calibrated confidence can improve collective reasoning under some protocols.

The architectural question is therefore not:

> How many agents agree?

It is:

> How much independent, discriminative evidence about the task survives the interaction protocol?

This distinction separates **social convergence** from **epistemic convergence**. The former measures agreement. The latter requires that agreement be explained by evidence that genuinely discriminates among competing hypotheses.

## 2. Independence, correlation, and effective epistemic sample size

Classical jury and ensemble arguments depend critically on assumptions about independence or controlled dependence. If several nominally distinct agents share training data, model architecture, retrieval sources, prompts, or failure modes, their errors may be correlated. Five agreeing outputs can therefore represent substantially less than five independent observations.

A useful diagnostic heuristic, under an exchangeable equal-correlation approximation, is

$$
n_{\mathrm{eff}} \approx \frac{n}{1+(n-1)\rho},
$$

where $n$ is nominal agent count and $\rho$ is average pairwise dependence. The expression is not a universal estimator for LLM systems. Its architectural role is to make one point explicit: **multiplicity without independence has diminishing epistemic value**.

For Goni, diversity should therefore be tracked as a property of evidence lineage, not merely model labels. Relevant axes include:

- model family and training lineage;
- prompt and reasoning procedure;
- retrieval source and corpus provenance;
- toolchain and verifier;
- data modality;
- sampling path;
- domain specialization.

The architecture should seek sufficient heterogeneity to reduce shared blind spots while retaining enough common structure for outputs to be comparable.

## 3. Delayed coupling as an information-flow control

Let $J_{ij}$ denote the influence available from agent $j$ to agent $i$ during a reasoning phase. Free-form debate permits $J_{ij} \neq 0$ early, before each agent has committed an independent hypothesis. A staged protocol instead enforces approximately

$$
J_{ij}=0
$$

during initial generation and only introduces controlled coupling after candidate solutions have been frozen.

The analogy to statistical-physics interaction models is useful but should remain an analogy. In Ising-like social-dynamics models, strong interaction terms can produce synchronization even when the external evidence fields do not warrant it. Recent preprint work applies a related formalism to LLM debate and reports collective transitions toward biased consensus under sufficiently strong conformity, with heterogeneity moderating the effect.

The design implication is narrower and more robust than the analogy: **delay exposure to peer reasoning until independent hypotheses exist as immutable comparison points**.

## 4. Conditional information gain as a normative criterion

Let

- $T$ denote the unknown task-relevant truth or state;
- $S$ a candidate solution;
- $R$ a review;
- $A$ a meta-review or audit.

A review is epistemically valuable to the extent that it contributes information about $T$ beyond what was already contained in $S$. In information-theoretic notation, the idealized condition is

$$
I(T;R\mid S)>0.
$$

Likewise, a meta-review adds epistemic value only insofar as

$$
I(T;A\mid S,R)>0.
$$

These expressions are conceptual criteria, not generally observable quantities in open-ended reasoning. Their purpose is to expose a common failure mode: additional tokens, additional agents, or additional layers of review can increase process volume without increasing information about the underlying state.

A review that merely restates the solution, or an audit that merely endorses a review's style, may have near-zero marginal epistemic value even if it appears sophisticated.

## 5. Blinding, anonymity, and evaluator bias

Anonymity and independence are different properties.

Removing an author's name can reduce status-like or brand-like effects, but LLM evaluators may still infer authorship or model family from stylistic regularities. LLM-as-a-judge systems can also exhibit order effects. Goni should therefore use **selective blinding** rather than equate anonymity with unbiased evaluation.

A structured candidate schema can reduce superficial variance while preserving substantive content:

1. claim;
2. evidence;
3. assumptions;
4. reasoning summary;
5. uncertainty;
6. falsification conditions;
7. proposed answer or action.

Provenance should remain fully recorded by the system even when temporarily hidden from reviewers. Randomized candidate order, balanced review assignments, and post-hoc bias checks should be treated as evaluation controls rather than guarantees.

## 6. Epistemic mechanism design

The orchestration protocol induces a game through what it rewards, selects, remembers, and exposes.

A system that rewards consensus implicitly favors outputs that are easy for other agents to endorse. A system that rewards later-validated correctness, calibrated probability, accurate defect detection, useful falsification, and independent evidence acquisition creates a different incentive surface.

When objective outcomes eventually materialize, proper scoring rules provide a principled basis for evaluating probabilistic forecasts and calibration. When objective truth is unavailable, peer-prediction mechanisms such as Bayesian Truth Serum are theoretically relevant but depend on assumptions that cannot be presumed for LLM agents.

Accordingly, Goni should distinguish:

- **outcome-scored domains**, where empirical calibration can accumulate over time;
- **externally verifiable domains**, where tools or primary evidence can close claims;
- **partially verifiable domains**, where only subsets of claims can be checked;
- **irreducibly judgmental domains**, where disagreement and uncertainty may need to remain unresolved.

## 7. Staged epistemic deliberation

The proposed architecture is:

$$
\boxed{\text{Independent commitment}
\rightarrow \text{Blinded critique}
\rightarrow \text{Blinded meta-review}
\rightarrow \text{External verification}
\rightarrow \text{Provenance-aware synthesis}}
$$

### Stage 1: independent commitment

Each solver receives the task contract and permitted evidence but not other solvers' outputs. It commits its answer, assumptions, evidence, uncertainty, and falsification conditions.

### Stage 2: blinded critique

Reviewers inspect frozen solutions under controlled presentation. Reviews are generated independently of one another. The purpose is defect discovery, not consensus formation.

### Stage 3: blinded meta-review

Auditors evaluate whether specific criticisms are valid, material, and adequately supported. They do not observe peer-auditor judgments before commitment.

### Stage 4: external verification

Where claims admit checks, the system moves from model opinion to reality-facing evidence: executable tests, formal computation, primary sources, retrieval with provenance, proof checking, measurements, or later observed outcomes.

This stage prevents an infinite regress of

$$
S \rightarrow R \rightarrow A \rightarrow A^2 \rightarrow A^3 \rightarrow \cdots
$$

because additional model opinion is not treated as the ultimate closure mechanism.

### Stage 5: provenance-aware synthesis

The final synthesizer receives the frozen evidence graph. It aggregates support by claim and evidence lineage, identifies shared dependencies, distinguishes independent confirmations from correlated repetitions, and preserves unresolved alternatives.

## 8. The claim, not the agent, is the primary epistemic object

A conventional council treats an agent's answer or vote as the unit of aggregation. Goni should instead reason over a graph whose central object is closer to

$$
\boxed{\text{claim}+\text{provenance}+\text{evidence}+\text{criticism}+\text{audit}+\text{verification state}}.
$$

A minimal claim record should be able to represent:

| Field | Function |
| --- | --- |
| `claim_id` | stable identity |
| `content` | proposition under evaluation |
| `origin` | solver/model/tool provenance |
| `evidence_refs` | supporting observations or sources |
| `assumptions` | conditions required for the claim |
| `falsifiers` | observations that would materially weaken it |
| `critique_refs` | independent objections |
| `audit_refs` | evaluations of those objections |
| `verification_state` | unverified, partially verified, verified within stated scope, contradicted |
| `confidence` | calibrated value only when meaningful |
| `lineage` | model, retrieval, tool, source, and dependency ancestry |

This representation makes it possible for two agents to agree without the system double-counting a shared source, and for a minority claim to remain alive when it has stronger independent evidence than the majority.

## 9. Aggregation without false precision

The synthesis layer may use a support function over factors such as independent evidence, external verification, calibration history, critique survival, and unresolved counterevidence. Unless the system has an explicit probabilistic model with validated calibration, that support value should **not** be presented as a posterior probability.

The aim is not to replace naive majority vote with an opaque numerical score. It is to preserve the structure needed for justified aggregation:

- how many independent evidence lineages support the claim;
- whether evidence is primary, derived, or circular;
- whether critiques survived independent audit;
- whether a deterministic or empirical check exists;
- whether dissent remains materially unresolved;
- how historically calibrated the contributing models and verifiers are in the relevant domain.

## 10. Failure modes the architecture must preserve as explicit risks

The staged design reduces several failure channels but does not eliminate them:

1. **correlated priors:** different models can inherit the same misconception;
2. **retrieval monoculture:** nominally independent agents can consult the same flawed source;
3. **reviewer monoculture:** critics can share the solver's blind spot;
4. **self-recognition:** style can leak model identity despite nominal anonymity;
5. **position and presentation bias:** evaluation can depend on ordering or verbosity;
6. **verification monoculture:** several tools can share one upstream dataset or implementation;
7. **independence theatre:** stochastic replicas of one model may appear more diverse than they are;
8. **recursive reassurance:** additional review layers can create confidence without new evidence;
9. **over-blinding:** hiding provenance can remove information that is legitimately relevant to reliability;
10. **compute inflation:** elaborate review graphs can cost more than their marginal epistemic value.

The stopping rule should therefore be economic as well as epistemic: add another reasoning or verification layer only while its expected marginal information and error reduction justify its latency, cost, and complexity.

## 11. Integration with Goni's existing architecture

This synthesis fits the existing Goni separation of cognition, governance, and evidence.

- **Context isolation** should enforce stage-specific information barriers so initial solvers, reviewers, and auditors receive only the information their role requires.
- **The Control Plane** should schedule assignments, freeze commitments, randomize evaluation order, and prevent premature cross-agent coupling.
- **The Pia / epistemic-control layer** should carry claim provenance, source lineage, contradiction state, confidence, review state, and verification status.
- **Execution tools** should provide the reality-facing verification mechanisms that terminate recursive model review.
- **Receipts** should record stage transitions, model/tool identities, evidence references, and the basis for synthesis. Receipts establish reconstructability, not truth.
- **Audit-grade work rules** should remain the governing discipline for scope declarations, evidence/inference separation, negative claims, and missing-evidence surfacing.

The proposal therefore adds no new sovereign authority plane. It specifies a reasoning protocol that can run inside Goni's existing governance boundaries.

## 12. Falsifiable research hypotheses

The architecture should be treated as a research hypothesis rather than accepted by intuition.

### H1: delayed coupling

At matched compute and token budgets, freezing independent initial hypotheses before cross-agent exposure should reduce conformity-driven error propagation relative to free-form debate.

### H2: lineage diversity

Heterogeneous evidence lineages should provide greater reliability gains than increasing the number of highly correlated replicas once nominal ensemble size passes a modest threshold.

### H3: controlled evaluation

Blinding, order randomization, and structured candidate schemas should reduce self-preference and presentation bias, subject to the countervailing cost of removing useful provenance.

### H4: external closure

After one competent critique and meta-review layer, external verification should usually provide greater marginal reliability than further recursive LLM review on claims that are directly verifiable.

### H5: claim-centric aggregation

Claim-level evidence graphs should preserve justified minority positions and unresolved uncertainty better than consensus-only aggregation without reducing performance on clearly decidable cases.

The accompanying experiment node specifies how these hypotheses should be tested against simpler baselines.

## 13. Research conclusion

The central design principle is:

> **Independent information before interaction. Claims before agents. Falsification before consensus. Verification before confidence.**

Multi-agent reasoning should not be optimized for making a group agree. It should be optimized for preserving multiple plausible hypotheses long enough for discriminative evidence to act on them.

The deepest architectural shift is therefore from

$$
\text{Who convinced the group?}
$$

to

$$
\text{Which claims survived independent attempts at falsification, and what evidence lineage explains that survival?}
$$

That framing is more compatible with Goni's broader doctrine of evidence-calibrated delegation: models may generate and critique hypotheses, but the system should preserve provenance, control information flow, distinguish inference from observation, and close verifiable disputes against the external world wherever possible.
