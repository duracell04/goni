---
id: ROBOT-01
type: SPEC
status: specified_only
---
# ROBOT-01 - Embodied Robot Control Plane
DOC-ID: ROBOT-01
Status: Specified only / roadmap

This spec defines the control-plane contract for embodied robots that observe,
move, manipulate, inspect, assist, or act in the physical world under delegated
authority from a principal.

Goni treats robot bodies, vendor skills, sensor streams, robot clouds, and
remote operators as replaceable execution substrates. They do not own the
principal's mandate, private memory, local policy, approval thresholds, or
receipt semantics.

Core doctrine:

```text
Personal AI owns mandate, memory, policy, and judgment.
Robot vendors provide body, sensors, skills, and actuation.
Cloud AI is optional, gated, and receipt-linked.
```

ROBOT-01 is specified only. It does not add a shipping schema table, define a
robot hardware platform, require a humanoid robot, or require any specific
robot vendor API. Implementations may later map this contract onto ROS,
vendor SDKs, fleet managers, teleoperation systems, industrial robot cells,
home robots, mobile manipulators, or humanoid platforms, but the Goni
authority model remains independent of those adapters.

## 1. Purpose

Embodied AI is a delegation problem before it is a robotics problem. A robot
that can see, move, touch, carry, unlock, clean, inspect, or assist people is
performing physical-world action under somebody's authority.

ROBOT-01 defines:

- the control boundary between local personal AI and robot execution,
- `RobotMandate`, the principal's bounded authorization for robot work,
- `EnvironmentScope`, the physical zone and social context in which a robot may
  act,
- `PhysicalActuationGrant`, the capability grant for movement and manipulation,
- `RobotAdapter`, the replaceable vendor or runtime bridge,
- `RobotReceiptBasis`, the receipt basis for physical-world action,
- task classes, cloud policy, safety posture, and conformance tests for
  embodied robot use.

The strategic claim behind this contract is that robot hardware may become
commoditized while private context, household memory, local policy, and trust
remain scarce. Goni therefore positions itself as the local command and
governance layer for embodied AI, not as the robot body.

## 2. Scope

This spec applies to broad embodied robot systems, including:

- humanoid robots,
- mobile manipulators,
- home and eldercare assistance robots,
- service, retail, reception, and guide robots,
- warehouse and logistics robots,
- industrial pilots and robot workcells,
- robot fleets that combine autonomous execution with remote supervision,
- vendor SDKs, ROS-style middleware, and robot-cloud adapters exposed to Goni.

This spec covers robot observation, extraction, memory, actuation, cloud use,
telemetry, remote operator access, and physical safety mediation. It does not
replace specialized standards for industrial safety, medical devices,
functional safety, machinery directives, workplace rules, building access,
labor law, privacy law, or insurance.

If a robot action purchases, negotiates, contracts, reserves funds, or pays, it
also falls under [DAT-01](/blueprint/30-specs/delegated-agent-treasury.md).

## 3. Control Boundary

Goni separates robot powers into distinct grants:

| Power | Meaning | Boundary question |
| --- | --- | --- |
| `robot_observation` | The robot may perceive sensor surfaces such as cameras, microphones, lidar, depth, force, touch, joint state, location, or environmental signals. | What may the robot sense, where, when, and under whose presence constraints? |
| `robot_extraction` | The robot or local AI may parse observed data into objects, people, rooms, hazards, inventory, activities, plans, or summaries. | What facts may be derived, and may extraction use cloud services? |
| `robot_memory` | Extracted facts, maps, object locations, household routines, or task outcomes may be stored, indexed, reused, synced, or forgotten. | What memory class may be written, retained, reviewed, or synced? |
| `robot_actuation` | The robot may move, navigate, grasp, carry, manipulate, clean, operate devices, interact with people, or change physical state. | What physical side effects are allowed, under which mandate, zone, safety envelope, approval, and receipt? |
| `physical_egress` | Robot data, telemetry, sensor summaries, maps, incidents, or command streams may leave the local node. | Which destination, payload class, redaction mode, purpose, and budget are permitted? |
| `remote_supervision` | A remote human or vendor system may observe, advise, approve, or control part of the robot session. | What can the supervisor see or do, and can their access expand the original mandate? |
| `cloud_reasoning` | A remote model or vendor cloud may perform planning, perception, skill selection, diagnostics, or fleet learning. | Which task, payload class, memory class, destination, and receipt tier allow remote reasoning? |

Granting one power MUST NOT imply any other power. A robot that may observe a
room may not automatically extract private facts, store memory, upload video,
call a remote operator, or perform physical actuation.

## 4. Canonical Flow

Every delegated robot action follows this logical flow:

