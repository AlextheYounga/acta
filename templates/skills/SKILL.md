---

name: acta
description: Create and maintain durable planning records for non-trivial software changes with the Acta CLI and Git Flow worktrees. Use when defining, planning, starting, clarifying, implementing, or resuming a change in an Acta-enabled repository.
---

# Acta

Use the `acta` CLI instead of reproducing its Git commands or path rules manually.

Acta records settled software changes. Planning documents must contain defined intent, chosen decisions, and concrete work. They must not contain unanswered questions, competing options, or speculative possibilities.

## Define the Change Before Starting

Before running `acta start`, settle the proposed change with the user.

The following must be understood:

* What was requested
* What problem is being solved
* What important terms mean
* What behavior or state should exist afterward
* What the change includes
* Which constraints apply
* What is explicitly excluded

Resolve material ambiguity before creating the planning record.

Do not write questions, `TBD` markers, placeholder decisions, or requests for clarification into Acta documents.

Do not begin implementation merely because the general direction appears clear.

You should also consult `docs/agents/conventions` for more information on how we write code.

## Start Work

For a non-trivial change:

1. Run:

   ```sh
   acta start <type>/<name>
   ```

2. Continue all planning and implementation inside the worktree printed by Acta.

3. Complete the generated files under:

   ```text
   docs/agents/plans/<branch-name>/
   ```

4. Finish the planning record before modifying implementation files.

5. Commit the planning files with the change they describe.

Do not use Acta for trivial edits that do not benefit from a durable planning record.

## Write `01-idea.md`

`01-idea.md` defines the settled change.

Treat it as the authoritative record of what the change means and what outcome is expected.

### Request

State what was requested using the user’s terminology.

Preserve distinctions the user made. Do not broaden or reinterpret the request without agreement.

### Problem

Describe the specific existing problem the change addresses.

Explain what is wrong, missing, confusing, unsafe, inefficient, or difficult about the current state.

Do not substitute an implementation preference for the actual problem.

### Definitions

Define any term whose meaning could materially affect the implementation.

Definitions are especially important for:

* Domain entities
* User roles
* State names
* System boundaries
* New project terminology
* Technical terms used differently across systems
* Operations such as create, replace, delete, archive, publish, migrate, synchronize, validate, or finish

A useful definition establishes:

* What the term refers to
* What qualifies as that term
* What it includes
* What it excludes
* How it differs from nearby concepts when confusion is plausible

Example:

> **Archive:** Mark an existing record as inactive while preserving it in storage. Archived records are excluded from normal active-record queries. Archiving does not delete the record.

Do not define ordinary words that have no plausible ambiguity.

Use defined terms consistently throughout every planning document and throughout the implementation. Do not silently revert to a broader, narrower, or conventional meaning later.

### Desired outcome

Describe the observable behavior or state that should exist when the work is complete.

Prefer concrete outcomes over aspirations.

Weak:

> Improve media management.

Strong:

> Administrators can select an existing media item when editing a page without uploading a duplicate file.

### Scope

State positively what the change includes.

The scope should identify the behavior, components, users, or workflows covered by the change.

### Constraints

Record fixed technical, product, or process requirements.

Constraints are conditions the implementation must obey, not implementation ideas the agent merely prefers.

### Exclusions

State nearby concerns that are deliberately not part of the change.

Use exclusions when a reasonable reader or agent might otherwise assume that related work is included.

`01-idea.md` must not contain implementation plans or unanswered questions.

## Write `02-plan.md`

`02-plan.md` is the authoritative current implementation plan.

Investigate the repository before completing it. Base the plan on the code that actually exists, not on assumptions, framework conventions, or conversation memory.

The plan must describe one chosen approach.

Do not preserve multiple alternatives after a decision has been made.

### Current behavior

Describe the relevant behavior found in the repository.

Reference the existing modules, files, data flow, commands, or processes needed to understand the change.

Record only current behavior that matters to the proposed change.

### Intended behavior

Describe how the relevant system behavior will work after implementation.

Use the definitions and desired outcome from `01-idea.md`.

The intended behavior must not quietly expand the original scope.

### Approach

Describe the chosen implementation approach.

Explain the meaningful sequence of changes without turning the plan into line-by-line coding instructions.

Prefer established project conventions and existing mechanisms over new abstractions.

Do not introduce speculative extension points, compatibility layers, generalized frameworks, or infrastructure that the defined change does not require.

### Responsibilities and boundaries

State where each behavior belongs.

For each meaningful responsibility, identify the existing layer, module, process, command, service, or file that should own it.

Ask:

> Where does this behavior belong?

