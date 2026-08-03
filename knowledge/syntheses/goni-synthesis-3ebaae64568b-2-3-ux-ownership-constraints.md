---
id: GONI-SYNTHESIS-3EBAAE64568B
title: 2.3 UX & ownership constraints
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Local-first and **offline-capable**: box must still be useful with no internet.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 2.3 UX & ownership constraints
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 2.3 UX & ownership constraints

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.3 UX & ownership constraints

- Local-first and **offline-capable**: box must still be useful with no internet.  
- **Full ownership**:
  - User owns the hardware, the base OS image, and their models/checkpoints.  
  - Telemetry is opt-in, not built-in.

- **Friendly setup**:
  - First boot: Goni appears as `GONI-SETUP` Wi-Fi or at a local URL (e.g. `https://goni.local`).  
  - Wizard: admin password, network config, optional WireGuard key, optional disk encryption.

---
