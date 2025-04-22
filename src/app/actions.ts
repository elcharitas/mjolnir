"use server";

import type {
	AnalyzerConfig,
	AnalysisResults,
	AnalyzeRequest,
} from "@/lib/analyzer";
import { execSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

// Path to the Rust analyzer binary
const ANALYZER_PATH = join(
	process.cwd(),
	"target/release/mjolnir_analyzer_cli",
);

export async function analyzeContract(
	code: string,
	config?: AnalyzerConfig,
): Promise<AnalysisResults | null> {
	try {
		if (!existsSync(ANALYZER_PATH)) {
			return null;
		}

		const request: AnalyzeRequest = { code, config };
		const response = execSync(`${ANALYZER_PATH}`, {
			input: JSON.stringify(request),
			encoding: "utf-8",
		});

		return JSON.parse(response) as AnalysisResults;
	} catch (error) {
		console.error("Error analyzing contract:", error);
		return null;
	}
}
