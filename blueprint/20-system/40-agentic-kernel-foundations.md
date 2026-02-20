# Agentic Kernel Foundations
Status: Specified only / roadmap

This document restates Goni's "agentic kernel" thesis in systems and security
language, grounded in canonical reference-monitor, capability, IFC, provenance,
and LLM-serving literature.

This is an analytical framing document, not a normative contract.

## 1. Threat model (high-level)

Adversarial or unreliable inputs include:
- prompt-injected retrieved text,
- compromised or malicious tool extensions,
- accidental over-privileged connectors,
- model outputs that attempt policy bypass.

Primary risk classes:
- unauthorized side effects,
- silent data exfiltration,
- untraceable actions,
- degraded responsiveness from memory or scheduler contention.

## 2. Kernel properties required

The trusted core is modeled as a reference-monitor style mechanism with three
required properties:
- always invoked (complete mediation),
- tamper resistant,
- small enough to analyze and validate.

This aligns with reference-monitor requirements and design principles in
classical OS security work [[anderson1972-reference-monitor]]
[[saltzer1975-protection]].

## 3. Mechanisms (minimal viable set)

### 3.1 Mediation at tool syscall boundary

Every effectful tool operation must pass the same mediation choke point:
- capability check,
- policy and budget decision,
- egress classification (if applicable),
- receipt emission.

Related foundations:
- reference monitor formulation [[anderson1972-reference-monitor]]
- protection model framing [[lampson1974-protection]].

### 3.2 Capability-based authority, not ambient authority

Authority should be explicit and attenuable:
- capability tokens encode scope, constraints, and expiry,
- delegation attenuates rather than amplifies authority,
- revocation and expiry are first-class operational controls.

Related foundations:
- least privilege and protection principles [[saltzer1975-protection]]
- practical capability mode in UNIX context [[watson2010-capsicum]]
- object-capability delegation reasoning [[miller2003-capability-myths]]
- hardware capability line for future hardening [[watson2014-cheri]].

### 3.3 Information-flow constrained egress

Default runtime posture is confinement:
- tool sandboxes run without ambient egress,
- policy-governed declassification is required for outbound flow,
- data classes drive redaction and release decisions.

Related foundations:
- OS-level IFC with minimal trusted core [[zeldovich2006-histar]]
- IFC integrated with standard OS abstractions [[krohn2007-flume]].

### 3.4 Provenance-by-construction ("receipts")

Effectful actions should emit mandatory structured provenance:
- entities, activities, and agents for each mediated step,
- causal linkage for inputs, decisions, and outputs,
- trace propagation for cross-component correlation.

Related foundations:
- W3C PROV family [[w3c2013-prov]]
- W3C Trace Context [[w3c2021-trace-context]].

### 3.5 Cognitive resource scheduling (KV cache as memory object)

Interactive agent scheduling must include LLM-serving memory realities:
- KV cache residency and fragmentation affect tail latency,
- scheduler decisions should include memory residency and bandwidth,
- eviction/admission policy must be explicit under mixed workloads.

Related foundations:
- PagedAttention and memory-management analysis in vLLM
  [[kwon2023-vllm]].

## 4. Evaluation protocol suggestions

The following test classes operationalize the thesis:

1. Mediation completeness tests
- prove no side effect can occur without policy path invocation.

2. Bypass resistance tests
- deny direct socket/file escape attempts from tool runtime.

3. Capability attenuation tests
- verify delegated authority cannot exceed parent grant.

4. Provenance completeness tests
- verify every mediated action has a receipt with linked decision metadata.

5. Egress and IFC tests
- verify declassification/egress policy behavior by data class.

6. Scheduler stress tests
- mixed interactive/background runs with KV pressure and tail-latency checks.

7. TCB minimization tracking
- keep a versioned map of trusted modules and review every expansion.

## 5. Relation to Goni contracts

This document informs (but does not replace):
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/isolation-and-tool-sandboxes.md`
- `blueprint/30-specs/network-gate-and-anonymity.md`
- `blueprint/30-specs/receipts.md`
- `blueprint/30-specs/scheduler-and-interrupts.md`

Related system note:
- `blueprint/20-system/45-kernel-blockchain-mapping.md`

## 6. Citation keys

See `blueprint/docs/references/bibliography.md` for annotated references:
- `[[anderson1972-reference-monitor]]`
- `[[saltzer1975-protection]]`
- `[[lampson1974-protection]]`
- `[[watson2010-capsicum]]`
- `[[miller2003-capability-myths]]`
- `[[watson2014-cheri]]`
- `[[zeldovich2006-histar]]`
- `[[krohn2007-flume]]`
- `[[kwon2023-vllm]]`
- `[[w3c2013-prov]]`
- `[[w3c2021-trace-context]]`
- `[[klein2009-sel4]]`
