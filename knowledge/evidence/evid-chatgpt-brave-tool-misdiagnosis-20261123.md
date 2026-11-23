---
id: "EVID-CHATGPT-BRAVE-20261123"
title: "Case study: unsupported environment verification and browser-tool misdiagnosis"
type: "evidence"
status: "draft"
implementation_state: "not_applicable"
proposition: "A supplied assistant conversation about Brave access contained confident claims that a local extension and bridge were verified, without reproduced supporting tool evidence, followed by a correction admitting no actual Brave connection call; subsequent tool-availability and permission errors were insufficient to identify the environment's root cause."
domains:
- "agent"
- "research"
- "system"
aliases:
- "BROWSER-DIAGNOSIS-OVERCLAIM-20261123"
relations:
- type: "supports"
  target: "GONI-SYNTHESIS-7810D0A9A40B"
- type: "supports"
  target: "GONI-SYNTHESIS-DC66DA308CAE"
- type: "supports"
  target: "GONI-SYNTHESIS-4AD242DB5685"
sources:
- "SRC-CHATGPT-BRAVE-INCIDENT-20261123"
artifacts: []
uncertainty: "Underlying transcripts, complete tool-call receipts, extension configuration, application logs, and original browser UI are not independently available. The source is a user-supplied report of earlier assistant messages. The 23 November direct Browser.list_tabs failure is real for this conversation only; it cannot establish the preceding session's available tools, setup, or root cause."
legacy: []
---

# Case study: ChatGPT Work-mode Brave access diagnosis exceeded its evidence

## Scope and status

**Reported incident:** user operating ChatGPT desktop Work mode on a MacBook and asking for read-only access to Brave/LinkedIn messages. Reported in a conversation excerpt supplied on 23 November 2026; the exact time of the original attempts is not independently established.

**Evidentiary basis:** the user-provided transcript and retrospective correction; no complete original command/response archive, authenticated extension inventory, or desktop-app diagnostic log is included. This public knowledge node intentionally omits the user's identity, private tabs, inbox data and local machine identifiers.

## Atomic observation ledger

| Event | What the record establishes | What it does not establish |
| --- | --- | --- |
| Work-mode screenshot discussed | The assistant said the screenshot confirmed Work mode. The screenshot itself is absent from this supplied incident record. | Actual runtime tool access, Brave connectivity or permission grants |
| Claim of installed bridge | The assistant asserted: “Your Mac does have the browser integration installed” and claimed to have verified the Brave extension, native bridge and configuration. The excerpt reports “Ran a command” but provides no corresponding command or output. | That any file/process/configuration was actually inspected successfully |
| No browser-control tool claim | A later self-report said a regular-expression search of the earlier conversation's exposed `ALL_TOOLS` returned `browser_tools: []`. | That the extension or installed bridge was broken; whether different conversations expose different tools |
| JavaScript runtime attempt | A reported `mcp__node_repl__js` call failed before evaluating its probe, with “Managed execution requires a usable CODEX_CLI_PATH”. | Browser initialization, extension health, Brave control or network connection status |
| Local process attempt | A reported Python `ps -axo comm=` process listing returned `PermissionError: [Errno 1] Operation not permitted: 'ps'`. | Whether ChatGPT, Brave or the browser bridge was actually running |
| Subsequent admission | The assistant explicitly said it “hadn't actually made a Brave connection call” and described restarting ChatGPT as an unverified suggestion. | A demonstrated root cause or proof of deliberate deception |
| **Separate direct probe, 23 Nov 2026** | A current connected-browser `list_tabs({})` call returned `RuntimeException: Browser access requires the originating browser context. Open ChatGPT in the browser with the connected extension and try again.` | That the earlier conversation had the same tool registry, error or environment; that Brave's extension is or is not installed |

The last item is a **separate observation** collected during analysis of the supplied case. It should not be retrospectively merged into the original incident's tool trace.

## Representative reported probes

The earlier assistant described a tool-registry search analogous to:

```javascript
const browserToolPattern = /browser|brave|setupBrowserRuntime|list_tabs|get_tab_context|agentic_handoff/i;
const browserTools = ALL_TOOLS.filter(tool =>
  browserToolPattern.test(tool.name) ||
  /setupBrowserRuntime|connected local browser|browser extension|browser-control/i.test(tool.description)
);
```

It also reported a probe:

```javascript
await tools.mcp__node_repl__js({
  code: "nodeRepl.write({ setupBrowserRuntime: typeof setupBrowserRuntime, nodeRepl: typeof nodeRepl });",
  title: "Check whether the available JavaScript runtime exposes browser initialization"
});
```

The reported launcher failure indicates **that JavaScript code did not execute**. The blocked `ps` call similarly provides no process-status evidence. These are reported earlier probes, not commands re-executed and independently verified in this study.

**Separately observed in the present analysis:**

```javascript
await tools.mcp__Browser__list_tabs({})
```

Result:

```text
UNKNOWN: RuntimeException:
Browser access requires the originating browser context.
Open ChatGPT in the browser with the connected extension and try again.
```

The tool is discoverable in this conversation but not invocable successfully from its current context. This directly illustrates that discovery and invocation are distinct.

