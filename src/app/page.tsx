"use client";

import { useState } from "react";
import Image from "next/image";
import AnalysisResults, {
	type AnalysisResultsProps,
} from "@/components/AnalysisResults";
import ContractInput from "@/components/ContractInput";
import { analyzeContract } from "./actions";
import { convertContract, type ContractType } from "@/lib/converter";
import toast from "react-hot-toast";

export default function Home() {
	const [contractCode, setContractCode] = useState("");
	const [isAnalyzing, setIsAnalyzing] = useState(false);
	const [isConverting, setIsConverting] = useState(false);
	const [analysisResults, setAnalysisResults] =
		useState<AnalysisResultsProps | null>(null);

	const handleAnalyze = async () => {
		const trimmedCode = contractCode.trim();
		if (!trimmedCode) {
			toast.error("No contract code provided");
			return;
		}

		// Basic contract structure validation
		const isInkContract =
			trimmedCode.includes("#[ink::contract]") ||
			trimmedCode.includes("#[contract]");
		const isSolidityContract =
			trimmedCode.includes("contract") && trimmedCode.includes("{");

		if (!isInkContract && !isSolidityContract) {
			toast.error(
				"Invalid contract: Must be either an Ink! or Solidity contract",
			);
			return;
		}

		setIsAnalyzing(true);
		try {
			const result = await analyzeContract(contractCode);
			setAnalysisResults(result);
		} catch (error) {
			console.error("Analysis failed:", error);
			toast.error("Analysis failed. Please try again.");
		} finally {
			setIsAnalyzing(false);
		}
	};

	const handleConvert = async (targetType: ContractType) => {
		const trimmedCode = contractCode.trim();
		if (!trimmedCode) {
			toast.error("No contract code provided");
			return;
		}

		setIsConverting(true);
		try {
			const result = await convertContract(contractCode, {
				target: targetType,
			});
			if (result) {
				setContractCode(result.convertedCode);
				toast.success(`Successfully converted to ${targetType}`);
			} else {
				toast.error(`Failed to convert to ${targetType}`);
			}
		} catch (error) {
			console.error("Conversion failed:", error);
			toast.error("Conversion failed. Please try again.");
		} finally {
			setIsConverting(false);
		}
	};

	return (
		<div className="min-h-screen bg-gradient-to-b from-background to-background/95 p-6 sm:p-10">
			<header className="max-w-6xl mx-auto mb-10">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<Image
							src="/mjolnir-logo.svg"
							alt="Mjolnir Logo"
							width={32}
							height={32}
							className="dark:invert"
						/>
						<h1 className="text-2xl font-bold font-[family-name:var(--font-geist-sans)]">
							Mjolnir
						</h1>
					</div>
					<div className="text-sm font-[family-name:var(--font-geist-mono)] opacity-70">
						Polkadot Smart Contract Analyzer
					</div>
				</div>
			</header>

			<main className="max-w-6xl mx-auto">
				<ContractInput
					contractCode={contractCode}
					setContractCode={setContractCode}
					isAnalyzing={isAnalyzing}
					onAnalyze={handleAnalyze}
					isConverting={isConverting}
					onConvert={handleConvert}
				/>

				{analysisResults && <AnalysisResults {...analysisResults} />}
			</main>

			<footer className="max-w-6xl mx-auto pt-8 border-t border-foreground/10 mt-12 text-center text-sm opacity-70 font-[family-name:var(--font-geist-mono)]">
				<p>Mjolnir - Polkadot Smart Contract Analysis Tool</p>
			</footer>
		</div>
	);
}
