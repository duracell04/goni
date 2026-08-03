---
id: GONI-SPEC-9D8A0A7F0F70
title: 3. Governed model stacks and adapters
type: specification
status: draft
implementation_state: specified_only
proposition: Goni does not treat personalization as hidden drift inside the model.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 3. Governed model stacks and adapters
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 3. Governed model stacks and adapters

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Governed model stacks and adapters

Goni does not treat personalization as hidden drift inside the model. Prompt
steering, memory/retrieval, adapters, and full fine-tuning are separate
governed layers with different reversibility and review requirements.

A governed model stack is:

```text
base model bundle
+ optional adapter set
+ prompt/policy bundle
+ memory or retrieval bundle refs
+ runtime config
```

Adapters may encode domain skill, writing style, user worldview, neutral
academic framing, skeptical critic behavior, legal caution, sales style, or
other lenses. Plural adapters are preferred over one implicit ideology: a route
may compare user-worldview, neutral, critic, or specialist outputs when policy
or task risk requires it.

LoRA, QLoRA, DPO-style preference adapters, sparse expert modules, and similar
artifacts MUST be versioned as governed artifacts. A runtime MUST NOT silently
load an undeclared adapter or mutate model behavior without an
AdaptationReceipt. Route receipts MUST show which base bundle, adapters,
prompt/policy bundle, memory/retrieval refs, and eval refs were active when
those choices affected output or tool eligibility.
