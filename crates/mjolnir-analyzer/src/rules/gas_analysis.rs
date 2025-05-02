use super::AnalysisRule;
use crate::models::{Category, Issue, Severity};

/// Rule to analyze gas costs per function
pub struct GasAnalysisRule {}

impl AnalysisRule for GasAnalysisRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut current_function = String::new();
        let mut in_function = false;
        let mut function_start_line = 0;

        for (line_num, line) in code.lines().enumerate() {
            let line = line.trim();

            // Detect function declarations
            if line.contains("fn ") && line.contains("(") {
                in_function = true;
                function_start_line = line_num + 1;
                current_function = line
                    .split("fn ")
                    .nth(1)
                    .unwrap_or("")
                    .split("(")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
            }

            if in_function {
                // Analyze storage operations
                if line.contains("storage") {
                    issues.push(Issue {
                        severity: Severity::Low,
                        message: format!(
                            "Storage operation in function '{}' costs ~20,000 gas",
                            current_function
                        ),
                        line: Some(line_num + 1),
                        recommendation: Some(
                            "Consider caching storage values in memory if accessed multiple times"
                                .to_string(),
                        ),
                    });
                }

                // Analyze external calls
                if line.contains(".call") || line.contains("transfer") {
                    issues.push(Issue {
                        severity: Severity::Medium,
                        message: format!(
                            "External call in function '{}' costs ~2,600 gas",
                            current_function
                        ),
                        line: Some(line_num + 1),
                        recommendation: Some(
                            "Batch external calls when possible to save gas".to_string(),
                        ),
                    });
                }

                // Analyze loops
                if line.contains("for") || line.contains("while") {
                    issues.push(Issue {
                        severity: Severity::Medium,
                        message: format!(
                            "Loop in function '{}' has variable gas cost",
                            current_function
                        ),
                        line: Some(line_num + 1),
                        recommendation: Some(
                            "Consider implementing gas limits for loops".to_string(),
                        ),
                    });
                }
            }

            if line.contains("}") && in_function {
                in_function = false;
            }
        }

        issues
    }

    fn category(&self) -> Category {
        Category::GasEfficiency
    }

    fn id(&self) -> &'static str {
        "gas_analysis"
    }

    fn description(&self) -> &'static str {
        "Analyzes gas costs for each function in the contract"
    }
}
