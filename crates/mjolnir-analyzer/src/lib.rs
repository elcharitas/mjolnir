//! Mjolnir Analyzer - A tool for analyzing Polkadot smart contracts
//!
//! This crate provides functionality to analyze smart contracts and generate
//! metrics on security, performance, gas efficiency, and code quality.

pub mod api;
mod models;
mod rules;

pub use models::{AnalysisResults, AnalyzerConfig, Category, Issue, Metrics, Severity};
pub use rules::{AnalysisRule, get_default_rules};

use std::collections::HashMap;

/// Main analyzer for smart contracts
pub struct Analyzer {
    rules: HashMap<String, Box<dyn AnalysisRule>>,
    config: AnalyzerConfig,
}

impl Analyzer {
    /// Create a new analyzer with default rules
    pub fn new() -> Self {
        let mut analyzer = Self {
            rules: HashMap::new(),
            config: AnalyzerConfig::default(),
        };

        // Register default rules
        for rule in get_default_rules() {
            analyzer.register_rule(rule.id(), rule);
        }

        analyzer
    }

    /// Create a new analyzer with custom configuration
    pub fn with_config(config: AnalyzerConfig) -> Self {
        let mut analyzer = Self {
            rules: HashMap::new(),
            config,
        };

        // Register default rules
        for rule in get_default_rules() {
            // Only register rules that are enabled in the config
            if analyzer.config.enabled_rules.contains(&"all".to_string())
                || analyzer
                    .config
                    .enabled_rules
                    .contains(&rule.id().to_string())
            {
                analyzer.register_rule(rule.id(), rule);
            }
        }

        analyzer
    }

    /// Register a new analysis rule
    pub fn register_rule(&mut self, name: &str, rule: Box<dyn AnalysisRule>) {
        self.rules.insert(name.to_string(), rule);
    }

    /// Analyze a smart contract and return results
    pub fn analyze(&self, code: &str) -> AnalysisResults {
        let mut all_issues = Vec::new();

        // Collect issues from all rules
        for rule in self.rules.values() {
            let issues = rule.analyze(code);
            all_issues.extend(issues);
        }

        // Calculate metrics based on issues
        let metrics = self.calculate_metrics(&all_issues, code);

        // Calculate overall score
        let score = self.calculate_score(&metrics, &all_issues);

        AnalysisResults {
            score,
            metrics,
            issues: all_issues,
        }
    }

    /// Calculate metrics based on issues and code
    fn calculate_metrics(&self, issues: &[Issue], code: &str) -> Metrics {
        // Count issues by severity and category
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        let mut performance_issues = 0;
        let mut security_issues = 0;
        let mut gas_issues = 0;
        let mut quality_issues = 0;

        for issue in issues {
            match issue.severity {
                Severity::High => high_count += 1,
                Severity::Medium => medium_count += 1,
                Severity::Low => low_count += 1,
            }
        }

        // Count issues by category
        for rule in self.rules.values() {
            match rule.category() {
                Category::Performance => {
                    performance_issues += issues
                        .iter()
                        .filter(|i| rule.analyze(code).contains(i))
                        .count()
                }
                Category::Security => {
                    security_issues += issues
                        .iter()
                        .filter(|i| rule.analyze(code).contains(i))
                        .count()
                }
                Category::GasEfficiency => {
                    gas_issues += issues
                        .iter()
                        .filter(|i| rule.analyze(code).contains(i))
                        .count()
                }
                Category::CodeQuality => {
                    quality_issues += issues
                        .iter()
                        .filter(|i| rule.analyze(code).contains(i))
                        .count()
                }
            }
        }

        let code_len = code.lines().count() as f32;
        let base_score = 100.0;

        let performance = (base_score - (performance_issues as f32 * 10.0).min(30.0)) as u8;
        let security = (base_score
            - (high_count as f32 * 15.0 + medium_count as f32 * 7.0 + low_count as f32 * 2.0)
                .min(30.0)) as u8;
        let gas_efficiency = (base_score - (gas_issues as f32 * 10.0).min(30.0)) as u8;
        let code_quality = (base_score - (quality_issues as f32 * 5.0).min(30.0)) as u8;

        Metrics {
            performance,
            security,
            gas_efficiency,
            code_quality,
        }
    }

    /// Calculate overall score based on metrics and issues
    fn calculate_score(&self, metrics: &Metrics, issues: &[Issue]) -> u8 {
        // Get weights from config or use defaults
        let weights = if let Some(custom_weights) = &self.config.custom_weights {
            (
                custom_weights.get("security").copied().unwrap_or(0.4),
                custom_weights.get("performance").copied().unwrap_or(0.2),
                custom_weights.get("gas_efficiency").copied().unwrap_or(0.3),
                custom_weights.get("code_quality").copied().unwrap_or(0.1),
            )
        } else {
            (0.4, 0.2, 0.3, 0.1) // Default weights
        };

        // Weight the metrics
        let weighted_score = (metrics.security as f32 * weights.0
            + metrics.performance as f32 * weights.1
            + metrics.gas_efficiency as f32 * weights.2
            + metrics.code_quality as f32 * weights.3) as u8;

        // Adjust for critical issues
        let critical_penalty = issues
            .iter()
            .filter(|i| i.severity == Severity::High)
            .count() as u8
            * 5;

        weighted_score.saturating_sub(critical_penalty)
    }
}

/// Convenience function to analyze a contract with default settings
pub fn analyze_contract(code: &str) -> AnalysisResults {
    let analyzer = Analyzer::new();
    analyzer.analyze(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer() {
        let analyzer = Analyzer::new();
        let code = r#"
            contract MyContract {
                function transfer(address to, uint amount) public {
                    // Missing checks-effects-interactions pattern
                    payable(to).transfer(amount);
                    balances[msg.sender] -= amount;
                }
                
                // Other contract code...
            }
        "#;

        let results = analyzer.analyze(code);

        // Verify we have at least one issue
        assert!(!results.issues.is_empty());

        // Verify metrics are calculated
        assert!(results.metrics.security <= 100);
        assert!(results.metrics.performance <= 100);
        assert!(results.metrics.gas_efficiency <= 100);
        assert!(results.metrics.code_quality <= 100);

        // Verify overall score is calculated
        assert!(results.score <= 100);
    }

    #[test]
    fn test_custom_config() {
        let config = AnalyzerConfig {
            enabled_rules: vec!["reentrancy".to_string()],
            custom_weights: Some(
                [
                    ("security".to_string(), 0.8),
                    ("performance".to_string(), 0.1),
                    ("gas_efficiency".to_string(), 0.05),
                    ("code_quality".to_string(), 0.05),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
        };

        let analyzer = Analyzer::with_config(config);
        let code = "function transfer() { /* code */ }";

        analyzer.analyze(code);
        // With custom config, we should only have reentrancy rule enabled
        assert!(analyzer.rules.len() == 1);
    }
}
