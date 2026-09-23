---
id: INTERFACE-MODEL-01
title: Access, invocation, trigger, transport, and action planes
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni classifies GUI, CLI, API, SDK, IPC, events, transports, MCP, A2A, and downstream actuation by architectural role instead of treating them as one flat family of interfaces.
domains:
- system
- interfaces
- interoperability
aliases:
- interface taxonomy
- access surface model
relations:
- type: refines
  target: GONI-THESIS-E1FB8B4F7772
sources:
- SRC-MCP-2026-07-28
- SRC-A2A-1-0
artifacts: []
uncertainty: This is a Goni architecture taxonomy assembled from standard systems concepts and the cited interoperability specifications; it is not claimed as a universal academic taxonomy.
legacy: []
---

# Access, invocation, trigger, transport, and action planes

GUI, CLI, API, MCP, A2A, cron, sockets, and webhooks answer different architectural questions. Goni separates them into six roles.

## 1. Human interaction plane

Ways a person interacts with Goni:

- GUI or web/mobile application,
- CLI or TUI,
- conversational REPL,
- IDE or editor integration,
- browser or OS extension,
- voice and multimodal interaction.

These surfaces describe presentation and human interaction. They do not determine how intelligent or agentic the underlying system is.

## 2. Software invocation plane

Ways another program invokes Goni:

- HTTP or RPC API,
- in-process SDK or library,
- IPC, local socket, named pipe, or stdin/stdout composition.

An SDK can invoke the same runtime without a network boundary; an API can expose it remotely or locally.

## 3. Trigger plane

Events that cause a run or workflow transition:

- direct user request,
- scheduler or cron,
- webhook,
- message queue or event bus,
- filesystem or file-drop event,
- operating-system event,
- another authorized agent task.

Triggering is orthogonal to both UI and intelligence. A background delegated workflow can be highly agentic without an open chat surface.

## 4. Semantic interoperability plane

Protocols that define what interoperating parties mean:

- MCP for tool/resource/context capability exchange,
- A2A for communication between independent agent systems.

These semantics may ride on different transports and remain subject to Goni policy.

## 5. Transport plane

Mechanisms that carry bytes or messages:

- HTTP,
- stdio,
- local sockets,
- WebSocket,
- RPC transports,
- queues.

Transport should not be confused with application semantics. MCP over stdio and MCP over HTTP can express similar application concepts across different process boundaries.

## 6. Action or actuator plane

Ways Goni changes or queries an external environment:

- structured APIs,
- CLI or shell commands,
- browser automation,
- GUI/computer use,
- device or robotic actuators.

The same label can occur on both sides of the runtime. A human may invoke Goni through a GUI while Goni itself uses a different GUI to operate a legacy application.

## Architectural direction

```text
human interaction ─┐
software invocation ├──> Goni runtime/kernel ──> action adapters ──> environment
triggers ──────────┘          │
                              ├── MCP ──> tools/resources
                              └── A2A ──> independent agents

transport carries the interactions underneath these semantic boundaries.
```

This taxonomy keeps access surfaces independent from cognition, authority, and agenticness.
