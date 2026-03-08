//! Embedded presets for ralph init command.
//!
//! This module embeds all preset YAML files at compile time, making the
//! binary self-contained. Users can initialize projects with presets
//! without needing access to the source repository.
//!
//! Canonical presets live in the shared `presets/` directory at the repo root.
//! The sync script (`scripts/sync-embedded-files.sh`) mirrors them into
//! `crates/ralph-cli/presets/` for `include_str!` to work with crates.io publishing.

/// An embedded preset with its name, description, and full content.
#[derive(Debug, Clone)]
pub struct EmbeddedPreset {
    /// The preset name (e.g., "feature")
    pub name: &'static str,
    /// Short description extracted from the preset's header comment
    pub description: &'static str,
    /// Full YAML content of the preset
    pub content: &'static str,
}

/// All embedded presets, compiled into the binary.
const PRESETS: &[EmbeddedPreset] = &[
    EmbeddedPreset {
        name: "bugfix",
        description: "Systematic bug reproduction, fix, and verification",
        content: include_str!("../presets/bugfix.yml"),
    },
    EmbeddedPreset {
        name: "code-assist",
        description: "TDD implementation from specs, tasks, or descriptions",
        content: include_str!("../presets/code-assist.yml"),
    },
    EmbeddedPreset {
        name: "debug",
        description: "Bug investigation and root cause analysis",
        content: include_str!("../presets/debug.yml"),
    },
    EmbeddedPreset {
        name: "deploy",
        description: "Deployment and Release Workflow",
        content: include_str!("../presets/deploy.yml"),
    },
    EmbeddedPreset {
        name: "docs",
        description: "Documentation Generation Workflow",
        content: include_str!("../presets/docs.yml"),
    },
    EmbeddedPreset {
        name: "feature",
        description: "Feature Development with integrated code review",
        content: include_str!("../presets/feature.yml"),
    },
    EmbeddedPreset {
        name: "fresh-eyes",
        description: "Implementation workflow with enforced repeated fresh-eyes self-review passes",
        content: include_str!("../presets/fresh-eyes.yml"),
    },
    EmbeddedPreset {
        name: "gap-analysis",
        description: "Gap Analysis and Planning Workflow",
        content: include_str!("../presets/gap-analysis.yml"),
    },
    EmbeddedPreset {
        name: "hatless-baseline",
        description: "Baseline hatless mode for comparison",
        content: include_str!("../presets/hatless-baseline.yml"),
    },
    EmbeddedPreset {
        name: "merge-loop",
        description: "Merges completed parallel loop from worktree back to main branch",
        content: include_str!("../presets/merge-loop.yml"),
    },
    EmbeddedPreset {
        name: "pdd-to-code-assist",
        description: "Full autonomous idea-to-code pipeline",
        content: include_str!("../presets/pdd-to-code-assist.yml"),
    },
    EmbeddedPreset {
        name: "pr-review",
        description: "Multi-perspective PR code review",
        content: include_str!("../presets/pr-review.yml"),
    },
    EmbeddedPreset {
        name: "refactor",
        description: "Code Refactoring Workflow",
        content: include_str!("../presets/refactor.yml"),
    },
    EmbeddedPreset {
        name: "research",
        description: "Deep exploration and analysis tasks",
        content: include_str!("../presets/research.yml"),
    },
    EmbeddedPreset {
        name: "review",
        description: "Code Review Workflow",
        content: include_str!("../presets/review.yml"),
    },
    EmbeddedPreset {
        name: "spec-driven",
        description: "Specification-Driven Development",
        content: include_str!("../presets/spec-driven.yml"),
    },
];

/// Returns all embedded presets.
pub fn list_presets() -> &'static [EmbeddedPreset] {
    PRESETS
}

/// Looks up a preset by name.
///
/// Returns `None` if the preset doesn't exist.
pub fn get_preset(name: &str) -> Option<&'static EmbeddedPreset> {
    PRESETS.iter().find(|p| p.name == name)
}

