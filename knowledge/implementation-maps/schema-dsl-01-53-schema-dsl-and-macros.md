---
id: SCHEMA-DSL-01
title: 53 Schema Dsl And Macros
type: implementation-map
status: draft
implementation_state: specified_only
proposition: "\uFEFF# 53 – Schema DSL & Codegen Macros (goni-schema) DOC-ID: SCHEMA-DSL-01 **Goal:** The code is the spec."
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/53-schema-dsl-and-macros.md
  heading: 53 Schema Dsl And Macros
  revision: 4165f3c79cdbd27663cc20ba23000952e0ebb10b
---

# 53 Schema Dsl And Macros

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

﻿# 53 – Schema DSL & Codegen Macros (goni-schema)
DOC-ID: SCHEMA-DSL-01

**Goal:** The code is the spec. The `goni-schema` crate owns a `define_tables!` DSL that emits Arrow `Schema` definitions, typed batch wrappers, and compile-time guards for SMA/ZCO/TXT.
