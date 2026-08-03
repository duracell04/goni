---
id: GONI-SYNTHESIS-5F678FC72698
title: 1. EXO – “Run your own AI cluster at home”
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Repository:** https://github.com/exo-explore/exo :contentReference[oaicite:0]{index=0} EXO is one of the most visible OSS projects explicitly targeting **“AI cluster at home with everyday devices.”** Its README describes EXO as: “Run your own AI cluster at home with everyday devices… unify your existing devices into one powerful GPU: iPhone, iPad, Android, Mac, NVIDIA, Raspberry Pi, pretty much any device!” :contentReference[oaicite:1]{index=1}'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 1. EXO – “Run your own AI cluster at home”
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. EXO – “Run your own AI cluster at home”

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. EXO – “Run your own AI cluster at home”

**Repository:**  
https://github.com/exo-explore/exo :contentReference[oaicite:0]{index=0}  

EXO is one of the most visible OSS projects explicitly targeting **“AI cluster at home with everyday devices.”** Its README describes EXO as:

> “Run your own AI cluster at home with everyday devices… unify your existing devices into one powerful GPU: iPhone, iPad, Android, Mac, NVIDIA, Raspberry Pi, pretty much any device!” :contentReference[oaicite:1]{index=1}  

Key characteristics (from EXO’s README):

- **Wide model support**  
  EXO supports multiple model families (LLaMA via MLX/tinygrad, Mistral, Llava, Qwen, DeepSeek, etc.).:contentReference[oaicite:2]{index=2}  

- **Dynamic model partitioning**  
  EXO “optimally splits up models based on the current network topology and device resources” so that larger models can be run across multiple devices.:contentReference[oaicite:3]{index=3}  

- **Automatic device discovery & P2P**  
  Devices discover each other automatically (UDP, manual, Tailscale modules), and EXO describes itself as **non–master-worker**: nodes connect peer-to-peer, every device is a “first-class citizen”.:contentReference[oaicite:4]{index=4}  

- **ChatGPT-compatible API**  
  EXO exposes an OpenAI/ChatGPT-compatible API endpoint (`/v1/chat/completions`) and a tinygrad-based Web UI. This makes it a drop-in replacement for model endpoints in existing apps.:contentReference[oaicite:5]{index=5}  

- **Heterogeneous devices**  
  The README explicitly lists configurations where “Raspberry Pi + Mac + laptop” type clusters run models by aggregating memory across devices. It emphasises that devices with weaker hardware can join, with trade-offs between latency and throughput.:contentReference[oaicite:6]{index=6}  

**Relevance to Goni**

EXO is essentially a **distributed inference engine plus API**:

- strong prior art on:
  - automatic discovery,
  - peer-to-peer partitioning,
  - OpenAI-compatible APIs over a heterogeneous cluster.

For Goni, EXO is a **conceptual ancestor**:

- Goni also cares about “mesh of heterogeneous nodes,”
- but Goni’s primary unit is a **strong local appliance**, not “whatever devices are lying around,” and it adds a larger focus on **personal data, UX, and local OS integration**.

---
