---
id: GONI-SPEC-14E53A20E861
title: 5. Example syscalls (non-exhaustive)
type: specification
status: draft
implementation_state: specified_only
proposition: fs.read(path, cap) fs.write(path, bytes, cap) net.egress(route, purpose, payload_ref, cap) vecdb.query(embedding, filters, cap) calendar.find(range, cap) email.draft(to, subject, body, cap)
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 5. Example syscalls (non-exhaustive)
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 5. Example syscalls (non-exhaustive)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Example syscalls (non-exhaustive)

- `fs.read(path, cap)`
- `fs.write(path, bytes, cap)`
- `net.egress(route, purpose, payload_ref, cap)`
- `vecdb.query(embedding, filters, cap)`
- `calendar.find(range, cap)`
- `email.draft(to, subject, body, cap)`