Do not place behavior in a new abstraction merely because it can be abstracted.

### Affected areas

List the files, modules, schemas, services, processes, or interfaces expected to change.

Keep this list grounded in repository investigation.

Do not create speculative file structures solely to make the plan look complete.

### Decisions

Record important implementation decisions and why they were chosen.

A decision should resolve a meaningful choice that affects the implementation.

Do not record ordinary coding details or obvious consequences of the approach.

### Risks

Record plausible regressions, failure modes, or compatibility concerns relevant to this specific change.

Do not produce a generic risk inventory.

### Validation

Define the checks and observable results that establish completion.

Validation should prove the desired outcome, not merely state that tests should pass.

Include the relevant combination of:

* Focused automated tests
* Broader regression tests
* Static analysis or linting
* Manual behavior checks
* Data or migration verification
* Review of the final diff

Do not begin implementation until `02-plan.md` reflects one settled and coherent approach.

Avoid unresolved language such as:

* `TBD`
* `Maybe`
* `Possibly`
* `Could`
* `Either`
* `Consider`
* `Depending on`
* `If needed`

When such uncertainty materially affects the implementation, settle it before proceeding.

## Write `03-tasks.md`

`03-tasks.md` is the authoritative execution state.

Replace the generated section comments with concrete tasks derived directly from `02-plan.md`.

Do not leave generic tasks such as:

* Investigate the current behavior
* Implement the change
* Update tests
* Finish documentation

Each task should:

* Describe one identifiable unit of work
* Refer to a real behavior or system boundary
* Be specific enough for another agent to execute
* Have a clear completion condition
* Contribute directly to the chosen plan
* Be ordered according to implementation dependencies

### Implementation

List concrete implementation tasks in dependency order.

Break work apart when separate tasks have independently meaningful completion states. Do not split work merely to create more checkboxes.

### Validation

List the exact checks needed to prove the desired outcome and guard against relevant regressions.

Tie validation tasks to the validation section in `02-plan.md`.

### Completion

List required cleanup, documentation, final review, or repository checks.

Do not add ceremonial completion tasks that provide no useful signal.

### Completion notes

After implementation, record:

* Meaningful deviations from the plan
* Validation results
* Important discoveries
* Deliberately unfinished work

Do not use completion notes as a transcript or diary.

Keep task checkboxes accurate throughout the work.

Do not mark a task complete when only part of its defined result has been achieved.

## Planning Gate

Implementation may begin only when:

* The request and problem are clearly stated.
* Important terms have settled definitions.
* The desired outcome is observable.
* Scope, constraints, and exclusions are explicit.
* The plan reflects the actual repository.
* The plan contains one chosen approach.
* Responsibilities are assigned to clear boundaries.
* Tasks are concrete and derived from the plan.
* Validation proves the desired outcome.
* No planning document contains unresolved questions or alternatives.

When these conditions are not met, continue defining the change before modifying implementation files.

## Resume Work

When resuming an existing change:

1. Read every numbered Markdown file in the active plan directory.
2. Read `01-idea.md` for the settled intent, terminology, scope, and constraints.
3. Follow `02-plan.md` for the authoritative current approach.
4. Follow `03-tasks.md` for the authoritative execution state.
5. Read later clarification files for decision history.
6. Confirm that every clarification has been incorporated into the three authoritative files.

Do not reconstruct the current state solely from conversation history.

Do not treat clarification history as a substitute for the current idea, plan, or tasks.

## Record Clarifications

Run:

```sh
acta clarify <descriptive-name>
```

only when new information materially changes a settled:

* Definition
* Desired outcome
* Scope boundary
* Constraint
* Exclusion
* Assumption
* Implementation decision
* Technical direction

A clarification records a settled answer or decision. It must never contain an unanswered question.

### Trigger

State the new information or instruction that required the clarification.

### Decision

State what was decided and why.

Use the updated definition or rule precisely.

### Supersedes

Identify the earlier definition, assumption, scope boundary, or decision being replaced.

When nothing is replaced, state that the clarification adds detail without changing an earlier decision.

### Effect on the record

Summarize the corresponding updates made to:

* `01-idea.md`
* `02-plan.md`
* `03-tasks.md`

A clarification is incomplete until the authoritative files reflect the new current truth.

Do not require future agents to reconcile contradictory documents.

## Keep the Record Focused

Acta planning files are not:

* Conversational journals
* Chat transcripts
* Scratchpads
* Lists of possible approaches
* Places to ask the user questions
* General project documentation
* Substitutes for repository investigation

Do not create competing plans elsewhere.

Leave finishing, merging, worktree removal, and branch cleanup to Git Flow and the user.
