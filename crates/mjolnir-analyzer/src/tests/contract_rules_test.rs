//! Tests for the contract analysis rules

use crate::{Analyzer, AnalyzerConfig, analyze_contract};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulnerability_detection() {
        // Test contract with multiple vulnerabilities
        let vulnerable_code = r#"
            contract VulnerableContract {
                uint public lastPrice = 0;
                mapping(address => uint) balances;
                
                function deposit() public payable {
                    balances[msg.sender] += msg.value;
                }
                
                function withdraw(uint amount) public {
                    // Reentrancy vulnerability
                    payable(msg.sender).transfer(amount);
                    balances[msg.sender] -= amount;
                }
                
                function setPrice(uint newPrice) public {
                    // Timestamp dependence
                    require(block.timestamp > lastPrice + 1 hours);
                    lastPrice = newPrice;
                }
                
                function multiTransfer(address[] memory recipients, uint amount) public {
                    // DoS with revert and block gas limit
                    for(uint i = 0; i < recipients.length; i++) {
                        payable(recipients[i]).transfer(amount);
                    }
                }
                
                function unsafeCall(address target, bytes memory data) public {
                    // Unchecked return value
                    target.call(data);
                }
                
                function calculateTotal(uint a, uint b) public pure returns (uint) {
                    // Integer overflow
                    return a + b;
                }
            }
        "#;

        let results = analyze_contract(vulnerable_code);

        // Verify that we detected the vulnerabilities
        assert!(
            results.issues.len() > 0,
            "No issues detected in vulnerable code"
        );

        // Check for specific vulnerabilities
        let has_reentrancy = results
            .issues
            .iter()
            .any(|i| i.message.contains("reentrancy"));
        let has_timestamp = results
            .issues
            .iter()
            .any(|i| i.message.contains("timestamp"));
        let has_dos = results.issues.iter().any(|i| i.message.contains("DoS"));
        let has_unchecked = results
            .issues
            .iter()
            .any(|i| i.message.contains("Unchecked"));
        let has_overflow = results
            .issues
            .iter()
            .any(|i| i.message.contains("overflow"));

        assert!(has_reentrancy, "Reentrancy vulnerability not detected");
        assert!(has_timestamp, "Timestamp dependence not detected");
        assert!(has_dos, "DoS vulnerability not detected");
        assert!(has_unchecked, "Unchecked return value not detected");
        assert!(has_overflow, "Integer overflow not detected");
    }

    #[test]
    fn test_code_quality_detection() {
        // Test contract with code quality issues
        let poor_quality_code = r#"
            pragma solidity ^0.8.0;
            
            contract PoorQualityContract {
                // Missing visibility specifier
                function doSomething() {
                    // Using deprecated function
                    uint blockHash = block.blockhash(0);
                }
                
                // Using tx.origin for authorization
                function sensitiveOperation() {
                    require(tx.origin == 0x123456789, "Not authorized");
                }
                
                // Assembly without comments
                function unsafeOperation() public {
                    assembly {
                        let x := 1
                        let y := 2
                        let z := add(x, y)
                    }
                }
            }
        "#;

        let results = analyze_contract(poor_quality_code);

        // Verify that we detected the code quality issues
        assert!(
            results.issues.len() > 0,
            "No issues detected in poor quality code"
        );

        // Check for specific code quality issues
        let has_missing_visibility = results
            .issues
            .iter()
            .any(|i| i.message.contains("visibility"));
        let has_deprecated = results
            .issues
            .iter()
            .any(|i| i.message.contains("deprecated"));
        let has_tx_origin = results
            .issues
            .iter()
            .any(|i| i.message.contains("tx.origin"));
        let has_assembly = results
            .issues
            .iter()
            .any(|i| i.message.contains("Assembly"));
        let has_floating_pragma = results.issues.iter().any(|i| i.message.contains("pragma"));

        assert!(has_missing_visibility, "Missing visibility not detected");
        assert!(has_deprecated, "Deprecated function not detected");
        assert!(has_tx_origin, "tx.origin usage not detected");
        assert!(has_assembly, "Undocumented assembly not detected");
        assert!(has_floating_pragma, "Floating pragma not detected");
    }

    #[test]
    fn test_rule_filtering() {
        // Test that we can filter rules
        let code = r#"
            contract TestContract {
                function withdraw(uint amount) public {
                    // Reentrancy vulnerability
                    payable(msg.sender).transfer(amount);
                    balances[msg.sender] -= amount;
                }
            }
        "#;

        // Create analyzer with only reentrancy rule enabled
        let config = AnalyzerConfig {
            enabled_rules: vec!["reentrancy".to_string()],
            custom_weights: None,
        };

        let analyzer = Analyzer::with_config(config);
        let results = analyzer.analyze(code);

        // Should only have reentrancy issues
        assert!(
            results
                .issues
                .iter()
                .all(|i| i.message.contains("reentrancy"))
        );
    }
}