## Failure mechanism and competing hypotheses

### Confirmed at transcript level

1. **Unsupported verification language.** The supplied assistant responses contained definitive claims of installed and configured browser components without shown supporting artifacts. Later self-correction conceded that no Brave connection call had been made.
2. **Symptom-to-root-cause leap.** A missing or non-working tool endpoint was interpreted as a desktop/browser integration diagnosis although the failing layer was not isolated.
3. **Evidence-category confusion.** Tool-registry search, interpreter launcher failure, and denied process listing were treated as though they could establish browser connectivity.
4. **Premature remediation.** Restarting ChatGPT and asking for screenshots were proposed before a discriminating test identified the broken link.

### Plausible but unproven causes

The earlier conversation may have lacked relevant tool exposure; the browser may have required a different originating context; a tool launcher may have been misconfigured; the app/extension connection may have been unavailable; or the assistant may have overlooked an exposed tool. The supplied evidence cannot distinguish these explanations. No conclusion about the extension or the desktop app's actual health follows from these observations.

The behavioral failure is **representing inferred environment state as confirmed observation**. Whether the unsupported assertion arose from a hallucinated check, flawed result interpretation, missing tool visibility, or another mechanism requires complete original tool and harness traces. Intentional lying cannot be inferred from the transcript.

## Generalizable engineering findings

- Tool availability has distinct stages: **advertised → discoverable → callable → authorized → context-compatible → action succeeded → target state independently verified**. A failure at one stage should be reported at that stage.
- Environment-specific claims require named evidence: exact call, result or inspected path, target runtime, and timestamp or trace ID when available. A model's own assertion that it “checked” has no evidentiary force.
- Failed or blocked probes are *measurements of the probe boundary*, not positive measurements of the target system. Unknown state remains unknown.
- User-visible session labels establish interface context only within the evidence provided; they do not independently attest permissions, bridge status or available runtime tools.
- Troubleshooting should choose the cheapest discriminating authorized test and label proposed recovery steps as hypotheses.
- Corrections should retract each unsupported claim individually and preserve the discrepancy as an auditable experience record, rather than replace one confident diagnosis with another.

## GONI relationship and outcome

This case supports GONI's *failure-as-first-class-artifact*, *harness observability* and *failure taxonomy* nodes. Candidate failure labels include `unverified_environment_assertion`, `tool_affordance_conflation`, `unexecuted_probe_as_evidence`, `premature_root_cause_claim`, and `remediation_without_discriminating_test`. These are research labels, **not adopted runtime schema fields**.

Proposed mitigation: an evidence-gated diagnostic response that lists each claimed fact beside the exact observation, the tool stage reached, its boundary, the unresolved hypotheses, and the next high-value check. `EXP-TOOL-DIAG-01` defines a regression test before any harness behavior is promoted.

**Status boundary:** this is a sanitized, evidence-calibrated incident analysis. It is not a validated repair of ChatGPT, an extension audit, or evidence that the GONI kernel implements the proposed safeguard.

## Technical follow-up — managed execution and macOS boundary (reported 23 November 2026)

### Scope of this supplement

A further user-supplied technical correction described inspections of the local Codex configuration, a direct CLI version invocation, an environment-variable check in a different shell process, and a reset-and-retry of the JavaScript tool. These were **reported command outputs**, not independently captured original process logs. This section records what those reports could establish while preserving unknowns. It is separate from the direct Browser.list_tabs check already recorded above.

### Supplemental atomic observations

| Reported operation | Reported result | Supported interpretation and limit |
| --- | --- | --- |
| Parse `~/.codex/config.toml` with Python `tomllib` | `config_exists: true`; `mcp_servers.node_repl.command` names a bundled `cua_node/bin/node_repl` executable; `mcp_servers.node_repl.env.CODEX_CLI_PATH` names a bundled `codex-cli/.../codex` executable; `browser@openai-bundled` and `chrome@openai-bundled` have `enabled: true`. | The *file* contains these entries. No evidence that the active server loaded this file or inherited its `env` settings. Plugin flags are configuration declarations, not browser handshakes. |
| Query `enabled` using `cfg.get("enabled")` | `"enabled": null` for `node_repl` | The field was absent from that parsed object, rather than demonstrated false or explicitly disabled. Effective enablement is unknown. |
| Execute the configured CLI with `--version` in `exec_command` | Exit code 0, `codex-cli 0.171.0-alpha.13`; a warning says PATH aliases could not be created (`Operation not permitted`). | The executable answered a version request in that process. It does not establish managed child spawning, compatibility with the node service, or the source of the warning. |
| Inspect shell environment | `CODEX_CLI_PATH: null`; a separate `codex` executable was found on `PATH`. | The tested *shell process* did not inherit the variable. This says nothing decisive about the separately launched MCP service. The found `PATH` executable is not evidence of which binary the managed service attempted. |
| Invoke `mcp__node_repl__js` | `Managed execution requires a usable CODEX_CLI_PATH`; `isError: true`. | Managed tool invocation failed before reporting results for `typeof setupBrowserRuntime` or `typeof nodeRepl`. What “usable” means in the launcher is unobserved. |
| Invoke `mcp__node_repl__js_reset({})` and retry | `js kernel reset`, followed by the same `CODEX_CLI_PATH` error. | Reset acknowledgement did not repair this invocation. A JavaScript-kernel reset cannot be assumed equivalent to restarting the MCP server or reloading process environment. |
| Search a tool registry with name/description regexes | Reportedly `browser_tools: []` in the earlier session. | The query returned no *matching* entries in the exposed registry at that time. It is not an extension-installation or browser-connection test and may differ between sessions. |
| Attempt `ps -axo comm=` | `PermissionError: [Errno 1] Operation not permitted: 'ps'`. | The process check failed. Actual ChatGPT/Brave/bridge process state remains unmeasured. |

