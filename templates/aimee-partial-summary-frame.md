Use the following summary frames as the authoritative record of this Aimee Codes session. Do not re-explain or revisit them unless asked. Additional frames will be added as the conversation progresses. Stay in role: Sage researches, Muse plans, Aimee implements and verifies. Specialists stay in their FE/BE/PLAT lane.

## Summary

{{#each messages}}
### {{inc @index}}. {{role}}

{{#each contents}}
{{#if text}}
````
{{text}}
````
{{/if}}
{{~#if tool_call}}
{{#if tool_call.tool.file_update}}
**Update:** `{{tool_call.tool.file_update.path}}`
{{else if tool_call.tool.file_read}}
**Read:** `{{tool_call.tool.file_read.path}}`
{{else if tool_call.tool.file_remove}}
**Delete:** `{{tool_call.tool.file_remove.path}}`
{{else if tool_call.tool.search}}
**Search:** `{{tool_call.tool.search.pattern}}`
{{else if tool_call.tool.skill}}
**Skill:** `{{tool_call.tool.skill.name}}`
{{else if tool_call.tool.sem_search}}
**Semantic Search:**
{{#each tool_call.tool.sem_search.queries}}
- `{{use_case}}`
{{/each}}
{{else if tool_call.tool.shell}}
**Execute:**
```
{{tool_call.tool.shell.command}}
```
{{else if tool_call.tool.mcp}}
**MCP:** `{{tool_call.tool.mcp.name}}`
{{else if tool_call.tool.task}}
**Agent task:** `{{tool_call.tool.task.agent_id}}` — {{tool_call.tool.task.task}}
{{else if tool_call.tool.todo_write}}
**Task Plan:**
{{#each tool_call.tool.todo_write.changes}}
{{#if (eq kind "added")}}
- [ADD] {{todo.content}}
{{else if (eq kind "updated")}}
{{#if (eq todo.status "completed")}}
- [DONE] ~~{{todo.content}}~~
{{else if (eq todo.status "in_progress")}}
- [IN_PROGRESS] {{todo.content}}
{{else}}
- [UPDATE] {{todo.content}}
{{/if}}
{{else if (eq kind "removed")}}
- [CANCELLED] ~~{{todo.content}}~~
{{/if}}
{{/each}}
{{/if~}}
{{/if~}}

{{/each}}

{{/each}}

---

Continue from this context. Prefer parallel specialist tasks for independent workstreams. Verify on the tree before claiming done.
