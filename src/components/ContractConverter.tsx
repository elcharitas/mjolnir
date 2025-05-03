import React from "react";
import type { ContractType } from "@/lib/converter";

type ContractConverterProps = {
	contractCode: string;
	setContractCode: (code: string) => void;
	isConverting: boolean;
	onConvert: (targetType: ContractType) => void;
};

export default function ContractConverter({
	contractCode,
	isConverting,
	onConvert,
}: ContractConverterProps) {
	return (
		<div className="flex items-center gap-4 mb-4">
			<button
				type="button"
				className={`rounded-full px-4 py-2 text-sm font-medium transition-colors ${
					isConverting
						? "bg-foreground/30 cursor-not-allowed"
						: "bg-foreground/10 hover:bg-foreground/20"
				}`}
				onClick={() => onConvert("solidity")}
				disabled={isConverting || !contractCode.trim()}
			>
				Convert to Solidity
			</button>
			<button
				type="button"
				className={`rounded-full px-4 py-2 text-sm font-medium transition-colors ${
					isConverting
						? "bg-foreground/30 cursor-not-allowed"
						: "bg-foreground/10 hover:bg-foreground/20"
				}`}
				onClick={() => onConvert("ink")}
				disabled={isConverting || !contractCode.trim()}
			>
				Convert to Ink!
			</button>
		</div>
	);
}
