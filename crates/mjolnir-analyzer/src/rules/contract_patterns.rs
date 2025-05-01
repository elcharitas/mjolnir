//! Rules for detecting common smart contract anti-patterns and code quality issues

use crate::models::{Category, Issue, Severity};
use crate::rules::AnalysisRule;

/// Rule to detect missing function visibility specifiers
pub struct MissingVisibilityRule {}

impl AnalysisRule for MissingVisibilityRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for function declarations without visibility specifiers
        for (i, line) in code.lines().enumerate() {
            if line.contains("function")
                && !line.contains("public")
                && !line.contains("private")
                && !line.contains("internal")
                && !line.contains("external")
            {
                issues.push(Issue {
                    severity: Severity::Low,
                    message: "Function missing explicit visibility specifier".to_string(),
                    line: Some(i + 1),
                    recommendation: Some(
                        "Always specify function visibility (public, private, internal, or external)".to_string(),
                    ),
                });
            }
        }

        issues
    }

    fn category(&self) -> Category {
        Category::CodeQuality
    }

    fn id(&self) -> &'static str {
        "missing_visibility"
    }

    fn description(&self) -> &'static str {
        "Detects functions missing explicit visibility specifiers"
    }
}

/// Rule to detect floating pragma versions
pub struct FloatingPragmaRule {}

impl AnalysisRule for FloatingPragmaRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if code.contains("pragma solidity")
            && (code.contains("^")
                || code.contains(">")
                || code.contains("<")
                || code.contains("~"))
        {
            vec![Issue {
                severity: Severity::Medium,
                message: "Floating pragma version".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("pragma solidity"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Lock pragma to specific compiler version for consistency and security"
                        .to_string(),
                ),
            }]
        } else {
            vec![]
        }
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "floating_pragma"
    }

    fn description(&self) -> &'static str {
        "Detects floating pragma versions"
    }
}

/// Rule to detect use of deprecated functions or patterns
pub struct DeprecatedPatternsRule {}

impl AnalysisRule for DeprecatedPatternsRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for deprecated functions
        let deprecated_functions = ["suicide", "block.blockhash", "sha3", "throw", "msg.gas"];

        for deprecated in deprecated_functions.iter() {
            if code.contains(deprecated) {
                issues.push(Issue {
                    severity: Severity::Medium,
                    message: format!("Use of deprecated function or pattern: {}", deprecated),
                    line: Some(
                        code.lines()
                            .position(|line| line.contains(deprecated))
                            .unwrap_or(0)
                            + 1,
                    ),
                    recommendation: Some(format!(
                        "Replace with recommended alternative according to Solidity documentation"
                    )),
                });
            }
        }

        issues
    }

    fn category(&self) -> Category {
        Category::CodeQuality
    }

    fn id(&self) -> &'static str {
        "deprecated_patterns"
    }

    fn description(&self) -> &'static str {
        "Detects use of deprecated functions or patterns"
    }
}

/// Rule to detect tx.origin usage for authorization
pub struct TxOriginAuthRule {}

impl AnalysisRule for TxOriginAuthRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if code.contains("tx.origin") {
            vec![Issue {
                severity: Severity::High,
                message: "Use of tx.origin for authorization".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("tx.origin"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some(
                    "Use msg.sender instead of tx.origin for authorization checks".to_string(),
                ),
            }]
        } else {
            vec![]
        }
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "tx_origin_auth"
    }

    fn description(&self) -> &'static str {
        "Detects use of tx.origin for authorization"
    }
}

/// Rule to detect assembly usage without comments
pub struct AssemblyUsageRule {}

impl AnalysisRule for AssemblyUsageRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut in_assembly = false;
        let mut assembly_start_line = 0;
        let mut has_comment = false;

        for (i, line) in code.lines().enumerate() {
            if line.contains("assembly") && !in_assembly {
                in_assembly = true;
                assembly_start_line = i + 1;
                has_comment = line.contains("//") || line.contains("/*");
            } else if in_assembly {
                if line.contains("}") {
                    if !has_comment {
                        issues.push(Issue {
                            severity: Severity::Medium,
                            message: "Assembly block without documentation".to_string(),
                            line: Some(assembly_start_line),
                            recommendation: Some(
                                "Document assembly blocks with detailed comments explaining the purpose and behavior".to_string(),
                            ),
                        });
                    }
                    in_assembly = false;
                } else if line.contains("//") || line.contains("/*") {
                    has_comment = true;
                }
            }
        }

        issues
    }

    fn category(&self) -> Category {
        Category::CodeQuality
    }

    fn id(&self) -> &'static str {
        "assembly_usage"
    }

    fn description(&self) -> &'static str {
        "Detects assembly usage without proper documentation"
    }
}
