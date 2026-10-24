---
id: GONI-SYNTHESIS-20260916-GLOBAL-RADAR
title: Global Goni Intelligence Radar - Snapshot 2026-09-16
type: synthesis
status: draft
implementation_state: not_applicable
proposition: This 2026-09-16 snapshot maps global communities, engineering projects, research venues, publications, model ecosystems, hardware ecosystems, and regional AI communities relevant to Goni across local inference, agents, memory, sovereignty, edge deployment, security, and adjacent systems research.
domains:
- research
- ecosystem
- market-intelligence
- local-ai
- inference
- agents
- hardware
aliases:
- Global Goni Intelligence Radar
- Goni research radar
relations:
- type: extends
  target: ADJACENT-PROJECTS
  note: Adds a time-stamped monitoring surface spanning communities, projects, publications, and regional ecosystems.
sources: []
artifacts: []
uncertainty: This is a point-in-time ecosystem snapshot. Community activity, ownership, URLs, model families, project scope, and relative strategic importance can change rapidly; each entry therefore carries explicit added and last-reviewed dates and should be re-audited before operational reliance.
legacy: []
---

# Global Goni Intelligence Radar

> **Snapshot date:** 2026-09-16  
> **Timezone:** Europe/Zurich  
> **Audit state:** point-in-time research snapshot  
> **Temporal rule:** every source below records when it was first added and when it was last reviewed. Future audits should update the individual `Last reviewed` field and append to the change log rather than silently removing historical context.

## Purpose

Goni sits at the intersection of local inference, open-weight models, agent orchestration, persistent memory, tool mediation, AI security, heterogeneous compute, edge deployment, sovereign infrastructure, and personal-AI product design. No single geographic ecosystem or publication captures that whole design space.

The radar therefore treats Chinese, North American, European, Japanese, Korean, Taiwanese, Singaporean, Indian, Middle Eastern, African, and Latin American ecosystems as parallel sources of technical evidence rather than organizing the field as a Western core plus regional alternatives.

## Priority model

- **P0 - continuous/core:** developments here can materially change Goni architecture, model/runtime selection, hardware assumptions, or category strategy.
- **P1 - regular:** review at least weekly or during relevant architecture/product work.
- **P2 - discovery/context:** useful for weak signals and ecosystem awareness, but generally too broad or noisy to serve as primary evidence.

## Evidence weighting

For architectural decisions, prefer evidence in roughly this order:

1. primary code, model releases, technical documentation, papers, and reproducible benchmarks;
2. maintainer discussions, issue trackers, engineering talks, and specialist workshops;
3. research/engineering publications with transparent methodology;
4. specialist community discussion;
5. media and broad social discussion as discovery surfaces rather than decision-grade evidence.

---

