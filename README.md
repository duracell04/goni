# Goni - Delegation OS

> Goni is a private operator appliance: a local-first digital double that refines your online life into briefs, decisions, and actions - quietly, with receipts, under your rules.

Status: Specified only / roadmap.

Goni is an open-source blueprint and prototype lab. It is not shippable hardware or a ready-made product.

## What this repo is

This is a **community-facing engineering blueprint**: a structured, mostly-Markdown knowledge base that captures the **constraints, architecture, decisions, trade-offs, and validation plans** for building the Goni hardware + kernel.

The goal is for others to **understand, reproduce, critique, and extend** the design without needing a full product codebase. Runnable artifacts (prototype code, tests, deployments) live in `goni-prototype-lab` (`https://github.com/duracell04/goni-prototype-lab`).

## What it is (plain English)

- Delegation, not chat: a calm operating layer for making and executing decisions.
- Three surfaces: Daily Brief, Action Cards, Vault.
- Receipts-first: actions are meant to be reviewable and reconstructable.
- Local-first by default, with explicit escalation when needed.

## What it is not

- A local chatbot box.
- A self-hosted cloud assistant.
- Autonomy without rules.

## How it works (mental model)

Observe -> Distill -> Propose/Act -> Attach receipts -> Store memory.

## Trust posture (design intent)

- Actions are proposed before they happen.
- Network access is explicit and gated.
- Receipts are attached to mediated actions.
- Budgets and policies bound delegation.
- No outbound telemetry by default.

## Repo layout (reader friendly)

- README.md: this page.
- blueprint/: the blueprint and prototype index (start at blueprint/README.md).

## Learn more

- Story: blueprint/docs/goni-story.md
- Glossary: blueprint/docs/glossary.md
- Full technical blueprint: blueprint/README.md
