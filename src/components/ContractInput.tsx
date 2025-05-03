import type { ContractType } from "@/lib/converter";

type ContractInputProps = {
	contractCode: string;
	setContractCode: (code: string) => void;
	isAnalyzing: boolean;
	onAnalyze: () => void;
	isConverting: boolean;
	onConvert: (targetType: ContractType) => void;
};

import ContractConverter from "./ContractConverter";
import CodeEditor from "@monaco-editor/react";

export default function ContractInput({
	contractCode,
	setContractCode,
	isAnalyzing,
	onAnalyze,
	isConverting,
	onConvert,
}: ContractInputProps) {
	return (
		<section className="mb-12">
			<div className="bg-foreground/5 backdrop-blur-sm rounded-xl p-6 sm:p-8 border border-foreground/10">
				<h2 className="text-xl font-semibold mb-4">
					Analyze Your Smart Contract
				</h2>
				<div className="mb-4">
					<CodeEditor
						height="40vh"
						theme="vs-dark"
						defaultLanguage="rust"
						value={contractCode}
						onChange={(value?: string) => setContractCode(value ?? "")}
						defaultValue="// Paste your smart contract code here (Ink! or Solidity)..."
						loading={
							<div className="flex items-center justify-center h-full">
								<div className="spinner-border text-primary animate-spin inline-block w-8 h-8 border-4 rounded-full" />
							</div>
						}
					/>
				</div>
				<ContractConverter
					contractCode={contractCode}
					setContractCode={setContractCode}
					isConverting={isConverting}
					onConvert={onConvert}
				/>
				<div className="flex justify-end">
					<button
						type="button"
						className={`rounded-full px-6 py-2.5 font-medium text-sm transition-colors ${isAnalyzing ? "bg-foreground/30 cursor-not-allowed" : "bg-foreground text-background hover:bg-foreground/90"}`}
						onClick={onAnalyze}
						disabled={isAnalyzing || !contractCode.trim()}
					>
						{isAnalyzing ? "Analyzing..." : "Analyze Contract"}
					</button>
				</div>
			</div>
		</section>
	);
}
