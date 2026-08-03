---
id: GONI-SYNTHESIS-A3E4B16B3C5A
title: 3.4 Sudo fatigue (notification overload)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Proactive agents + strict permissions can → many prompts: “Approve this email?” “Approve this calendar move?” “Approve this archive?” **Risk:** Users either: blindly approve everything (no real safety), or turn off features (no real value).'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 3.4 Sudo fatigue (notification overload)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3.4 Sudo fatigue (notification overload)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Sudo fatigue (notification overload)

- Proactive agents + strict permissions can → many prompts:
  - “Approve this email?”
  - “Approve this calendar move?”
  - “Approve this archive?”

**Risk:**  
Users either:
- blindly approve everything (no real safety), or
- turn off features (no real value).

**Mitigation path:**

- **Trust scores and promotion:**
  - if a pattern has been approved N times, promote it to auto-allowed, with anomaly checks.
- **Bundling decisions:**
  - daily/weekly approval sheets (“Here’s 12 drafts, 4 archivals; approve all/none/per item”).
- **Per-surface tuning:**
  - different interaction volume on mobile vs desktop vs passive surfaces.

---
