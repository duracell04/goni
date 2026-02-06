# Non-bypass integration tests

These tests are placeholders for egress and capability bypass checks.

This README also defines the latency and safety metrics we expect to report from
non-bypass integration runs that stream tokens, call tools, and emit receipts.

## Streaming latency: what the four numbers mean

Think of a request as a timeline:

`t0 (user sends)` -> networking -> queue/scheduler -> prefill -> decode tokens
-> optional tools/RAG -> more decode -> stream ends.

### 1) Time to first chunk (TTFC)

Time to first streamed bytes reaching the client (HTTP headers, first SSE
event, keep-alive chunk, or a "stream started" marker).

This mostly reflects:
- network + TLS + routing
- request admission
- "something started streaming"

Important: the first chunk is not always "the model is generating." Some stacks
emit an early "working..." chunk before model compute begins.

### 2) Time to first token (TTFT)

Time until the model produces its first generated token. It includes:
- queue wait (if any)
- prompt assembly (system + user + context)
- prefill compute
- first decode step

For interactive UX, TTFT is usually the single most important latency metric.

### 3) Time to first summary token

A product-specific marker: "first token that belongs to the summary section."
It implies a phased pipeline, for example:
- Phase A: quick acknowledgement
- Phase B: retrieval/tool calls / heavy reasoning
- Phase C: summary output begins

If this is late relative to TTFT, it usually indicates retrieval/tools or
intentional deferral of the "useful" output until evidence is gathered.

### 4) Response time

Total time until the system reports completion (stream closed / final output).

Long tails after summary starts typically mean:
- summary is only one section and more content follows, or
- generation is slow (low tokens/sec), or
- tools are still running / waiting / throttled.

## Why these matter to Goni

- First chunk: "Did the system respond at all?" (liveness)
- First token: "It's alive and not stuck." (perceived responsiveness)
- First summary token: "It delivered the useful part." (value latency)
- Total time: "How long until it's fully done?" (completion latency)

For an operator appliance, value latency (time to first actionable output)
matters as much as TTFT.

## Goni-grade scorecard (what to track)

### 1) Interactive UX

- TTFC (time to first chunk)
- TTFT (time to first token)
- TTFA (time to first actionable output)
- token gap distribution: p50 / p95 / p99 max gap
- sustained token rate during decode (tokens/sec)
- tail latency: TTFT p50/p95/p99, total time p50/p95/p99

### 2) Scheduler / QoS

- time-to-preempt (interactive arrives -> background yields)
- worst-case non-preemptible region (quantum)
- cancel-to-quiescent (no more tokens, tools stopped, resources released)
- cancel-to-safe-state (rollback/compensation if supported)
- TTFT inflation factor = TTFT(mixed) / TTFT(interactive-only)
- queue wait time and background WIP
- drop/defer rate for background tasks under load

### 3) Tool execution

- time-to-tool-plan
- tool call duration + retries/backoff
- tool result parse/validation time
- success rate and partial failure rate
- idempotency violations
- undo success rate (if reversible actions exist)
- approval rate, time-to-approval, user edit distance (if HITL)

### 4) Retrieval / Vault

- retrieval latency p50/p95/p99
- indexing/update latency
- cache hit rate
- Recall@k / citation coverage / unsupported claim rate
- vault growth rate, compaction ratio
- sync delta size and convergence time (if multi-device)

### 5) Security / governance

- egress decision latency
- bytes egressed per request + per domain
- blocked egress attempts and redaction events
- receipt coverage (% of tool calls + egress with receipts)
- receipt verification time + log growth rate
- sandbox escape attempts detected
- policy bypass attempts
- TCB size (even approximate early on)

### 6) Resource / hardware

- prefill latency, decode latency per token
- tokens/sec
- KV cache memory vs context length
- memory bandwidth utilization (if measurable)
- joules per token (or power draw)
- time to thermal throttle + performance collapse curve

### 7) Reliability

- crash-free sessions %
- tool error rate
- timeout rate
- MTTR
- update rollback success (if shipped)
- "stuck stream" rate

## Suggested trace spans (for reproducible runs)

1) ingress
2) policy_check
3) context_assemble
4) prefill
5) decode_stream
6) tool_calls[] (nested)
7) egress[] (network gate decisions + bytes)
8) receipt_write
9) complete

Derived metrics:
- TTFC = first output in decode_stream (or earlier if heartbeat)
- TTFT = first token in decode_stream
- first summary token = first token after summary_section_start marker
- response time = complete

## Diagnostic reading example

If TTFC and TTFT are fast but "first summary token" is late, the system is
likely spending time in retrieval/tools or deliberately deferring the summary
until evidence is gathered. If total time is very long after the summary
begins, generation is slow or tools are still running.
