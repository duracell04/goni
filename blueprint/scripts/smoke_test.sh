#!/usr/bin/env bash
set -euo pipefail

BASE_URL=${GONI_ORCH_URL:-http://localhost:7000}
RECEIPT_FILE=${GONI_RECEIPTS_FILE:-./receipts.jsonl}
QDRANT_URL=${QDRANT_HTTP_URL:-http://localhost:6333}

curl -s "$BASE_URL/healthz" >/dev/null
curl -s "$QDRANT_URL/healthz" >/dev/null

curl -s "$BASE_URL/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d '{"model":"local","messages":[{"role":"user","content":"hello"}],"max_tokens":16}' \
  >/dev/null

if [ ! -s "$RECEIPT_FILE" ]; then
  echo "receipt file missing: $RECEIPT_FILE" >&2
  exit 1
fi

echo "smoke test ok"
