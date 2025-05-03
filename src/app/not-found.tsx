import Image from "next/image";
import Link from "next/link";

export default function NotFound() {
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

			<main className="max-w-6xl mx-auto text-center">
				<div className="bg-foreground/5 backdrop-blur-sm rounded-xl p-6 sm:p-8 border border-foreground/10">
					<h2 className="text-4xl font-bold mb-4">404</h2>
					<p className="text-xl mb-8">Page Not Found</p>
					<Link
						href="/"
						className="inline-block rounded-full px-6 py-2.5 font-medium text-sm transition-colors bg-foreground text-background hover:bg-foreground/90"
					>
						Return Home
					</Link>
				</div>
			</main>

			<footer className="max-w-6xl mx-auto pt-8 border-t border-foreground/10 mt-12 text-center text-sm opacity-70 font-[family-name:var(--font-geist-mono)]">
				<p>Mjolnir - Polkadot Smart Contract Analysis Tool</p>
			</footer>
		</div>
	);
}
