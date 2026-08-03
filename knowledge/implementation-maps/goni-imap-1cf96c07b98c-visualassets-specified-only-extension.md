---
id: GONI-IMAP-1CF96C07B98C
title: VisualAssets (specified-only extension)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: visual_asset_id = row_id Fields: asset_type: dict<uint8, utf8> (logo|screenshot|product_photo|diagram|social_post|document_page|mask|generated_output), source_hash: fixed_size_binary[32], storage_uri: utf8, mime_type: utf8, rights_status: dict<uint8, utf8> (owned|licensed|public|unknown), privacy_class: dict<uint8, utf8>, permission_scope: dict<uint8, utf8>, project_refs: list<utf8>, person_refs: list<utf8>, style_tags: list<utf8>, detected_object_refs: list<utf8>, ocr_chunk_refs: list<fixed'
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
  heading: VisualAssets (specified-only extension)
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# VisualAssets (specified-only extension)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### VisualAssets (specified-only extension)
- PK: `visual_asset_id = row_id`
- Fields: `asset_type: dict<uint8, utf8> (logo|screenshot|product_photo|diagram|social_post|document_page|mask|generated_output)`, `source_hash: fixed_size_binary[32]`, `storage_uri: utf8`, `mime_type: utf8`, `rights_status: dict<uint8, utf8> (owned|licensed|public|unknown)`, `privacy_class: dict<uint8, utf8>`, `permission_scope: dict<uint8, utf8>`, `project_refs: list<utf8>`, `person_refs: list<utf8>`, `style_tags: list<utf8>`, `detected_object_refs: list<utf8>`, `ocr_chunk_refs: list<fixed_size_binary[16]>`, `embedding_refs: list<fixed_size_binary[16]>`, `approved_output_refs: list<fixed_size_binary[16]>`, `receipt_refs: list<fixed_size_binary[16]>`, `created_at: timestamp(ms)`, `provenance: map<utf8, utf8>`
- Notes: Stores governed metadata and refs only. Raw image binaries, masks, and full OCR text are content-addressed artifacts outside this row; OCR text lives in Chunks when retained. This table is specified-only until added to the executable schema DSL.
