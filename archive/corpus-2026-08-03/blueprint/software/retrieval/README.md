# Retrieval

Status: specified only / roadmap

Unified retrieval API for dense, sparse, hybrid, and graph search.

Normative contract:
- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md)

Roadmap note:
- Retrieval remains the default evidence-selection baseline.
- Graph traversal is a governed retrieval signal for context assembly, not a
  replacement for Work Order binding, policy filtering, reranking, or receipts.
- CGG-01 defines ContextPack assembly: the compiled context bundle that records
  selected context, omitted candidates, compression policy, and receipt refs.
- A separate research lane may compare retrieval against programmatic
  long-context reading and hybrid retrieval + reading strategies.
- That comparison does not imply that retrieval is deprecated or replaced.
