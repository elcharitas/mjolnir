"use server";

import { execSync } from "node:child_process";
import { join } from "node:path";

export type ContractType = "ink" | "solidity";

export type ConversionConfig = {
	target: ContractType;
	optimize?: boolean;
};

export type ConversionResult = {
	convertedCode: string;
	targetType: ContractType;
	compilationOutput?: string;
};

// Path to the converter binary
const CONVERTER_PATH = join(
	process.cwd(),
	"target/release/mjolnir_converter_cli",
);

export async function convertContract(
	code: string,
	config: ConversionConfig,
): Promise<ConversionResult | null> {
	try {
		const request = { code, config };
		const response = execSync(`${CONVERTER_PATH}`, {
			input: JSON.stringify(request),
			encoding: "utf-8",
		});

		return JSON.parse(response) as ConversionResult;
	} catch (error) {
		console.error("Error converting contract:", error);
		return null;
	}
}
