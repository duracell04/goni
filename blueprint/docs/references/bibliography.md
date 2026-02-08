# Bibliography (annotated)

Key: [[liu2023-lost-middle]]
Claim: Long-context LMs show positional sensitivity; evidence in the middle is
used less reliably than evidence near prompt boundaries.
Relevance:
- Motivates bounded, curated context projection instead of transcript growth.
- Supports the TXT axiom and plane separation for context discipline.
Used in:
- `blueprint/software/50-data/10-axioms-and-planes.md` (Empirical motivation)

Key: [[greshake2023-indirect-prompt-injection]]
Claim: Untrusted retrieved text can inject instructions that hijack tool use
and control flow in LLM-integrated systems.
Relevance:
- Motivates separating untrusted text from control/execution planes.
- Supports redaction and minimization before remote escalation.
Used in:
- `blueprint/software/50-data/40-privacy-and-text-confinement.md` (Empirical motivation)

Key: [[tworek2026-decoder]]
Claim: Reported view that deployed models do not robustly learn from mistakes in
normal use, and that this limits autonomous improvement without system support.
Relevance:
- Motivates explicit, system-managed learning layers and failure recovery.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Constraints and rationale)

Key: [[apple2025-illusion-thinking]]
Claim: Evaluation suggests accuracy can collapse as problem complexity grows,
with reasoning effort rising then dropping despite remaining budget.
Relevance:
- Motivates complexity guardrails and unstuck primitives in runtime design.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Engineering constraint)

Key: [[comment2025-illusion-thinking]]
Claim: Critique argues that some collapse results may be artifacts of evaluation
design (token limits, misclassification, or unsatisfiable instances).
Relevance:
- Supports a conservative stance: treat collapse as a risk, validate in-house.
Used in:
- `blueprint/20-system/50-learning-loop.md` (Risk framing)
