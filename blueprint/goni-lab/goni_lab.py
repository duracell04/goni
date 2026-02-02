#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import sys
from datetime import datetime, timezone


def load_scenario(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def synthetic_metrics(scenario_name):
    h = hashlib.sha256(scenario_name.encode("utf-8")).digest()
    base = h[0] % 50 + 50
    return {
        "ttft_p50_ms": base,
        "ttft_p95_ms": base + 40,
        "ttft_p99_ms": base + 80,
        "cancel_p95_ms": base + 20,
        "cancel_p99_ms": base + 60,
    }


def run_bench(args):
    scenario = load_scenario(args.scenario)
    name = scenario.get("name", os.path.basename(args.scenario))
    metrics = synthetic_metrics(name)
    out = {
        "scenario": name,
        "mode": "synthetic",
        "metrics": metrics,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }
    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            json.dump(out, f, indent=2)
    print(json.dumps(out, indent=2))
    return 0


def run_conformance(args):
    out = {
        "status": "skipped",
        "reason": "conformance harness is a scaffold",
    }
    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            json.dump(out, f, indent=2)
    print(json.dumps(out, indent=2))
    return 0


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)

    bench = sub.add_parser("bench")
    bench.add_argument("--scenario", required=True)
    bench.add_argument("--out", required=False)
    bench.set_defaults(func=run_bench)

    conf = sub.add_parser("conformance")
    conf.add_argument("--out", required=False)
    conf.set_defaults(func=run_conformance)

    args = ap.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
