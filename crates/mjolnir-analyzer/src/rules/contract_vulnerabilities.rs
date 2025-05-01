//! Rules for detecting common smart contract vulnerabilities and anti-patterns

use crate::models::{Category, Issue, Severity};
use crate::rules::AnalysisRule;

// ReentrancyRule is already defined in mod.rs

/// Rule to detect DoS vulnerabilities
pub struct DoSVulnerabilityRule {}

impl AnalysisRule for DoSVulnerabilityRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for unbounded loops with external calls
        if code.contains("for") && 
           (code.contains("transfer") || code.contains(".call")) {
            issues.push(Issue {
                severity: Severity::High,
                message: "DoS vulnerability: unbounded loop with external calls".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("for"))
                        .unwrap_or(0)
                        + 1,
                ),
                recommendation: Some("Use pull payment pattern instead of push payments in loops".to_string()),
            });
        }
        
        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "dos_vulnerability"
    }

    fn description(&self) -> &'static str {
        "Detects potential DoS vulnerabilities"
    }
}

/// Rule to detect integer overflow/underflow vulnerabilities
pub struct IntegerOverflowRule {}

impl AnalysisRule for IntegerOverflowRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for arithmetic operations without SafeMath or overflow checks
        if (code.contains("+") || code.contains("-") || code.contains("*") || code.contains("/")) 
            && !code.contains("SafeMath") 
            && !code.contains("checked_add") 
            && !code.contains("checked_sub") 
            && !code.contains("checked_mul") 
            && !code.contains("checked_div") {
            
            issues.push(Issue {
                severity: Severity::High,
                message: "Potential integer overflow/underflow vulnerability".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains("+") || 
                            line.contains("-") || 
                            line.contains("*") || 
                            line.contains("/")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Use SafeMath library or checked arithmetic operations".to_string(),
                ),
            });
        }
        
        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "integer_overflow"
    }

    fn description(&self) -> &'static str {
        "Detects potential integer overflow/underflow vulnerabilities"
    }
}

/// Rule to detect unprotected self-destruct vulnerabilities
pub struct SelfDestructRule {}

impl AnalysisRule for SelfDestructRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if (code.contains("selfdestruct") || code.contains("self-destruct") || code.contains("suicide")) 
            && !code.contains("onlyOwner") 
            && !code.contains("require(msg.sender") {
            
            vec![Issue {
                severity: Severity::High,
                message: "Unprotected self-destruct functionality".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains("selfdestruct") || 
                            line.contains("self-destruct") || 
                            line.contains("suicide")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Add proper access control to self-destruct functionality".to_string(),
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
        "unprotected_selfdestruct"
    }

    fn description(&self) -> &'static str {
        "Detects unprotected self-destruct functionality"
    }
}

/// Rule to detect timestamp dependence vulnerabilities
pub struct TimestampDependenceRule {}

impl AnalysisRule for TimestampDependenceRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if code.contains("block.timestamp") || code.contains("now") {
            vec![Issue {
                severity: Severity::Medium,
                message: "Contract logic depends on block timestamp".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains("block.timestamp") || 
                            line.contains("now")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Avoid using block.timestamp for critical logic as it can be manipulated by miners".to_string(),
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
        "timestamp_dependence"
    }

    fn description(&self) -> &'static str {
        "Detects timestamp dependence vulnerabilities"
    }
}

/// Rule to detect front-running vulnerabilities
pub struct FrontRunningRule {}

impl AnalysisRule for FrontRunningRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if (code.contains("price") || code.contains("rate") || code.contains("swap")) 
            && !code.contains("commit-reveal") 
            && !code.contains("timelock") {
            
            vec![Issue {
                severity: Severity::Medium,
                message: "Potential front-running vulnerability".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains("price") || 
                            line.contains("rate") || 
                            line.contains("swap")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Consider implementing commit-reveal pattern or timelock mechanisms".to_string(),
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
        "front_running"
    }

    fn description(&self) -> &'static str {
        "Detects potential front-running vulnerabilities"
    }
}

/// Rule to detect unchecked return values
pub struct UncheckedReturnRule {}

impl AnalysisRule for UncheckedReturnRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for low-level calls without return value checks
        if (code.contains(".call{") || code.contains(".call(")) 
            && !code.contains("require(") 
            && !code.contains("assert(") {
            
            issues.push(Issue {
                severity: Severity::High,
                message: "Unchecked return value from low-level call".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains(".call{") || 
                            line.contains(".call(")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Always check return values from low-level calls".to_string(),
                ),
            });
        }
        
        // Check for send/transfer without proper error handling
        if (code.contains(".send(") || code.contains(".transfer(")) 
            && !code.contains("require(") {
            
            issues.push(Issue {
                severity: Severity::Medium,
                message: "Potential unchecked send/transfer".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains(".send(") || 
                            line.contains(".transfer(")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Check return value of .send() or use .transfer() with proper error handling".to_string(),
                ),
            });
        }
        
        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "unchecked_return"
    }

    fn description(&self) -> &'static str {
        "Detects unchecked return values from external calls"
    }
}