---
id: GONI-IMAP-DA385C831586
title: 2. Intended responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: When implemented: **Secure overlay** Configure WireGuard tunnels between nodes.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/mesh-and-wireguard.md
  heading: 2. Intended responsibilities
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 2. Intended responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Intended responsibilities

When implemented:

- **Secure overlay**
  - Configure WireGuard tunnels between nodes.
  - Manage keys, rotation, and node admission.

- **Node discovery**
  - Maintain a small registry of reachable nodes and their capabilities.

- **Task routing**
  - Extend JobDescriptor with a 	arget_node: NodeId.
  - Decide whether to execute locally or forward to another node.

---
