use std::num::NonZeroUsize;

/// Host-supplied configuration used by the skills extension.
#[derive(Clone, Debug, PartialEq)]
pub struct SkillsExtensionConfig {
    /// Whether the available-skills catalog is included in model context.
    pub include_instructions: bool,
    /// Optional token budget override for the available-skills catalog.
    pub max_context_tokens: Option<NonZeroUsize>,
    /// Fraction of the model context window reserved for the available-skills catalog
    /// when `max_context_tokens` is unset.
    pub listing_budget_fraction: f64,
    /// Whether bundled skills are eligible for discovery.
    pub bundled_skills_enabled: bool,
    /// Whether orchestrator-owned skills are eligible for discovery.
    pub orchestrator_skills_enabled: bool,
    /// Whether cheap skill selectors run in shadow mode without changing prompt contents.
    pub shadow_selection_enabled: bool,
}
