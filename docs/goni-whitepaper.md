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

## Latent prediction (JEPA / VL-JEPA influence)

Goni is infrastructure-first and model-agnostic, but it is informed by a simple cognitive stance:

**Understanding lives in latent state; language is an optional interface.**

Practically, this means Goni prefers to:

- form **compact latent representations** of the current situation (context, goals, constraints),
- update those representations by **predicting in latent space** (what matters next, what is missing, what is likely), and
- only invoke a **decoder / verbaliser** when words are needed (explanations, drafts, chat output).

This framing is inspired by Joint Embedding Predictive Architectures (JEPA) and multimodal variants (VL-JEPA): rather than optimizing for pixel-level reconstruction or token-level next-word prediction, the system learns or maintains a latent "world state" and predicts representations of targets or future states.

Goni does not require a single training objective to be "the truth". Instead, this influence is captured as an interface and execution pattern:

- **Encoders** map observations to latent state.
- A **Predictor** performs latent updates and tool routing.
- An **Optional Decoder** produces language as a downstream view.

## Local OS agents vs cloud agents

Cloud agents are typically remote workflows that loop over LLM tokens and
external tools. Goni OS agents are **local processes** running under a kernel
that owns state, policy, and budgets. The LLM is a **rare, budgeted interrupt**
used for ambiguity resolution or human-facing output, not the control loop.

This is infrastructure-first and model-agnostic: JEPA-style latent prediction is
compatible inspiration, not a mandatory training objective.

## Latent-first as power and thermal strategy

Latent-first cognition is a **power/thermal constraint**, not only a modeling
preference. Always-on encoders + predictor can remain in a low-power envelope,
while decoder/LLM paths are invoked only when a real decision branch exists.
This keeps continuous cognition feasible on local hardware.

## Local vs cloud efficiency (IPW)

Saad-Falcon et al. (2025) define **Intelligence per Watt (IPW)** and publish a harness that measures accuracy, energy, and latency per query across model × hardware pairs. We adopt IPW as the way to argue "local-first": for any workload we compare local IPW to cloud baselines and treat hybrid routing savings as a design constraint, not an optimisation afterthought.

## Context and memory management (MemGPT)

MemGPT (Packer et al., 2023) frames LLM context as **virtual memory**: prompt window ≈ RAM, external stores ≈ Disk, with explicit paging calls. Goni internalises this in the planes: Memory/Context planes page Arrow/ANN/graph stores into context, LLM engines remain stateless, and paging/syscalls (e.g. `MEM_READ`, `MEM_WRITE`) are first-class kernel APIs rather than prompt tricks.

## Memory geometry and planes

Explosive Neural Networks (Aguilera et al., 2025) shows high-order interactions on curved manifolds improve memory capacity/recall. We treat this as theoretical backing for **decoupling memory geometry from reasoning**: the Memory Plane can use non-Euclidean/graph indexes layered on the Arrow spine without forcing the same geometry on the reasoning engines.

## Observation → Reflection → Planning

Generative Agents (Park et al., 2023) demonstrates that long-horizon coherence requires an explicit Observation–Reflection–Planning loop. Goni encodes this as a Control/Memory-plane obligation: raw events land in the Arrow spine (Observation), periodic jobs distill reflections/long-term facts (Reflection), and planners use both current state and reflections to schedule actions and suggestions (Planning).

## Cognitive offloading stance

Risko & Gilbert (2016) warn that uncritical **cognitive offloading** erodes skills and understanding. Goni’s UX defaults bias toward **Socratic mode**: surfacing retrieved memories, asking for confirmation on creative/learning tasks, and making provenance/attribution visible so the user stays in the loop instead of outsourcing everything.

## Determinism and self-loop stability

Autoregressive self-loops are chaotic: tiny numeric noise can flip token choices over long runs. For regulated or auditable workflows, Goni exposes a **deterministic preset** (temperature 0, fixed seed where supported, batch size 1, single worker/thread or CPU, TF32 off on NVIDIA, deterministic backend flag such as vLLM `--enable-deterministic-inference`). Hardware/driver hashes are logged with deterministic runs so trajectories can be reproduced, even if throughput is lower.
