---
id: CI-ASSURANCE-01
title: "Semantic CI and configuration-change assurance"
type: specification
status: draft
implementation_state: specified_only
proposition: "CI must check delivery configuration against application contracts, attribute failures, and apply proportionate assurance to consequential changes."
domains:
- software
- validation
- delivery
aliases: []
relations:
- type: refines
  target: CI-VALIDATION-01
- type: depends_on
  target: OPS-CONTRACT-01
- type: depends_on
  target: DELIVERY-MIN-01
sources:
- SRC-BARDSIRI2026-STUCK-ON-RED
- SRC-KIM2026-WORKFLOW-CHECKER
- SRC-ZHANG2026-K8S-CONFIG
artifacts: []
uncertainty: "Specified future CI requirements. Existing knowledge-graph validation does not prove implementation of the deployment and recovery checks listed here."
legacy: []
---

# Semantic CI and configuration-change assurance

## Verification layers

- **Fast checks:** Schema, provenance, deterministic catalogue, syntax, and directly affected application tests should run on proposed changes.
- **Cross-artifact checks:** Compare deployment declarations with application requirements: immutable image and dependency versions, persistent state paths, network/egress rules, secrets, service prerequisites, readiness, and schema migrations. Syntactically valid YAML is insufficient evidence of runtime compatibility.
- **Targeted integration checks:** Changes to policy, capabilities, persistence, receipts, tool execution, egress, and deployment topology require relevant denial, fault-injection, restart, and recovery tests before promotion.
- **Release checks:** Qualify the actual versioned artifact in each supported deployment topology, preserving a known-good recovery target.

## Diagnose failures before retry

Capture workflow version, commit SHA, trigger, stage, duration, source-change status, relevant dependency state, and cause classification: application, configuration, CI harness, external service/network, or undetermined. Track repeated failing runs and time-to-recovery. Investigate failing streaks, including those without repository changes, before changing source code or retrying blindly.

Changes spanning application code and deployment infrastructure, including AI-generated changes, require a single cross-boundary review of the application/deployment contract.

## Keep the delivery system bounded

Remove equivalent PR/push work where validation and branch-protection guarantees are preserved. Evaluate each check by material defects uniquely detected versus execution, false-failure, maintenance, and investigation costs. Configuration assurance is a risk-control mechanism, not a goal to maximize pipeline stages.
