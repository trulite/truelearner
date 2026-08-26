---
name: skill-author
description: Create or revise repo-local agent skills. Use when adding, simplifying, or restructuring a SKILL.md or its agents/openai.yaml metadata in this repository.
---

# Skill Authoring

```text
Skill request
     |
     v
Read repository boundaries,
current skill, and real workflow
     |
     v
Draw one ASCII text diagram
     |
     v
Add only lagom instructions
     |
     v
Keep responsibilities separate
 AGENTS.md -> routing and boundaries
 SKILL.md  -> procedure and decisions
 scripts   -> deterministic mechanics
     |
     v
Sync agents/openai.yaml
     |
     v
Check trigger, links, and contradictions
```

## Rules

- Lead with one ASCII text diagram; do not use Mermaid.
- Write the shortest instructions that still produce correct, safe action.
- Remove history, duplicated process, commentary, and obvious advice.
- Keep essential choices, ownership boundaries, and stop conditions explicit.
- Use imperative language and a precise triggering description.
- Keep `agents/openai.yaml` consistent with the skill.
