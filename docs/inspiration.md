# Inspiration – Builders & Thinkers We Track

> This file is a **people radar** for Goni.
>
> It lists the builders, writers, and teams whose work most closely aligns with:
> - OwnYourAI / sovereign infra / local-first LLMs, and  
> - Agentic / LinkedIn / content-automation stacks.
>
> It’s not exhaustive; it’s a living list of “who to watch” for infra patterns, narratives, and practical architectures.

---

## A. “OwnYourAI” / Sovereign-Infra / Local-LLM Builders

These are closest to the **Goni / Mitko** energy:
- local-first infra,  
- benchmarks, economics of local vs cloud,  
- “own your stack” ideology.

---

### 1. Mitko Vasilev – **OwnYourAI**

**Focus**

- Sovereign, on-prem AI: from “dream workstations” to **multi-agent local AI clouds** (vLLM, GGUF, MLX, ROCm, etc.).  
- Strong thesis:  
  > “Make sure you own your AI. AI in the cloud is not aligned with you; it’s aligned with the company that owns it.”

**Key takeaways**

- Think in **infra + economics**: GPU choice, VRAM, bandwidth, token/sec, total cost vs API bills.  
- Treat local AI as **core infrastructure**, not just a dev toy.  
- Pushes the idea that **national / enterprise sovereignty** starts with *owning compute and models*.

**Links**

- Hugging Face (OwnYourAI / mitkox): https://huggingface.co/mitkox  
- LinkedIn: https://www.linkedin.com/in/ownyourai/  
- X: https://x.com/iotcoi  

---

### 2. Fabio Matricardi – **“The Poor GPU Guy” / Local GPT Command Center**

**Focus**

- “Local GPT command center” architectures for **low-hardware setups** (mini-PCs, old GPUs, Intel iGPUs).  
- Deep dives into **OpenVINO, llama.cpp, Ollama**, and hybrid stacks.

**Key takeaways**

- Writes practical series like:
  - *How to ditch the Cloud and own your AI*  
  - *Local GPT: make AI easy and build your own Command Center*  
- Very good at **step-by-step infra**: from zero ? a usable local GPT “engine room.”

**Links**

- Medium profile: https://medium.com/@fabio.matricardi  
- “Local GPT: make AI easy and build your own Command Center”:  
  https://blog.stackademic.com/local-gpt-make-ai-easy-and-build-your-own-command-center-caba6ded2855  

---

### 3. Ndamulelo Nemakhavhani – **CRUD Flow / Private AI Bootcamp**

**Focus**

- CRUD Flow blog: turn a **regular PC into a local AI workstation** (Ollama + local tools) with privacy in mind.  
- Mix of **AI, cloud engineering, data science** with an emphasis on **reproducible how-tos**.

**Key takeaways**

- “Private AI Bootcamp 101” vibe:  
  - starting from scratch,  
  - clear steps to get local models + web UIs running,  
  - keep everything under your control.  
- Good pedagogy; respects constraints of **non-H100 hardware**.

**Links**

- Blog: https://blog.ndamulelo.co.za/  
- GitHub: https://github.com/ndamulelonemakh  
- LinkedIn: https://www.linkedin.com/in/ndamulelonemakhavhani/  

---

### 4. Martin Treiber – **IKANGAI / Local LLM Infra & Agentic Web**

**Focus**

- Long-form guides on **running LLMs locally**:
  - hardware (VRAM, memory bandwidth),
  - software (runtimes, quantisation),
  - performance tuning.  
- Writes on **“The Agentic Web”** and infra for agent workloads.

**Key takeaways**

- Article: *The Complete Guide to Running LLMs Locally: Hardware, Software, and Performance Essentials* is a 40-minute “playbook” on local LLM infra.  
- Frames performance in terms of **prefill vs decode**, **bandwidth vs raw TFLOPs**, not just “bigger GPU good”.

**Links**

- Guide: https://www.ikangai.com/the-complete-guide-to-running-llms-locally-hardware-software-and-performance-essentials/  
- IKANGAI – AI projects: https://www.ikangai.com/ai-projects/  

---

### 5. Manish (InstaVM) – **“I Want Everything Local” / Offline Workspace**

**Focus**

