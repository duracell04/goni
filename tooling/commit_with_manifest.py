#!/usr/bin/env python3
"""Create a COMMIT_STANDARD commit with exact staged-file accounting."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


def run(repo: Path, *args: str, input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", "-c", "safe.directory=*", "-C", str(repo), *args],
        input=input_text,
        check=True,
        text=True,
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--subject", required=True)
    parser.add_argument("--intent", required=True)
    parser.add_argument("--rationale", required=True)
    parser.add_argument("--status", choices=("specified-only", "implemented-untested", "implemented-and-tested"), required=True)
    parser.add_argument("--contract", action="append", default=[])
    parser.add_argument("--evidence", action="append", default=[])
    parser.add_argument("--unresolved", action="append", default=[])
    args = parser.parse_args()
    repo = args.repo.resolve()
    files_result = subprocess.run(
        ["git", "-c", "safe.directory=*", "-C", str(repo), "diff", "--cached", "--name-only"],
        check=True,
        stdout=subprocess.PIPE,
        text=True,
        encoding="utf-8",
    )
    files = sorted(line for line in files_result.stdout.splitlines() if line)
    if not files:
        parser.error("the index contains no changed files")

    def bullets(values: list[str]) -> str:
        return "\n".join(f"- {value}" for value in (values or ["none"]))

    message = (
        f"{args.subject}\n\n"
        f"Intent:\n{args.intent}\n\n"
        f"Rationale:\n{args.rationale}\n\n"
        f"Status:\n{args.status}\n\n"
        f"Contracts:\n{bullets(args.contract)}\n\n"
        f"Files:\n{bullets(files)}\n\n"
        f"Evidence:\n{bullets(args.evidence)}\n\n"
        f"Unresolved:\n{bullets(args.unresolved)}\n"
    )
    run(repo, "commit", "-F", "-", input_text=message)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
