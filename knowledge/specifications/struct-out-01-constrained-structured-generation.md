---
id: STRUCT-OUT-01
title: Constrained Structured Generation
type: specification
status: draft
implementation_state: specified_only
proposition: Goni should use deterministic constrained-generation and schema-validation mechanisms to enforce machine-readable output structure where practical, while keeping semantic correctness, tool appropriateness, and authorization separate from syntactic validity.
domains:
- inference
- tools
- harness
- system
aliases:
- structured output boundary
- grammar-constrained tool generation
relations:
- type: depends_on
  target: TOOL-SURFACE-01
- type: depends_on
  target: HARNESS-RUNTIME-01
- type: refines
  target: GONI-PRINCIPLE-MODEL-ADAPTER-01
sources:
- SRC-DONG2024-XGRAMMAR
- SRC-GENG2025-JSONSCHEMABENCH
artifacts: []
uncertainty: Backend grammar coverage, latency overhead, schema support, and model-quality effects vary by runtime and deployment profile and require direct evaluation.
legacy: []
---

# Constrained Structured Generation

Goni separates semantic judgment from structural validity.

The model may decide:

- whether a tool should be proposed;
- which visible operation best matches the objective;
- the semantic content of arguments;
- whether to answer, clarify, replan, or request an action.

The inference runtime or deterministic validation layer should enforce, where practical:

- valid output envelope type;
- parseable JSON or equivalent structured representation;
- required fields;
- allowed enum values;
- type constraints;
- tool and operation identifier grammar; and
- schema-level structural requirements.

The core boundary is:

```text
probabilistic semantics
+
deterministic structure
```

## Proposal envelope

A model-facing structured response may use a discriminated envelope such as:

```text
FinalAnswer
| ClarificationRequest
| ReplanProposal
| ToolProposal
| EscalationRequest
```

Each variant should have an explicit schema appropriate to the deployment profile.

For `ToolProposal`, the schema should reference the current `ModelToolView` and include the selected tool/operation identifier plus structured arguments. The harness rejects proposals that reference tools outside the current surface even if their textual form is otherwise valid.

## Validation stages

Structured generation does not collapse all validation into the decoder.

The intended sequence is:

```text
constrained generation
-> parse / schema validation
-> tool-surface validation
-> semantic / state validation
-> kernel authorization
-> execution
```

A syntactically valid proposal can still be semantically wrong, stale, unsafe, irrelevant, or unauthorized.

## Backend portability

Provider-native function calling, JSON-schema modes, CFG/grammar engines, llama.cpp grammars, XGrammar-compatible engines, or equivalent mechanisms may implement the structural layer. Goni should preserve one canonical semantic contract and use deployment-specific adapters for the concrete decoding mechanism.

Fallback free-form parsing is lower assurance and should be surfaced in the deployment profile and evaluation receipt rather than silently treated as equivalent to constrained generation.

## Evaluation

Each structured-output backend should be measured for:

- schema coverage;
- structural success rate;
- compilation/setup latency;
- token-generation overhead;
- unsupported-schema behavior;
- semantic task quality under constraints;
- failure-mode observability; and
- interaction with tool-call accuracy.

Structural compliance is evidence about representation quality, not evidence that the resulting action is correct or authorized.
