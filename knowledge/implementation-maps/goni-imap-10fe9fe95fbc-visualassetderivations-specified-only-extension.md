---
id: GONI-IMAP-10FE9FE95FBC
title: VisualAssetDerivations (specified-only extension)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: derivation_id = row_id Fields: source_asset_ids: list<fixed_size_binary[16]>, output_asset_id: fixed_size_binary[16], work_order_id: fixed_size_binary[16], done_contract_hash: fixed_size_binary[32], workflow_hash: fixed_size_binary[32], model_bundle_ids: list<utf8>, mask_refs: list<utf8>, control_refs: list<utf8>, transformation_summary: map<utf8, utf8>, verification_summary: map<utf8, utf8>, rollback_ref?: utf8, receipt_id: fixed_size_binary[16], created_at: timestamp(ms), provenance: map<u'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: VisualAssetDerivations (specified-only extension)
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# VisualAssetDerivations (specified-only extension)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### VisualAssetDerivations (specified-only extension)
- PK: `derivation_id = row_id`
- Fields: `source_asset_ids: list<fixed_size_binary[16]>`, `output_asset_id: fixed_size_binary[16]`, `work_order_id: fixed_size_binary[16]`, `done_contract_hash: fixed_size_binary[32]`, `workflow_hash: fixed_size_binary[32]`, `model_bundle_ids: list<utf8>`, `mask_refs: list<utf8>`, `control_refs: list<utf8>`, `transformation_summary: map<utf8, utf8>`, `verification_summary: map<utf8, utf8>`, `rollback_ref?: utf8`, `receipt_id: fixed_size_binary[16]`, `created_at: timestamp(ms)`, `provenance: map<utf8, utf8>`
- Notes: Records visual provenance and rollback references without embedding raw prompts, raw image content, or unbounded OCR text. This table is specified-only until added to the executable schema DSL.
