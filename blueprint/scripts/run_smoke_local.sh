#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
export LLM_STUB=1
export GONI_RECEIPTS_FILE="$ROOT_DIR/receipts.jsonl"

cd "$ROOT_DIR/software/kernel"
cargo run -p goni-http &
PID=$!

sleep 2
bash "$ROOT_DIR/scripts/smoke_test.sh"

kill $PID
