---
id: GONI-SYNTHESIS-7BC3EA033DB7
title: '6. Local-first shift: hardware and economics signals'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Key drivers (system-level, not vendor-specific): Latency and privacy push personal workloads toward local inference.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: '6. Local-first shift: hardware and economics signals'
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6. Local-first shift: hardware and economics signals

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Local-first shift: hardware and economics signals

Key drivers (system-level, not vendor-specific):

- Latency and privacy push personal workloads toward local inference.
- Memory bandwidth often limits LLM throughput more than peak FLOPS.
- The economic unit shifts from "GPU hour" to "device amortization."
- Cloud remains a logged escalation path for hard or fresh-web tasks.

Vendor spec anchors (illustrative, not requirements):

- Apple MacBook Pro specs: https://www.apple.com/macbook-pro/specs/
- AMD Ryzen AI 300 series: https://www.amd.com/en/partner/articles/ryzen-ai-300-series-processors.html
- NVIDIA Jetson Orin: https://www.nvidia.com/en-us/autonomous-machines/embedded-systems/jetson-orin/

---
