export type Severity = "high" | "medium" | "low";

export interface Issue {
	severity: Severity;
	message: string;
	line?: number;
	recommendation?: string;
}

export interface Metrics {
	performance: number;
	security: number;
	gas_efficiency: number;
	code_quality: number;
}

export interface AnalysisResults {
	score: number;
	metrics: Metrics;
	issues: Issue[];
}

export interface AnalyzerConfig {
	enabled_rules: string[];
	custom_weights?: Record<string, number>;
}

export interface AnalyzeRequest {
	code: string;
	config?: AnalyzerConfig;
}
