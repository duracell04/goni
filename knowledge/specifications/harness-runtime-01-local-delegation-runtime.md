---
id: HARNESS-RUNTIME-01
title: Local Delegation Runtime
type: specification
status: draft
implementation_state: specified_only
proposition: Goni should execute each delegated task as an explicit resumable state machine connecting intent compilation, WorkOrder and DoneContract binding, context and tool-surface projection, model inference, proposal validation, kernel authorization, transactional execution, observation, verification, receipts, and state updates.
domains:
- agent
- harness
- kernel
- system
aliases:
- local harness runtime
- delegation runtime state machine
relations:
- type: depends_on
  target: DELEG-INT-01
- type: depends_on
  target: GONI-SPEC-F37FC6D98E05
- type: depends_on
  target: GONI-SPEC-59E3DFC23CC8
- type: depends_on
  target: GONI-SPEC-B49DB23CF412
- type: depends_on
  target: TOOL-01
- type: depends_on
  target: SPEC-TXN-01
- type: depends_on
  target: REC-01
- type: depends_on
  target: SCHED-01
- type: refines
  target: GONI-PRINCIPLE-GOV-LOOP-01
sources:
- SRC-LIN2026-AGENTIC-HARNESS-ENGINEERING
artifacts: []
uncertainty: This is a logical runtime contract. Concrete process topology, persistence backend, timeout values, retry budgets, and concurrency policy remain implementation and evaluation questions.
legacy: []
---

# Local Delegation Runtime

The local harness is the execution coordinator between Goni's canonical task contract and the kernel authority boundary. It is not the authority root and does not turn model output into permission.

## Canonical runtime states

A delegated run should move through explicit states:

```text
COMPILE
-> CONTEXT
-> TOOL_PROJECT
-> INFER
-> VALIDATE
-> AUTHORIZE
-> EXECUTE
-> OBSERVE
-> VERIFY
-> UPDATE
-> CONTINUE | REPLAN | ESCALATE | COMPLETE | FAIL
```

### COMPILE

Compile principal intent into a stable WorkOrder and DoneContract through `DELEG-INT-01`. The contract fixes the objective, constraints, authority assumptions, acceptance criteria, verification requirements, and stop conditions before consequential execution begins.

### CONTEXT

Materialize the smallest sufficient ContextPack from current policy, task state, governed memory, retrieved evidence, recent observations, and any required source waypoints. Context construction must preserve provenance and trust classes.

### TOOL_PROJECT

Derive the model-visible tool surface from installed capabilities, current policy, WorkOrder relevance, authority eligibility, and runtime constraints. Tool visibility is a cognitive input decision; actual execution authority remains a later kernel decision.

### INFER

Invoke the selected deployment profile with the compiled context, model-specific adapter, and bounded tool surface. Inference may yield a final answer, clarification request, replan proposal, or structured tool proposal.

### VALIDATE

Validate machine-readable structure, schema conformance, referenced tool and operation identifiers, state assumptions, and proposal completeness. Structural validity does not establish semantic correctness or permission.

### AUTHORIZE

Submit any effectful proposal to the kernel. The kernel evaluates policy, capability scope, risk, approval requirements, preconditions, egress, sandbox, and credential constraints. A denial returns observable state to the harness; it does not become a model-side exception to ignore.

### EXECUTE

Execute an authorized operation under TOOL-01, SPEC-TXN-01, sandbox, credential, idempotency, retry, and receipt contracts.

### OBSERVE

Acquire the actual resulting environment state or structured tool result. The external observation, not the generator's expectation, becomes the basis for downstream verification.

### VERIFY

Compare observed state and evidence with the DoneContract and operation-specific postconditions. Verification may accept, retry safely, compensate, replan, escalate, or fail.

### UPDATE

Persist task state, belief updates, receipts, memory mutations, checkpoints, and unresolved uncertainty according to their respective contracts. Disposable model context and KV state remain reconstructible derivatives rather than canonical task state.

## Run invariants

- Model output is a proposal until the appropriate authority boundary accepts it.
- Every mutating tool operation uses the canonical transaction and idempotency contracts.
- A model or process restart must not erase the canonical WorkOrder, DoneContract, completed effects, pending approvals, or receipt chain.
- Retries use observed failure state and policy; blind repetition is not a recovery strategy.
- The harness stops when the DoneContract is satisfied, execution is blocked, authority requires principal action, or the configured stop condition is reached.
- Model replacement, quantization changes, or adapter changes may alter execution scaffolding but do not enlarge the current authority corridor.

## Resumability

A resumable run requires enough externalized state to reconstruct the next permitted transition without replaying the entire conversation as authoritative memory. At minimum this includes:

- WorkOrder and DoneContract refs;
- current runtime state;
- completed and pending operation refs;
- current task and belief state;
- relevant checkpoint or ContextPack refs;
- receipts and policy decisions;
- pending approvals or principal-owned uncertainties; and
- last observed postconditions relevant to continuation.

The runtime therefore treats conversation history as one possible evidence source, not as the sole state machine.
