---
id: EVID-LONGCTX-01
type: EVIDENCE
status: specified_only
---
# EVID-LONGCTX-01 Reading Strategy Comparison

Goal: compare four long-context reading strategies on the same fixed corpus and
task fixtures without treating any single paper or social post as an automatic
architecture decision.

## Strategies
- Full-context baseline.
- Current RAG/context assembly baseline.
- Programmatic corpus-reading baseline.
- Hybrid retrieval + programmatic reading baseline.

## Fixture families
- Long single-document QA.
- Multi-document synthesis.
- Citation/span extraction.
- Needle-in-large-corpus lookup.
- Long-horizon reasoning over distant evidence spans.

## Metrics
- answer quality
- citation/span fidelity
- latency
- total tokens
- tool-call count
- `tokens_scanned` or `document_bytes_scanned`
- `slice_count`
- `subread_count`
- `recursion_depth`
- branch_count
- variant_count_requested
- failure_mode_label

## Evidence artifact links
- TBD
