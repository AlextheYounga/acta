pub mod agentsmd;
pub mod clarify;
pub mod init;
pub mod start;

pub const IDEA_TEMPLATE: &str = include_str!("../../templates/planning/01-idea.md");
pub const PLAN_TEMPLATE: &str = include_str!("../../templates/planning/02-plan.md");
pub const TASKS_TEMPLATE: &str = include_str!("../../templates/planning/03-tasks.md");
pub const CLARIFICATION_TEMPLATE: &str = include_str!("../../templates/planning/clarity.md");
pub const AGENT_TEMPLATES: &[(&str, &str)] = &[
    ("genericdev", include_str!("../../templates/agentsmd/genericdev.md")),
    ("ponytail", include_str!("../../templates/agentsmd/ponytail.md")),
    ("wisedev", include_str!("../../templates/agentsmd/wisedev.md")),
];
