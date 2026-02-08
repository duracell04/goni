# Metrics Scorecard (Performance + Safety)
DOC-ID: METRICS-01
Status: Specified only / roadmap

This document defines a professional, Goni-grade metrics scorecard for a
streaming "LLM + tools" system. It separates user-visible latency from
pipeline internals, and adds governance and safety coverage.

## Scope
- Applies to streaming requests that may include retrieval, tools, and receipts.
- Defines metrics as traceable events in a request timeline.
- Avoids implementation claims unless evidence is linked elsewhere.

## Baseline streaming metrics (definitions)
The following four metrics should be pinned to explicit pipeline events.

| Metric | Definition | Event marker |
| --- | --- | --- |
| Time to first chunk (TTFC) | Time from client send to first streamed bytes reaching the client. | First SSE/stream frame emitted, even if it is a heartbeat. |
| Time to first token (TTFT) | Time from client send to the first model-generated token. | First token event in `decode_stream`. |
| Time to first summary token (TTFST) | Time from client send to the first token that belongs to the summary section. | First token after an explicit `summary_section_start` marker. |
| Response time | Time from client send to stream completion. | Stream end or `complete` event. |

Notes:
- TTFC can be a liveness signal, not proof that model compute started.
- TTFST requires a deterministic marker emitted by the app, not model heuristics.

## Standard request spans (trace framing)
Instrument each request with spans so metrics are explainable.

- `ingress` (request received, validated)
- `policy_check` (capability evaluation and permission gating)
- `context_assemble` (Vault/RAG retrieval and prompt build)
- `prefill` (prompt processing)
- `decode_stream` (token generation)
- `tool_call` (one span per tool)
- `egress` (Network Gate decision and bytes)
- `receipt_write` (receipt creation and integrity)
- `complete` (finalization)

## Derived user-experience metrics
These are the metrics that define perceived value.

| Metric | Definition |
| --- | --- |
| Time to first actionable (TTFA) | Time to first actionable output, such as an action card, a concrete next step, or a "I can do X/Y/Z" option. |
| Token gap p95/p99 | Maximum inter-token pause, measured during `decode_stream`. |
| Sustained token rate | Tokens per second while streaming, excluding long tool waits. |
| Tail latency | Response time p95/p99, not just averages. |

## Goni-grade scorecard (what to measure)
This is the minimal scorecard for an operator appliance.

### 1) Interactive UX
- TTFC, TTFT, TTFA (p50/p95/p99).
- Token gap p95/p99.
- Sustained token rate.

### 2) Scheduler and isolation
- Time-to-preempt (interactive arrival to background yield).
- Cancellation latency (cancel to quiescent, cancel to safe state).
- Mixed workload inflation factor (TTFT mixed / TTFT idle).

### 3) Tool execution
- Tool plan latency (decision to call).
- Tool call duration and retries.
- Tool success rate, partial failure rate, undo success rate.

### 4) Retrieval and memory (Vault)
- Retrieval latency p50/p95/p99.
- Recall@k or a proxy for retrieval quality.
- Citation coverage or unsupported claim rate.

### 5) Governance and receipts
- Egress decision latency and bytes egressed per request.
- Blocked egress attempts and redaction events.
- Receipt coverage and receipt verification time.

### 6) Resources and thermals
- Prefill latency and decode latency per token.
- Tokens per second and KV cache size per context length.
- Time to throttle and performance under throttle.

### 7) Reliability
- Crash-free sessions.
- Tool error rate and timeout rate.
- Stuck-stream rate (stream starts, never completes).

## Example report (with deltas)
Given:
- TTFC: 0.663s
- TTFT: 1.139s
- TTFST: 18.872s
- Response time: 86.061s

Derived:
- TTFT to TTFST gap: 17.733s (value arrives late).
- Summary start to completion: 67.189s (long tail after summary begins).

## Reporting guidance
- Always report p50/p95/p99 for TTFT and response time.
- Separate compute time from waiting time (queue, tools, retrieval).
- Distinguish "model tokens" from "heartbeat chunks."

## Interpretation guide (quick diagnostics)
- Good TTFC and TTFT with slow TTFST usually means tool or retrieval latency.
- Long tail after summary start can indicate slow decode or heavy post-summary work.
- Large token gaps often feel worse than steady slow output.

## Related docs
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Observability](/blueprint/docs/observability.md)
