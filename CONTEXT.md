# Acta

Acta is a small Rust CLI and accompanying agent skill for creating consistent, durable planning documentation for software changes.

The project is intentionally narrow. It does not attempt to replace Git, Git Flow, issue trackers, project-management systems, or coding agents. Its purpose is to give each proposed code change a predictable Git context and a predictable place for agents and humans to record the reasoning behind that change.

## Motivation

Coding agents frequently create planning documents inconsistently:

* Plans are placed in arbitrary directories.
* Temporary scratch files are mixed with durable documentation.
* Different agents use different naming conventions.
* Plans become detached from the branches and code they describe.
* Important clarifications remain trapped in conversation history.
* A later agent cannot easily determine the current plan or task state.
* Work may begin in the main checkout instead of an isolated branch or worktree.

Existing planning and specification systems often solve these problems by introducing substantial process, tooling, terminology, and generated structure.

Acta should solve the smaller underlying problem:

> Every non-trivial proposed change should have a consistent branch, worktree, and committed planning record.

## Philosophy

Acta is named after the Latin word *acta*, referring to acts, proceedings, or recorded events.

The planning documents are not intended to be speculative bureaucracy or a conversational diary. They are the durable record of how a proposed change moved from an idea into an implementation.

Acta should favor:

* Plain Markdown files
* Ordinary Git repositories
* Existing Git Flow behavior
* Explicit and predictable paths
* Minimal configuration
* Small, understandable commands
* Durable context that travels with the branch
* Human-readable output
* Safe and reversible Git operations

Acta should avoid:

* Reimplementing Git Flow
* Inventing a new branching model
* Managing pull requests or issue trackers
* Becoming a general project-management system
* Complex document schemas
* Hidden state databases
* Large dependency trees
* Automatically moving the user into another directory
* Performing unrelated repository operations

## Core Workflow

A proposed change begins with a Conventional Commit-style change type and a short name.

Examples:

```text
feat/adaptive-question-selection
fix/payment-callback-timeout
refactor/media-storage
docs/deployment-guide
perf/query-caching
```

The branch type should correspond to a configured Git Flow topic type.

For example:

```bash
git flow config add topic refactor develop --prefix=refactor/
```

Acta should use Git Flow as the authority for:

* Topic branch types
* Branch prefixes
* Parent branches
* Starting branches
* Finishing branches
* Merge behavior

Acta should not independently encode assumptions such as every topic branch starting from `develop`. It should rely on the repository’s Git Flow configuration wherever possible.

A typical workflow will be:

1. The user or agent proposes a non-trivial change.
2. Acta starts the appropriate Git Flow topic branch.
3. Acta restores the original checkout to its previous branch.
4. Acta creates a linked worktree for the new topic branch.
5. Acta creates the planning directory inside the new worktree.
6. Acta writes the initial Markdown planning files.
7. The planning files are committed to the topic branch.
8. Planning and implementation continue inside the linked worktree.
9. Clarification files are added when material decisions or constraints change.
10. The current plan and tasks are updated as the work evolves.
11. Git Flow remains responsible for finishing the topic branch.

Acta should not automatically change the invoking shell’s working directory. It should print the created worktree path clearly so the user or agent can continue from that location.

## Repository Layout

Acta-managed worktrees should live under:

```text
.worktrees/
```

The path should mirror the full topic branch name:

```text
.worktrees/<type>/<change-name>/
```

Example:

```text
.worktrees/refactor/media-storage/
```

The `.worktrees/` directory should not be committed. Acta should add it to the repository-local Git exclusion file rather than modifying the project’s committed `.gitignore`.

Acta should resolve the repository’s common Git directory using Git itself:

```bash
git rev-parse --git-common-dir
```

It should then manage an identifiable, idempotent block in:

```text
<GIT_COMMON_DIR>/info/exclude
```

Example:

```gitignore
# begin acta
/.worktrees/
# end acta
```

Acta must not assume that `.git` is a directory. Inside linked worktrees, `.git` is commonly a file that points to the actual Git metadata directory.

## Planning Documents

Planning documents should be committed to the topic branch under:

```text
docs/agents/plans/<branch-name>/
```

Because branch names contain slashes, this naturally creates nested directories.

Example:

```text
docs/
└── agents/
    └── plans/
        └── refactor/
            └── media-storage/
                ├── 01-idea.md
                ├── 02-plan.md
                └── 03-tasks.md
```

Each proposed change initially receives three files.

### `01-idea.md`

This records the original intent of the change.

It should answer:

* What was requested?
* What problem is being solved?
* What outcome is desired?
* What constraints are already known?
* What is explicitly outside the scope?
* Which questions remain open?

This file should become mostly stable once the proposed change is understood. It represents the reason for the work rather than the current implementation strategy.

