# Prototype Track 01 - Hello Deterministic RAG

Goal: prove the Arrow spine + deterministic selector + local inference on real repo docs. One repeatable run should produce identical outputs twice.

## What this track exercises
- Planes: Data (Arrow zero-copy), Context (submodular selector), Execution (deterministic profile)
- Invariants: deterministic selection, bounded context budget, no silent cloud calls

## Hero scenario
1) Ingest markdown from `/docs`, `/hardware`, `/software` (and optional `/docs/assets/ai-2027` snapshot) into Qdrant or a mock batch.
2) Run a seeded query via `goni-http`.
3) Observe: selected chunk IDs + scores, total context tokens, final prompt, generated text. Re-run and confirm identical output.

## Quickstart
```bash
# 1) Start stack (local-only)
docker-compose -f blueprint/software/docker-compose.yml up

# 2) Ingest docs
python blueprint/prototype/01-rag-slice/ingest.py --src ./docs ./hardware ./software ./docs/assets/ai-2027 \
  --qdrant-url http://localhost:6333 \
  --collection goni-proto-01 \
  --embed-model sentence-transformers/all-MiniLM-L6-v2 \
  --deterministic

# 3) Run one query through goni-http
curl -X POST http://localhost:7000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"local-small","messages":[{"role":"user","content":"Summarize the LLM council triggers."}],"max_tokens":128}'

# 4) Check logs
# - Selected chunk IDs + scores
# - Context tokens used
# - Final prompt (with context block)
# - Same output on rerun
```

## Notes / assumptions
- Embeddings: CPU-safe MiniLM keeps demo runnable on dev laptops.
- Determinism: set `LLM_DETERMINISTIC=1` and `LLM_SEED=42` for vLLM; ingest script fixes random seeds.
- Fallback: if Qdrant is unavailable, ingest writes a single mock Arrow batch to disk and the selector consumes it; still proves determinism.
- AI-2027 snapshot (optional): drop a local PDF/HTML into `blueprint/docs/assets/ai-2027` and include that path in `--src` to let the RAG demo answer safety/strategy questions from the curated corpus.

## Acceptance for this track
- Deterministic rerun: identical selected chunks + tokens + model output.
- Visible evidence: log snippet or saved JSON showing context selection and prompt.
- No network keys required; cloud/council path must be disabled.
