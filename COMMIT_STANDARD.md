# Commit standard

Repository history is architectural provenance. Every non-baseline commit in the reconstruction must be reviewable and use this message anatomy:

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

Topic commits contain one coherent concern. Integration uses `--no-ff` merge commits so topic history and rationale remain visible. Historical commits are never rewritten. The strict validator compares the `Files` section with the commit's actual first-parent diff.
