# OS Completeness Checklist (Goni OS)

This list keeps Goni from becoming "kernel-only" without the product surfaces
users need to trust and operate the system.

---

## Minimum required surfaces

- **Settings**: policy defaults, privacy, remote access, backup, updates.
- **Dashboard**: current state, running agents, interrupts, budgets, health.
- **App/Agent manager**: install, remove, permissions, logs, versions.
- **Identity**: local accounts, SSO tokens, session visibility.
- **Observability**: resource usage, storage health, write budgets, wake counts.

---

## Exit criteria for "complete enough" MVP

- Every capability requested by an agent is visible to the user.
- Remote access status is clear and can be revoked in one place.
- System health and storage risk are legible without a terminal.
- Agents can be installed, updated, disabled, and audited without SSH.
