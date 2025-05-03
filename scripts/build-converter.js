#!/usr/bin/env node

/**
 * Build script for the Mjolnir Converter
 *
 * This script builds the Rust converter CLI tool and ensures it's available
 * for the Node.js integration.
 */

const { execSync } = require("node:child_process");
const { join } = require("node:path");

const ROOT_DIR = process.cwd();
const CRATES_DIR = join(ROOT_DIR, "crates");
const CONVERTER_DIR = join(CRATES_DIR, "mjolnir-converter");

console.log("Building Mjolnir Converter CLI...");

try {
	// Build the converter in release mode
	execSync("cargo build --release --bin mjolnir_converter_cli", {
		cwd: CONVERTER_DIR,
		stdio: "inherit",
	});

	console.log("Mjolnir Converter CLI built successfully!");
} catch (error) {
	console.error("Failed to build Mjolnir Converter CLI:", error);
	process.exit(1);
}
