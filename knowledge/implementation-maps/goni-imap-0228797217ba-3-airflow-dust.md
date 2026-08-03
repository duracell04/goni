---
id: GONI-IMAP-0228797217BA
title: 3. Airflow & dust
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Aim for **large, low-RPM fans** and **straight flow paths**.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/30-mechanical/enclosure-notes.md
  heading: 3. Airflow & dust
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Airflow & dust

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Airflow & dust

- Aim for **large, low-RPM fans** and **straight flow paths**.
- Use **removable dust filters** on all intakes.
- Ensure no “hot spot recirculation loops”:
  - PSU should not ingest APU exhaust,
  - NVMe should have at least some directed airflow.

---
