---
id: GONI-SYNTHESIS-7FC5847C3FD5
title: 5.6 Cognitive offloading backlash
type: synthesis
status: draft
implementation_state: specified_only
proposition: Users may start **over-delegating** to Goni for tasks they could do themselves.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 5.6 Cognitive offloading backlash
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 5.6 Cognitive offloading backlash

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.6 Cognitive offloading backlash

- Users may start **over-delegating** to Goni for tasks they could do themselves.
- Cognitive offloading research shows this can **erode skills and judgement** over time ("Google effect").

**Risk:**  
Perception that Goni dulls users' abilities or hides provenance could trigger trust loss or scrutiny.

**Mitigation path:**

- Default to **Socratic mode** on creative/learning flows (ask before fully automating).
- Always surface provenance: what was retrieved, what was generated, which model/route.
- Keep user-in-the-loop gates for irreversible actions; promote automation only after repeated approvals and anomaly checks.

---
