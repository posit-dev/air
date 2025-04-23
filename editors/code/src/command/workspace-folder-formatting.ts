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
 * # Tab closing
 *
 * Because we use the embedded Air CLI to perform the formatting, we force all
 * relevant tabs to be saved and closed before proceeding. This has two main
 * benefits:
 *
 * - The Air CLI can't know about any in-memory changes that may have occurred
 *   in "dirty" editors. Forcing them to save and close causes any changes
 *   to be written to disk first, syncing VS Code with the CLI.
 *
 * - The LSP server part of Air needs to perfectly track changes for any open
 *   files. Closing the tabs first switches the server's "source of truth" from
 *   the client version of the file to the on-disk version of the file, which
 *   should ensure the LSP server won't have any chance of getting out of sync.
 *   This may be overkill, since the LSP server seems to handle git branch
 *   changes well, but it's still not a bad idea.
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

		const allTabsClosed = await closeAllTabs(workspaceFolder);
		if (!allTabsClosed) {
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
 * Close all open editor tabs relevant to the workspace folder
 *
 * - Filters to only tabs living under the chosen workspace folder
 * - Asks the user if they are okay with us closing the editor tabs
 * - Asks the user to save or discard any dirty editor tabs
 */
async function closeAllTabs(
	workspaceFolder: vscode.WorkspaceFolder,
): Promise<boolean> {
	// Collect all tabs from all tab groups
	const allTabs = vscode.window.tabGroups.all.flatMap((group) => group.tabs);

	// Filter down to only tabs containing a text based resource who's uri
	// prefix matches the workspace folder we are going to format. This way
	// we don't have to close anything outside the workspace folder.
	const allRelevantTabs = allTabs.filter((tab) => {
		const input = tab.input;

		if (!(input instanceof vscode.TabInputText)) {
			return false;
		}

		return input.uri.fsPath.startsWith(workspaceFolder.uri.fsPath);
	});

	if (allRelevantTabs.length === 0) {
		// Nothing to close!
		return true;
	}

	const answer = await vscode.window.showInformationMessage(
		`All editors within the ${workspaceFolder.name} workspace folder must be closed before formatting. Proceed with closing these editors?`,
		{ modal: true },
		"Yes",
		"No",
	);

	if (answer !== "Yes") {
		// User said `"No"` or bailed from the menu
		return false;
	}

	// Close all tabs at once, each dirty tab will prompt the user to save or discard any changes
	return await vscode.window.tabGroups.close(allRelevantTabs);
}
