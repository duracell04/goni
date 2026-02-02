# Taxonomy and IDs
DOC-ID: SYS-02
Status: Specified only / roadmap

This page defines naming, IDs, and link conventions for blueprint docs.

## DOC-ID conventions
- SYS-xx: system hubs and navigation
- SPEC-xx or existing codes (NET-01, REC-01, etc.): normative specs
- SUB-xx: subsystem packets
- ADR-xxxx: decision records
- HREQ-01 / SREQ-01: requirements

## File naming
- Use lowercase with hyphens for new files.
- Index files are named 00-index.md to create a stable entry point.

## Optional maturity tag
You may add a "Maturity" field (Draft, Candidate, Stable, Deprecated) when it
helps review flow. Do not replace the required Status vocabulary.

## Link contract
Every hub or subsystem packet should include:
- Upstream (what this depends on)
- Downstream (what depends on this)
- Adjacent (close neighbors)

Avoid plain-text file references when a relative link can be used.

## Folder index rule
Any folder with more than three docs should have a 00-index.md that lists:
- Purpose
- Canonical read order
- Outbound links to related hubs/specs
