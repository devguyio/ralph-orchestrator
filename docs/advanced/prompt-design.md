# Prompt Design Rationale

This document explains the **why** behind Ralph's system prompt design choices. Understanding these patterns helps you customize prompts effectively and debug unexpected agent behavior.

## Core Design Principles

### 1. Fresh Context is a Feature, Not a Bug

Every iteration starts with a clean context window. This is intentional:

```
You are Ralph. You are running in a loop. You have fresh context each iteration.
```

**Why**:

- Prevents context exhaustion on long tasks
- Enables backpressure (orchestrator can intervene between iterations)
- Makes agent behavior more predictable
- Allows recovery from confused states

!!! note "Trade-off"
    The agent must re-read files each iteration, but this cost is worth the reliability gains.

### 2. One Task Per Iteration

```
You MUST complete only one atomic task for the overall objective.
Leave work for future iterations.
```

**Why**:

- Atomic commits with clear purpose
- Natural checkpoints for human review
- Easier debugging (which iteration broke things?)
- Backpressure can be applied after each task

!!! warning "Anti-pattern avoided"
    Agents trying to do everything in one massive context-filling run, then running out of tokens mid-implementation.

### 3. Scratchpad as Memory Substitute

```
Track progress in `.ralph/agent/scratchpad.md`
```

**Why**:

- Simple, human-readable persistence
- Works with any agent (no special memory features needed)
- Version controllable
- Grep-able for debugging

**The checkbox notation** (`[ ]`, `[x]`, `[~]`) was chosen because:

- Parseable by regex
- Familiar to developers (GitHub/Markdown)
- Visual progress indicator
- Agents understand it without explanation

---

## Language Patterns

### RFC 2119 Keywords

Ralph uses RFC 2119 language consistently:

| Keyword | Meaning | Agent Interpretation |
|---------|---------|---------------------|
| `MUST` | Absolute requirement | Failure to comply = violation |
| `MUST NOT` | Absolute prohibition | Doing this = violation |
| `SHOULD` | Strong recommendation | Exceptions need justification |
| `MAY` | Optional | Agent's discretion |

**Why this matters**: Agents trained on technical documentation (RFCs, specs) recognize these keywords and treat them with appropriate weight. Using casual language ("you should probably...") leads to inconsistent compliance.

**Example**:
```markdown
# Weak (inconsistent compliance)
You should run tests before committing.

# Strong (consistent compliance)
You MUST run tests before committing.
```

### Numbered Guardrails

```markdown
### GUARDRAILS
999. Fresh context each iteration - scratchpad is memory
1000. Don't assume 'not implemented' - search first
```

**Why numbering starts at 999**:

- High numbers signal importance
- Stands out visually
- Avoids collision with step numbers (1, 2, 3...)
- Easier to reference in debugging ("violated guardrail 1001")

### Imperative Voice

All instructions use imperative voice:

```markdown
# Good (imperative)
You MUST update the scratchpad.
You MUST NOT skip tests.

# Bad (passive/suggestive)
The scratchpad should be updated.
Tests shouldn't be skipped.
```

**Why**: Imperative voice assigns clear responsibility and reduces ambiguity.

---

## Structural Patterns

### Bookending

The OBJECTIVE appears twice - near the start and in the DONE section:

```markdown
## OBJECTIVE
**This is your primary goal...**
> Add user authentication

...

## DONE
**Remember your objective:**
> Add user authentication
```

**Why**: In long prompts, agents attend most strongly to content at the beginning and end. Bookending ensures the goal isn't lost in the middle of workflow instructions.

### Progressive Disclosure

Prompts only include relevant sections:

| Context | Included | Excluded |
|---------|----------|----------|
| Solo mode | Full workflow | Hats topology |
| Active hat | Hat instructions | Full topology |
| Fast path | Delegation only | Planning steps |

**Why**: Token efficiency and focus. An agent doesn't need to see hat topology if there are no hats.

### Section Ordering

Sections follow a specific order:

1. **Context first** (OBJECTIVE, PENDING EVENTS) - What to work on
2. **Identity and state** (ORIENTATION, SCRATCHPAD) - Who you are
3. **Workflow** (WORKFLOW, HATS) - How to proceed
4. **Mechanics** (EVENT WRITING) - Tool usage
5. **Termination** (DONE) - When to stop

**Why**: This order matches the agent's decision flow:

1. "What am I supposed to do?" (context)
2. "What do I know/have?" (state)
3. "How should I do it?" (workflow)
4. "What tools do I use?" (mechanics)
5. "When am I done?" (termination)

---

## Specific Design Decisions

### Why "ralph emit" Instead of Direct File Writes?

```markdown
You MUST use `ralph emit` to write events.
You MUST NOT use echo/cat to write events.
```

**Reason**: Shell escaping breaks JSON. This command:
```bash
echo '{"topic": "build.done", "data": "tests passed"}' >> .ralph/events.jsonl
```

Fails subtly with quotes, newlines, special characters. The `ralph emit` command handles all escaping internally.

### Why "Stop After Publishing"?

```markdown
You MUST stop working after publishing an event.
```

**Reason**: Without this constraint, an agent might:

1. Publish `build.done`
2. Continue working on the next task
3. But the next iteration (expecting to handle `build.done`) conflicts

