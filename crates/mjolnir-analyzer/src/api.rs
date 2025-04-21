//! API integration for the analyzer

use crate::{AnalysisResults, AnalyzerConfig, analyze_contract};
use serde::{Deserialize, Serialize};

/// Request format for the analyzer API
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    /// The smart contract code to analyze
    pub code: String,
    /// Optional configuration for the analyzer
    #[serde(default)]
    pub config: Option<AnalyzerConfig>,
}

/// Response format for the analyzer API
#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    /// The analysis results
    #[serde(flatten)]
    pub results: AnalysisResults,
}

/// Process an analysis request and return a response
pub fn process_request(request: &str) -> Result<String, String> {
    // Parse the request
    let analyze_request: AnalyzeRequest =
        serde_json::from_str(request).map_err(|e| format!("Failed to parse request: {}", e))?;

    // Analyze the contract
    let results = if let Some(config) = analyze_request.config {
        let analyzer = crate::Analyzer::with_config(config);
        analyzer.analyze(&analyze_request.code)
    } else {
        analyze_contract(&analyze_request.code)
    };

    // Create the response
    let response = AnalyzeResponse { results };

    // Serialize the response
    serde_json::to_string(&response).map_err(|e| format!("Failed to serialize response: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_request() {
        let request = r#"{
            "code": "contract Test { function transfer() { /* code */ } }"
        }"#;

        let response = process_request(request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert!(parsed.get("score").is_some());
        assert!(parsed.get("metrics").is_some());
        assert!(parsed.get("issues").is_some());
    }

    #[test]
    fn test_process_request_with_config() {
        let request = r#"{
            "code": "contract Test { function transfer() { /* code */ } }",
            "config": {
                "enabled_rules": ["reentrancy"],
                "custom_weights": {
                    "security": 0.8,
                    "performance": 0.1,
                    "gas_efficiency": 0.05,
                    "code_quality": 0.05
                }
            }
        }"#;

        let response = process_request(request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert!(parsed.get("score").is_some());
    }
}
