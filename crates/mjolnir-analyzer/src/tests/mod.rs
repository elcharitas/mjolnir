//! Tests for the analyzer

#[cfg(test)]
mod contract_rules_test;

#[cfg(test)]
mod basic_test {
    #[cfg(test)]
    mod tests {
        use crate::*;

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
}
