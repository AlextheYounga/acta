# Acta

> "Acta" (noun) The Latin word acta primarily means "things done," "deeds," or official records and proceedings.

Acta creates lightweight planning records for non-trivial software changes. It uses Git Flow topic branches and linked worktrees so each change has a predictable place for its plan.

## Setup

Initialize a Git repository:

```bash
acta init
```

Initialization installs the embedded Acta skill at `~/.agents/skills/acta/SKILL.md` and places the bundled conventions in `docs/agents/conventions/`, without overwriting existing files.

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

Choose a worktree and copy its path to the clipboard:

```bash
acta worktrees
```

Clipboard support checks `wl-copy` (Wayland), `pbcopy` (macOS), `xclip`, and `xsel` in that order.

## Development

```bash
cargo test
cargo run -- --help
```