The strict stop-after-publish rule ensures clean handoffs.

### Why Subagent Limits?

```markdown
You MAY use parallel subagents (up to 10) for searches.
You MUST NOT use more than 1 subagent for build/tests.
```

**Reason**:

- Search is embarrassingly parallel (no conflicts)
- Build/test operations can conflict (port bindings, file locks, race conditions)

### Why "Don't Assume - Search First"?

```markdown
1000. Don't assume 'not implemented' - search first
```

**Reason**: A common failure mode:

1. User asks: "Add error handling to the login function"
2. Agent assumes there's no error handling
3. Agent writes duplicate error handling
4. Code review catches the duplication

The guardrail forces verification before implementation.

### Why "Capture the Why"?

```markdown
1002. Commit atomically - one logical change per commit, capture the why
```

**Reason**: Agents tend to write commit messages describing the "what":
```
Update user.py
```

The guardrail encourages:
```
Fix race condition in user session cleanup

The previous implementation could leave orphaned sessions when
two logout requests arrived simultaneously.
```

---

## Mode-Specific Design

### Solo Mode: REPEAT vs EXIT

**Without memories** (REPEAT):
```markdown
### 5. REPEAT
You MUST continue until all tasks are `[x]` or `[~]`.
```

**With memories** (EXIT):
```markdown
### 5. EXIT
You MUST exit after completing ONE task.
```

**Why the difference**:

- Without memories, scratchpad checkboxes are the only state
- Agent must iterate until all checkboxes are done
- With memories/tasks, orchestrator tracks state externally
- Exiting after one task enables finer backpressure

### Multi-Hat Mode: DELEGATE

```markdown
### 2. DELEGATE
You MUST publish exactly ONE event to hand off to specialized hats.
You MUST NOT do implementation work — delegation is your only job.
```

**Why**: In multi-hat mode, Ralph is a coordinator, not a worker. Without this explicit constraint, Ralph might start implementing instead of delegating.

### Fast Path

```markdown
**FAST PATH**: You MUST publish `{starting_event}` immediately.
You MUST NOT plan or analyze — delegate now.
```

**Why**: When `starting_event` is configured and no scratchpad exists, planning is redundant. The configuration already expresses the intent. Skip straight to delegation.

---

## Failure Modes and Mitigations

!!! warning "Common Issues"
    These are the most frequent problems encountered when prompts aren't followed correctly, along with how the prompt design mitigates them.

### Agent Ignores Instructions

**Symptom**: Agent does something explicitly prohibited.

**Mitigation in prompts**:

- Use `MUST NOT` (stronger than "don't")
- Repeat critical constraints in multiple sections
- Use numbered guardrails (create accountability)

### Agent Gets Lost in Workflow

**Symptom**: Agent spends iterations on mechanics instead of the task.

**Mitigation in prompts**:
```markdown
You MUST NOT get distracted by workflow mechanics — they serve this goal.
```

The OBJECTIVE bookending also helps refocus.

### Agent Declares Done Prematurely

**Symptom**: Agent outputs `LOOP_COMPLETE` with tasks remaining.

**Mitigation in prompts**:
```markdown
**Before declaring completion:**
1. Run `ralph tools task ready` to check for open tasks
2. If any tasks are open, complete them first
```

### Agent Publishes Invalid Events

**Symptom**: Agent emits an event that no hat subscribes to.

**Mitigation in prompts**:
```markdown
You MUST publish one of: `build.task`, `review.request`
```

Explicit enumeration of valid events prevents invention.

---

## Customization Guidelines

### Adding Guardrails

Add project-specific rules in config:

```yaml
core:
  guardrails:
    - "All new code must have tests"
    - "Use TypeScript strict mode"
    - "No console.log in production code"
```

These appear as numbered rules (1003, 1004, 1005...).

### Customizing Hat Instructions

For nuanced hat behavior:

```yaml
hats:
  builder:
    instructions: |
      You are a senior engineer who values:
      - Clean, readable code over clever code
      - Comprehensive tests over quick fixes
      - Following existing patterns in the codebase

      Before implementing, always check for similar code.
      After implementing, always run the full test suite.
```

### Changing Completion Promise

For CI/CD integration:

```yaml
event_loop:
  completion_promise: "##TASK_COMPLETE##"
```

Use distinctive strings that won't appear accidentally.

---

## Testing Prompt Changes

!!! tip "Debugging Prompts"
    Always test prompt changes with diagnostics enabled to see exactly what's being sent to the agent.

When modifying prompts:

1. **Enable diagnostics** to see generated prompts:
   ```bash
   RALPH_DIAGNOSTICS=1 ralph run -p "test task"
   ```

2. **Check the prompt file**:
   ```bash
   cat .ralph/diagnostics/*/orchestration.jsonl | jq '.prompt'
   ```

3. **Run multiple iterations** to verify consistency

4. **Test edge cases**:
   - Empty scratchpad
   - All tasks complete
   - Failed tests
   - Multi-hat handoffs

---

## See Also

- [System Prompts Reference](./system-prompts.md) - Complete prompt documentation
- [Orchestration Workflow](./orchestration-workflow.md) - Execution flow
- [Custom Hats](./custom-hats.md) - Multi-hat configuration
- [Diagnostics](./diagnostics.md) - Debugging tools