- Building a **fully offline AI workspace**:  
  - local LLMs (Ollama),  
  - local code execution (container / coderunner),  
  - Playwright for browser automation,  
  - assistant-UI for chat.

**Key takeaways**

- Blog post: *I Want Everything Local — Building My Offline AI Workspace* is a blueprint for:
  - “no cloud, no remote code execution” as a hard requirement,  
  - planning ? LLM ? code ? sandboxed VM, all local.  
- Very relevant to Goni’s “local OS + AI OS on top” story.

**Links**

- InstaVM / secure code execution: https://instavm.io/  
- “I Want Everything Local — Building My Offline AI Workspace”:  
  https://instavm.io/blog/building-my-offline-ai-workspace  

---

### 6. Saad Mujeeb – **Local Workstations vs Data-Center GPUs**

**Focus**

- Economics of **local AI workstations vs rented GPUs** (H100/H200 etc.).  
- Cloud solutions architect background; speaks **CFO/CTO language**.

**Key takeaways**

- Article: *Local AI Workstations vs Data Center GPUs: The Development Strategy That’s Saving Companies Millions* breaks down:
  - when a **one-time capex AI devbox** beats cloud opex,  
  - what workloads fit local vs remote,  
  - concrete examples (32B models, 10–25 concurrent users, 200+ tokens/sec).  
- Great ammo for justifying **“Own Your AI” with numbers**.

**Links**

- Blog: https://saadmujeeb.com/blog/local-ai-workstations-vs-data-center-gp-us-the-development-strategy-that-s-saving-companies-millions/  
- LinkedIn: https://www.linkedin.com/in/saadmujeeb/  

---

### 7. Mehul Gupta – **vLLM & Local API Serving**

**Focus**

- vLLM-based **self-hosted APIs** that mimic the OpenAI API; how to move workloads from cloud ? local or on-prem.  
- Part of “Data Science in Your Pocket” – practical LLM engineering.

**Key takeaways**

- Articles like *Wish to host local LLMs as APIs? Use vLLM* and *Ollama vs vLLM: What to use for LLM inferencing?* show:
  - how to spin up vLLM as an **OpenAI-compatible server**,  
  - when to pick vLLM vs Ollama,  
  - how to layer routing and observability on top.  
- Very relevant for Goni’s **“local /v1/chat/completions”** story.

**Links**

- vLLM article:  
  https://medium.com/data-science-in-your-pocket/wish-to-host-local-llms-as-apis-use-vllm-feb00ec79edf  
- Data Science in Your Pocket: https://medium.com/data-science-in-your-pocket  

---

### 8. Effendy Zulkifly – **Sovereign LLM Criteria**

**Focus**

- “Sovereign AI / LLM” in a **national / linguistic** context (Malaysia, Malay language).  
- Defines what makes a **truly sovereign** LLM vs marketing.

**Key takeaways**

- Post: *SOVEREIGN AI / LLM: 6 PROOF CRITERIA* gives a checklist:
  1. Model origin (pretrained from scratch vs finetune of LLaMA/Mistral).  
  2. Infrastructure & compute sovereignty (domestic data centers).  
  3. Data sovereignty (local linguistic, cultural, legal data).  
  4. Governance & ownership (local control).  
  5. Ecosystem & talent.  
  6. Exportability / competitiveness.  
- Good mental model for framing **Goni as sovereign infra**, not just “runs locally”.

**Links**

- “SOVEREIGN AI / LLM: 6 PROOF CRITERIA” (LinkedIn):  
  https://www.linkedin.com/posts/effendyzulkifly_sovereign-ai-llm-6-proof-criteria-the-activity-7391350813039714304-seR3  

---

### 9. Biswanath Das – **Local LLM Platform (Fintech)**

**Focus**

- Building a **fully offline Local LLM Platform** for finance:
  - on-prem RAG,  
  - LoRA/QLoRA on proprietary datasets,  
  - zero internet connectivity.

**Key takeaways**

- LinkedIn posts like *Introducing Local LLM Platform: Secure, On-Premise AI for Finance* position:
  - “AI Security is Local, Not Cloud”,  
  - **air-gapped, regulated-sector** AI (banks, trading floors, risk).  
- Great for Goni’s longer-term **enterprise / regulated sector** narrative.

**Links**

