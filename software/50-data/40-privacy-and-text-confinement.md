# 40 – Privacy & Text Confinement
DOC-ID: PRIV-01

## 1. Text Confinement Theorem
The only columns in the system with `LargeUtf8` and average length > 512 bytes are:
- `Chunks.text`
- `Prompts.text`

All other columns are bounded ≤ 256 bytes or numeric/dictionary encoded.

**Corollary:** Any export of 𝒦 ∪ ℰ is safe for analytics/off-device use; no PII-bearing text exists in those planes.

## 2. Enforcement
- Schema DSL rejects `LargeUtf8` for planes 𝒦 and ℰ at compile time.
- Schema validator scans Arrow `Schema` definitions and fails CI if new `LargeUtf8` columns are added outside 𝒜/𝒳 allowances.
- Manual review: any addition to `51-schemas-mvp.md` must justify text placement and update this theorem if necessary.

## 3. Storage Boundaries
- Plane 𝒜 text is persisted (Parquet + Lance v2). Plane 𝒳 text is ephemeral (memory, optional encrypted spill).
- Control/Execution planes never store raw text; hashes/IDs only.

## 4. Redaction Hooks
- `Prompts.is_redacted` signals PII scrubbing; derived datasets must honor it when materializing contexts or logs.
- No downstream pipeline may reconstruct prompts from hashes; hashes serve correlation only.
