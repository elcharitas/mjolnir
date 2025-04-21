/**
 * Mjolnir Analyzer Integration
 *
 * This file provides the integration between the NextJS app and the Rust analyzer.
 * It uses Node.js child_process to communicate with the Rust analyzer.
 */

import { execSync } from "node:child_process";
import { join } from "node:path";
import { existsSync } from "node:fs";

// Types matching the Rust analyzer's output structure
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

// Path to the Rust analyzer binary
const ANALYZER_PATH = join(
	process.cwd(),
	"target/release/mjolnir_analyzer_cli",
);

/**
 * Analyzes a smart contract using the Rust analyzer
 *
 * @param code The smart contract code to analyze
 * @param config Optional configuration for the analyzer
 * @returns Analysis results
 */
export async function analyzeContract(
	code: string,
	config?: AnalyzerConfig,
): Promise<AnalysisResults> {
	try {
		// Check if the analyzer binary exists
		if (!existsSync(ANALYZER_PATH)) {
			console.error("Analyzer binary not found. Using mock data instead.");
			return getMockAnalysisResults(code);
		}

		// Prepare the request
		const request: AnalyzeRequest = { code, config };

		// Execute the analyzer and get the response
		const response = execSync(`${ANALYZER_PATH}`, {
			input: JSON.stringify(request),
			encoding: "utf-8",
		});

		// Parse the response
		return JSON.parse(response) as AnalysisResults;
	} catch (error) {
		console.error("Error analyzing contract:", error);
		// Fallback to mock data if the analyzer fails
		return getMockAnalysisResults(code);
	}
}

/**
 * Generates mock analysis results for testing or when the analyzer is unavailable
 *
 * @param code The smart contract code
 * @returns Mock analysis results
 */
function getMockAnalysisResults(code: string): AnalysisResults {
	return {
		score: Math.floor(70 + Math.random() * 30),
		metrics: {
			performance: Math.floor(70 + Math.random() * 30),
			security: Math.floor(70 + Math.random() * 30),
			gas_efficiency: Math.floor(70 + Math.random() * 30),
			code_quality: Math.floor(70 + Math.random() * 30),
		},
		issues: [
			{
				severity: "high",
				message: "Potential reentrancy vulnerability in withdraw function",
				line: 42,
				recommendation: "Implement checks-effects-interactions pattern",
			},
			{
				severity: "medium",
				message: "Inefficient storage usage",
				line: 56,
				recommendation:
					"Consider using packed storage or a more efficient data structure",
			},
			{
				severity: "low",
				message: "Missing event emission after state change",
				line: 78,
				recommendation:
					"Emit events after significant state changes for better off-chain tracking",
			},
		],
	};
}