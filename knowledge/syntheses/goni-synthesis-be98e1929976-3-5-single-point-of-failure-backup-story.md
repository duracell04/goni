---
id: GONI-SYNTHESIS-BE98E1929976
title: 3.5 Single point of failure & backup story
type: synthesis
status: draft
implementation_state: specified_only
proposition: One box = one SSD, one PSU, one LAN node.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 3.5 Single point of failure & backup story
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3.5 Single point of failure & backup story

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 Single point of failure & backup story

- One box = one SSD, one PSU, one LAN node.
- Hardware can die, be stolen, or be misconfigured.

**Risk:**  
If it’s not clear that data is safely backed up and restorable, high-value users will not entrust their “second brain” to it.

**Mitigation path:**

- **Built-in backup module:**
  - encrypted backups to user’s existing NAS or cloud (Backblaze, S3, etc.).
- **Encrypted export format:**
  - “Goni snapshot” bundles that contain data + config.
- **One-click restore** flow to new hardware.

---