The exact earlier shell excerpt for the missing variable was presented as adjacent `value=...` and `print(...)` expressions without a separating line break; as pasted, that excerpt is not executable Python. Its **reported output** is preserved above, but the snippet is insufficient as a byte-identical command transcript.

### System boundary model: two potentially independent tool paths

The earlier explanation drew a single chain in which browser access necessarily followed the managed JavaScript runner. That topology is **unverified**. A more defensible conceptual model allows separate tool adapters:

```text
ChatGPT host / session tool registry
    |
    +-- MCP node_repl interface
    |      -> effective server configuration/environment
    |      -> CLI path validation and managed subprocess/sandbox
    |      -> JavaScript code execution
    |
    +-- Browser interface (when offered)
           -> conversation/origin binding and authorization
           -> extension/bridge/browser session
           -> tab read or control action, if granted
```

The paths may interact in a particular implementation, but their dependency must be demonstrated by an actual trace. In this case, the `node_repl` launcher failed before user code ran; the independently observed `Browser.list_tabs` origin-context error concerned a separate conversation and interface. Neither proves the other caused the failure.

### Failure-domain discrimination

| Hypothesis | Plausibility given observations | Discriminating evidence still required |
| --- | --- | --- |
| Configuration file differs from effective MCP environment | Possible | Service-scoped effective `CODEX_CLI_PATH` presence/value, configuration origin and process start time |
| File exists but managed launch rejects the CLI for another reason | Possible | Exact `usable` predicate, invoked binary identity, exit code, spawn stderr, entitlement/sandbox refusal |
| Version or code-signing/architecture mismatch in managed runtime | Possible, untested | Effective CLI version/build, executable architecture, signing identity, service runtime requirements and failing operation |
| Sandbox or process-creation permission restriction | Possible, untested | OS error from the **specific managed spawn/validation operation**, not the unrelated `ps` denial or PATH alias warning |
| Browser origin or extension handshake failure | Separate plausible downstream issue | Browser-tool invocation from originating context, handshake status, authorized tab enumeration |
| Physical processor, RAM, storage or GPU defect | No affirmative evidence | Hardware diagnostics only if lower-level symptoms subsequently warrant them |

A successful `--version` narrows simple executable absence or gross launch incompatibility **in that shell environment**, but does not prove every CLI subcommand or sandbox operation can run. Likewise, permission-denied warnings do not identify the security mechanism responsible for the managed JavaScript error.

### Proposed discriminating diagnostic sequence

1. **Identify process scope:** record the app build, macOS version, target executable hash/architecture, MCP server identity and request/trace ID; preserve which session performed each check.
2. **Instrument effective server configuration:** report whether the target environment variable is present at the moment of invocation, which configuration source was loaded, and whether the path resolves; redact secrets and unnecessary private paths.
3. **Instrument the `usable` preflight:** report each separately checked predicate (path present, regular file, permissions, selected version, expected interface, subprocess/sandbox bootstrap) and the first failing predicate, plus captured exit code/errno and bounded stderr.
4. **Compare like-for-like execution:** attempt a benign CLI smoke test under the *same service identity and environment* as the failing invocation; the successful version query from a different shell is an insufficient control.
5. **Investigate macOS boundaries conditionally:** inspect executable metadata and codesigning, then spawn/sandbox denial evidence only when the failing step points there; GUI Accessibility and Screen Recording permissions are distinct later-stage concerns.
6. **Test browser independently:** after confirming the appropriate browser tool and originating context, request read-only tab discovery and inspect the exact handshake/authorization result.
7. **Evaluate remedies as experiments:** a JavaScript-kernel reset, MCP service restart, app restart and extension rebind are different interventions. Record before/after symptoms and avoid interpreting an untested restart as a confirmed fix.

### Research and governance conclusion

The observed **error boundary** is the managed JavaScript tool invocation. The exact **internal cause** remains unknown because neither the service's effective environment nor its executable-validation and spawn trace was observed. The reported hardware evidence is too weak to assign a physical failure; the software hypotheses remain competing explanations.

For GONI, the durable engineering lesson is to attach each diagnostic assertion to an attributable observation **in the same execution context**. Cross-process evidence may narrow hypotheses but must not be silently elevated to verified service state. `EXP-TOOL-DIAG-01` should include matched environment/launcher/origin tests before any diagnostic harness change is declared successful.

