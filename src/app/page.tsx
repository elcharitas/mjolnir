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
					<div className="flex items-center gap-6">
						<div className="text-sm font-[family-name:var(--font-geist-mono)] opacity-70">
							Polkadot Smart Contract Analyzer
						</div>
						<a
							href="https://github.com/elcharitas/mjolnir"
							target="_blank"
							rel="noopener noreferrer"
							className="flex items-center gap-2 text-sm font-[family-name:var(--font-geist-mono)] hover:opacity-80"
						>
							<svg
								height="20"
								width="20"
								viewBox="0 0 16 16"
								className="fill-current"
							>
								<title>Star</title>
								<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
							</svg>
							<span>Star</span>
						</a>
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
