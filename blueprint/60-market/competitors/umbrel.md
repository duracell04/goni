# Umbrel - Competitor Snapshot (Local-First Appliance + App Store)

Last updated: 2026-02-08 (notes-based, verify sources)

Category: home server OS + appliance + app store.

## Why it matters for Goni
- It is a shipped example of a local-first appliance with an app store and one-click UX.
- It validates the "OS is free-ish, appliance is premium" distribution loop.
- It shows how fast app-store ecosystems become governance and security problems.

## Product and distribution model
- umbrelOS: software installable on commodity hardware with one-click OS updates and app installs.
- Hardware SKUs: Umbrel Home and Umbrel Pro as premium appliances.
- Distribution loop: easy software adoption plus paid hardware upsell.

## Ecosystem model
- Official app store plus community app stores.
- UX promise: fast onboarding, one-click installs, and simple updates.

## Licensing posture
- Source-available (PolyForm Noncommercial 1.0.0) with commercial use gated.

## Security and governance lessons
- App marketplaces are the primary supply-chain risk surface.
- Community stores add velocity but widen trust boundaries.

## Goni implications
- Any future "Goni Agent Store" must assume hostile supply chain conditions.
- Required controls include signing, provenance, capability declarations, sandbox mapping,
  receipts on install or run, and quarantine flows for revocations.
- Goni must differentiate on governed agency, not just app convenience.

## Related Goni specs
- [Agent manifest](/blueprint/30-specs/agent-manifest.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Receipts](/blueprint/30-specs/receipts.md)

## Sources (to verify)
- https://umbrel.com/
- https://umbrel.com/umbrelos
- https://umbrel.com/umbrel-home
- https://umbrel.com/umbrel-pro
- https://apps.umbrel.com/
- https://github.com/getumbrel/umbrel
- https://blog.getumbrel.com/everything-you-need-to-know-about-umbrels-new-license-f9807203a57e
- https://community.umbrel.com/t/umbrel-0-5-2-is-here-with-community-app-stores-tor-access-toggle-search-in-app-store-and-more/9981
- https://community.umbrel.com/t/how-insecure-is-umbrel-really/1649
