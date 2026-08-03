---
id: GONI-IMAP-ADB33DF49DDC
title: 2.1 Facility-location with relevance
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Context selection is framed as a submodular maximisation problem: Given: Ground set \(V\) (retrieved chunks).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 2.1 Facility-location with relevance
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2.1 Facility-location with relevance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Facility-location with relevance

Context selection is framed as a submodular maximisation problem:

Given:

- Ground set \(V\) (retrieved chunks).  
- Similarity kernel \(k(i,j) = \cos(e_i, e_j) \ge 0\).  
- Relevance weights \(r_j \ge 0\).  
- Costs \(c_j\) and budget \(B\).

Define:
$$
F(S) = \sum_{i \in V} \max_{j \in S} k(i,j) + \gamma \sum_{j \in S} r_j, \quad S \subseteq V.
$$

This is a classic **facility-location** term (coverage of \(V\) by facilities \(S\)) plus a modular term (relevance of facilities).
