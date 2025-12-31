# Olares - Reference Product Pattern (Personal Local Cloud OS)

Pattern: **personal local cloud OS**

Olares matters as a reference product pattern, not a competitor note. It shows what
users respond to when self-hosting is framed as a complete system rather than a
stack of parts.

---

## 1. What Olares proves users want

- A single, legible message: **own your data** with local-first defaults.
- A complete system: built-in apps, settings, dashboard, and SSO, not just a model.
- A credible ecosystem: installable apps, sandboxing, and a clear permissions story.
- Remote access that works: safe defaults, clear status, and simple setup.

---

## 2. What to copy vs avoid

**Copy**

- Sovereignty framing as the north star.
- Product packaging: a complete system, not a model demo.
- App ecosystem with sandboxing and SSO.
- Remote access as a first-class feature.

**Avoid**

- Hosting-first drift (Kubernetes-heavy, general self-hosting OS positioning).
- Becoming a cloud distro instead of a cognition-first OS.

---

## 3. Wedge comparisons (what Goni is and is not)

Goni should explicitly position against:

- NAS + local chatbot with RAG
- LangChain loops run locally
- Self-hosted cloud distro

And lead with:

- Cognitive OS kernel
- Audited syscalls and capability-gated tools
- Agents as local processes with budgets and policy

---

## 4. Implications for Goni OS (requirements + roadmap)

These are enforced as requirements and architecture decisions elsewhere in the repo.
See `software/10-requirements.md`, `software/20-architecture.md`, and
`software/90-decisions.md`.

- Default local-first; network access is an explicit capability with visible policy.
- Agents are installable packages with manifests, permissions, and budgets.
- One identity plane governs UI, agents, and tools (SSO and audit attribution).
- Remote presence is a first-class capability, not an afterthought.
- OS completeness ships early: settings, dashboard, agent manager, identity, and
  observability.