- Local LLM Platform overview (LinkedIn):  
  https://www.linkedin.com/posts/biswanath-das_ai-fintech-llm-activity-7387576299504078848-iG23  

---

### 10. Caleb Stephenson – **“Costco Computer” Sovereign AI Narrative**

**Focus**

- Real-world experiments with **commodity hardware**:
  - buying a consumer PC (Costco),  
  - testing local LLMs,  
  - comparing to SaaS AI subscriptions.

**Key takeaways**

- Story: *How I built a chatbot with a Costco computer and open-source models* (LinkedIn post) ends with cancelling a SaaS AI subscription because the **local stack was good enough**.  
- Good anecdotal proof: you don’t need an H100 farm to own your AI.

**Links**

- “I built a chatbot with a Costco computer and open-source models” (LinkedIn):  
  https://www.linkedin.com/posts/caleb-stephenson-mt-ent-maturity_using-a-local-llm-putting-it-all-together-activity-7394117050807296000-8r_U  

---

### 11. Piotr Kuczynski – **Desktop Benchmarking for Coding Agents**

**Focus**

- Concrete **tokens/sec & context window** benchmarks on consumer GPUs for coding agents and local LLMs.  

**Key takeaways**

- Repeatedly shares benchmarks like:
  > “I get ~25 tokens/sec with GLM-4.6 on my desktop GPU workstation, with a 128k token context window for coding agents. All tokens are on the house. No weekly rate limits.”  
- Pure signal on what’s possible on a **single, well-built devbox**.

**Links**

- Example benchmark post (GLM-4.6, 25 tok/s):  
  https://www.linkedin.com/posts/pkuczynski_thats-the-direction-ive-been-saying-we-activity-7383458242938421248-5FXH  

---

### 12. Clara / Claraverse – **Offline Modular AI Workspace**

**Focus**

- Open-source **offline AI workspace**:
  - local LLM chat (llama.cpp / Ollama),  
  - agents, automations (n8n),  
  - image generation (ComfyUI / SD),  
  - all in one dashboard.

**Key takeaways**

- ClaraVerse is described as **“privacy-first, fully local AI workspace”**:
  - 100 % local processing,  
  - zero telemetry,  
  - MIT-licensed,  
  - runs entirely offline with modular widgets (chat, agents, image gen, workflows).  
- Clara is effectively a **reference product** for how to bundle local models + tools into a coherent UX.

**Links**

- Site: https://claraverse.space/  
- GitHub: https://github.com/claraverse-space/ClaraVerse  
- Reddit (r/claraverse): https://www.reddit.com/r/claraverse/  

---

## B. Agent / LinkedIn / Content-Automation Builders

These are more **application-layer agent builders** (esp. LinkedIn/content), but still with a **builder / reproducible-workflow** mindset.

---

### 13. Manoj Bhopi – **LinkedIn Post Agents (Dev-Focused)**

**Focus**

- AI agents that **turn engineering ideas into LinkedIn posts**:
  - content automation for developers and engineering leaders,
  - .NET, Azure, DevOps background.

**Key takeaways**

- Posts like *Built an AI agent to write LinkedIn posts for me* document:
  - taking raw notes / code / ideas,  
  - turning them into structured posts with an AI agent in the middle,  
  - friction reduction between “I had an idea” and “I published it”.  

**Links**

- LinkedIn: https://www.linkedin.com/in/manojbhopi/  

---

### 14. Deepak Rajput – **Agentic AI for LinkedIn & Marketing**

> (There are multiple Deepak Rajputs; here we track the **agentic AI / marketing** voice.)

**Focus**

- **Agentic AI** for marketing and growth:
  - automating Google Ads, SEO, LinkedIn content,  
  - using agents for overnight SEO, lead generation, etc.

**Key takeaways**

- Posts like *Agentic AI: From Chat to Autonomous Work* and *Top 10 Agentic AI tools for marketers to delegate work* are clean mental frameworks for:
  - moving from “helper tools” ? **delegated agents**.  
- Good reference for **how to talk about agents** with marketing and growth teams.

**Links**

- LinkedIn: https://www.linkedin.com/in/deepakrajput  

---

### 15. Irene Chan – **No-Code LinkedIn Agents & Content Systems**

