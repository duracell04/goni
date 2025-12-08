# AI-2027 Snapshot (local RAG only)

This folder holds an offline copy of the AI-2027 scenario for safety/strategy Q&A. The repo ships placeholders; add your own snapshot locally.

How to use:
1) Download AI-2027 as PDF or HTML into this folder (e.g., `ai-2027-2025-01-10.pdf`).
2) Duplicate `manifest.json.example` to `manifest.json` and fill `source_url`, `retrieved_at` (ISO), `format`, `file`, and `sha256` of your snapshot.
3) Ingest with the prototype script, adding this folder to `--src` so it joins the Arrow spine for RAG.
4) Keep it local-only; do not upload the snapshot if you do not have redistribution rights.

