# Goni Whitepaper

Human-readable description of the Goni concept.

> **Recommended reading order**
>
> - Start with [`goni-story.md`](./goni-story.md) if you want the high-level narrative.
> - Read this whitepaper (`goni-whitepaper.md`) for the kernel/planes/Arrow details.
> - Check [`goni-swot.md`](./goni-swot.md) for the market / positioning view.
> - See [`related-projects.md`](./related-projects.md) for prior art and neighbors.

## Related work & inspiration

Goni does not exist in a vacuum. It is heavily informed by:

- **Existing systems and research** on home AI clusters, distributed inference, and local LLM infrastructure.
- **Builders and thinkers** who are pushing the "OwnYourAI", sovereign infra, and practical agent workflows forward.

For a structured overview, see:

- [`related-projects.md`](./related-projects.md)  
  Survey of systems like EXO, MultiCortex EXO, Cake, prima.cpp, Beowulf AI Cluster, and llama.cpp - i.e. the technical prior art around home clusters and distributed inference.

- [`inspiration.md`](./inspiration.md)  
  A "people map" of OwnYourAI / sovereign-infra builders and agent/content-automation builders whose work shapes Goni's hardware, software, and narrative.

Both documents are maintained as **living references** and should be consulted when:

- evaluating new architectural ideas (to avoid reinventing solved problems), or  
- framing Goni for different audiences (engineers, infra people, founders, regulated-sector clients).

## Determinism and self-loop stability

Autoregressive self-loops are chaotic: tiny numeric noise can flip token choices over long runs. For regulated or auditable workflows, Goni exposes a **deterministic preset** (temperature 0, fixed seed where supported, batch size 1, single worker/thread or CPU, TF32 off on NVIDIA, deterministic backend flag such as vLLM `--enable-deterministic-inference`). Hardware/driver hashes are logged with deterministic runs so trajectories can be reproduced, even if throughput is lower.