**Focus**

- No-code **LinkedIn/content agents**:
  - AI agents that curate, draft, and schedule content,  
  - targeted at early-stage AI startups and solo founders.

**Key takeaways**

- Blog post: *How I Built an AI Agent for Creating LinkedIn Posts (No Code)* shows:
  - wiring together tools (APIs + no-code platforms) into a repeatable system,  
  - reclaiming hours per week by offloading posting and curation.  
- Very useful for **operationalising content agents** without heavy infra.

**Links**

- Site: https://irenechan.co/  
- “How I Reclaimed 2.5 Hours Weekly with My No-Code AI LinkedIn Agent” (LinkedIn):  
  https://www.linkedin.com/pulse/how-i-reclaimed-25-hours-weekly-my-no-code-ai-linkedin-irene-chan-5furc  

---

### 16. sujeetgund – **Open-Source LinkedIn Post Generator Agent**

**Focus**

- Open-source **LinkedIn post generator agent** using Google ADK and multi-agent orchestration.

**Key takeaways**

- Repo: `linkedin-post-generator-agent` is a complete, fork-able:
  - multi-phase agent pipeline (research ? outline ? draft ? hashtags ? image suggestion),  
  - uses the Google Gemini API and Google Agent Development Kit.  
- Great reference for **open, reproducible LinkedIn agents**.

**Links**

- GitHub: https://github.com/sujeetgund/linkedin-post-generator-agent  

---

### 17. Cli3nts – **AI Agent for LinkedIn**

**Focus**

- Productised **LinkedIn AI agent**:
  - automates prospecting, replies, content, and comments,
  - built as a B2B SaaS.

**Key takeaways**

- Website copy: “Your AI Agent to scale your LinkedIn presence” – automates:
  - inbound replies and comments,  
  - outbound prospecting DMs,  
  - post scheduling / copy.  
- Useful reference for **pricing, packaging, and UX** of a LinkedIn agent SaaS.

**Links**

- Site: https://www.cli3nts.com/  

---

### 18. Jacob Bank – **Relay.app (Workflow Agents for Email & Calendar)**

**Focus**

- Workflow automation beyond simple triggers:
  - **Relay.app** is a platform to create AI agents that operate across email, calendar, and SaaS tools.

**Key takeaways**

- Background: ex-Director of Product Management at Google (Gmail, Google Calendar); founder of Timeful (smart calendar acquired by Google).  
- Relay.app’s positioning: creating **workflow agents** that can:
  - watch email and calendar,  
  - trigger complex workflows,  
  - coordinate multi-step processes.

**Links**

- Relay.app: https://www.relay.app/  
- LinkedIn: https://www.linkedin.com/in/jacobbank  

---

### 19. Damian Player – **Agent Integrator / AI Automation for Agencies**

**Focus**

- Builds and teaches **AI automation systems for agencies**:
  - founder of Agent Integrator,  
  - focus on n8n / agents / agency business models.

**Key takeaways**

- Agent Integrator positions itself as an **AI automation agency & partner**, building systems and teaching people to build AI agencies that scale.  
- Damian’s X feed is a steady stream of **agent use-cases** and agency-focused framing.

**Links**

- Agent Integrator: https://agentintegrator.io/  
- LinkedIn (company): https://www.linkedin.com/company/agentintegrator  
- X: https://x.com/damianbplayer  

---

### 20. Subhan Qureshi – **Agentic AI Strategy & Ops**

**Focus**

- Writes on **agentic AI in enterprise**:
  - autonomous agents,  
  - operations,  
  - decision-making.

**Key takeaways**

- Posts like *Agentic AI: The Future of Enterprise Operations* and *Agentic AI: Transforming Business with Autonomous AI Systems* frame:
  - when agents go from “nice-to-have” ? core operating layer,  
  - what metrics and governance matter.  
- Good for **strategy slides** and explaining agents to business audiences.

**Links**

- LinkedIn: https://www.linkedin.com/in/iamsubhanqureshi  

---

### 21. Elewachi Emmanuel – **n8n / Zapier / Voice Agents**

**Focus**

- Practical **no-code automation**:
  - n8n, Zapier, Make.com,  
  - voice agents that answer calls and book appointments,  
  - content & workflow automation.