```text
Human intent
-> Work Order / Done Contract
-> Robot Mandate
-> Environment Scope
-> Safety Envelope
-> Skill / Tool Call
-> Local Policy Check
-> Robot Execution
-> Verification
-> Receipt
-> Memory Update
```

The flow may stop early when policy denies a transition, when approval is
required, when sensor evidence is insufficient, when a safety envelope is
violated, when the mandate is revoked, or when the robot adapter cannot provide
required receipt evidence. A stopped flow still emits an auditable denial,
escalation, or no-op record.

Robot actions are tool-mediated side effects. They must preserve the
DELEG-INT-01 Work Order, Done Contract, autonomy mode, risk basis, capability
token, boundary basis, and receipt chain required by TOOL-01 and REC-01.

## 5. Normative Objects

### 5.1 RobotMandate

`RobotMandate` is the formal authorization that defines what robot work may be
attempted on behalf of the principal.

Minimum logical fields:

```yaml
robot_mandate:
  mandate_ref:
  principal_ref:
  robot_ref:
  work_order_id:
  purpose:
  allowed_task_classes:
  allowed_environment_refs:
  autonomy_mode:
  approval_thresholds:
  supervision_requirement:
  safety_envelope_ref:
  egress_policy_ref:
  evidence_requirements:
  revocation_ref:
  policy_hash:
  valid_from:
  expires_at:
  receipt_requirement:
  provenance:
```

The mandate grants no ambient physical authority. It only permits the task
classes, environments, autonomy mode, and evidence posture explicitly defined
by policy. A robot adapter cannot broaden the mandate through vendor defaults,
remote operator procedures, or fleet policy.

### 5.2 EnvironmentScope

`EnvironmentScope` defines where, when, and around whom a robot may observe,
extract, remember, or act.

Minimum logical fields:

```yaml
environment_scope:
  environment_ref:
  scope_type: "home | office | warehouse | factory | retail | care_site | outdoor | mixed"
  allowed_zones:
  denied_zones:
  sensitive_zones:
  people_presence_rules:
  pet_or_child_rules:
  private_area_rules:
  time_windows:
  allowed_surface_refs:
  prohibited_object_classes:
  map_ref:
  supervision_posture:
  emergency_stop_ref:
  receipt_ref:
```

Examples of denied or sensitive zones include bedrooms, bathrooms, medical
areas, locked storage, nurseries, financial document areas, restricted
workcells, private offices, hazardous machinery zones, and doors or locks when
policy does not authorize access.

### 5.3 PhysicalActuationGrant

`PhysicalActuationGrant` is the capability grant for robot movement and
manipulation.

Minimum logical fields:

```yaml
physical_actuation_grant:
  grant_ref:
  robot_ref:
  allowed_motion_classes:
  allowed_manipulation_classes:
  allowed_task_classes:
  payload_limits:
  contact_limits:
  speed_or_force_limits:
  tool_or_end_effector_limits:
  device_operation_rules:
  person_interaction_rules:
  irreversible_action_rules:
  approval_requirement:
  emergency_stop_requirement:
  rollback_or_repair_ref:
  idempotency_rule:
  policy_hash:
  receipt_ref:
```

Physical actuation is default-deny. Movement, grasping, carrying, device
operation, door or lock interaction, cleaning, disposal, person assistance,
and security patrol are distinct action classes and may require distinct
approval gates.

### 5.4 RobotAdapter

`RobotAdapter` is the replaceable bridge between Goni and a robot vendor,
runtime, fleet manager, or middleware stack.

Minimum logical fields:

```yaml
robot_adapter:
  adapter_ref:
  vendor_robot_id:
  robot_model:
  firmware_or_runtime_ref:
  supported_task_classes:
  supported_skill_refs:
  sensor_surfaces:
  safety_capabilities:
  local_execution_capabilities:
  cloud_dependency_declaration:
  telemetry_surfaces:
  remote_operator_capabilities:
  audit_log_refs:
  attestation_refs:
  adapter_policy_hash:
  provenance:
```

The adapter declares capability and evidence surfaces. It does not define
authority. Vendor logs, robot-cloud audit trails, ROS bags, telemetry streams,
or teleoperation logs may support a Goni receipt, but they do not replace the
canonical receipt.

### 5.5 RobotReceiptBasis

`RobotReceiptBasis` is the robot-specific receipt basis attached to a REC-01
receipt when robot observation, extraction, memory, actuation, egress, or
supervision affects an action.

Minimum logical fields:

