# Editorial policy

## Canon and atomicity

Only nodes under `knowledge/` are canonical. A node expresses one independently reviewable major proposition. Context may be included to make the proposition intelligible, but unrelated normative claims belong in separate nodes. Syntheses explain and sequence; they do not silently redefine atomic nodes.

## Authority and status

All migrated nodes are `status: draft`. Draft does not mean unimportant; it means the proposition remains reviewable. Status changes require a later explicit governance decision.

`implementation_state` is a separate factual field:

- `specified_only`: a design or requirement exists without pinned implementation evidence.
- `implemented_untested`: pinned implementation exists but no pinned test evidence supports it.
- `implemented_tested`: pinned implementation and test evidence both exist.
- `not_applicable`: the node is not an implementation-bearing claim.

Use `specified_only` by default for design-bearing material. Words such as implemented, enforced, verified, validated, guaranteed, and non-bypassable require repository-and-full-commit evidence and a stated boundary.

## Provenance and disagreement

Preserve original revision and legacy location. Do not merge conflicting claims into false consensus. Represent objections, alternatives, uncertainty, and evidence limitations directly. Evidence records what was observed; experiments state what will be tested.

## Sources and derivation

External sources receive stable `SRC-*` records. A citation alone does not establish a GONI claim. Evidence nodes identify the reported observation, GONI's interpretation, limitations, and the proposition supported or challenged.
