---
id: GONI-SYNTHESIS-AF00EE33D7AF
title: 4. prima.cpp – distributed llama.cpp for home clusters
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Repository:** https://gitee.com/eopsu/prima.cpp :contentReference[oaicite:17]{index=17} The README: “prima.cpp is a **distributed implementation of llama.cpp** that lets you **run 70B-level LLMs on your everyday devices**—laptops, desktops, phones, and tablets (GPU or no GPU, it’s all good).”:contentReference[oaicite:18]{index=18}'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 4. prima.cpp – distributed llama.cpp for home clusters
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4. prima.cpp – distributed llama.cpp for home clusters

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. prima.cpp – distributed llama.cpp for home clusters

**Repository:**  
https://gitee.com/eopsu/prima.cpp :contentReference[oaicite:17]{index=17}  

The README:

> “prima.cpp is a **distributed implementation of llama.cpp** that lets you **run 70B-level LLMs on your everyday devices**—laptops, desktops, phones, and tablets (GPU or no GPU, it’s all good).”:contentReference[oaicite:18]{index=18}  

Prima.cpp has both:

- a **production codebase** (C++/C around llama.cpp), and  
- an **academic paper** describing its scheduling algorithms and benchmarks.  

From the paper (“PRIMA.CPP: Speeding Up 70B-Scale LLM Inference on Heterogeneous and Low-Resource Home Clusters”):  

- It is a **distributed on-device inference system** that runs **30–70B LLMs** on consumer home clusters with:
  - mixed CPU/GPU capabilities,
  - insufficient RAM/VRAM per device,
  - slow disks,
  - Wi-Fi links,
  - heterogeneous OSs.

- It introduces:
  - **PRP** (pipelined-ring parallelism) with prefetching to hide disk IO,  
  - **Halda**, an optimisation-based layer partitioning algorithm that models compute, communication, memory, and disk, and chooses which devices to use and how to assign layers.  

- Experiments compare prima.cpp with llama.cpp, EXO, and dllama:  
  prima.cpp achieves **5–17× lower TPOT (time per output token)** for large models on home clusters, while keeping memory pressure low enough that user apps remain responsive.  

**Why it is important prior art:**

- It **directly tackles Goni-like constraints** (home devices, low RAM/VRAM, Wi-Fi, heterogeneity) but for distributed inference only.
- It more or less formalises “home cluster LLM inference” as a research problem, with a proper scheduler and performance model.

**Relevance to Goni**

Goni is **not** trying to replicate prima.cpp’s distributed scheduling in v1, but:

- if you ever want to run **one big model across multiple Goni nodes**, prima.cpp’s ideas (PRP, Halda) are the strongest reference for:
  - how to partition layers,
  - how to factor in Wi-Fi and disk bandwidth,
  - how to keep user experience (other apps) unaffected.

Prima.cpp plus llama.cpp define a “research baseline” for distributed LLM inference in home clusters that Goni should acknowledge.

---
