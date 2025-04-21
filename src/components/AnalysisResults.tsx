import React from "react";

type Metric = {
	performance: number;
	security: number;
	gas_efficiency: number;
	code_quality: number;
};

type Issue = {
	severity: "high" | "medium" | "low";
	message: string;
	line?: number;
	recommendation?: string;
};

export type AnalysisResultsProps = {
	score: number;
	metrics: Metric;
	issues: Issue[];
};

export default function AnalysisResults({
	score,
	metrics,
	issues,
}: AnalysisResultsProps) {
	return (
		<section className="mb-12">
			<div className="bg-foreground/5 backdrop-blur-sm rounded-xl p-6 sm:p-8 border border-foreground/10">
				<div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6">
					<h2 className="text-xl font-semibold">Analysis Results</h2>
					<div className="mt-2 sm:mt-0 flex items-center gap-2">
						<div className="text-sm font-[family-name:var(--font-geist-mono)] opacity-70">
							Overall Score:
						</div>
						<div
							className={`text-lg font-bold ${score >= 90 ? "text-green-500" : score >= 70 ? "text-yellow-500" : "text-red-500"}`}
						>
							{score}/100
						</div>
					</div>
				</div>

				<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
					{Object.entries(metrics).map(([key, value]) => (
						<div
							key={key}
							className="bg-background/50 rounded-lg p-4 border border-foreground/10"
						>
							<div className="text-sm font-[family-name:var(--font-geist-mono)] opacity-70 mb-1">
								{key.replace("_", " ").toUpperCase()}
							</div>
							<div className="flex items-center justify-between">
								<div className="text-2xl font-bold">{value}</div>
								<div className="w-12 h-12 relative">
									<svg viewBox="0 0 36 36" className="w-full h-full">
										<title>Svg</title>
										<path
											d="M18 2.0845
                        a 15.9155 15.9155 0 0 1 0 31.831
                        a 15.9155 15.9155 0 0 1 0 -31.831"
											fill="none"
											stroke="#E6E6E6"
											strokeWidth="3"
											strokeDasharray="100, 100"
										/>
										<path
											d="M18 2.0845
                        a 15.9155 15.9155 0 0 1 0 31.831
                        a 15.9155 15.9155 0 0 1 0 -31.831"
											fill="none"
											stroke={
												value >= 90
													? "#10B981"
													: value >= 70
														? "#F59E0B"
														: "#EF4444"
											}
											strokeWidth="3"
											strokeDasharray={`${value}, 100`}
										/>
									</svg>
								</div>
							</div>
						</div>
					))}
				</div>

				<div>
					<h3 className="text-lg font-semibold mb-3">Issues Found</h3>
					<div className="space-y-3">
						{issues.map((issue) => (
							<div
								key={issue.message}
								className="bg-background/50 rounded-lg p-4 border border-foreground/10"
							>
								<div className="flex items-start gap-3">
									<div
										className={`w-2 h-2 mt-1.5 rounded-full ${issue.severity === "high" ? "bg-red-500" : issue.severity === "medium" ? "bg-yellow-500" : "bg-blue-500"}`}
									/>
									<div className="flex-1">
										<div
											className={`text-xs font-bold uppercase mb-1 ${issue.severity === "high" ? "text-red-500" : issue.severity === "medium" ? "text-yellow-500" : "text-blue-500"}`}
										>
											{issue.severity} Severity{" "}
											{issue.line && `(Line ${issue.line})`}
										</div>
										<div className="text-sm mb-2">{issue.message}</div>
										{issue.recommendation && (
											<div className="text-xs bg-foreground/5 p-2 rounded border border-foreground/10">
												<span className="font-semibold">Recommendation:</span>{" "}
												{issue.recommendation}
											</div>
										)}
									</div>
								</div>
							</div>
						))}
					</div>
				</div>
			</div>
		</section>
	);
}
