use super::AnalysisRule;
use crate::models::{Category, Issue, Severity};

/// Rule to analyze security vulnerabilities
pub struct SecurityAnalysisRule {}

impl AnalysisRule for SecurityAnalysisRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for unchecked return values
        for (line_num, line) in code.lines().enumerate() {
            let line = line.trim();

            // Check for unchecked external calls
            if (line.contains(".call") || line.contains("transfer"))
                && !line.contains("require")
                && !line.contains("assert")
            {
                issues.push(Issue {
                    severity: Severity::High,
                    message: "Unchecked return value from external call".to_string(),
                    line: Some(line_num + 1),
                    recommendation: Some(
                        "Always check return values from external calls".to_string(),
                    ),
                });
            }

            // Check for integer overflow vulnerabilities
            if line.contains("+") || line.contains("-") || line.contains("*") {
                if !line.contains("checked_") && !line.contains("SafeMath") {
                    issues.push(Issue {
                        severity: Severity::High,
                        message: "Potential integer overflow/underflow".to_string(),
                        line: Some(line_num + 1),
                        recommendation: Some(
                            "Use checked arithmetic operations or SafeMath library".to_string(),
                        ),
                    });
                }
            }

            // Check for unprotected function calls
            if line.contains("pub fn")
                && !line.contains("#[access_control]")
                && !line.contains("onlyOwner")
            {
                issues.push(Issue {
                    severity: Severity::Medium,
                    message: "Unprotected public function".to_string(),
                    line: Some(line_num + 1),
                    recommendation: Some(
                        "Consider adding access control to sensitive functions".to_string(),
                    ),
                });
            }

            // Check for timestamp dependencies
            if line.contains("block.timestamp") || line.contains("now") {
                issues.push(Issue {
                    severity: Severity::Medium,
                    message: "Timestamp dependency".to_string(),
                    line: Some(line_num + 1),
                    recommendation: Some(
                        "Be aware that timestamps can be manipulated by miners".to_string(),
                    ),
                });
            }

            // Check for potential DoS vectors
            if line.contains("for") && line.contains("transfer") {
                issues.push(Issue {
                    severity: Severity::High,
                    message: "Potential DoS vector in loop with transfers".to_string(),
                    line: Some(line_num + 1),
                    recommendation: Some(
                        "Implement pull payment pattern instead of push payments in loops"
                            .to_string(),
                    ),
                });
            }
        }

        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "security_analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes common security vulnerabilities in smart contracts"
    }
}
