import { type NextRequest, NextResponse } from "next/server";
import { analyzeContract } from "@/lib/analyzer";

// This route uses the Rust analyzer integration from @/lib/analyzer
// The integration handles communication with the Rust analyzer crate

export async function POST(request: NextRequest) {
	try {
		const { code, config } = await request.json();

		if (!code || typeof code !== "string") {
			return NextResponse.json(
				{ error: "Contract code is required" },
				{ status: 400 },
			);
		}

		// Call the Rust analyzer through our integration
		const analysisResults = await analyzeContract(code, config);

		return NextResponse.json(analysisResults);
	} catch (error) {
		console.error("Error analyzing contract:", error);
		return NextResponse.json(
			{ error: "Failed to analyze contract" },
			{ status: 500 },
		);
	}
}
