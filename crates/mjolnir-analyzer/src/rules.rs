//! Analysis rules for smart contracts

use crate::models::{Category, Issue, Severity};

/// Trait for implementing analysis rules
pub trait AnalysisRule {
    /// Analyze the code and return any issues found
    fn analyze(&self, code: &str) -> Vec<Issue>;

    /// Get the category this rule belongs to
    fn category(&self) -> Category;

    /// Get a unique identifier for this rule
    fn id(&self) -> &'static str;

    /// Get a human-readable description of this rule
    fn description(&self) -> &'static str;
}

/// Rule to detect reentrancy vulnerabilities
pub struct ReentrancyRule {}

impl AnalysisRule for ReentrancyRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        // Simplified implementation - in reality, this would use proper parsing
        if code.contains("transfer") && !code.contains("checks-effects-interactions") {
            vec![Issue {
                severity: Severity::High,
                message: "Potential reentrancy vulnerability in withdraw function".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("transfer"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some("Implement checks-effects-interactions pattern".to_string()),
            }]
        } else {
            vec![]
        }
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "reentrancy"
    }

    fn description(&self) -> &'static str {
        "Detects potential reentrancy vulnerabilities in contract functions"
    }
}

/// Rule to check for efficient storage usage
pub struct StorageEfficiencyRule {}

impl AnalysisRule for StorageEfficiencyRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        // Simplified implementation
        if code.contains("storage") && !code.contains("packed") {
            vec![Issue {
                severity: Severity::Medium,
                message: "Inefficient storage usage".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("storage"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Consider using packed storage or a more efficient data structure".to_string(),
                ),
            }]
        } else {
            vec![]
        }
    }

    fn category(&self) -> Category {
        Category::GasEfficiency
    }

    fn id(&self) -> &'static str {
        "storage_efficiency"
    }

    fn description(&self) -> &'static str {
        "Checks for efficient storage usage patterns"
    }
}

/// Rule to check for event emissions after state changes
pub struct EventEmissionRule {}

impl AnalysisRule for EventEmissionRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        // Simplified implementation
        if code.contains("state") && !code.contains("emit") {
            vec![Issue {
                severity: Severity::Low,
                message: "Missing event emission after state change".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("state"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Emit events after significant state changes for better off-chain tracking"
                        .to_string(),
                ),
            }]
        } else {
            vec![]
        }
    }

    fn category(&self) -> Category {
        Category::CodeQuality
    }

    fn id(&self) -> &'static str {
        "event_emission"
    }

    fn description(&self) -> &'static str {
        "Checks for event emissions after state changes"
    }
}

/// Rule to check for gas optimization patterns
pub struct GasOptimizationRule {}

impl AnalysisRule for GasOptimizationRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for expensive operations in loops
        if code.contains("for") && (code.contains("storage") || code.contains("call")) {
            issues.push(Issue {
                severity: Severity::Medium,
                message: "Expensive operation inside loop".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("for"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Move expensive operations outside of loops when possible".to_string(),
                ),
            });
        }

        // Check for unnecessary storage reads
        if code.contains("storage") && code.contains("read") && code.contains("loop") {
            issues.push(Issue {
                severity: Severity::Low,
                message: "Multiple storage reads that could be cached".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("storage") && line.contains("read"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Cache storage values in memory when reading multiple times".to_string(),
                ),
            });
        }

        issues
    }

    fn category(&self) -> Category {
        Category::GasEfficiency
    }

    fn id(&self) -> &'static str {
        "gas_optimization"
    }

    fn description(&self) -> &'static str {
        "Identifies patterns that could be optimized for gas efficiency"
    }
}

/// Rule to check for security best practices
pub struct SecurityBestPracticesRule {}

impl AnalysisRule for SecurityBestPracticesRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for unchecked external calls
        if code.contains("call") && !code.contains("require") {
            issues.push(Issue {
                severity: Severity::High,
                message: "Unchecked external call result".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("call"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some("Always check the return value of external calls".to_string()),
            });
        }

        // Check for proper access control
        if code.contains("function")
            && !code.contains("onlyOwner")
            && !code.contains("require(msg.sender")
        {
            issues.push(Issue {
                severity: Severity::Medium,
                message: "Function may lack proper access control".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("function") && !line.contains("private"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Implement access control for sensitive functions".to_string(),
                ),
            });
        }

        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "security_best_practices"
    }

    fn description(&self) -> &'static str {
        "Checks for adherence to security best practices"
    }
}

/// Get all default rules
pub fn get_default_rules() -> Vec<Box<dyn AnalysisRule>> {
    vec![
        Box::new(ReentrancyRule {}),
        Box::new(StorageEfficiencyRule {}),
        Box::new(EventEmissionRule {}),
        Box::new(GasOptimizationRule {}),
        Box::new(SecurityBestPracticesRule {}),
    ]
}
