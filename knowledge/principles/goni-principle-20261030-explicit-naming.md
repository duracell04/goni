---
id: GONI-PRINCIPLE-20261030-EXPLICIT-NAMING
title: Explicit naming and semantic clarity
type: principle
status: draft
implementation_state: specified_only
proposition: Goni uses complete, descriptive, consistent names for concepts, instructions, code, and human interfaces so their meanings are recoverable without decoding shorthand.
domains:
- agent
- software
- product
aliases: []
relations:
- type: depends_on
  target: GONI-DECISION-20261030-MINIMAL-SUFFICIENT-SOFTWARE
sources: []
artifacts: []
uncertainty: Established external identifiers and historical repository contracts require compatibility handling; terminology checks may need a human review of context.
legacy: []
---

# Explicit naming and semantic clarity

Goni communicates concepts with complete, descriptive terms. Names should make their purpose understandable to humans and artificial intelligence agents without requiring a separate acronym glossary.

## Naming contract

1. Write complete terms in prose, interface labels, architecture diagrams, agent instructions, and new documentation. Use **Minimal Sufficient Software** in full.
2. Give functions, variables, types, file names, configuration keys, policies, and tools descriptive names that reflect their observable purpose and authority.
3. Assign one canonical name to each concept. When two concepts differ, name the difference explicitly.
4. Reuse a concept's canonical name across the blueprint, source code, agent messages, and interface descriptions.
5. Explain inherited identifiers and established technical terms on first human-facing use. Preserve externally required protocol fields, stable node identifiers, historical links, and established proper names such as Goni and JEV for compatibility.
6. Review new terminology for ambiguous abbreviations, overloaded labels, misleading synonyms, and terminology that requires unstated prior context.

## Scope and rationale

Acronyms and compressed shorthand increase the interpretation burden across multiple models, contributors, and operating contexts. Shorter names are worthwhile only when they remain unambiguous under the intended use; otherwise explicitness is the simpler design.

This principle applies to newly authored material and meaningful revisions. Existing identifiers may be migrated deliberately through versioned compatibility changes, preserving provenance and existing consumers.