```yaml
robot_basis:
  robot_ref:
  adapter_ref:
  mandate_ref:
  environment_scope_refs:
  physical_actuation_grant_ref:
  task_class:
  skill_ref:
  sensor_basis_refs:
  map_or_zone_refs:
  policy_decision:
  safety_envelope_result:
  verification_result:
  intervention_state:
  escalation_or_denial_reason:
  egress_refs:
  remote_supervision_refs:
  output_or_state_delta_refs:
  rollback_or_repair_ref:
```

`robot_basis` stores compact refs, hashes, bounded summaries, and replay
metadata. It MUST NOT store raw private home maps, raw video, raw audio, full
sensor logs, unrestricted telemetry streams, or unbounded transcripts by
default.

## 6. Task Classes

ROBOT-01 defines the following task classes:

| Task class | Meaning |
| --- | --- |
| `robot.observe` | Sense an environment without extracting durable facts or acting. |
| `robot.inspect` | Analyze a bounded object, zone, condition, hazard, inventory, or state. |
| `robot.navigate` | Move through allowed zones without manipulation. |
| `robot.fetch` | Locate, grasp, and bring a permitted object. |
| `robot.carry` | Transport a permitted object between allowed zones. |
| `robot.sort` | Classify and place permitted objects into allowed destinations. |
| `robot.clean` | Perform bounded cleaning or tidying tasks. |
| `robot.assist_person` | Physically assist or interact with a person under strict policy. |
| `robot.operate_device` | Operate switches, appliances, doors, locks, tools, controls, or machinery. |
| `robot.security_patrol` | Observe or inspect for safety/security anomalies under explicit privacy rules. |
| `robot.remote_supervised_action` | Execute a robot action with remote human or vendor supervision. |

Policy may add narrower task classes. Mixed tasks inherit the strictest
approval, safety, egress, and receipt requirements among their component
classes.

## 7. Cloud Policy

Cloud reasoning, vendor telemetry, robot fleet learning, diagnostics, remote
operator access, and external model calls are egress. They require explicit
authorization through [NET-01](/blueprint/30-specs/network-gate-and-anonymity.md)
and must preserve payload class, destination, purpose, redaction mode, budget,
and receipt metadata.

The default posture is local-first:

- private household memory is not sent to robot vendors by default,
- raw video, raw audio, private maps, and full sensor logs are denied for cloud
  upload unless policy explicitly permits the payload class,
- cloud planning may receive bounded task summaries and redacted refs only when
  the egress grant permits them,
- fleet learning may use compact, de-identified, policy-approved summaries only
  when the principal grants that use,
- remote supervision may not expand the original mandate or introduce new
  actuation powers,
- vendor clouds and remote operators cannot bypass Goni receipts.

If cloud reasoning is unavailable or denied, the robot action must either run
locally within its mandate, ask for approval, degrade to observe-only or
proposal mode, or block.

## 8. Safety Posture

Physical actuation is default-deny. Goni MUST require stricter gates for:

- people-facing actions,
- medical, health, eldercare, childcare, accessibility, or physical assistance
  tasks,
- security patrol, monitoring, access control, doors, locks, windows, alarms,
  and private areas,
- stove, oven, appliance, machinery, vehicle, tool, electrical, chemical, heat,
  water, fire, or hazardous-material actions,
- irreversible property actions such as disposal, destruction, movement to
  inaccessible areas, or actions with no reliable repair path,
- financial, purchasing, delivery, contract, or payment-linked robot actions,
- actions around children, pets, guests, workers, bystanders, or sleeping or
  incapacitated people.

Required safety controls include:

- revocation and emergency stop paths,
- denied-zone fail-closed behavior,
- explicit approval for high-risk actions,
- local policy mediation before command execution,
- verification before memory update or task completion,
- receipt-linked incident, escalation, denial, or intervention status,
- bounded rollback, repair, compensation, or "no rollback available" metadata.

When required safety, approval, supervision, or receipt support is missing,
execution fails closed.

## 9. Tool And Receipt Interface

Robot skill calls are TOOL-01 tool calls. They must preserve:

- `boundary_basis`,
- `work_order_id`,
- `done_contract_hash`,
- `autonomy_mode`,
- `risk_score`,
- `risk_basis`,
- `capability_token_id`,
- `idempotency_key` for mutating calls,
- `undo_strategy_ref` or explicit no-rollback metadata,
- `task_class` with a `robot.*` value when robot behavior affects the action.

Receipts for robot-mediated actions may include `robot_basis`, analogous to
`visual_basis`. `robot_basis` extends REC-01 and is not a separate receipt
type. It stores compact refs and hashes rather than raw sensor streams, full
video, raw audio, private household maps, unrestricted telemetry, or
unbounded transcripts.

Robot adapters are external execution substrates. Their logs can support
evidence, but they cannot be the only terminal record of a mediated physical
effect.

## 10. Financial And Commercial Linkage

