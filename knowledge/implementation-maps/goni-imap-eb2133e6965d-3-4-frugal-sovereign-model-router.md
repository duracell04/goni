---
id: GONI-IMAP-EB2133E6965D
title: 3.4 Frugal sovereign model router
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni's router is local-first and sovereignty-aware.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.4 Frugal sovereign model router
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.4 Frugal sovereign model router

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Frugal sovereign model router

Goni's router is local-first and sovereignty-aware. It uses FrugalGPT-style
cascades, but the objective is not only cost and quality: the route must also
account for privacy leakage risk, latency, energy/thermal budget, audit burden,
data locality, external dependency cost, and the active approval corridor.

The operating rule is: run the smallest local computation that can safely solve
the task, then escalate only when confidence, risk, freshness, or capability
constraints justify the extra cost.

Default route order:

```text
rule/cache/memory -> local small -> local large -> local tools/RAG
-> local multi-agent check -> cloud Council -> premium cloud vote
```

The first sufficient local route wins. The cloud-side Council is an escalation
tier, not the default decoder.

DSpark is evidence for the same systems pattern at token scale: cheap draft
work is useful only when a stronger verifier, calibrated confidence estimates,
and load-aware scheduling decide how much draft to trust. For Goni, this maps
to local routers and small models drafting or classifying, stronger local models
verifying, and cloud/council routes remaining exceptional rather than default
intelligence. [[cheng2026-dspark]] [[deepseek2026-v4-dspark-hf]]

At the minimal formal level we distinguish two local model classes and one
remote escalation class:

- Small model \(M_s\) with cost \(c_s\) (tokens/s, energy).  
- Large model \(M_\ell\) with cost \(c_\ell \gg c_s\).
- Remote Council route \(M_r\) with external cost, latency, and privacy terms.

For a request \(x\) and preliminary small-model answer \(\hat{y}_s\), we compute a **calibrated confidence** \(p(x) \in [0,1]\).

Router policy:

1. If \(p(x) > \tau_{\mathrm{accept}}\): accept small model output.  
2. If \(p(x) < \tau_{\mathrm{escalate}}\) and early in the generation: escalate.  
3. Else compare expected value of escalation vs continuation.

Escalation to the cloud-side multi-model path (the [LLM Council](/blueprint/docs/llm-council.md)) follows the triggers in Section 3 of that doc: explicit user request, high difficulty/safety-critical classification, or long-context needs that exceed local comfort.

It is also allowed when current public information is required and the outgoing
payload is public, redacted, or explicitly approved. It is not a default route
for ordinary private context, routine drafting, or tasks where a local verifier
has sufficient confidence.

The router MUST NOT send raw private or sensitive context to \(M_r\) by default.
It must either keep execution local, use a redacted/public-only payload, or
require the configured approval corridor.

Each routing decision emits receipt metadata (`llm_route`) containing the
classification, selected route, models considered/used, redaction requirement,
privacy class sent, and policy decision.

We treat this as a contextual routing problem with side information (the
features used to estimate \(p(x)\)); the prototype is threshold-based, and the
evaluation lane may later train a learned router from preference/regret data.

> **Theorem 3.2 (Regret bound, sketch).**  
> Suppose the confidence estimator is \(\epsilon\)-calibrated and the reward gap between correct/incorrect decisions is bounded. Then there exists a threshold policy (approximated by our router) whose regret \(R_T\) over \(T\) requests satisfies:
> $$
> \frac{R_T}{T} \le \beta(\epsilon)
> $$
> with \(\beta(\epsilon)\) small. In practice we target \(\beta(\epsilon) \le 0.07\).

> **Invariant K2 (Router regret).**  
> On benchmark datasets, empirical regret of `goni-router` compared to an oracle policy that knows ground-truth â€œdifficultyâ€ labels must stay below 0.07.
