# 52 – Zero-Copy Mechanics

Zero-copy operations over Arrow buffers (see `95-theory` for proofs). Outputs mutate booleans/indices; text moves exactly once into an LLM buffer.

## 1. Submodular Selection (Context Window)
- Inputs: 𝒜.Embeddings.vector, 𝒜.Chunks.text (for final take), 𝒳.ContextItems (cost, marginal_gain).
- Process: compute gains; greedy/lazy-greedy knapsack; update `ContextItems.selected` and `ContextItems.rank`.
- Output: mask drives an Arrow `take` on `Chunks.text`. Copies: 0.

## 2. Lyapunov Scheduling (Control)
- Inputs: 𝒦.Tasks.expected_cost_tokens, queue weights (from QueueSnapshot when materialised).
- Process: drift-plus-penalty; select task_ids for dispatch; update `Tasks.state`.
- Output: prioritized `task_id` vector; telemetry in ℰ. Copies: 0 (metadata only).

## 3. Router Regret Analysis (Control→Execution)
- Inputs: 𝒦.RouterDecisions (policy choice + features), ℰ.LlmCalls (latency/tokens), ℰ.Metrics (quality proxies).
- Process: join on `request_id`; compute regret = f(features, latency, quality); populate `offline_reward_estimate`.
- Copies: 0; Arrow compute over numeric/dictionary columns.

## 4. KV Paging (Context Memory)
- Inputs: 𝒳.ContextItems plus internal page table.
- Process: update residency flags; evict without touching `Chunks.text`.
- Copies: 0.