### `02-plan.md`

This is the authoritative current implementation plan.

It should describe:

* Current repository behavior
* Relevant findings from repository investigation
* The proposed technical approach
* Affected modules, files, schemas, or processes
* Important design decisions
* Risks and possible regressions
* Validation requirements
* Known deviations from the original idea

When the implementation approach changes, this file should be updated so that it always represents the current truth.

Agents should not be required to reconstruct the active plan by reading an entire history of clarification files.

### `03-tasks.md`

This is the authoritative execution state.

It should contain concrete, checkable work items covering:

* Preparation
* Implementation
* Tests
* Validation
* Cleanup
* Documentation
* Completion notes

Another agent should be able to resume the work by reading the planning directory and determining which tasks are complete, which remain, and whether the plan has changed.

## Clarification Files

Material changes in scope, constraints, assumptions, or technical direction should receive numbered clarification files.

Examples:

```text
04-authentication-scope-clarity.md
05-cache-behavior-clarity.md
06-deployment-order-clarity.md
```

A clarification file should record:

* What triggered the clarification
* What was decided
* Why the decision was made
* Which previous assumption or decision it supersedes
* How the decision affects the implementation

Clarification files preserve the history of the change, but they must not become competing sources of current truth.

After creating a clarification file:

* `02-plan.md` should be updated to reflect the current plan.
* `03-tasks.md` should be updated to reflect the current execution state.

Follow-up documents should continue using monotonically increasing numeric prefixes.

The first version of Acta does not need to enforce a fixed vocabulary for every possible follow-up document. It should provide a safe mechanism for creating the next numbered Markdown file with a descriptive slug.

## Initial CLI Responsibilities

The first version of the CLI should remain small.

A tentative command surface is:

```bash
acta start <type>/<name>
acta clarify <name>
acta init
acta agentsmd
```

Exact naming may change during implementation.

### `acta start`

The start command should:

1. Verify that the current directory belongs to a Git repository.
2. Verify that the requested topic type exists in the Git Flow configuration.
3. Validate the requested change name.
4. Record the currently checked-out branch.
5. Use Git Flow to start the requested topic branch.
6. Restore the original checkout to its previous branch.
7. Create a linked worktree for the new branch.
8. Add the Acta exclusion block if needed.
9. Create the planning directory inside the new worktree.
10. Write the embedded initial Markdown templates.
11. Print the resulting branch, worktree, and plan paths.

Example:

```bash
acta start refactor/media-storage
```

Expected branch:

```text
refactor/media-storage
```

Expected worktree:

```text
.worktrees/refactor/media-storage
```

Expected planning directory inside that worktree:

```text
docs/agents/plans/refactor/media-storage
```

Acta should not automatically commit the initial files unless that behavior is deliberately added later. The first implementation should prefer explicit user control over commits.

### `acta clarify`

The clarification command should operate on the current Acta-managed branch.

It should:

1. Determine the current branch.
2. Resolve the corresponding plan directory.
3. Find the highest existing numeric prefix.
4. Create the next numbered Markdown file.
5. Refuse to overwrite an existing file.
6. Use a small embedded clarification template.

Example:

```bash
acta clarify authentication-scope
```

Possible result:

```text
04-authentication-scope-clarity.md
```

### `acta init`

The init command should prepare an existing Git repository for Acta.

It should first verify that `~/.agents/skills/acta/SKILL.md` exists. It should then create the planning directory root, copy missing convention templates into `docs/agents/conventions/`, create `.worktrees/`, and add the Acta exclusion block to the repository-local Git exclude file. Initialization should be idempotent, preserve existing convention files, and should not configure Git Flow or modify unrelated committed project files.

### `acta agentsmd`

The agentsmd command should present the Markdown files in `templates/agentsmd/` as a selection menu, then write the selected template to `AGENTS.md` in the current directory. It must refuse to overwrite an existing `AGENTS.md`.

## Git Flow Integration

Acta should depend on Git Flow Next rather than classic Git Flow when custom topic types are required.

A repository may configure topic types such as:

```bash
git flow config add topic feat develop --prefix=feat/
git flow config add topic fix develop --prefix=fix/
git flow config add topic refactor develop --prefix=refactor/
git flow config add topic docs develop --prefix=docs/
```

**THIS CLI WILL NOT OVERWRITE GIT FLOW SETTINGS**

Acta should discover configuration instead of maintaining its own duplicate list of allowed branch types.

The initial implementation should focus on starting topic branches and creating worktrees. Finishing branches should remain a direct Git Flow responsibility unless a clear need emerges for Acta to coordinate worktree cleanup around the finish operation.

A possible future command may wrap finishing safely, but it is not part of the initial scope.

## Agent Skill

Acta will include a small skill that teaches coding agents how and when to use the CLI.

