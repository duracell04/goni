# Goni OS Learning Path
Status: Specified only / roadmap

This is the recommended learning order for contributors building Goni OS.
The sequence optimizes for execution safety first, then ecosystem scale.

## 1. Linux isolation and kernel primitives

Learn:
- namespaces, cgroups, seccomp, capabilities,
- AppArmor/SELinux basics,
- process and filesystem isolation tradeoffs.

Outcome:
- implement and reason about non-bypassable tool sandboxes.

## 2. Systems programming (Rust-first)

Learn:
- process control, async I/O, structured error handling,
- safe FFI boundaries and serialization choices.

Outcome:
- build reliable kernel services and mediation pipelines.

## 3. Policy engine and capabilities model

Learn:
- deny-by-default policy design,
- capability tokens, expiry, budget constraints,
- policy evaluation patterns (OPA/Rego or equivalent).

Outcome:
- enforce explicit authority for every side effect.

## 4. Receipts, integrity, and cryptographic basics

Learn:
- hashing and chain integrity,
- signatures and key lifecycle basics,
- tamper-evident audit log design.

Outcome:
- produce verifiable receipts with replayable provenance.

## 5. Software supply chain security

Learn:
- reproducible builds,
- signed artifacts,
- SBOM and provenance concepts (for example SLSA levels).

Outcome:
- trusted updates and auditable extension delivery.

## 6. Extension and skill runtime governance

Learn:
- manifest-driven plugin contracts,
- permission scoping and sandbox profiles,
- verification, revocation, and quarantine workflows.

Outcome:
- extension ecosystem without ambient authority.

## 7. Network governance and secret boundaries

Learn:
- egress allowlist design,
- service boundary enforcement,
- secret storage and connector isolation.

Outcome:
- prevent silent exfiltration and uncontrolled remote actions.

## 8. Operations and enterprise trust posture

Learn:
- fleet updates, rollback, incident response,
- observability for policy and runtime events,
- compliance mapping basics (SOC2/ISO-oriented controls).

Outcome:
- convert architecture claims into operational trust.

## Practical rule

If unsure what to learn next, prioritize what improves:
1. non-bypassability,
2. auditability,
3. safe extensibility.
