#!/usr/bin/env bash
set -euo pipefail

OUT_DIR=${1:-demo/output}
mkdir -p "$OUT_DIR"

python goni-lab/goni_lab.py bench --scenario goni-lab/scenarios/mixed.json --out "$OUT_DIR/bench.json"

cat > "$OUT_DIR/action_card.json" <<'EOF'
{
  "card_id": "demo-card-1",
  "title": "Review demo output",
  "status": "proposed",
  "rationale": {"reason": "demo"},
  "receipt_id": "demo-receipt-1"
}
EOF

cat > "$OUT_DIR/receipts.jsonl" <<'EOF'
{"receipt_id":"demo-receipt-1","timestamp":"demo","action_type":"demo","policy_decision":"allow","input_hash":"demo","output_hash":"demo"}
EOF

echo "Demo output written to $OUT_DIR"
