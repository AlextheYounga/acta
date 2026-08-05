---
name: acta
description: Manage durable planning records for non-trivial software changes with the Acta CLI and Git Flow worktrees. Use when proposing, planning, starting, clarifying, implementing, or resuming a code change in an Acta-enabled repository.
---

# Acta

Use the `acta` CLI instead of reproducing its Git commands or path rules manually.

## Start Work

1. Run `acta start <type>/<name>` before modifying implementation files for a non-trivial change.
2. Continue all planning and implementation in the worktree printed by Acta.
3. Complete the initial files under `docs/agents/plans/<branch-name>/`:
   - Treat `01-idea.md` as the stable original intent.
   - Treat `02-plan.md` as the authoritative current implementation plan.
   - Treat `03-tasks.md` as the authoritative execution state.
4. Keep task checkboxes accurate while working.
5. Commit planning files with the code they describe.

Do not use Acta for trivial edits that do not benefit from a durable plan.

## Resume Work

1. Read every numbered Markdown file in the active plan directory.
2. Follow `02-plan.md` for the current approach and `03-tasks.md` for remaining work.
3. Do not reconstruct current state solely from clarification history or conversation history.

## Record Clarifications

Run `acta clarify <descriptive-name>` only when scope, constraints, assumptions, or technical direction materially change.

After creating a clarification:

1. Record the trigger, decision, reason, superseded assumption, and implementation effect.
2. Update `02-plan.md` to reflect the current plan.
3. Update `03-tasks.md` to reflect the current execution state.

Do not use planning files as a conversational journal or create competing plans elsewhere. Leave finishing, merging, and branch cleanup to Git Flow and the user.
