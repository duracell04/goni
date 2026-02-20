# Trust Model
Status: Specified only / roadmap

This document summarizes the system trust model for Goni as a local-first
agentic kernel.

Normative contracts still live in:
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/isolation-and-tool-sandboxes.md`
- `blueprint/30-specs/network-gate-and-anonymity.md`
- `blueprint/30-specs/receipts.md`

## 1. Security-kernel framing

Goni is modeled as a reference-monitor style kernel for agent actions:
- every effectful action must be mediated,
- untrusted components must not bypass policy,
- the trusted core must stay small enough to test and audit.

This is design intent for the blueprint stage.

## 2. Trusted computing base (TCB) scope

Current intended TCB components:
- capability and policy decision path,
- tool mediation boundary,
- egress gate,
- receipt integrity path (hash chain and verification),
- minimal scheduler controls tied to mediation guarantees.

Everything else is outside the TCB by default.

## 3. Trust boundaries

Untrusted by default:
- model output text,
- retrieved external text,
- third-party tools and extensions,
- connector responses from remote services.

Trusted only through mediation:
- authority tokens/capabilities,
- policy decisions and budget checks,
- egress approval outcomes,
- receipt emission and verification.

## 4. Required security properties

- Complete mediation: no side effect without policy check.
- Least privilege: capabilities are scoped, attenuable, and expire.
- Confinement by default: no ambient network authority.
- Auditability: mediated actions emit verifiable receipts.
- Revocability: authority and extension trust can be withdrawn quickly.

## 5. Explicit assumptions

- Host OS primitives (namespaces/cgroups/seccomp/capabilities or equivalent)
  are available and correctly configured.
- Only approved egress path is reachable from runtime components.
- Time source and key material are sufficient for receipt integrity checks.

If these assumptions fail, guarantees degrade to best effort.

## 6. Non-goals (for this layer)

- This document does not define contributor governance roles.
- This document does not replace threat modeling details in
  `blueprint/docs/threat-model.md`.
- This document does not assert implemented enforcement without evidence.
