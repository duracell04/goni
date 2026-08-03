---
id: GONI-SYNTHESIS-FB2E233C74F9
title: 5. Beowulf AI Cluster – Ansible-powered AI cluster harness
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Repository:** https://github.com/geerlingguy/beowulf-ai-cluster :contentReference[oaicite:23]{index=23} Beowulf AI Cluster is an **Ansible project** for deploying AI workloads and benchmarks across “random computers with random capabilities.” From the README:​:contentReference[oaicite:24]{index=24} “Beowulf AI Cluster… an AI cluster deployed with Ansible on random computers with random capabilities.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/related-projects.md
  heading: 5. Beowulf AI Cluster – Ansible-powered AI cluster harness
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 5. Beowulf AI Cluster – Ansible-powered AI cluster harness

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
