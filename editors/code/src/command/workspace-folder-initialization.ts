import * as vscode from "vscode";

import { Cmd, Ctx } from "../context";
import {
	saveAllDirtyWorkspaceTextDocuments,
	selectWorkspaceFolder,
} from "../workspace";
import { workspaceFolderFormatting } from "./workspace-folder-formatting";
import { fileExists } from "../utils";

/**
 * Initialize a workspace folder for use with Air
 *
 * - Creates an empty `air.toml` if neither `air.toml` nor `.air.toml` already exist
 * - Creates `extensions.json` recommending `Posit.air-vscode`
 * - Updates `settings.json` to format-on-save R and Quarto, and sets default formatters
 * - Updates `.Rbuildignore` to ignore `.vscode/` and `air.toml` if one existed
 *
 * Optionally, the user can request an immediate formatting of the workspace folder
 * after initialization
 */
export async function workspaceFolderInitialization(
	workspaceFolder: vscode.WorkspaceFolder,
	binaryPath: string,
) {
	const allSaved = await saveAllDirtyWorkspaceTextDocuments(workspaceFolder);
	if (!allSaved) {
		return;
	}

	await createAirToml(workspaceFolder);
	await createExtensionsJson(workspaceFolder);
	await updateSettingsJson(workspaceFolder);
	await updateRbuildignore(workspaceFolder);

	const message = `Successfully initialized the ${workspaceFolder.name} workspace folder.`;
	const shouldFormatItem = `Click to format the ${workspaceFolder.name} workspace folder`;

	const item = await vscode.window.showInformationMessage(
		message,
		shouldFormatItem,
	);

	if (item === shouldFormatItem) {
		await workspaceFolderFormatting(workspaceFolder, binaryPath);
	}
}

export function workspaceFolderInitializationCallback(ctx: Ctx): Cmd {
	return async () => {
		const workspaceFolder = await selectWorkspaceFolder();

		if (!workspaceFolder) {
			return;
		}

		const binaryPath = ctx.lsp.getBinaryPath();

		await workspaceFolderInitialization(workspaceFolder, binaryPath);
	};
}

async function createAirToml(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<void> {
	const airTomlUri = vscode.Uri.joinPath(workspaceFolder.uri, "air.toml");
	const dotAirTomlUri = vscode.Uri.joinPath(workspaceFolder.uri, ".air.toml");

	if ((await fileExists(airTomlUri)) || (await fileExists(dotAirTomlUri))) {
		return;
	}

	// Create an empty `air.toml`
	const content = new TextEncoder().encode("");
	await vscode.workspace.fs.writeFile(airTomlUri, content);
}

/**
 * Create `extensions.json` with Air recommended
 *
 * Unlike `settings.json`, there is no API for interacting with an
 * `extensions.json` file. To keep things simple, we never try and update an
 * existing one, we only create one if the user didn't have one already. We
 * don't really want to get into the json parsing business just for this.
 */
async function createExtensionsJson(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<void> {
	const vscodeUri = vscode.Uri.joinPath(workspaceFolder.uri, ".vscode");
	const extensionsUri = vscode.Uri.joinPath(vscodeUri, "extensions.json");

	if (await fileExists(extensionsUri)) {
		// Just bail if the user already has `extensions.json`, we don't
		// try to update an existing one
		return;
	}

	if (!(await fileExists(vscodeUri))) {
		await vscode.workspace.fs.createDirectory(vscodeUri);
	}

	let contents = `
{
    "recommendations": [
        "Posit.air-vscode"
    ]
}
	`;
	contents = contents.trim();
	contents = contents + "\n";

	const bytes = new TextEncoder().encode(contents);

	await vscode.workspace.fs.writeFile(extensionsUri, bytes);
}

/**
 * This seems to be the only way to update language specific settings for a
 * particular workspace folder in a way that:
 *
 * - Doesn't destroy existing `[r]` or `[quarto]` settings that we aren't
 *   updating (like `config.update("[r]", value)` naively does)
 * - Doesn't pull extra inherited global settings (like `config.get()` would do)
 *
 * The trick is to `inspect()` all of the `[r]` configuration specific to just
 * the `workspaceFolder` and bulk update it, retaining all old settings but
 * overriding the ones we care about updating.
 *
 * This does unfortunately drop comments in the `[r]` and `[quarto]` sections,
 * but we can't do better.
 *
 * It would be great if you could do `update("[r].editor.formatOnSave")` to
 * precisely target a single value of a single language, but you cannot.
 */
async function updateSettingsJson(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<void> {
	const config = vscode.workspace.getConfiguration(
		undefined,
		workspaceFolder,
	);

	const configR = config.inspect("[r]")?.workspaceFolderValue || {};
	await config.update(
		"[r]",
		{
			...configR,
			"editor.formatOnSave": true,
			"editor.defaultFormatter": "Posit.air-vscode",
		},
		vscode.ConfigurationTarget.WorkspaceFolder,
	);

	const configQuarto = config.inspect("[quarto]")?.workspaceFolderValue || {};
	await config.update(
		"[quarto]",
		{
			...configQuarto,
			"editor.formatOnSave": true,
			"editor.defaultFormatter": "quarto.quarto",
		},
		vscode.ConfigurationTarget.WorkspaceFolder,
	);
}

async function updateRbuildignore(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<void> {
	const rbuildignoreUri = vscode.Uri.joinPath(
		workspaceFolder.uri,
		".Rbuildignore",
	);

	if (!(await fileExists(rbuildignoreUri))) {
		// Do nothing if the user doesn't have one, i.e. an R project
		// or the rare R package without an `.Rbuildignore`
		return;
	}

	const buffer = await vscode.workspace.fs.readFile(rbuildignoreUri);
	const content = new TextDecoder().decode(buffer);

	const newline = content.includes("\r\n") ? "\r\n" : "\n";
	const lines = content.split(newline);

	if (lines.at(-1) === "") {
		// Drop final line if it's empty, i.e. we split on newlines and the
		// file ended with a newline. This avoids writing a blank line between
		// the existing content and our new lines.
		lines.pop();
	}

	let anyMissing = false;
	const patterns = ["^\\.vscode$", "^[.]?air[.]toml$"];

	for (const pattern of patterns) {
		const exists = lines.find((line) => line === pattern);

		if (!exists) {
			anyMissing = true;
			lines.push(pattern);
		}
	}

	if (anyMissing) {
		const content = lines.join(newline) + newline;
		const buffer = new TextEncoder().encode(content);
		await vscode.workspace.fs.writeFile(rbuildignoreUri, buffer);
	}
}
