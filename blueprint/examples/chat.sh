#!/usr/bin/env bash
set -euo pipefail
curl -s http://localhost:7000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"local","messages":[{"role":"user","content":"hello"}],"max_tokens":16}'
