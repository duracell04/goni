# 40.10 – API Surface (HTTP & Local Access)

Status: v0.1 (2025-11-30)  
Scope: Single-node Goni kernel, MVP API only  
Audience: Client implementers, systems engineers

---

## 1. Overview

The **API surface** is the formal contract between a Goni node and external clients.

We model the node as computing (possibly with randomness):

\mathsf{Serve} : \mathsf{Req}_{\text{Goni}} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}


where:

- \(\mathsf{Req}_{\text{Goni}}\) = set of valid HTTP requests under /v1/chat/completions,
- Stream(Token) = a (possibly streaming) sequence of output tokens,
- Log = internal metrics, traces, and tool results written into \(\mathcal{A}\).

All other components (CLI, dashboard, plugins) are clients of this function.

---

## 2. Endpoint and compatibility

### 2.1 Endpoint

For the MVP, the normative endpoint is:

`	ext
POST /v1/chat/completions
Host: 127.0.0.1:PORT  (loopback by default)
Content-Type: application/json
`

The server **binds to loopback only** in default configuration.

> **Invariant API-1 (Local-only by default)**
> In default “local-trust” mode, the HTTP server must not listen on non-loopback interfaces without explicit configuration.

### 2.2 Compatibility with OpenAI

Let:

* \(\mathsf{Req}_{\text{OA}}\) = set of valid OpenAI chat/completions requests (for chat models),
* \(\mathsf{Req}_{\text{Goni}}\) = set of valid Goni requests.

For supported model names, we require:

* \(\mathsf{Req}_{\text{OA}} \subseteq \mathsf{Req}_{\text{Goni}}\),
* Fields shared with OpenAI semantics retain those semantics.

Goni-specific extensions are prefixed with goni_ and must be ignored by generic OpenAI clients.

---

## 3. Request and response (logical schema)

### 3.1 Request

We model a request as:

R = (\text{messages}, \text{model}, \text{tools}, \text{stream}, \text{extras})


* **messages** – ordered list of chat messages
  \(m_k = (\text{role}, \text{content}, \text{name?})\), with role ∈ {system, user, assistant, tool}.
* **model** – string identifier, e.g. "goni-small", "goni-large", "goni-auto".
* **tools** *(optional)* – OpenAI-style function calling schema.
* **stream** – boolean; 	rue → server-sent events; alse → single JSON.
* **extras** *(Goni extensions)* – optional fields such as:

  * goni_profile: "interactive" | "background" | "maintenance" (hint for \(\mathcal{K}\)),
  * goni_rag_mode: "off" | "auto" | "strict" (hint for \(\mathcal{X}\)).

We do not prescribe a full JSON schema here; the implementation follows OpenAI’s spec plus these extensions.

### 3.2 Response

**Non-streaming**:

* Single JSON object containing:

  * id, created, model,
  * choices[0].message (final assistant message),
  * usage (prompt/completion token counts).

**Streaming**:

* Server-sent events (SSE):

  * Each event has a data: line with a JSON payload (choices[0].delta).
  * The stream ends with data: [DONE].

> **Invariant API-2 (streaming monotonicity)**
> For a given request \(R\), the concatenation of all streamed delta.content fragments per choice must equal the message.content of the corresponding non-streaming response (up to tokenisation whitespace).

---

## 4. Mapping API calls to planes

Serving a single HTTP request proceeds conceptually as:

1. **Orchestrator**
   Normalises \(R\) into a JobDescriptor:
   
   J = (\text{class}, \text{budget}, \text{tools}, \text{profile}, \dots)
   
   where:

   * class ∈ {interactive, background, maintenance},
   * udget encodes token/time limits.

2. **Control Plane (\mathcal{K})**

   * Enqueues \(J\) into the appropriate class queue.
   * Scheduler (MaxWeight) decides when it runs.
   * Router decides which model tier to use (goni-small / goni-large / etc).

3. **Context Plane (\mathcal{X})** (if RAG enabled)

   * Retrieves candidate chunks from VecDB.
   * Runs submodular selection under token budget to choose context set \(S \subseteq V\).
   * Builds a PromptPlan.

4. **Execution Plane (\mathcal{E})**

   * LLM Runtime runs inference on the PromptPlan with chosen model.
   * Emits a token stream.

5. **Data Plane (\mathcal{A})**

   * Records metrics, traces, and tool outputs as Arrow tables.

These steps implement the abstract \(\mathsf{Serve}\) function.

---

## 5. Error handling and rate limiting

### 5.1 Errors

Typical error cases:

* 400 Bad Request – malformed JSON or clearly invalid fields.
* 429 Too Many Requests – explicit rate limit / overload response.
* 503 Service Unavailable – resources temporarily exhausted.
* 500 Internal Server Error – unexpected failure.

> **Invariant API-3 (no silent overload)**
> Overload or rate limiting must result in explicit HTTP errors (429/503); the node may not silently queue unboundedly in ways that violate the Control Plane’s latency and stability assumptions.

### 5.2 Rate limiting (conceptual)

The MVP may not enforce strong limits, but the *model* is:

* Per-identity counters:

  * equests_per_minute,
  * 	okens_per_minute,
  * concurrent_requests.
* Limits chosen so they are compatible with stability conditions in \(\mathcal{K}\) (utilisation < \(\alpha\)).

---

## 6. Versioning and stability

All endpoints are under the /v1 prefix.

> **Invariant API-4 (v1 semantic stability)**
> For any request \(r \in \mathsf{Req}_{\text{v1}}\) that was valid at time \(t_0\), its meaning under /v1 at time \(t_1 \ge t_0\) must not change in a way that breaks well-behaved clients (no backwards-incompatible type/behaviour changes).

Evolution rules:

* Adding new **optional** fields is allowed.
* Adding new **endpoints** beside /v1/chat/completions is allowed.
* Changing semantics of existing fields or removing them requires a new version prefix (e.g. /v2).

---

## 7. Non-goals for MVP

The v1 API explicitly does **not** attempt to:

* Expose admin / metrics endpoints (that will be a separate “admin API”).
* Handle multimodal inputs (audio, images) – can be added later as separate endpoints.
* Provide multi-tenant isolation; the MVP assumes a single trust domain per node.

The intent is a **small, stable surface** that can be relied on locally, while leaving room for future extensions.
