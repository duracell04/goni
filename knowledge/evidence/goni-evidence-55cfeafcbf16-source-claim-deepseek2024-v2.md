---
id: GONI-EVIDENCE-55CFEAFCBF16
title: 'Source claim: deepseek2024-v2'
type: evidence
status: draft
implementation_state: not_applicable
proposition: DeepSeek-V2 reports that Multi-head Latent Attention compresses the conventional KV-cache representation into a latent representation, demonstrating that KV representation size can be optimized separately from sequence selection.
domains:
- research
- kernel
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-DEEPSEEK2024-V2
artifacts: []
uncertainty: MLA is architecture-specific evidence, not proof that the same representation scheme should be imposed on Goni runtimes or models.
legacy: []
---

# Source claim: deepseek2024-v2

DeepSeek-V2 introduces Multi-head Latent Attention (MLA) as an architectural
method for reducing the memory footprint of the KV cache by compressing the
representation that must be retained for attention.

For Goni, the important systems implication is that KV optimization has at least
two independent axes:

1. **sequence selection** — how many historical states remain active;
2. **representation compression** — how many bytes each retained state requires.

The source supports treating these as distinct experimental variables. Goni
should preserve backend modularity and benchmark architecture-specific
compression rather than making MLA itself a kernel invariant.
