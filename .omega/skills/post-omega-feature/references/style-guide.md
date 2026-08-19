# Twitter Post Style Guide - OmegaLoops Features

## Tone

- Direct and technical, written by a developer, for developers.
- Confident but not hype-y. Let the feature speak for itself.
- Conversational, not corporate.

## Vocabulary

**Prefer:**
- "OmegaLoops", "agent", "task", "context", "codebase", "workflow"
- Short, active-voice sentences.
- Concrete nouns over abstract ones ("file watcher" not "intelligent monitoring capability").

**Avoid:**
- "Omega" alone as the product name. Always use "OmegaLoops".
- Em dashes (--) anywhere in the post. Use commas, colons, or periods instead.
- "excited to announce", "thrilled", "proud to share"
- "game changer", "revolutionary", "supercharge", "unlock", "seamlessly"
- Passive voice ("it can be used to...")
- Jargon that non-Rust developers won't know (unless the feature is Rust-specific)

## Structure Template

```
[Problem statement or developer benefit, 1 sentence]
[What the feature does / how it works, 1 sentence]
[Optional: when to use it or a concrete example, 1 sentence]

#OmegaLoops #[FeatureTag] #AICode
```

## Approved Hashtags

Always end with `#OmegaLoops`. Add 1-2 from the list below that best fit:

- `#AICode` - general AI-assisted coding posts
- `#DevTools` - tooling and workflow improvements
- `#RustLang` - Rust-specific features
- `#CLI` - command-line interface features
- `#CodeReview` - review and diff-related features
- `#Agents` - agent orchestration features
- `#ContextWindow` - context management features
- `#Autocomplete` - code completion features

## Example Posts

**Custom agents:**
> OmegaLoops lets you define custom agents for specific tasks: code review, refactoring, docs. Each agent gets its own system prompt and tool set. Less context noise, better results.
>
> #OmegaLoops #Agents #DevTools

**Shell integration:**
> OmegaLoops's shell plugin tracks your terminal history and feeds relevant context to the agent. No more copy-pasting commands to explain what went wrong.
>
> #OmegaLoops #CLI #DevTools

**Multi-file edits:**
> OmegaLoops can plan and apply changes across multiple files in a single task. Rename a type, update all call sites, fix the tests, done in one pass.
>
> #OmegaLoops #AICode #DevTools

**Context compaction:**
> Long tasks no longer blow up the context window. OmegaLoops automatically compacts older turns while keeping the essential state. Tasks that used to fail mid-way now run to completion.
>
> #OmegaLoops #ContextWindow #AICode

## Checklist Before Finalizing

- [ ] 2-3 sentences, fits ~280 characters
- [ ] No banned phrases
- [ ] No em dashes
- [ ] Product is referred to as "OmegaLoops" throughout
- [ ] Leads with benefit or problem, not feature name
- [ ] Does not reference the attached video
- [ ] Ends with `#OmegaLoops` and 1-2 relevant hashtags
