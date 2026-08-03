---
id: GONI-SYNTHESIS-2D07E7EB7500
title: 3.1 Connector hell (integrations are brittle)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Goni’s promise depends on: Gmail/Outlook/IMAP connectivity Calendars (Google, iCloud, Exchange) Cloud drives (Drive, OneDrive, iCloud, Dropbox) Banks / fintech APIs / scraped exports Health data (Apple Health, Google Fit, etc.) These APIs: change schemas, get new auth flows, rate limit aggressively, sometimes break without notice.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-swot.md
  heading: 3.1 Connector hell (integrations are brittle)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3.1 Connector hell (integrations are brittle)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Connector hell (integrations are brittle)

- Goni’s promise depends on:
  - Gmail/Outlook/IMAP connectivity
  - Calendars (Google, iCloud, Exchange)
  - Cloud drives (Drive, OneDrive, iCloud, Dropbox)
  - Banks / fintech APIs / scraped exports
  - Health data (Apple Health, Google Fit, etc.)
- These APIs:
  - change schemas,
  - get new auth flows,
  - rate limit aggressively,
  - sometimes break without notice.

**Risk:**  
If connectors are flaky, the user ends up babysitting account connections instead of “touching grass”.

**Mitigation path (design assumptions):**

- **Community driver model** like Home Assistant (integration bundles maintained in-repo).
- **Graceful degradation** – the box still works on what it has; surfaces connector issues clearly.
- **Standardised ingestion formats** – one internal contract per domain (mail, events, transactions), regardless of provider.

---