## 1. Reddit and broad practitioner communities

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [r/LocalLLaMA](https://www.reddit.com/r/LocalLLaMA/) | P0 | Open-weight releases, GGUF, quantization, consumer hardware, benchmarks, local-agent experiments | 2026-09-16 | 2026-09-16 |
| [r/LocalLLM](https://www.reddit.com/r/LocalLLM/) | P0 | Practical local deployments, privacy, local coding, self-hosted experimentation | 2026-09-16 | 2026-09-16 |
| [r/llamacpp](https://www.reddit.com/r/llamacpp/) | P0 | llama.cpp, GGUF, CPU/GPU inference, optimization | 2026-09-16 | 2026-09-16 |
| [r/StrixHalo](https://www.reddit.com/r/StrixHalo/) | P0 | AMD Strix Halo, unified memory, ROCm/Vulkan, very large local models | 2026-09-16 | 2026-09-16 |
| [r/ROCm](https://www.reddit.com/r/ROCm/) | P0 | AMD compute, non-CUDA inference, kernel/runtime issues | 2026-09-16 | 2026-09-16 |
| [r/ollama](https://www.reddit.com/r/ollama/) | P1 | Local serving, model management, tool use, deployment UX | 2026-09-16 | 2026-09-16 |
| [r/selfhosted](https://www.reddit.com/r/selfhosted/) | P1 | Sovereignty, local-first infrastructure, privacy, dependency minimization | 2026-09-16 | 2026-09-16 |
| [r/HomelabAI](https://www.reddit.com/r/HomelabAI/) | P1 | Local AI rigs, VRAM/RAM economics, MCP, RAG, agents | 2026-09-16 | 2026-09-16 |
| [r/RAG](https://www.reddit.com/r/RAG/) | P1 | Retrieval, memory, reranking, vector/database architecture | 2026-09-16 | 2026-09-16 |
| [r/OpenWebUI](https://www.reddit.com/r/OpenWebUI/) | P1 | Local AI UX, RAG, tools, MCP, identity and deployment problems | 2026-09-16 | 2026-09-16 |
| [r/AI_Agents](https://www.reddit.com/r/AI_Agents/) | P2 | Agent products, orchestration frameworks and emerging patterns | 2026-09-16 | 2026-09-16 |
| [r/MachineLearning](https://www.reddit.com/r/MachineLearning/) | P2 | Broad ML research and paper discovery | 2026-09-16 | 2026-09-16 |
| [r/homeassistant](https://www.reddit.com/r/homeassistant/) | P2 | Ambient/local assistants, local voice, persistent environmental state | 2026-09-16 | 2026-09-16 |

---

## 2. Global engineering and runtime sources

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [llama.cpp Discussions](https://github.com/ggml-org/llama.cpp/discussions) | P0 | GGUF, edge/local inference, quantization, long context, CPU/GPU offload, speculative decoding | 2026-09-16 | 2026-09-16 |
| [Hugging Face](https://huggingface.co/) | P0 | Open-weight models, datasets, implementations and release discovery | 2026-09-16 | 2026-09-16 |
| [Hugging Face Forums](https://discuss.huggingface.co/) | P0 | Fine-tuning, inference, agents, model integration and deployment | 2026-09-16 | 2026-09-16 |
| [Hugging Face Daily Papers](https://huggingface.co/papers) | P0 | Fast research discovery across models, agents, context, efficiency and systems | 2026-09-16 | 2026-09-16 |
| [vLLM](https://github.com/vllm-project/vllm) | P0 | High-performance LLM serving and accelerator integration | 2026-09-16 | 2026-09-16 |
| [SGLang](https://github.com/sgl-project/sglang) | P0 | Serving, structured generation, agent/model runtime infrastructure | 2026-09-16 | 2026-09-16 |
| [GPU MODE](https://gpu-mode.github.io/) | P0 | GPU kernels, CUDA/Triton, low-level performance and inference optimization | 2026-09-16 | 2026-09-16 |
| [Ollama](https://github.com/ollama/ollama) | P1 | Local serving, model packaging and developer ergonomics | 2026-09-16 | 2026-09-16 |
| [Open WebUI Discussions](https://github.com/open-webui/open-webui/discussions) | P1 | Real-world local AI product, RAG, identity, tools and deployment failure modes | 2026-09-16 | 2026-09-16 |
| [EleutherAI](https://www.eleuther.ai/) | P1 | Open-model research and technical community | 2026-09-16 | 2026-09-16 |
| [Hacker News](https://news.ycombinator.com/) | P1 | Early discovery of runtimes, agents, local systems, research and startups | 2026-09-16 | 2026-09-16 |

---

## 3. Research, publications and technical analysis

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [MLSys](https://mlsys.org/) | P0 | ML systems, inference, scheduling, hardware, agents and deployment | 2026-09-16 | 2026-09-16 |
| [MLSys Proceedings](https://proceedings.mlsys.org/) | P0 | Primary papers on ML systems and efficient inference | 2026-09-16 | 2026-09-16 |
| [arXiv cs.DC](https://arxiv.org/list/cs.DC/recent) | P0 | Distributed computing, heterogeneous execution, systems architecture | 2026-09-16 | 2026-09-16 |
| [arXiv cs.AI](https://arxiv.org/list/cs.AI/recent) | P0 | Agents, planning, reasoning and AI systems | 2026-09-16 | 2026-09-16 |
| [arXiv cs.CL](https://arxiv.org/list/cs.CL/recent) | P0 | Language models, reasoning, context, memory and model architecture | 2026-09-16 | 2026-09-16 |
| [SemiAnalysis](https://semianalysis.com/) | P0 | AI hardware, datacenters, inference economics and systems analysis | 2026-09-16 | 2026-09-16 |
| [InferenceX](https://inferencex.semianalysis.com/) | P0 | Hardware/runtime inference benchmarking | 2026-09-16 | 2026-09-16 |
| [Sebastian Raschka / Ahead of AI](https://sebastianraschka.com/) | P1 | Open-weight architecture, reasoning, inference and local coding agents | 2026-09-16 | 2026-09-16 |
| [Simon Willison](https://simonwillison.net/tags/llms/) | P1 | Practical LLM engineering, tool use, agents, local models and security | 2026-09-16 | 2026-09-16 |
| [Latent.Space](https://www.latent.space/) | P1 | AI engineering, infrastructure, models and agent ecosystem | 2026-09-16 | 2026-09-16 |
| [LessWrong - AI Control](https://www.lesswrong.com/tag/ai-control) | P1 | Delegation, containment, control and agent-safety concepts | 2026-09-16 | 2026-09-16 |
| [Alignment Forum](https://www.alignmentforum.org/) | P1 | Technical alignment/control and agent-safety research | 2026-09-16 | 2026-09-16 |
| [DeepLearning.AI - The Batch](https://www.deeplearning.ai/the-batch/) | P2 | Broad industry/research filtering and trend discovery | 2026-09-16 | 2026-09-16 |

---

## 4. China - primary technical ecosystem

### 4.1 Infrastructure, inference and local AI

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [ModelScope / 魔搭社区](https://www.modelscope.cn/) | P0 | Chinese open-model hub, datasets, deployment, agents and developer community | 2026-09-16 | 2026-09-16 |
| [ModelScope ms-swift](https://github.com/modelscope/ms-swift) | P0 | Training, fine-tuning and inference across Chinese/global model families | 2026-09-16 | 2026-09-16 |
| [KTransformers / 清华 MADSys](https://github.com/kvcache-ai/ktransformers) | P0 | CPU-GPU heterogeneous inference, MoE placement and RAM-heavy local execution | 2026-09-16 | 2026-09-16 |
| [OpenBMB / 面壁智能](https://www.openbmb.cn/) | P0 | Efficient/local models, agent research, multimodality and edge deployment | 2026-09-16 | 2026-09-16 |
| [AgentCPM](https://github.com/OpenBMB/AgentCPM) | P0 | Edge-scale/local agents, long-horizon tool use, agent training and evaluation | 2026-09-16 | 2026-09-16 |
| [MiniCPM](https://github.com/OpenBMB/MiniCPM) | P0 | Small on-device language models and efficient local inference | 2026-09-16 | 2026-09-16 |
| [MiniCPM-V](https://github.com/OpenBMB/MiniCPM-V) | P0 | Efficient multimodal local/on-device models | 2026-09-16 | 2026-09-16 |
| [MindSpore / 昇思](https://www.mindspore.cn/) | P0 | Chinese AI framework, heterogeneous execution and accelerator integration | 2026-09-16 | 2026-09-16 |
| [Huawei Ascend / 昇腾](https://www.hiascend.com/) | P0 | Domestic NPU stack and non-NVIDIA model execution | 2026-09-16 | 2026-09-16 |
| [Alibaba MNN](https://github.com/alibaba/MNN) | P0 | Mobile, PC and IoT inference with strong edge/local focus | 2026-09-16 | 2026-09-16 |
| [LMDeploy](https://github.com/InternLM/lmdeploy) | P0 | Deployment, quantization and serving in the OpenMMLab/InternLM ecosystem | 2026-09-16 | 2026-09-16 |
| [Xinference / Xorbits](https://github.com/xorbitsai/inference) | P1 | Local/distributed inference abstraction across model families | 2026-09-16 | 2026-09-16 |
| [OpenI / 启智社区](https://www.openi.org.cn/) | P1 | Chinese open AI development, models, datasets and heterogeneous compute | 2026-09-16 | 2026-09-16 |
| [OpenCompass](https://github.com/open-compass/opencompass) | P1 | Chinese/global model evaluation and benchmarking | 2026-09-16 | 2026-09-16 |
| [LLaMA-Factory](https://github.com/hiyouga/LLaMA-Factory) | P1 | Fine-tuning and adaptation across many open model families | 2026-09-16 | 2026-09-16 |
| [InternLM](https://github.com/InternLM) | P1 | Models, deployment and agent research from the Shanghai AI Lab ecosystem | 2026-09-16 | 2026-09-16 |

### 4.2 Chinese model laboratories and company feeds

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [DeepSeek](https://github.com/deepseek-ai) | P0 | MoE, reasoning, inference efficiency, open model releases | 2026-09-16 | 2026-09-16 |
| [Qwen / 通义千问](https://github.com/QwenLM) | P0 | Small-to-large open models, multimodality, tools and agents | 2026-09-16 | 2026-09-16 |
| [Moonshot AI / Kimi](https://www.moonshot.cn/) | P0 | Long context, agents, model architecture and product/runtime directions | 2026-09-16 | 2026-09-16 |
| [Zhipu AI / GLM / 智谱](https://github.com/THUDM) | P0 | GLM models, agents and Chinese model ecosystem | 2026-09-16 | 2026-09-16 |
| [MiniMax](https://www.minimaxi.com/) | P0 | Foundation models, multimodality and agentic systems | 2026-09-16 | 2026-09-16 |
| [InternLM / 书生](https://internlm.intern-ai.org.cn/) | P1 | Shanghai AI Lab open-model ecosystem | 2026-09-16 | 2026-09-16 |
| [Baichuan](https://www.baichuan-ai.com/) | P1 | Chinese foundation models and enterprise deployment | 2026-09-16 | 2026-09-16 |
| [StepFun / 阶跃星辰](https://www.stepfun.com/) | P1 | Foundation models and multimodal development | 2026-09-16 | 2026-09-16 |

### 4.3 Chinese publications and communities

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [InfoQ 中国 / 极客邦](https://www.infoq.cn/) | P0 | Production engineering, agents, architecture, databases and inference | 2026-09-16 | 2026-09-16 |
| [机器之心 / Synced](https://www.jiqizhixin.com/) | P0 | Chinese research, laboratories, papers and technical AI coverage | 2026-09-16 | 2026-09-16 |
| [量子位 / QbitAI](https://www.qbitai.com/) | P1 | Fast-moving Chinese research, startup and hardware coverage | 2026-09-16 | 2026-09-16 |
| [新智元 / AI Era](https://aiera.com.cn/) | P1 | Broad early-warning source for Chinese labs, hardware and startups | 2026-09-16 | 2026-09-16 |
| [WaytoAGI / 通往 AGI 之路](https://www.waytoagi.com/) | P1 | Builder/product community and agent experimentation | 2026-09-16 | 2026-09-16 |
| [知乎 / Zhihu](https://www.zhihu.com/) | P1 | Specialist engineering essays, project teams and technical discussion | 2026-09-16 | 2026-09-16 |
| [哔哩哔哩 / Bilibili](https://www.bilibili.com/) | P1 | Engineering talks, tutorials, SIG meetings and project presentations | 2026-09-16 | 2026-09-16 |
| WeChat technical groups | P0* | Project-level discussion, maintainer communities and release support; monitor selected official groups rather than the whole platform | 2026-09-16 | 2026-09-16 |
| Feishu / Lark technical groups | P0* | Developer/project communities, particularly OpenBMB and adjacent ecosystems | 2026-09-16 | 2026-09-16 |
| DingTalk technical groups | P1* | Framework/project support communities | 2026-09-16 | 2026-09-16 |
| [Gitee](https://gitee.com/) | P1 | Chinese code hosting and domestic open-source discovery | 2026-09-16 | 2026-09-16 |

`*` Platform-level source. Track selected official project groups, not the platform indiscriminately.

---

## 5. Japan

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [LLM-jp](https://llm-jp.nii.ac.jp/en/home-en/) | P0 | Japanese national/research LLM community, open models, corpora and tools | 2026-09-16 | 2026-09-16 |
| [LLM-jp resources and meetings](https://llm-jp.nii.ac.jp/en/resources-en/) | P0 | Working groups, technical talks, benchmarks, safety, agents and multimodality | 2026-09-16 | 2026-09-16 |
| [Preferred Networks / PLaMo](https://www.preferred.jp/en/projects/plamo/) | P1 | Japanese open/on-prem models and domestic AI infrastructure | 2026-09-16 | 2026-09-16 |
| [Sakana AI](https://sakana.ai/) | P1 | Novel model composition, agent/research architectures and frontier AI | 2026-09-16 | 2026-09-16 |
| [Qiita](https://qiita.com/) | P2 | Japanese developer and implementation discovery | 2026-09-16 | 2026-09-16 |
| [Zenn](https://zenn.dev/) | P2 | Japanese engineering articles and implementation notes | 2026-09-16 | 2026-09-16 |

---

## 6. South Korea

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [vLLM KR Community](https://vllm.ai/blog/tags/community) | P0 | Production inference, serving, accelerator integration and Korean deployment practice | 2026-09-16 | 2026-09-16 |
| [PyTorch Korea](https://pytorch.kr/) | P1 | Korean ML systems/developer ecosystem | 2026-09-16 | 2026-09-16 |
| [Rebellions](https://rebellions.ai/) | P1 | Korean AI accelerator hardware and sovereign compute | 2026-09-16 | 2026-09-16 |
| [SqueezeBits](https://www.squeezebits.com/) | P1 | LLM inference optimization and accelerator/software efficiency | 2026-09-16 | 2026-09-16 |
| [LG AI Research / EXAONE](https://www.lgresearch.ai/) | P1 | Korean foundation/open models and applied research | 2026-09-16 | 2026-09-16 |
| [NAVER / HyperCLOVA X](https://clova.ai/) | P1 | Korean foundation models and domestic ecosystem | 2026-09-16 | 2026-09-16 |
| [Upstage / Solar](https://www.upstage.ai/) | P1 | Efficient/open model development and deployment | 2026-09-16 | 2026-09-16 |
| [KoreanLLM.org](https://koreanllm.org/) | P2 | Independent Korean-model comparison and discovery | 2026-09-16 | 2026-09-16 |

---

## 7. Taiwan

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [TAIDE](https://www.taide.tw/) | P0 | Taiwan sovereign/open LLM programme and Traditional Chinese models | 2026-09-16 | 2026-09-16 |
| [Twinkle AI](https://twinkleai.tw/en/about) | P1 | Traditional-Chinese open-source model community and local datasets | 2026-09-16 | 2026-09-16 |
| [Twinkle Hub](https://twinkleai.tw/en/news) | P1 | MCP, local/government data and AI workflow integration | 2026-09-16 | 2026-09-16 |
| [Taiwan AI Labs](https://ailabs.tw/) | P1 | Local models, privacy, applied AI and Taiwanese AI infrastructure | 2026-09-16 | 2026-09-16 |

---

## 8. Singapore and Southeast Asia

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [AI Singapore](https://aisingapore.org/) | P0 | Regional AI infrastructure, open research and model programmes | 2026-09-16 | 2026-09-16 |
| [SEA-LION](https://sea-lion.ai/) | P0 | Open multilingual models for Southeast Asian languages and contexts | 2026-09-16 | 2026-09-16 |
| [SEA-LION GitHub](https://github.com/aisingapore/sealion) | P0 | Models, code, evaluation and implementation evidence | 2026-09-16 | 2026-09-16 |
| [AIRS 2026](https://airs-workshop.github.io/) | P1 | Agentic AI as a systems workload: orchestration, serving, memory, security and deployment | 2026-09-16 | 2026-09-16 |
| [ClawCon Singapore](https://clawcon.com/) | P2 | Agent-builder and product ecosystem discovery | 2026-09-16 | 2026-09-16 |
| [sgai.md](https://sgai.md/) | P2 | Singapore AI community/resource directory | 2026-09-16 | 2026-09-16 |
| [Jan / Homebrew Computer Company](https://jan.ai/) | P1 | Local-first desktop AI and private model execution | 2026-09-16 | 2026-09-16 |

---

## 9. Europe and Switzerland

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [OpenEuroLLM](https://www.openeurollm.eu/) | P0 | Transparent multilingual European foundation models and sovereign AI | 2026-09-16 | 2026-09-16 |
| [OpenEuroLLM GitHub](https://github.com/OpenEuroLLM) | P0 | Training, evaluation, post-training and data infrastructure | 2026-09-16 | 2026-09-16 |
| [Apertus](https://www.apertus-ai.org/) | P0 | Swiss fully open foundation model and sovereign-AI stack | 2026-09-16 | 2026-09-16 |
| [Swiss AI GitHub](https://github.com/swiss-ai) | P0 | Apertus training, data, fine-tuning and serving artifacts | 2026-09-16 | 2026-09-16 |
| [Mistral AI](https://mistral.ai/) | P0 | European open/open-weight models and inference ecosystem | 2026-09-16 | 2026-09-16 |
| [ETH AI Center](https://ai.ethz.ch/) | P1 | Swiss AI research, systems and applied engineering | 2026-09-16 | 2026-09-16 |
| [EPFL AI Center](https://ai.epfl.ch/) | P1 | Swiss AI research and sovereign/open-model ecosystem | 2026-09-16 | 2026-09-16 |
| [ELLIS](https://ellis.eu/) | P1 | European machine-learning research network | 2026-09-16 | 2026-09-16 |

---

## 10. India

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [AI4Bharat](https://ai4bharat.iitm.ac.in/) | P0 | Open multilingual Indian AI, datasets, models and tooling | 2026-09-16 | 2026-09-16 |
| [AI4Bharat GitHub](https://github.com/AI4Bharat) | P0 | Primary implementation and dataset evidence | 2026-09-16 | 2026-09-16 |
| [BharatGen](https://bharatgen.com/) | P1 | Indian sovereign/generative AI and multilingual foundation models | 2026-09-16 | 2026-09-16 |
| Indian Indic-model developer communities | P2 | Regional model, benchmark and deployment discovery | 2026-09-16 | 2026-09-16 |

---

## 11. Middle East

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [Technology Innovation Institute / Falcon](https://falconllm.tii.ae/) | P0 | Open foundation models, efficient models and sovereign AI | 2026-09-16 | 2026-09-16 |
| [MBZUAI](https://mbzuai.ac.ae/) | P1 | Models, agents and AI research | 2026-09-16 | 2026-09-16 |
| [Core42](https://www.core42.ai/) | P1 | Sovereign compute, model deployment and regional AI infrastructure | 2026-09-16 | 2026-09-16 |

---

## 12. Africa

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [Masakhane](https://www.masakhane.io/) | P0 | African-language NLP/LLM research and grassroots open community | 2026-09-16 | 2026-09-16 |
| [Masakhane GitHub](https://github.com/masakhane-io) | P1 | Models, datasets and implementations | 2026-09-16 | 2026-09-16 |
| Masakhane community / Slack | P1 | Distributed African NLP/LLM research community | 2026-09-16 | 2026-09-16 |

---

## 13. Latin America and Iberian-language ecosystem

| Source | Priority | Goni signal | Added | Last reviewed |
| --- | --- | --- | --- | --- |
| [SomosNLP](https://somosnlp.org/en) | P1 | Spanish-language open NLP/LLM research, datasets, evaluation and community | 2026-09-16 | 2026-09-16 |
| [SomosNLP resources](https://somosnlp.org/recursos) | P1 | Spanish open models, datasets, courses and research initiatives | 2026-09-16 | 2026-09-16 |
| Comunidad IA LATAM | P2 | Regional builder/startup discovery surface | 2026-09-16 | 2026-09-16 |
| Regional Hugging Face communities | P2 | Model, dataset and benchmark discovery | 2026-09-16 | 2026-09-16 |

---

## 14. Goni core monitoring funnel

The complete radar provides coverage. It should not be polled uniformly.

### Models

P0/P1 emphasis: DeepSeek, Qwen, OpenBMB/MiniCPM, Kimi, GLM, MiniMax, SEA-LION, LLM-jp, Apertus, OpenEuroLLM, Mistral and Falcon.

### Inference and serving

P0/P1 emphasis: llama.cpp, KTransformers, MNN, vLLM, SGLang, LMDeploy, Xinference, MindSpore/Ascend, ROCm, GPU MODE, Rebellions and SqueezeBits.

### Local hardware and heterogeneous compute

P0/P1 emphasis: r/StrixHalo, r/ROCm, GPU MODE, SemiAnalysis/InferenceX, Ascend, Rebellions and hardware-specific runtime discussions.

### Memory, context and retrieval

P0/P1 emphasis: r/RAG, Hugging Face Daily Papers, arXiv cs.CL/cs.AI/cs.DC, MLSys and project-specific memory/RAG research discovered through the broader radar.

### Agents and orchestration

P0/P1 emphasis: AgentCPM, ModelScope, OpenBMB, Open WebUI, Latent.Space, AIRS, WaytoAGI and newly emerging agent-OS/delegation projects.

### Authority, security and delegation

P0/P1 emphasis: AI Control research, Alignment Forum, relevant LessWrong technical work, agent sandboxing, capability security, tool mediation, authenticated delegation and formal systems/security literature.

### Community early warning

P0/P1 emphasis: r/LocalLLaMA, Hacker News, InfoQ China, 机器之心, 量子位, LLM-jp, ModelScope community and selected official WeChat/Feishu groups.

## 15. Monitoring principle

The radar is organized by **technical function with parallel geographic ecosystems**, not as a Western stack plus regional alternatives. For example, an inference review should compare llama.cpp, vLLM, SGLang, KTransformers, LMDeploy, MNN, MindSpore, ROCm, Rebellions and SqueezeBits on the same problem surface.

The research question is therefore:

> Which architecture, model, runtime, hardware substrate, memory design, or governance mechanism best solves the relevant Goni constraint under current evidence?

rather than:

> Which non-Western tool is equivalent to the incumbent Western tool?

This reduces geographic and technical-selection bias and is especially important for Goni because advances in heterogeneous inference, efficient edge models, sovereign stacks, multilingual systems and local agents are distributed across several largely separate ecosystems.

## 16. Update protocol

For each future audit:

1. preserve `Added` as the first date the source entered this radar;
2. update `Last reviewed` only after checking the source again;
3. mark dormant, archived, merged, renamed or redirected sources explicitly rather than silently deleting them;
4. add new sources with the current date in both date columns;
5. change priorities only with a short reason in the change log;
6. prefer stable project/community landing pages over transient individual posts;
7. preserve historical snapshots through Git history and the dated change log below;
8. treat community/social/media sources as discovery inputs and verify consequential architectural claims against primary material.

## 17. Change log

| Date | Change |
| --- | --- |
| 2026-09-16 | Initial global radar created from the audited Goni ecosystem scan. Expanded beyond the original English-language/local-LLM focus to include China, Japan, South Korea, Taiwan, Singapore/Southeast Asia, Europe/Switzerland, India, the Middle East, Africa and Latin America/Iberian-language ecosystems. Added per-source temporal fields so future audits remain historically legible. |
