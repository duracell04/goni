# First-party LLM Chats - Supplier / Ecosystem Card

Snapshot date: 2026-05-13

## What it is
This supplier card tracks first-party chat products backed primarily by the
provider's own model family. It is market intelligence, not Goni routing policy.

This list is not exhaustive and changes quickly. Consumer chat products,
branding, availability, and model routing policies should be rechecked before
using this card for decisions.

This is not a list of AI wrappers, aggregators, multi-model clients, or local
frontends. Products such as Poe, You.com, OpenRouter chat surfaces, Perplexity,
Open WebUI, and LM Studio are excluded unless the product is primarily backed by
the provider's own model family.

## Products to track

| Chat | Company | Model family | Status / notes | Source |
|---|---|---|---|---|
| ChatGPT | OpenAI | GPT / OpenAI models | Verified from primary source. First-party OpenAI assistant. | https://chatgpt.com/ |
| Claude | Anthropic | Claude | Verified from primary source. First-party Anthropic assistant. | https://claude.com/product/overview |
| Gemini | Google | Gemini | Verified from primary source. First-party Google assistant. | https://gemini.google/about/ |
| Grok | xAI | Grok | Verified from primary source. First-party xAI assistant. | https://x.ai/grok |
| DeepSeek | DeepSeek | DeepSeek | Verified from primary source. First-party DeepSeek chat. | https://chat.deepseek.com/ |
| Kimi | Moonshot AI | Kimi | Verified from primary source. First-party Moonshot AI assistant. | https://www.kimi.com/help/getting-started/overview |
| Le Chat | Mistral AI | Mistral models | Verified from primary source. First-party Mistral AI assistant. | https://docs.mistral.ai/le-chat/overview |
| Meta AI | Meta | Llama / Meta AI models | Verified from primary source. First-party Meta assistant. | https://ai.meta.com/meta-ai/ |
| Qwen Studio | Alibaba | Qwen | Verified from primary source. First-party Alibaba/Qwen assistant. | https://chat.qwen.ai/ |
| Z.ai / ChatGLM | Zhipu AI | GLM | Verified from near-official source. First-party Zhipu/GLM assistant surface; naming and branding should be rechecked periodically. | https://z.ai/company |
| Baidu ERNIE | Baidu | ERNIE | Verified from primary source. First-party Baidu assistant; product naming may appear as ERNIE Bot, Wenxin Yiyan, or Wenxiaoyan. | https://yiyan.baidu.com/ |
| Doubao / Cici | ByteDance | Doubao | Caution: first-party ByteDance assistant family, but primary-source verification is incomplete for current international naming. | Needs stronger primary-source verification. |
| Tencent Yuanbao | Tencent | Hunyuan / Yuanbao | Caution: primary-source verification is incomplete; some product modes may expose non-Tencent models. | Needs stronger primary-source verification. |
| Pi | Inflection AI | Inflection / Pi | Caution: track operational status and model ownership carefully because Inflection has pivoted toward enterprise use. | https://pi.ai/; https://status.inflection.ai/ |

## Relationship to Goni
- This file informs market and ecosystem awareness only.
- It does not define Goni council seats, routing policy, exact model IDs, or
  deployment configuration.
- Exact Goni council model IDs belong in `goni-prototype-lab:config/council.yaml`
  or environment variables.

## Related docs
- [LLM Council](/blueprint/docs/llm-council.md)
- [Model registries](/blueprint/60-market/suppliers/model-registries.md)
- [Technology Intel Rules](/blueprint/60-market/TECH-INTEL-RULES.md)
