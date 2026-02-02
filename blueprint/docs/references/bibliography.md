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
