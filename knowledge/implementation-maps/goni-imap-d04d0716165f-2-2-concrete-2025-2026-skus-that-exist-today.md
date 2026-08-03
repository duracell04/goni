---
id: GONI-IMAP-D04D0716165F
title: 2.2 Concrete 2025/2026 SKUs that exist today
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'These are the reference devices/boards that make the APU-centric concept concrete: Product: Framework Desktop Mainboard (AMD Ryzen AI Max 300 series) Config: **Ryzen AI Max+ 395 ???'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 2.2 Concrete 2025/2026 SKUs that exist today
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2.2 Concrete 2025/2026 SKUs that exist today

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Concrete 2025/2026 SKUs that exist today

These are the reference devices/boards that make the APU-centric concept concrete:

#### A1) Framework Desktop Mainboard (primary MVP compute module)

- Product: Framework Desktop Mainboard (AMD Ryzen AI Max 300 series)
- Config: **Ryzen AI Max+ 395 ??? 128 GB**
- Availability snapshot: listed on Framework Marketplace (pre-order batches).
  Source: Framework product page.

Why it matters for Goni:

- sold as a **standalone mainboard**, so we can build our own enclosure/PSU/front panel,
- Mini-ITX-style mounting + standard ATX PSU compatibility makes mechanical integration realistic,
- provides a clean ???board swap??? path for future APU generations.

Links:
- https://frame.work/products/framework-desktop-mainboard-amd-ryzen-ai-max-300-series?v=FRAFMK0002

#### A2) HP Z2 Mini G1a (off-the-shelf fallback box)

- Product: **HP Z2 Mini G1a Workstation**
- Example CH config: Ryzen AI Max+ PRO 395, **128 GB**, 1 TB SSD, Radeon 8060S
- Availability snapshot (HP CH store): listed configuration available.
  Aggregator listings show similar configurations (reseller and config dependent).

Why it matters for Goni:

- immediately usable as a dev/reference machine,
- proves the ???APU + 128 GB unified memory in a tiny box??? thesis,
- but is less modular than a board-based design.

Links:
- https://www.hp.com/ch-en/shop/products/desktops/hp-z2-mini-g1a-workstation-desktop-pc-a40q4et-uuz
- https://www.toppreise.ch/preisvergleich/Server-Workstations/HP-Z2-Mini-G1a-Workstation-AMD-Ryzen-AI-Max-PRO-395-A40Q4ET-p816469

#### A3) GMKtec EVO-X2 (mini-PC yardstick)

- Product: **GMKtec EVO-X2** (Ryzen AI Max+ 395)
- Example CH listing (Digitec): 128 GB / 2 TB class configuration (delivery depends on batch).

Why it matters for Goni:

- useful as an availability yardstick for ???APU box??? market reality,
- less useful as a Goni hardware base because it???s a closed OEM mini-PC.

Links:
- https://de.gmktec.com/en/products/gmktec-evo-x2-amd-ryzen%E2%84%A2-ai-max-395-mini-pc-1
- https://www.digitec.ch/en/s1/product/gmktec-evo-x2-amd-ryzen-ai-max-395-128gb-ram-2tb-ssd-pc-61575547

---
