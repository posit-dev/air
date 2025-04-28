import * as vscode from "vscode";

import { Cmd, Ctx } from "../context";
import * as output from "../output";
import { isError, isResult, runCommand } from "../process";

/**
 * Format a workspace folder
 *
 * # Workspace folder selection
 *
 * - If 0 workspace folders are open, errors
 * - If 1 workspace folder is open, automatically uses it
 * - If >1 workspace folders are open, asks the user to choose
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
export function workspaceFolderFormatting(ctx: Ctx): Cmd {
	return async () => {
		const binaryPath = ctx.lsp.getBinaryPath();

		const workspaceFolder = await selectWorkspaceFolder();
		if (!workspaceFolder) {
			return;
		}

		const allSaved =
			await saveAllDirtyWorkspaceTextDocuments(workspaceFolder);
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
	};
}

async function selectWorkspaceFolder(): Promise<
	vscode.WorkspaceFolder | undefined
> {
	const workspaceFolders = vscode.workspace.workspaceFolders;

	if (!workspaceFolders || workspaceFolders.length === 0) {
		vscode.window.showErrorMessage(
			"You must be inside a workspace to format a workspace folder.",
		);
		return undefined;
	}

	if (workspaceFolders.length === 1) {
		return workspaceFolders[0];
	}

	// Let the user select a workspace folder if >1 are open, may be
	// `undefined` if user bails from quick pick!
	const workspaceFolder =
		await selectWorkspaceFolderFromQuickPick(workspaceFolders);

	return workspaceFolder;
}

async function selectWorkspaceFolderFromQuickPick(
	workspaceFolders: readonly vscode.WorkspaceFolder[],
): Promise<vscode.WorkspaceFolder | undefined> {
	// Show the workspace names
	const workspaceFolderNames = workspaceFolders.map(
		(workspaceFolder) => workspaceFolder.name,
	);

	const workspaceFolderName = await vscode.window.showQuickPick(
		workspaceFolderNames,
		{
			canPickMany: false,
			title: "Which workspace folder should be formatted?",
		},
	);

	if (!workspaceFolderName) {
		// User bailed from the quick pick
		return undefined;
	}

	// Match selected name back to the workspace folder
	for (let workspaceFolder of workspaceFolders) {
		if (workspaceFolder.name === workspaceFolderName) {
			return workspaceFolder;
		}
	}

	// Should never get here
	output.log(
		`Matched a workspace folder name, but unexpectedly can't find corresponding workspace folder. Folder name: ${workspaceFolderName}.`,
	);
	return undefined;
}

/**
 * Save all open dirty editor tabs relevant to the workspace folder
 *
 * - Filters to only tabs living under the chosen workspace folder
 * - Asks the user if they are okay with us saving the editor tabs
 */
async function saveAllDirtyWorkspaceTextDocuments(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<boolean> {
	const textDocuments = dirtyWorkspaceTextDocuments(workspaceFolder);

	if (textDocuments.length === 0) {
		// Nothing to save!
		return true;
	}

	// Ask the user if we can save them
	const answer = await vscode.window.showInformationMessage(
		`All editors within the ${workspaceFolder.name} workspace folder must be saved before formatting. Proceed with saving these editors?`,
		{ modal: true },
		"Yes",
		"No",
	);

	if (answer !== "Yes") {
		// User said `"No"` or bailed from the menu
		return false;
	}

	// Save all documents, and ensure that all successfully saved
	const savedPromises = textDocuments.map((textDocument) =>
		textDocument.save(),
	);
	const saved = await Promise.all(savedPromises);
	return saved.every((save) => save);
}

function dirtyWorkspaceTextDocuments(
	workspaceFolder: vscode.WorkspaceFolder,
): vscode.TextDocument[] {
	return vscode.workspace.textDocuments.filter((document) => {
		if (document.isClosed) {
			// Not actually synchonized. This document will be refreshed when the document is reopened.
			return false;
		}

		if (!document.isDirty) {
			// Nothing to do
			return false;
		}

		if (document.isUntitled) {
			// These aren't part of the workspace folder
			return false;
		}

		if (!document.uri.fsPath.startsWith(workspaceFolder.uri.fsPath)) {
			// The document must live "under" the chosen workspace folder for us to care about it
			return false;
		}

		return true;
	});
}
