# Glossary

Terms as this product uses them.

**Anda** — WEB3 layer adding append-only, hash-chained session pathway checkpoints to conversations; enables chat-only rollback and durable session memory. [Integrations](../integrations/anda-kip.md)

**Capsule / eternal receipt** — content-addressed record of a checkpoint exported to an eternal backend (`local`, `ic_oss`, `canister`, `s3`). [Anda](../integrations/anda-kip.md)

**Command** — slash-invocable workflow package defined as markdown with frontmatter; built-ins ship in `commands/`, user ones live in `.aimee/commands/`. [Slash commands](../usage/commands.md)

**Composition root** — the layer that builds and wires all dependencies at startup (`AimeeAPI::init`); nothing else composes services. [Architecture](../architecture/api.md)

**Flock** — the three-agent model: Sage researches, Muse plans, Aimee implements and verifies. [The flock](../getting-started/the-flock.md)

**Goal probes (/goal)** — five human-in-the-loop questions a standing goal must answer before autonomous execution: done-ness, verification, boundaries, owner, tracking. [Autonomy](../concepts/autonomy.md)

**HITL** — human-in-the-loop; design invariant meaning spend, payments, and stop-and-ask decisions require a human.

**KIP** — knowledge-graph protocol used via Cognitive Nexus; checkpoints can be recorded into KIP when `[anda] kip_enabled = true`. [Anda](../integrations/anda-kip.md)

**Mode** — ask / plan / act; determines which agent responds and its write scope. [Three modes](../getting-started/modes.md)

**Muse plan** — checkbox markdown file under `plans/`; reviewable contract for execution. [Plans and todos](../usage/plans-and-todos.md)

**MCP** — Model Context Protocol; external tool servers configured per project or globally whose tools join the registry. [Tools overview](../concepts/tools-overview.md)

**Pod** — isolated container workspace provisioned by `aimee pod` for untrusted or reproducible work. **Sandbox** — by contrast, a git worktree via `--sandbox`. [Pods](../surfaces/pods.md)

**Prompt uplift** — automatic sharpening of incoming requests (restating intent, surfacing unknowns) before execution.

**Restricted mode** — config flag requiring explicit permission grants for every tool execution. [Security](../operations/security.md)

**Skill** — folder with `SKILL.md` teaching task-type knowledge, loaded on demand by the `skill` tool. [Skills & commands](../concepts/skills-commands-templates.md)

**Specialist** — built-in subagent dispatched via `task`: frontend (`fe-*`), backend (`be-*`), platform (`plat-*`). Orchestrators never nest. [Swarm](../usage/swarm.md)

**Swarm** — parallel fan-out of specialists against one decomposed goal under a standing `/goal` loop. [Swarm runs](../usage/swarm.md)

**Template** — prompt partial in `templates/` shaping loop behavior (reminders, verification framing); production surface, edit deliberately.

**Tool catalog** — the sixteen registered tools in `aimee_domain::ToolCatalog`; descriptions ≤1024 chars in editable files. [Tool catalog](../reference/tools/catalog.md)

**Wire protocol** — how a provider's API speaks: OpenAI, OpenAI Responses, Anthropic, Bedrock, Google, or OpenCode. [Providers](../integrations/providers.md)

## See also

* [FAQ](faq.md)
* [What is Aimee Codes?](../about.md)

<!-- sources: AIMEE.md full -->