When a robot action searches for goods or services, negotiates, contracts,
reserves budget, purchases, pays, accepts delivery terms, or creates a
financially binding commitment, the action MUST also satisfy
[DAT-01](/blueprint/30-specs/delegated-agent-treasury.md).

Examples include:

- a home robot ordering supplies,
- a warehouse robot paying for an API or task-specific service,
- a service robot accepting delivery or repair terms,
- a robot booking external maintenance,
- a robot choosing between vendors for replacement parts.

The robot mandate does not create financial authority. Financial authority
requires a Delegated Agent Treasury, Negotiation Mandate, budget reservation,
approval thresholds, and settlement receipt path under DAT-01.

## 11. Market Rationale

The market rationale for ROBOT-01 is non-normative. Humanoid and embodied
robotics appear to be splitting into a capability race and a manufacturing
race. US and European companies often lead the AI narrative, commercial pilot
story, and enterprise-partnership framing, while Chinese companies appear
strong in low-cost hardware, shipment volume, and state-backed
industrialization.

This supports, but does not prove, the Goni thesis:

```text
Robot hardware may become more available and less differentiated.
Private context, local memory, policy, and trust remain scarce.
```

In this framing, Goni is the sovereign command center for robots, devices,
documents, and daily life. Robot vendors may own the body, but the principal's
local AI should own memory, permission, context, and judgment.

## 12. Invariants

- Robot actuation requires a Work Order, Done Contract, Robot Mandate,
  capability token, policy decision, and receipt path.
- Robot observation does not imply extraction, memory, cloud upload,
  supervision, or actuation.
- Physical actuation is default-deny.
- Private maps, raw video, raw audio, full sensor logs, and unrestricted
  telemetry are not stored in receipts by default.
- Vendor cloud, fleet learning, diagnostics upload, and remote supervision
  require explicit egress grants through NET-01.
- Remote supervisors cannot expand the principal's mandate.
- Movement into denied zones fails closed and remains auditable.
- People-facing and high-risk physical actions require stricter gates than
  object-only logistics actions.
- Emergency stop, revocation, expiry, or superseded mandate state prevents
  future robot actuation.
- Financial robot actions also satisfy DAT-01.
- Third-party robot logs cannot replace canonical Goni receipts.

## 13. Related Specs

- [Delegation interface](/blueprint/30-specs/delegation-interface.md)
- [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md)
- [Vision, memory, and actuation boundaries](/blueprint/30-specs/vision-memory-actuation-boundaries.md)
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Delegated Agent Treasury](/blueprint/30-specs/delegated-agent-treasury.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)

## 14. Downstream

- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Policy schema](/blueprint/schemas/policy/policy.schema.json)
- [Receipt schema](/blueprint/schemas/receipts/receipt.schema.json)

## 15. External References

- [Goldman Sachs: humanoid robot market could reach $38B by 2035](https://www.goldmansachs.com/insights/articles/the-global-market-for-robots-could-reach-38-billion-by-2035)
- [Morgan Stanley: humanoid robots could reach a $5T market by 2050](https://www.morganstanley.com/ideas/humanoid-robots)
- [Agility Robotics: Digit moved over 100,000 totes](https://www.agilityrobotics.com/content/digit-moves-over-100k-totes)
- [Apptronik: $520M Series A-X with Google and Mercedes-Benz participation](https://finance.yahoo.com/news/apptronik-closes-over-935-million-140000689.html)
- [Boston Dynamics: Atlas humanoid robot](https://bostondynamics.com/products/atlas/)
- [AgiBot/Omdia shipment-share claim](https://www.agibot.com/article/231/detail/33.html)
- [Unitree/PRNewswire 2025 shipment claim](https://www.prnewswire.com/news-releases/unitree-ranks-no1-globally-in-humanoid-robot-shipments-exceeding-5-500-units-in-2025--302674729.html)

## Conformance Tests

- Robot actuation cannot occur without Work Order, Done Contract, Robot
  Mandate, capability token, and receipt path.
- Observation does not imply extraction, memory, cloud upload, or actuation.
- Private home maps, raw video, raw audio, and full sensor logs are omitted
  from receipts by default.
- Vendor cloud or fleet-learning upload is denied without explicit egress and
  payload classification.
- Robot movement into denied zones fails closed and emits an auditable denial.
- People-facing or high-risk physical actions require stricter approval than
  object-only logistics actions.
- Remote supervision cannot expand the original principal mandate.
- Financial or purchasing actions performed by a robot also satisfy DAT-01.
- Emergency stop, revocation, or mandate expiry prevents future robot
  actuation.
- Third-party robot logs are insufficient as the only evidence for a mediated
  physical action.
