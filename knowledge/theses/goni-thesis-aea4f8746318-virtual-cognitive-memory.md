---
id: GONI-THESIS-AEA4F8746318
title: Virtual Cognitive Memory
type: thesis
status: draft
implementation_state: specified_only
proposition: A sovereign personal AI should treat active context as a bounded cache over a larger recoverable cognitive state rather than as the memory system itself.
domains:
- memory
- architecture
aliases: []
relations:
- type: refines
  target: GONI-THESIS-B09E3475FE2C
- type: depends_on
  target: GONI-SYNTHESIS-2F274E6CBCF8
sources:
- SRC-PACKER2023-MEMGPT
- SRC-KWON2023-PAGEDATTENTION
artifacts: []
uncertainty: The proposition is architectural and specified-only. The virtual-memory analogy is a design abstraction; empirical benefit for GONI must be established by evaluation.
legacy: []
---

# Virtual Cognitive Memory

GONI distinguishes the model's currently active context from the larger state the system can recover. At time `t`, let:

`C_t ⊆ A_t ⊆ R_t`

where `C_t` is active context, `A_t` is addressable memory available to retrieval, and `R_t` is recoverable state, including governed memory plus authoritative external state such as files, Git, databases, source artifacts, and receipts.

The architectural implication is that context-window pressure should ordinarily be handled as a residency problem rather than as an instruction to destroy state. Information may leave active context while remaining recoverable. A later need for that information should therefore trigger retrieval or rehydration before the system substitutes a confident reconstruction.

This thesis extends the existing continuity-layer thesis without replacing GONI's semantic memory taxonomy. Working, semantic, episodic, procedural, and evidence-bearing state describe what a memory item is. Residency describes where and how readily it can be recovered.

The design objective is not maximal retained context. It is high task-relevant recoverability under bounded latency, compute, and context budgets. A compact system should therefore optimize the probability of correct future action given active context, recoverable memory, and authoritative state rather than optimize token retention in isolation.
