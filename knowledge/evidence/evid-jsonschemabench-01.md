---
id: EVID-JSONSCHEMABENCH-01
title: 'Source claim: constrained structured-output engines require independent evaluation'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Geng et al. introduce JSONSchemaBench with 10,000 real-world JSON schemas and evaluate constrained-decoding systems across compliance, schema coverage, efficiency, and output quality, demonstrating that structured-generation backends have materially different capabilities and limitations.
domains:
- research
- inference
- evaluation
aliases: []
relations:
- type: supports
  target: STRUCT-OUT-01
- type: supports
  target: LOCAL-TOOL-EVAL-01
sources:
- SRC-GENG2025-JSONSCHEMABENCH
artifacts: []
uncertainty: JSON-schema conformance is only one dimension of agent tool reliability and does not measure Goni authorization, state correctness, or semantic tool selection by itself.
legacy: []
---

# Source claim: constrained structured-output engines require independent evaluation

JSONSchemaBench evaluates structured-generation engines using a large corpus of real-world schemas and separates compliance, coverage, efficiency, and generated-output quality.

For Goni, the result supports treating the structured-output backend as a deployment-profile component that must be tested rather than assumed interchangeable.

Schema-valid output remains distinct from correct tool selection and authorized execution.
