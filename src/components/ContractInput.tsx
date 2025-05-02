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
					<textarea
						className="w-full h-64 p-4 bg-background border border-foreground/20 rounded-lg font-[family-name:var(--font-geist-mono)] text-sm focus:outline-none focus:ring-2 focus:ring-foreground/30"
						placeholder="Paste your smart contract code here (Ink! or Solidity)..."
						value={contractCode}
						onChange={(e) => setContractCode(e.target.value)}
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
