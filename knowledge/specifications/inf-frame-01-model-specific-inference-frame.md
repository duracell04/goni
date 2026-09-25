---
id: INF-FRAME-01
title: Model-Specific Inference Frame
type: specification
status: draft
implementation_state: specified_only
proposition: An InferenceFrame is the deterministic model- and runtime-specific serialization of a ContextPack, model stack, stable instruction modules, and tool declarations into the exact inference input consumed by an LLM runtime.
domains:
- software
- system
aliases:
- inference-frame
relations:
- type: depends_on
  target: CTX-COMP-01
- type: depends_on
  target: MODEL-REG-01
- type: refines
  target: GONI-SPEC-B49DB23CF412
sources: []
artifacts: []
uncertainty: The logical frame is specified only. Concrete serialization formats, tokenizer bindings, tool-schema encodings, and runtime adapters require implementation and model-specific conformance tests.
legacy: []
---

# Model-Specific Inference Frame

A `ContextPack` is model-independent semantic state. An `InferenceFrame` is
the exact runtime-facing representation produced from that state.

[
oxed{	ext{ContextPack} 
eq 	ext{token sequence}}
]

The serialization boundary is:

[
	ext{ContextPack}
+
	ext{ModelProfile}
+
	ext{RuntimeProfile}
ightarrow
	ext{InferenceFrame}.
]

## 1. Logical shape

A conforming representation MUST preserve:

```yaml
inference_frame_id:
context_pack_id:
model_bundle_id:
adapter_set_hash:
tokenizer_hash:
chat_template_hash:
system_bundle_hash:
tool_schema_hash:
ordered_segment_refs:
prefix_module_refs:
prompt_token_count:
expected_output_budget:
serialization_version:
serialization_hash:
created_at:
provenance:
receipt_ref:
```

The exact token bytes or IDs may be retained only where the governing privacy,
storage, and replay policy permits them. Hashes and stable refs SHOULD be used
when they provide sufficient reconstruction evidence without retaining raw
private text.

## 2. Deterministic serialization

Given identical semantic inputs, immutable model stack, tokenizer, chat
template, tool schema, serialization version, and deterministic profile, an
audit-grade serializer SHOULD produce identical ordered segments and the same
serialization hash.

Model-specific concerns include:

- tokenizer and special-token conventions;
- chat-template semantics;
- system/user/assistant role encoding;
- tool or function-schema encoding;
- structured-output constraints;
- model context-window limits;
- adapter or learned-prefix stack;
- runtime-specific position semantics where relevant.

## 3. Model independence of ContextPack

Runtime details MUST NOT be written back into the semantic meaning of the
ContextPack. Multiple models may consume the same ContextPack through distinct
InferenceFrames, making model/runtime comparison possible without reconstructing
semantic context from scratch.

## 4. Provenance

The receipt path SHOULD allow a reviewer to distinguish:

[
	ext{same semantic evidence}
]

from:

[
	ext{same exact model input}.
]

A ContextPack hash proves semantic compilation identity under its declared
contract. An InferenceFrame serialization hash proves the runtime-facing
serialization identity under its declared model/runtime stack.
