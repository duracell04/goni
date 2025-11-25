# Related Projects – Home AI Clusters and Distributed Inference

> This document summarises projects that are **closest in spirit to Goni**:  
> running large models on **heterogeneous consumer hardware** at home, and/or  
> turning random devices into an **AI cluster**.

The aim is:

- to give engineers and stakeholders a **shared map of prior art**,  
- to be clear about **what problems are already well explored**,  
- and to clarify **how Goni’s scope differs** (personal AI appliance + mesh, not just raw distributed inference).

We focus on primary sources: official GitHub repositories and authors’ papers.

---

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

## 2. MultiCortex EXO – bootable EXO cluster OS

**Repository:**  
https://github.com/cabelo/multicortex-exo :contentReference[oaicite:7]{index=7}  

MultiCortex EXO packages EXO into a **bootable Linux image** so that any x86_64 computer can become a node in an EXO cluster by booting from USB.

From the repository README:

> “MultiCortex EXO is a portable system that can be booted from a USB flash drive, with the fantastic EXO project pre-installed… It allows any computer to become a node for creating a decentralized AI framework. It allows pooling of computing power from multiple devices, leveraging CPUs, GPUs, NPUs, and other accelerators.” :contentReference[oaicite:8]{index=8}  

Additional points:

- Built on **openSUSE JeOS** (JeOS image) and the openSUSE for Innovators initiative.:contentReference[oaicite:9]{index=9}  
- The README emphasises **privacy and local control**, and the goal of letting non-Linux experts spin up an EXO cluster easily.:contentReference[oaicite:10]{index=10}  

**Relevance to Goni**

MultiCortex EXO shows one path to:

- “**cluster-on-a-stick**” experience: plug USB → boot into EXO cluster node,
- minimal onboarding for non-technical users to join a distributed AI framework.

Goni does something similar in spirit (appliance that “just joins the mesh”), but:

- it controls the **hardware and OS image** rather than relying on generic PCs,  
- and focuses on **one polished node experience** rather than designing a universal live-USB.

---

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

## 5. Beowulf AI Cluster – Ansible-powered AI cluster harness

**Repository:**  
https://github.com/geerlingguy/beowulf-ai-cluster :contentReference[oaicite:23]{index=23}  

Beowulf AI Cluster is an **Ansible project** for deploying AI workloads and benchmarks across “random computers with random capabilities.”

From the README:​:contentReference[oaicite:24]{index=24}  

> “Beowulf AI Cluster… an AI cluster deployed with Ansible on random computers with random capabilities. The project can test various distributed AI clustering tools on various clusters.”

Key properties:

- **Ansible-based provisioning**  
  One main playbook (`main.yml`) with two plays:
  1. Setup – downloads and compiles all code required to run an AI model.
  2. Benchmark – runs AI benchmarks and prints results.:contentReference[oaicite:25]{index=25}  

- **llama.cpp and distributed-llama integration**  
  The repo includes roles and tags to:
  - build and benchmark **llama.cpp** on individual nodes and clusters (RPC mode),  
  - run **distributed-llama** benchmarks across the cluster.:contentReference[oaicite:26]{index=26}  

- **Manual EXO benchmarking support**  
  There is an EXO section that sets up EXO, with manual steps to launch it on each node and run tests. The README notes that EXO’s development appears to have slowed, so EXO testing is manual rather than fully automated.:contentReference[oaicite:27]{index=27}  

- **Real cluster tests**  
  The author, Jeff Geerling, uses this project with:
  - clusters of Framework Desktop mainboards,  
  - various AMD/NVIDIA GPUs,  
  - Raspberry Pi nodes and other devices.:contentReference[oaicite:28]{index=28}  

**Relevance to Goni**

Beowulf is **not a model runtime**; it is an **automation harness**:

- It shows how to:
  - provision a cluster in a repeatable way on heterogeneous hardware,  
  - run consistent benchmarks against multiple distributed inference libraries.

For Goni, Beowulf is a good reference for:

- how to **set up multi-node environments** (using tools like Ansible),
- how to define **repeatable performance tests** for LLM inference on your mesh.

---

## 6. llama.cpp – Single-node inference baseline

**Repository:**  
https://github.com/ggml-org/llama.cpp :contentReference[oaicite:29]{index=29}  

llama.cpp is the upstream project many of these systems build upon.

From the README:

