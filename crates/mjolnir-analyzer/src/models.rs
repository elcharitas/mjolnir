//! Models for the analyzer results and metrics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Severity level for issues found during analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    High,
    Medium,
    Low,
}

/// Represents an issue found during contract analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Issue {
    pub severity: Severity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
}

/// Metrics calculated during contract analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub performance: u8,
    pub security: u8,
    pub gas_efficiency: u8,
    pub code_quality: u8,
}

/// Results of a contract analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub score: u8,
    pub metrics: Metrics,
    pub issues: Vec<Issue>,
}

/// Category of analysis rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Security,
    Performance,
    GasEfficiency,
    CodeQuality,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Security => "security",
            Category::Performance => "performance",
            Category::GasEfficiency => "gas_efficiency",
            Category::CodeQuality => "code_quality",
        }
    }
}

/// Configuration for the analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    pub enabled_rules: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_weights: Option<HashMap<String, f32>>,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled_rules: vec!["all".to_string()],
            custom_weights: None,
        }
    }
}
