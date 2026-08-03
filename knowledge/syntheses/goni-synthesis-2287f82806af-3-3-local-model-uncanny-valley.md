---
id: GONI-SYNTHESIS-2287F82806AF
title: 3.3 Local model uncanny valley
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Small local models: are great at reading, clustering, summarising, are still shaky at perfect human-like writing and complex reasoning.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 3.3 Local model uncanny valley
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3.3 Local model uncanny valley

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Local model uncanny valley

- Small local models:
  - are great at reading, clustering, summarising,
  - are still shaky at perfect human-like writing and complex reasoning.
- If local drafts feel like “AI slop”, users lose trust quickly.

**Risk:**  
If quality isn’t good enough, people will fall back to ChatGPT-in-browser and ignore Goni’s drafting.

**Mitigation path:**

- **Honest tiering:**  
  - Local = read, organise, detect, rank.  
  - Cloud (via anonymised requests) = final polish + doppelgänger voice.
- **Fine-tuning + style prompts** on local corpora for common patterns (short replies, standard templates).
- **User-friendly routing settings:**  
  - let users choose where they insist on SOTA vs where “good enough + private” is fine.

---
