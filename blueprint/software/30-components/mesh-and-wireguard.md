# 99 – Mesh and WireGuard

Status: Future – out of scope for MVP  
Scope: Multi-node coordination & secure overlay (design placeholder)

---

## 1. Role in the system (future)

This component will eventually:

- Connect multiple Goni nodes into a secure mesh network,
- Provide encrypted connectivity (WireGuard) between nodes,
- Enable cross-node service discovery and job routing,
- Expose the *illusion of a single Goni* backed by several physical devices.

It is a **purely future concern**; the MVP node is defined to be single-node in 20-architecture and 30-conformance.

---

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

## 3. Non-responsibilities

- ? Intra-node scheduling (?? still runs per node).  
- ? Strong distributed consensus or transactions.  
- ? Making ?? (Arrow data) magically globally consistent; we accept eventual consistency at higher layers.

---

## 4. MVP position

For the MVP / prototype:

- There is **no mesh implementation**.
- All traffic and scheduling remain strictly **local**.
- ??, ??, ??, ?? must *not* assume mesh exists; they only see a single-node environment.

This file exists to document the intended multi-node story and to constrain future design, not to mandate any implementation in v1.



## 5. Upstream
- [OS and base image](/blueprint/software/30-components/os-and-base-image.md)
- [Sync policies](/blueprint/30-specs/sync-policies.md)

## 6. Downstream
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Remote LLM Architecture](/blueprint/software/docs/remote-llm-architecture.md)

## 7. Adjacent
- [Security overview](/blueprint/software/security/00-overview.md)
- [Runtime overview](/blueprint/software/runtime/00-overview.md)

