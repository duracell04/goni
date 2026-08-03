---
id: GONI-IMAP-D34A99498570
title: 3.2 Practical runtime options by tier
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: | Tier | Hardware target | “Works now” runtime path | Notes | | v1 (APU) | Ryzen AI Max+ 395 class | llama.cpp (Vulkan/HIP) or validated ROCm path | vLLM ROCm officially targets specific AMD GPUs; APU validation is a concrete task, not an assumption.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/25-hardware-layers-and-supplier-map.md
  heading: 3.2 Practical runtime options by tier
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 3.2 Practical runtime options by tier

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Practical runtime options by tier

| Tier | Hardware target | “Works now” runtime path | Notes |
|------|------------------|--------------------------|-------|
| v1 (APU) | Ryzen AI Max+ 395 class | llama.cpp (Vulkan/HIP) or validated ROCm path | vLLM ROCm officially targets specific AMD GPUs; APU validation is a concrete task, not an assumption. |
| Pro (NVIDIA) | RTX 4090/5090 class | vLLM (CUDA) | Best current throughput and ecosystem maturity. |
| Max | multi-GPU / mixed | vLLM + custom | Requires more orchestration (sharding, networking, scheduling). |
