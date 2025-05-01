//! Advanced rules for detecting complex smart contract vulnerabilities

use crate::models::{Category, Issue, Severity};
use crate::rules::AnalysisRule;

/// Rule to detect DoS with unexpected revert vulnerabilities
pub struct DosWithRevertRule {}

impl AnalysisRule for DosWithRevertRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for loops that iterate over arrays with external calls inside
        if (code.contains("for") || code.contains("while")) && 
           (code.contains(".transfer") || code.contains(".send") || code.contains(".call")) {
            
            issues.push(Issue {
                severity: Severity::High,
                message: "Potential DoS with unexpected revert vulnerability".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            (line.contains("for") || line.contains("while")) && 
                            (line.contains(".transfer") || line.contains(".send") || line.contains(".call"))
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Use pull payment pattern instead of pushing payments in loops".to_string(),
                ),
            });
        }
        
        issues
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn id(&self) -> &'static str {
        "dos_with_revert"
    }

    fn description(&self) -> &'static str {
        "Detects potential DoS with unexpected revert vulnerabilities"
    }
}

/// Rule to detect block gas limit vulnerabilities
pub struct BlockGasLimitRule {}

impl AnalysisRule for BlockGasLimitRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for unbounded operations that might hit block gas limit
        if (code.contains("for") || code.contains("while")) && 
           (code.contains("array") || code.contains("mapping") || code.contains("[]")){            
            issues.push(Issue {
                severity: Severity::Medium,
                message: "Potential block gas limit vulnerability with unbounded operation".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            (line.contains("for") || line.contains("while")) && 
                            (line.contains("array") || line.contains("mapping") || line.contains("["))
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Implement pagination or limit the number of iterations to avoid hitting block gas limit".to_string(),
                ),
            });
        }
        
        issues
    }

    fn category(&self) -> Category {
        Category::GasEfficiency
    }

    fn id(&self) -> &'static str {
        "block_gas_limit"
    }

    fn description(&self) -> &'static str {
        "Detects operations that might hit block gas limit"
    }
}

/// Rule to detect force-sending ether vulnerabilities
pub struct ForceSendEtherRule {}

impl AnalysisRule for ForceSendEtherRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if code.contains("selfdestruct") || code.contains("suicide") {
            vec![Issue {
                severity: Severity::Medium,
                message: "Contract uses selfdestruct which can force-send ether".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| 
                            line.contains("selfdestruct") || line.contains("suicide")
                        )
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Be aware that contracts can receive ether via selfdestruct even without fallback or receive functions".to_string(),
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
        "force_send_ether"
    }

    fn description(&self) -> &'static str {
        "Detects force-sending ether vulnerabilities"
    }
}

/// Rule to detect signature malleability vulnerabilities
pub struct SignatureMalleabilityRule {}

impl AnalysisRule for SignatureMalleabilityRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        if code.contains("ecrecover") && !code.contains("ecrecover(hash, v, r, s)") {
            vec![Issue {
                severity: Severity::Medium,
                message: "Potential signature malleability vulnerability".to_string(),
                line: Some(
                    code.lines()
                        .position(|line| line.contains("ecrecover"))
                        .unwrap_or(0) + 1,
                ),
                recommendation: Some(
                    "Ensure signatures are properly validated and consider using OpenZeppelin's ECDSA library".to_string(),
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
        "signature_malleability"
    }

    fn description(&self) -> &'static str {
        "Detects potential signature malleability vulnerabilities"
    }
}

/// Rule to detect weak randomness vulnerabilities
pub struct WeakRandomnessRule {}

impl AnalysisRule for WeakRandomnessRule {
    fn analyze(&self, code: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check for common weak sources of randomness
        let weak_sources = [
            "block.timestamp", 
            "now", 
            "block.number", 
            "blockhash",
            "block.difficulty",
            "block.coinbase",
            "block.gaslimit",
            "msg.gas",
            "tx.gasprice",
        ];
        
        for source in weak_sources.iter() {
            if code.contains(source) && 
               (code.contains("random") || code.contains("lottery") || code.contains("select") || code.contains("winner")) {
                issues.push(Issue {
                    severity: Severity::High,
                    message: format!("Weak randomness using {}", source),
                    line: Some(
                        code.lines()
                            .position(|line| line.contains(source))
                            .unwrap_or(0) + 1,
                    ),
                    recommendation: Some(
                        "Use a secure source of randomness such as Chainlink VRF or commit-reveal schemes".to_string(),
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
        "weak_randomness"
    }

    fn description(&self) -> &'static str {
        "Detects weak sources of randomness"
    }
}