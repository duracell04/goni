#!/usr/bin/env bash
set -euo pipefail

echo "docker:" $(command -v docker >/dev/null 2>&1 && echo ok || echo missing)
