---
id: GONI-SYNTHESIS-B9CF49AAF255
title: 3. Cake – Rust + Candle distributed inference
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Repository:** https://github.com/evilsocket/cake :contentReference[oaicite:11]{index=11} Cake is a **Rust framework** for distributed inference of large models (LLMs and Stable Diffusion) based on the **Candle** tensor library.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 3. Cake – Rust + Candle distributed inference
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Cake – Rust + Candle distributed inference

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Cake – Rust + Candle distributed inference

**Repository:**  
https://github.com/evilsocket/cake :contentReference[oaicite:11]{index=11}  

Cake is a **Rust framework** for distributed inference of large models (LLMs and Stable Diffusion) based on the **Candle** tensor library.

From the README:

> “`Cake` is a Rust framework for distributed inference of large models like LLaMA3 and Stable Diffusion based on Candle. The goal of the project is being able to run big (70B+) models by repurposing consumer hardware into an heterogeneous cluster of iOS, Android, macOS, Linux and Windows devices, effectively leveraging planned obsolescence as a tool to make AI more accessible and democratic.” :contentReference[oaicite:12]{index=12}  

Key characteristics:

- **Rust + Candle**  
  Cake is written in Rust and uses Candle as the core ML backend.:contentReference[oaicite:13]{index=13}  

- **Distributed LLM and SD inference**  
  It supports:
  - distributed LLaMA-family models (via “sharding” transformer blocks across workers),  
  - distributed Stable Diffusion by assigning components (UNet, VAE, CLIP) to different workers.:contentReference[oaicite:14]{index=14}  

- **Topology-driven layer partitioning**  
  A `topology.yml` file maps model layers or components to specific devices (`linux_server_1`, `iphone`, `ipad`, etc.). Example in README shows layers of a transformer split across GPUs and mobile devices.:contentReference[oaicite:15]{index=15}  

- **OpenAI-compatible API**  
  The “master” node exposes an OpenAI-compatible REST API for chat completions and an image endpoint for SD image generation.:contentReference[oaicite:16]{index=16}  

**Relevance to Goni**

Cake is very close to EXO in **intent**, but:

- uses **Rust/Candle** instead of Python/MLX/tinygrad,  
- expects more manual topology specification (though still automatable),  
- has strong emphasis on using **mobile devices and legacy hardware**.

For Goni:

- Cake is a good **reference for Rust-based distributed inference** and for how to expose a cluster as a single OpenAI-compatible API.  
- Goni, however, is *not* intended to orchestrate phones and tablets directly at MVP stage; it starts with one capable appliance and optional additional nodes.

---
