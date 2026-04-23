---
id: EVID-LONGCTX-02
type: EVIDENCE
status: specified_only
---
# EVID-LONGCTX-02 Corpus-Reading Fidelity

Goal: measure whether programmatic corpus-reading strategies preserve evidence
faithfulness and answer-span fidelity when compared with the current retrieval
baseline.

## Scenario
- Fix corpus, task, and latency/token/tool budgets.
- Compare answer claims against gold evidence spans.
- Evaluate whether corpus-reading loses or distorts evidence that RAG/context
  assembly would preserve.

## Metrics
- evidence_span_count
- answer_span_fidelity
- unsupported-claim rate
- missed-evidence rate
- strategy_switch_count
- quality delta vs RAG/context assembly

## Failure modes
- loses evidence that RAG would have retrieved
- high-confidence answer without evidence-span support
- decomposition drifts away from the source question

## Evidence artifact links
- TBD
