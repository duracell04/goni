---
id: GONI-SPEC-1F99F78D930A
title: 6.3 Encryption and Secure Channels
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system should: offer **encrypted storage** for sensitive data (e.g.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 6.3 Encryption and Secure Channels
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 6.3 Encryption and Secure Channels

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.3 Encryption and Secure Channels

- The system should:
  - offer **encrypted storage** for sensitive data (e.g. via disk encryption).
  - use **encrypted communication channels** for:
    - API calls,
    - mesh traffic between nodes,
    - remote access.
