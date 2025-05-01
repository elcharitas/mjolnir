//! Analysis rules for smart contracts

mod advanced_vulnerabilities;
mod contract_patterns;
mod contract_vulnerabilities;

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
        let mut issues = Vec::new();

        // Check for state changes after external calls
        let has_transfer = code.contains("transfer") || code.contains(".call");
        let has_state_change_after_call = code.lines().enumerate().any(|(i, line)| {
            if line.contains("transfer") || line.contains(".call") {
                // Check if any line after this contains state changes
                code.lines().skip(i + 1).any(|next_line| {
                    next_line.contains("=")
                        || next_line.contains("[]")
                        || next_line.contains("balances")
                })
            } else {
                false
            }
        });

        if has_transfer && has_state_change_after_call {
            issues.push(Issue {
                severity: Severity::High,
                message: "Potential reentrancy vulnerability in withdraw function".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("transfer") || line.contains(".call"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some("Implement checks-effects-interactions pattern: perform all state changes before making external calls".to_string()),
            });
        }

        issues
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

// Re-export the vulnerability and pattern rules
pub use advanced_vulnerabilities::*;
pub use contract_patterns::*;
pub use contract_vulnerabilities::*;

/// Get all default rules
pub fn get_default_rules() -> Vec<Box<dyn AnalysisRule>> {
    vec![
        // Original rules
        Box::new(ReentrancyRule {}),
        Box::new(StorageEfficiencyRule {}),
        Box::new(EventEmissionRule {}),
        Box::new(GasOptimizationRule {}),
        Box::new(SecurityBestPracticesRule {}),
        // Vulnerability rules
        Box::new(IntegerOverflowRule {}),
        Box::new(SelfDestructRule {}),
        Box::new(TimestampDependenceRule {}),
        Box::new(FrontRunningRule {}),
        Box::new(UncheckedReturnRule {}),
        Box::new(DoSVulnerabilityRule {}),
        // Pattern rules
        Box::new(MissingVisibilityRule {}),
        Box::new(FloatingPragmaRule {}),
        Box::new(DeprecatedPatternsRule {}),
        Box::new(TxOriginAuthRule {}),
        Box::new(AssemblyUsageRule {}),
        // Advanced vulnerability rules
        Box::new(DosWithRevertRule {}),
        Box::new(BlockGasLimitRule {}),
        Box::new(ForceSendEtherRule {}),
        Box::new(SignatureMalleabilityRule {}),
        Box::new(WeakRandomnessRule {}),
    ]
}
