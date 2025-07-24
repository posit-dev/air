import * as vscode from "vscode";

import { Cmd, Ctx } from "../context";
import * as output from "../output";
import { isError, isResult, runCommand } from "../process";
import {
	saveAllDirtyWorkspaceTextDocuments,
	selectWorkspaceFolder,
} from "../workspace";

/**
 * Format a workspace folder
 *
 * # Tab saving
 *
 * Because we use the embedded Air CLI to perform the formatting, we force all
 * relevant tabs to be saved before proceeding. The Air CLI can't know about any
 * in-memory changes that may have occurred in "dirty" editors. Forcing them to
 * save causes any changes to be written to disk first, syncing VS Code with the
 * CLI.
 *
 * We don't force the tabs to be closed. We've seen that VS Code seems to sync
 * the LSP client and server well when performing a git branch switch (as long
 * as all files are saved beforehand), so we are hopeful it should also behave
 * well when we format with the CLI.
 *
 * # Forward propagation of settings
 *
 * If no `air.toml` is tied to a document, then typically in the LSP document
 * settings are forward propagated from the client to the server, and those are
 * used during formatting when that document is open. If an `air.toml` does
 * exist, those settings override document settings.
 *
 * Because we go through the Air CLI here, forward propagated document settings
 * are ignored, so even if you don't have an `air.toml`, you will get Air's
 * default indentation settings.
 *
 * We can't simply require an `air.toml` at the workspace folder root to avoid
 * confusion here, because you may instead have an `air.toml` at
 * `{root}/R/air.toml` that would handle any R files in your project. Instead,
 * we hope this isn't common enough to come up much in practice.
 */
export async function workspaceFolderFormatting(
	workspaceFolder: vscode.WorkspaceFolder,
	binaryPath: string,
) {
	const allSaved = await saveAllDirtyWorkspaceTextDocuments(workspaceFolder);
	if (!allSaved) {
		return;
	}

	const workspaceFolderPath = workspaceFolder.uri.fsPath;

	// i.e., `air format {workspaceFolderPath} --no-color`
	const args = ["format", workspaceFolderPath, "--no-color"];

	// This should not matter since the path is explicitly supplied, but better to be safe
	const options = {
		cwd: workspaceFolderPath,
	};

	// Resolves when the spawned process closes or errors
	const result = await runCommand(binaryPath, args, options);

	let anyErrors = false;

	if (isError(result)) {
		// Something went horribly wrong in the process spawning or shutdown process
		anyErrors = true;
		output.log(
			`Errors occurred while formatting the ${workspaceFolder.name} workspace folder.\n${result.error.message}`,
		);
	}

	if (isResult(result)) {
		if (result.code !== 0) {
			// Air was able to run and exit, but we had an error along the way
			output.log(
				`Errors occurred while formatting the ${workspaceFolder.name} workspace folder.\n${result.stderr}`,
			);
			anyErrors = true;
		}
	}

	if (anyErrors) {
		const answer = await vscode.window.showInformationMessage(
			`Errors occurred while formatting the ${workspaceFolder.name} workspace folder. View the logs?`,
			{ modal: true },
			"Yes",
			"No",
		);

		if (answer === "Yes") {
			output.show();
		}

		return;
	}

	vscode.window.showInformationMessage(
		`Successfully formatted the ${workspaceFolder.name} workspace folder.`,
	);
}

export function workspaceFolderFormattingCallback(ctx: Ctx): Cmd {
	return async () => {
		const workspaceFolder = await selectWorkspaceFolder();

		if (!workspaceFolder) {
			return;
		}

		const binaryPath = ctx.lsp.getBinaryPath();

		await workspaceFolderFormatting(workspaceFolder, binaryPath);
	};
}
