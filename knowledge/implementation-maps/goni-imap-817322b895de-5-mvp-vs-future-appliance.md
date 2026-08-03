---
id: GONI-IMAP-817322B895DE
title: 5. MVP vs future “appliance”
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**MVP / prototype** Run on any modern 64-bit Linux.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/os-and-base-image.md
  heading: 5. MVP vs future “appliance”
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 5. MVP vs future “appliance”

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. MVP vs future “appliance”

**MVP / prototype**

- Run on any modern 64-bit Linux.
- Dependencies (Rust toolchain, model runtimes) installed manually or via container.
- No opinionated disk layout beyond “persistent dirs exist”.

**Future appliance**

- Curated base image(s) and hardware targets.
- Encrypted volumes for models/data.
- Preconfigured systemd units / k8s manifests with health checks and auto-update strategy.

Those details live in blueprint/hardware/ and infra repos; this file records the *software-visible OS assumptions* only.
