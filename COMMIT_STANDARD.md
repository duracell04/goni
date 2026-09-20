# Commit standard

Repository history is architectural provenance. Every non-baseline **decision commit** in the reconstruction must be reviewable and use this message anatomy:

```text
kg(<scope>): <imperative summary>

Intent:
<what becomes true>

Rationale:
<why this boundary and approach were chosen>

Status:
specified-only | implemented-untested | implemented-and-tested

Contracts:
- <canonical node, schema, or none>

Files:
- <every changed path relative to the first parent>

Evidence:
- <test, pinned artifact, or none>

Unresolved:
- <question or none>
```

Topic commits contain one coherent concern. Integration uses two-parent `--no-ff` merge commits so topic history and rationale remain visible. A merge is an integration event, not a new decision unit. Integration merges are graph/provenance wrappers rather than new decision units: they require a non-empty integration summary, while the substantive parent commits retain the full anatomy above. Octopus merges are rejected. Historical commits are never rewritten. For non-merge decision commits, the strict validator compares the `Files` section with the commit's actual first-parent diff.