**Key takeaways**

- Posts + shoutouts (e.g. “Elewachi built a Voice AI Agent that answers inbound calls and books appointments, with n8n + Airtable”) illustrate:
  - solid examples of **vertical automations**,  
  - using mainstream tools to build agents in the wild.  

**Links**

- LinkedIn: https://www.linkedin.com/in/elewachi-emmanuel  

---

### 22. John Santaferraro – **Agentic AI Thought Leadership**

**Focus**

- Industry analyst & CEO of Ferraro Consulting, focusing on **agentic AI in enterprises**.

**Key takeaways**

- Talks about:
  - “The Agents are Coming” and “The Agentic AI Future is Here”,  
  - multi-agent orchestration (Task, Persona, Workgroup agents).  
- Useful for **high-level narratives** and analyst framing (“Top 10 thought leader for agentic AI”).

**Links**

- LinkedIn: https://www.linkedin.com/in/johnsantaferraro  
- Blog: https://www.ferraroconsulting.com/blog  

---

### 23. Omar Sar (“elvis”) – **Agentic Workflows & Code Iteration Loops**

**Focus**

- Developer-focused **agentic workflows**:
  - iterative code improvement via LLM loops,  
  - “write better code” prompt loops and their effects.

**Key takeaways**

- Frequently highlighted (e.g. in *Generative Programmer* newsletter) as:
  - example of using LLMs to iteratively refactor and improve code,  
  - proof that **iteration loops** can drive quality, not just single-shot prompts.

**Links**

- X handle often referenced as “elvis (Omar Sar)” – find via X search for `"Omar Sar" agents` for the latest profile.  

*(Add direct profile link once you pin it down.)*

---

### 24. Mustafa Shoukat – **AI Agents Stack Cartographer**

**Focus**

- Mapping the **AI agents stack**:
  - infra tools,  
  - orchestration platforms,  
  - agent frameworks and no-code tools.

**Key takeaways**

- Curates “what exists” so builders don’t miss entire categories.  
- Good for staying updated on **new infra / agent platforms**.

**Links**

- (Add LinkedIn / X link once confirmed – placeholder.)  

---

### 25. Ravi Ranjan – **No-Code Automation for Business**

**Focus**

- Accessible **no-code automation**:
  - Zapier / Make / n8n for real businesses,  
  - easy-to-follow automation tutorials.

**Key takeaways**

- Examples across:
  - recruiting,  
  - analytics,  
  - consulting workflows.  
- Very relevant when you want **client-friendly “before/after” stories**.

**Links**

- (Add LinkedIn link when pinned – placeholder.)  

---

### 26. Elvis S. – **Multi-Agent OSS Frameworks**

**Focus**

- Surfaces **open-source multi-agent frameworks**:  
  - symbolic control,  
  - multi-agent communication,  
  - more advanced than “one LLM + a few tools”.

**Key takeaways**

- Good radar for when Goni’s agent layer needs to go beyond:
  - one orchestrator,
  - into genuine **multi-agent ecosystems**.

**Links**

- (Add GitHub / X handle once you stabilise which “Elvis S.” you mean – placeholder.)  

---

### 27. Lindy & Relevance AI Teams – **Platform-Level Multi-Agent Workflows**

**Focus**

- **Lindy** – AI employee platform (sales, support, ops).  
- **Relevance AI** – tools for building multi-agent workflows & dashboards for enterprises.

**Key takeaways**

- Both show:
  - what **production UX** for multi-agent systems looks like,  
  - how to do **analytics, guardrails, and iteration** on agents in the field.  
- Useful inspiration for Goni’s future **dashboard and orchestration UX**.

**Links**

- Lindy: https://www.lindy.ai/  
- Relevance AI: https://relevance.ai/  

---

## How to use this list in Goni

- When you design **hardware + infra** ? look at Section A.  
- When you design **agent workflows + UX** ? look at Section B.  
- When you write **narratives (decks, README, whitepaper)** ? steal language and framing from the people who already resonate with the “Own Your AI” and “agents that actually do work” stories.

Treat this as a **living file**:

- add new people as you discover them,  
- promote some to “core influences” for specific parts of Goni (hardware, orchestrator, UX),  
- and link concrete patterns from their work into your architecture docs.
