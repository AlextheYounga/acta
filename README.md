# Acta

> "Acta" (noun) The Latin word acta primarily means "things done," "deeds," or official records and proceedings.

Acta creates lightweight planning records for non-trivial software changes. It uses Git Flow topic branches and linked worktrees so each change has a predictable place for its plan.

## Setup

Install the skill from `templates/skills/acta/SKILL.md` at:

```text
~/.agents/skills/acta/SKILL.md
```

Then initialize a Git repository:

```bash
acta init
```

## Commands

Start a change with a configured Git Flow topic prefix:

```bash
acta start feat/my-change
```

Create a clarification for the current plan:

```bash
acta clarify authentication-scope
```

Choose an `AGENTS.md` template for the current directory:

```bash
acta agentsmd
```

## Development

```bash
cargo test
cargo run -- --help
```