The skill should instruct agents to:

* Use Acta for non-trivial proposed code changes.
* Run Acta before modifying implementation files.
* Continue planning and implementation inside the returned worktree.
* Read all numbered files in the active plan directory before resuming work.
* Treat `01-idea.md` as the original intent.
* Treat `02-plan.md` as the authoritative current plan.
* Treat `03-tasks.md` as the authoritative execution state.
* Create clarification files only for material changes.
* Update the current plan and task files after a clarification.
* Keep task checkboxes accurate throughout implementation.
* Avoid using planning files as a stream-of-consciousness journal.
* Avoid creating competing planning documents elsewhere.
* Commit planning files with the code change they describe.
* Leave Git Flow responsible for branch lifecycle operations.

The skill should call the Acta CLI rather than reproducing Git commands or path construction manually.

The skill template lives at `templates/skills/acta/SKILL.md` and should be installed at `~/.agents/skills/acta/SKILL.md`. The `init` command should verify that installation exists before preparing a repository, but should not modify the user's global skills directory.

## Rust Implementation Direction

Acta should be implemented as a small Rust CLI using clap.

Markdown templates should live as ordinary files in the repository and be embedded at compile time:

```rust
const IDEA_TEMPLATE: &str =
    include_str!("../templates/01-idea.md");

const PLAN_TEMPLATE: &str =
    include_str!("../templates/02-plan.md");

const TASKS_TEMPLATE: &str =
    include_str!("../templates/03-tasks.md");

const CLARIFICATION_TEMPLATE: &str =
    include_str!("../templates/clarity.md");
```

Acta should invoke Git and Git Flow using `std::process::Command`.

It should not construct shell command strings. Arguments should be passed directly to avoid quoting problems, shell interpolation, and platform-specific behavior.

The first implementation should prefer the Rust standard library unless a dependency clearly reduces complexity.

A possible initial structure is:

```text
acta/
├── Cargo.toml
├── CONTEXT.md
├── templates/
│   ├── 01-idea.md
│   ├── 02-plan.md
│   ├── 03-tasks.md
│   └── clarity.md
└── src/
    ├── main.rs
    ├── cli.rs
    ├── git.rs
    ├── git_flow.rs
    ├── paths.rs
    ├── exclude.rs
    ├── templates.rs
    └── commands/
        ├── mod.rs
		├── init.rs # Sets up project initially
        ├── start.rs # Starts new plan
        └── clarify.rs # Creates new clarification for branch plan
```

This structure is provisional. Files should be split according to responsibility rather than maintained merely to match an initial diagram.

## Safety Requirements

Acta will execute operations that change Git state. Safety and understandable failures are therefore core requirements.

The CLI should:

* Refuse to overwrite existing planning files.
* Refuse to reuse an occupied worktree path.
* Detect existing branches before starting work.
* Preserve the user’s original checkout branch.
* Avoid deleting branches or worktrees it did not create.
* Report the exact command that failed without exposing unnecessary internals.
* Keep exclusion-file changes idempotent.
* Preserve unrelated contents of the exclusion file.
* Clean up partial state when a multi-step operation fails.
* Never silently finish, merge, reset, or delete a branch.
* Never automatically switch the parent shell’s directory.

## Non-Goals for the Initial Version

The first version of Acta will not:

* Create tests
* Replace Git Flow
* Finish or merge branches
* Delete branches or worktrees
* Create pull requests
* Communicate with hosting-provider APIs
* Integrate with issue trackers
* Maintain a database
* Provide a Kanban interface
* Parse or semantically validate Markdown plans
* Enforce a comprehensive planning methodology
* Manage multiple agents
* Run implementation tasks
* Automatically approve plans
* Automatically commit changes
* Move completed plans into an archive
* Generate release notes
* Install or configure Git Flow automatically

These may be reconsidered only when concrete usage demonstrates a need.

## Current Decisions

The following decisions have been made:

* The project is named **Acta**.
* Acta will be implemented in Rust.
* Acta will integrate with Git Flow Next.
* Conventional Commit-style types will be represented as Git Flow topic types.
* Topic branches will receive linked worktrees.
* Worktrees will live beneath `.worktrees/`.
* `.worktrees/` will be excluded through the repository-local Git exclude file.
* Planning documents will be committed to the repository with an automated
* Planning documents will live under `docs/agents/plans/<branch-name>/`.
* Initial plans will contain `01-idea.md`, `02-plan.md`, and `03-tasks.md`.
* Material follow-up documents will use sequential numeric prefixes.
* Clarification history will supplement, not replace, the authoritative plan and task files.
* Markdown templates will be stored as separate files and embedded into the binary.
* Git Flow will remain responsible for branch relationships and finishing behavior.
* An accompanying skill will teach agents to use Acta consistently.
