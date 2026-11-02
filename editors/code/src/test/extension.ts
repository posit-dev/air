import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

import { AIR_EXTENSION_ID } from "../constants";

// The test folder contains a default workspace configuration with formatting
// settings that apply to our tests
export function testPath(file: string): string {
	return path.join(extension().extensionPath, "src", "test", file);
}

export function snapshotPath(file: string): string {
	return testPath(path.join("snapshots", file));
}

export function extension() {
	const extension = vscode.extensions.getExtension(AIR_EXTENSION_ID);

	if (extension === undefined) {
		throw new Error(`Extension ${AIR_EXTENSION_ID} not found`);
	}

	return extension;
}

export function api() {
	return extension().exports;
}

export function internalApi() {
	return extension().exports.__private;
}

export async function waitLsp() {
	// Make sure air-vscode is activated as activation events might not have
	// been triggered
	await extension().activate();

	// No-op if LSP is already started. Doesn't run concurrently with the `start()`
	// task started by `activate()` so is safe to use to wait until the LSP is fully
	// started up.
	await internalApi().ctx.lsp.start();
}

export async function withToml(
	content: string,
	run: () => Promise<void>,
): Promise<void> {
	const createdToml: Promise<void> =
		internalApi().ctx.lsp.waitForSettingsNotification();

	const tomlPath = testPath("air.toml");
	fs.writeFileSync(tomlPath, content);

	try {
		await createdToml;
		await run();
	} finally {
		fs.unlinkSync(tomlPath);
	}
}
