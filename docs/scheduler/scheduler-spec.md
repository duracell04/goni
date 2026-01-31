# Scheduler Spec (implementation notes)

Status: specified only / roadmap

This document is implementation-facing. The normative contract is
in docs/specs/scheduler-and-interrupts.md.

## QoS classes
- interactive
- background
- maintenance

## Invariants
- interactive has priority over background
- preemption points at token boundaries
