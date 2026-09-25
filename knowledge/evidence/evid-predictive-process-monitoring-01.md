---
id: EVID-PREDICTIVE-PROCESS-MONITORING-01
title: Predictive process monitoring supports next-event prediction from traces
type: evidence
status: draft
implementation_state: not_applicable
proposition: Predictive process monitoring applies predictive models to running process cases and can forecast next events, outcomes, or remaining time, supporting Goni's proposal to learn next-work expectations from historical execution trajectories.
domains:
- research
- delegation
aliases: []
relations:
- type: supports
  target: WORKFLOW-LEARNING-01
- type: supports
  target: ANTICIPATED-WORKORDER-01
sources:
- SRC-CERAVOLO2024-PREDICTIVE-PROCESS-MONITORING
artifacts: []
uncertainty: The source surveys business-process settings; Goni must validate whether comparable predictive performance transfers to heterogeneous personal workflows.
legacy: []
---

# Predictive process monitoring supports next-event prediction from traces

Predictive process monitoring extends process mining from retrospective
description toward runtime forecasting of an active case.

For Goni, this supports treating completed execution trajectories as a dataset
for estimating likely next work, while leaving the authority decision to a
separate kernel path.