> “The main goal of `llama.cpp` is to enable LLM inference with minimal setup and state-of-the-art performance on a wide range of hardware – locally and in the cloud.” :contentReference[oaicite:30]{index=30}  

Key features relevant here:

- Pure C/C++ implementation, MIT-licensed.  
- Supports:
  - Apple silicon (Metal), x86 with AVX/AVX2/AVX-512/AMX, RISC-V, and various GPU backends (CUDA, HIP, Vulkan, SYCL).:contentReference[oaicite:31]{index=31}  
- Implements many quantisation schemes (1.5–8 bits) to fit large models into modest memory.:contentReference[oaicite:32]{index=32}  
- Includes a **server mode** (`llama-server`) exposing an OpenAI-compatible HTTP API.:contentReference[oaicite:33]{index=33}  

EXO, prima.cpp, distributed-llama and others all use llama.cpp as either:

- the **baseline** for comparison, or  
- the **core inference engine** extended to distributed execution.

For Goni, llama.cpp (or an equivalent runtime like vLLM) is a natural choice for:

- single-node inference baseline,  
- potential multi-node experiments later.

---

## 7. How Goni fits in this landscape

Putting all of this together:

- **EXO / MultiCortex EXO / Cake / prima.cpp** focus on  
  **distributed inference**, with the cluster itself as the primary product:
  - “take all your devices and turn them into one big GPU”.

- **Beowulf AI Cluster** focuses on  
  **deployment and benchmarking**: making it easy to test llama.cpp, EXO, distributed-llama, etc., on “random computers”.

- **llama.cpp** focuses on  
  **maximising single-node inference performance** across as many architectures as possible.

### What Goni shares

Goni’s design clearly shares some DNA:

- with **EXO / Cake / prima.cpp**:
  - cluster of heterogeneous devices,  
  - OpenAI-compatible API,  
  - interest in running >30B models at home.

- with **Beowulf**:
  - desire for repeatable, automated provisioning of nodes,  
  - repeatable benchmarking.

- with **llama.cpp**:
  - focus on running models efficiently on a **single machine** with limited but well-managed resources,  
  - interest in quantisation and mixed CPU/GPU/NPU use.

### What Goni does differently (by design)

The **Goni MVP** concept (as defined elsewhere in this repo) is intentionally narrower and more product-oriented:

- Goni assumes a **strong local node** (small appliance with a high-end APU and 128 GB unified memory),  
  not “any old device you can find”.

- Goni is not primarily a **research platform for arbitrary clusters**; it is a **personal AI appliance** with:
  - local-first assistant,  
  - RAG on personal data,  
  - optional multi-node mesh,  
  - optional integration with heavier nodes (e.g. Grace Blackwell GN100).

- Goni’s success is measured less by “how big a model can we push over Wi-Fi” and more by:
  - **latency and reliability for day-to-day assistant tasks**,  
  - **privacy guarantees**,  
  - **ease of setup and operation** for non-experts.

In that sense, you can think of Goni as:

> sitting one layer **above** EXO / Cake / prima.cpp / llama.cpp,  
> borrowing their insights and sometimes their runtimes,  
> but wrapping them into a **deliberately opinionated, appliance-grade product**.

---

## 8. References

- EXO – “Run your own AI cluster at home with everyday devices”  
  https://github.com/exo-explore/exo :contentReference[oaicite:34]{index=34}  

- MultiCortex EXO – bootable EXO-based AI cluster OS  
  https://github.com/cabelo/multicortex-exo :contentReference[oaicite:35]{index=35}  

- Cake – “Distributed LLM and StableDiffusion inference for mobile, desktop and server”  
  https://github.com/evilsocket/cake :contentReference[oaicite:36]{index=36}  

- prima.cpp – “Speeding up 70B-level LLM inference on low-resource everyday home clusters”  
  Code: https://gitee.com/eopsu/prima.cpp :contentReference[oaicite:37]{index=37}  
  Paper: https://arxiv.org/html/2504.08791v2   

- Beowulf AI Cluster – “AI Cluster deployed with Ansible on random computers with random capabilities”  
  https://github.com/geerlingguy/beowulf-ai-cluster :contentReference[oaicite:39]{index=39}  

- llama.cpp – “LLM inference in C/C++”  
  https://github.com/ggml-org/llama.cpp :contentReference[oaicite:40]{index=40}  
