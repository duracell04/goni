---
id: GONI-SPEC-5FAD21682D7E
title: 8. Visual memory
type: specification
status: draft
implementation_state: specified_only
proposition: Visual memory is governed asset metadata plus content-addressed artifacts.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 8. Visual memory
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 8. Visual memory

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Visual memory

Visual memory is governed asset metadata plus content-addressed artifacts. Raw
image binaries, masks, and large OCR text do not live in Control-plane rows.
They are addressed by hash and storage URI, with metadata stored in canonical
VisualAssets and VisualAssetDerivations rows.

Required logical metadata includes:

- `visual_asset_id`
- asset type (`logo | screenshot | product_photo | diagram | social_post | document_page | mask | generated_output`)
- source hash and storage URI
- rights status (`owned | licensed | public | unknown`)
- privacy class and permission scope
- project/person refs
- style tags
- detected object refs
- OCR chunk refs
- embedding refs
- derived-from refs
- approved-output refs
- receipt refs

Visual memory writes are governed memory writes. They must carry provenance,
permission scope, receipt refs, and review/expiry policy where applicable.
