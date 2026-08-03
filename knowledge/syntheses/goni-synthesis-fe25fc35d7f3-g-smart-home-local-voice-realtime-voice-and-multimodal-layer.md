---
id: GONI-SYNTHESIS-FE25FC35D7F3
title: G) Smart-home, local voice, realtime voice, and multimodal layer
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Specified design intent: | Project | Confidence | Goni relevance | | Home Assistant / Nabu Casa | verified | Smart-home integration baseline and practical voice/home ecosystem.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: G) Smart-home, local voice, realtime voice, and multimodal layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# G) Smart-home, local voice, realtime voice, and multimodal layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### G) Smart-home, local voice, realtime voice, and multimodal layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| Home Assistant / Nabu Casa | `verified` | Smart-home integration baseline and practical voice/home ecosystem. |
| Home Assistant Voice Assist | `verified` | Native HA voice path. |
| Rhasspy | `stale/deprecated` | Offline voice assistant lineage; much of the practical path moved into HA/Wyoming. |
| OpenVoiceOS | `verified` | Privacy-oriented open voice platform. |
| Willow | `verified` | ESP32-S3/local voice hardware path. |
| Wyoming | `verified` | Protocol/integration layer for STT/TTS/wake-word services. |
| Whisper / Faster Whisper | `verified` | Local speech-to-text candidates. |
| Piper / Piper TTS | `verified` | Local text-to-speech candidate. |
| openWakeWord | `verified` | Local wake-word detection candidate. |
| LiveKit | `verified` | Realtime voice/video agent infrastructure. |
| Pipecat | `verified` | Realtime conversational AI/voice agent framework. |
| Daily | `verified` | Realtime voice/video infrastructure supplier. |
| ESPHome / ESP32 voice satellites | `verified` | Low-cost local voice satellite hardware path. |
| Linux Voice Assistant | `needs verification` | Possible Wyoming/HA-adjacent voice satellite lineage; verify exact project. |
| Leon | `verified` | Open-source personal assistant reference. |
| Home Guardian | `needs verification` | Academic/offline smart-home voice prototype; verify primary source before relying. |
| Mycroft AI | `stale/deprecated` | Important voice-assistant lineage; OpenVoiceOS is the more current path. |
| Snips | `stale/deprecated` | France-origin privacy voice lineage; acquired/legacy status. |
| Mingyuyue | `candidate/unverified` | Chinese modular voice framework claim needs official source check. |
| wukong-robot | `verified` | Chinese voice assistant/smart speaker project with HA/MQTT relevance. |

Goni implication:

- Home Assistant is the practical smart-home substrate.
- Wyoming, Piper, Whisper/Faster Whisper, and openWakeWord are the modular
  local voice stack.
- LiveKit/Pipecat are separate realtime-agent infrastructure and should not be
  collapsed into smart-home automation.
