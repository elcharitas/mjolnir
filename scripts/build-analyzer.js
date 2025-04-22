#!/usr/bin/env node

/**
 * Build script for the Mjolnir Analyzer
 *
 * This script builds the Rust analyzer CLI tool and ensures it's available
 * for the Node.js integration.
 */

const { execSync } = require("node:child_process");
const { join } = require("node:path");

const ROOT_DIR = process.cwd();
const CRATES_DIR = join(ROOT_DIR, "crates");
const ANALYZER_DIR = join(CRATES_DIR, "mjolnir-analyzer");

console.log("Building Mjolnir Analyzer CLI...");

try {
	// Build the analyzer in release mode
	execSync("cargo build --release --bin mjolnir_analyzer_cli", {
		cwd: ANALYZER_DIR,
		stdio: "inherit",
	});

	console.log("Mjolnir Analyzer CLI built successfully!");
} catch (error) {
	console.error("Failed to build Mjolnir Analyzer CLI:", error);
	process.exit(1);
}