/// Returns a formatted list of preset names for error messages.
pub fn preset_names() -> Vec<&'static str> {
    PRESETS.iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ralph_core::RalphConfig;

    #[test]
    fn test_list_presets_returns_all() {
        let presets = list_presets();
        assert_eq!(presets.len(), 16, "Expected 16 presets");
    }

    #[test]
    fn test_get_preset_by_name() {
        let preset = get_preset("feature");
        assert!(preset.is_some(), "feature preset should exist");
        let preset = preset.unwrap();
        assert_eq!(preset.name, "feature");
        assert!(!preset.description.is_empty());
        assert!(!preset.content.is_empty());
    }

    #[test]
    fn test_merge_loop_preset_is_embedded() {
        let preset = get_preset("merge-loop").expect("merge-loop preset should exist");
        assert_eq!(
            preset.description,
            "Merges completed parallel loop from worktree back to main branch"
        );
        // Verify key merge-related content
        assert!(preset.content.contains("RALPH_MERGE_LOOP_ID"));
        assert!(preset.content.contains("merge.start"));
        assert!(preset.content.contains("MERGE_COMPLETE"));
        assert!(preset.content.contains("conflict.detected"));
        assert!(preset.content.contains("conflict.resolved"));
        assert!(preset.content.contains("git merge"));
        assert!(preset.content.contains("git worktree remove"));
    }

    #[test]
    fn test_get_preset_invalid_name() {
        let preset = get_preset("nonexistent-preset");
        assert!(preset.is_none(), "Nonexistent preset should return None");
    }

    #[test]
    fn test_all_presets_have_description() {
        for preset in list_presets() {
            assert!(
                !preset.description.is_empty(),
                "Preset '{}' should have a description",
                preset.name
            );
        }
    }

    #[test]
    fn test_all_presets_have_content() {
        for preset in list_presets() {
            assert!(
                !preset.content.is_empty(),
                "Preset '{}' should have content",
                preset.name
            );
        }
    }

    #[test]
    fn test_preset_content_is_valid_yaml() {
        for preset in list_presets() {
            let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(preset.content);
            assert!(
                result.is_ok(),
                "Preset '{}' should be valid YAML: {:?}",
                preset.name,
                result.err()
            );
        }
    }

    #[test]
    fn test_preset_names_returns_all_names() {
        let names = preset_names();
        assert_eq!(names.len(), 16);
        assert!(names.contains(&"feature"));
        assert!(names.contains(&"debug"));
        assert!(names.contains(&"merge-loop"));
        assert!(names.contains(&"code-assist"));
        assert!(names.contains(&"fresh-eyes"));
    }

    #[test]
    fn test_review_uses_staged_adversarial_completion_contract() {
        let preset = get_preset("review").expect("review preset should exist");
        let config =
            RalphConfig::parse_yaml(preset.content).expect("embedded preset YAML should parse");

        assert_eq!(
            config.event_loop.required_events,
            vec![
                "review.section".to_string(),
                "analysis.complete".to_string()
            ]
        );

        let reviewer = config
            .hats
            .get("reviewer")
            .expect("reviewer hat should exist");
        assert_eq!(
            reviewer.triggers,
            vec!["review.start".to_string(), "analysis.complete".to_string()]
        );
        assert_eq!(
            reviewer.publishes,
            vec!["review.section".to_string(), "REVIEW_COMPLETE".to_string()]
        );
        assert!(reviewer.instructions.contains("On `review.start`:"));
        assert!(
            reviewer
                .instructions
                .contains("Emit exactly one `review.section`")
        );
        assert!(reviewer.instructions.contains("On `analysis.complete`:"));
        assert!(
            reviewer
                .instructions
                .contains("Emit exactly one `REVIEW_COMPLETE`")
        );
        assert!(
            reviewer
                .instructions
                .contains("❌ Emit `REVIEW_COMPLETE` on the initial `review.start` pass")
        );

        let analyzer = config
            .hats
            .get("analyzer")
            .expect("analyzer hat should exist");
        assert_eq!(analyzer.triggers, vec!["review.section".to_string()]);
        assert_eq!(analyzer.publishes, vec!["analysis.complete".to_string()]);
        assert_eq!(analyzer.default_publishes, None);
        assert!(
            analyzer
                .instructions
                .contains("Emit exactly one `analysis.complete`")
        );
        assert!(
            analyzer
                .instructions
                .contains("adversarial or failure-path case")
        );
    }

    #[test]
    fn test_debug_uses_staged_adversarial_fix_contract() {
        let preset = get_preset("debug").expect("debug preset should exist");
        let config =
            RalphConfig::parse_yaml(preset.content).expect("embedded preset YAML should parse");

        assert_eq!(
            config.event_loop.required_events,
            vec![
                "hypothesis.test".to_string(),
                "hypothesis.confirmed".to_string(),
                "fix.applied".to_string(),
                "fix.verified".to_string(),
            ]
        );

        let investigator = config
            .hats
            .get("investigator")
            .expect("investigator hat should exist");
        assert_eq!(
            investigator.triggers,
            vec![
                "debug.start".to_string(),
                "hypothesis.rejected".to_string(),
                "hypothesis.confirmed".to_string(),
                "fix.verified".to_string(),
            ]
        );
        assert_eq!(
            investigator.publishes,
            vec![
                "hypothesis.test".to_string(),
                "fix.propose".to_string(),
                "DEBUG_COMPLETE".to_string(),
            ]
        );
        assert!(
            investigator
                .instructions
                .contains("On `debug.start` or `hypothesis.rejected`:")
        );
        assert!(
            investigator
                .instructions
                .contains("Emit exactly one `hypothesis.test`")
        );
        assert!(
            investigator
                .instructions
                .contains("On `hypothesis.confirmed`:")
        );
        assert!(
            investigator
                .instructions
                .contains("Emit exactly one `fix.propose`")
        );
        assert!(investigator.instructions.contains("On `fix.verified`:"));
        assert!(
            investigator
                .instructions
                .contains("Emit exactly one `DEBUG_COMPLETE`")
        );
        assert!(
            investigator
                .instructions
                .contains("❌ Emit undeclared topics like `debug.start`")
        );
        assert!(
            investigator
                .instructions
                .contains("❌ Skip the event chain by doing fix or verification work inline")
        );

        let tester = config.hats.get("tester").expect("tester hat should exist");
        assert_eq!(tester.triggers, vec!["hypothesis.test".to_string()]);
        assert_eq!(
            tester.publishes,
            vec![
                "hypothesis.confirmed".to_string(),
                "hypothesis.rejected".to_string(),
            ]
        );
        assert!(
            tester
                .instructions
                .contains("nearby adversarial or neighboring failure-path case")
        );

        let fixer = config.hats.get("fixer").expect("fixer hat should exist");
        assert_eq!(
            fixer.publishes,
            vec!["fix.applied".to_string(), "fix.blocked".to_string()]
        );
        assert_eq!(fixer.default_publishes.as_deref(), Some("fix.blocked"));
        assert!(!fixer.instructions.contains("Commit"));
        assert!(
            fixer
                .instructions
                .contains("❌ Make commits in this preset")
        );

        let verifier = config
            .hats
            .get("verifier")
            .expect("verifier hat should exist");
        assert_eq!(
            verifier.publishes,
            vec!["fix.verified".to_string(), "fix.failed".to_string()]
        );
        assert_eq!(verifier.default_publishes.as_deref(), Some("fix.failed"));
        assert!(
            verifier
                .instructions
                .contains("Re-run at least one nearby adversarial or failure-path case.")
        );
    }
}
