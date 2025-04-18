import * as cp from "child_process";
import * as vscode from "vscode";

import { Cmd, Ctx } from "../context";
import * as output from "../output";

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

		let stderr = "";
		let anyErrors = false;

		// i.e., `air format {workspaceFolderPath} --no-color`
		const args = ["format", workspaceFolderPath, "--no-color"];

		// This should not matter since the path is explicitly supplied, but better to be safe
		const options = {
			cwd: workspaceFolderPath,
		};

		const finishedFormatting = new Promise<void>((resolve) => {
			// Use spawn instead of exec to avoid maxBufferExceeded error
			const p = cp.spawn(binaryPath, args, options);

			p.stderr.setEncoding("utf8");
			p.stderr.on("data", (data) => (stderr += data));

			p.on("error", () => {
				anyErrors = true;
				return resolve();
			});

			p.on("close", (code) => {
				if (code !== 0) {
					anyErrors = true;
				}
				return resolve();
			});
		});

		await finishedFormatting;

		if (anyErrors) {
			output.log(
				`Errors occurred while formatting the ${workspaceFolder.name} workspace folder.\n${stderr}`,
			);

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
 *
 * For safety, we save and close all relevant editor tabs to ensure that all
 * contents have been written to disk before running the CLI, which only pulls
 * from disk and would not have access to any in memory changes. This also helps
 * ensure that the LSP server stays in sync with any changes that are made when
 * the CLI runs and changes contents on disk. Closing the files is a way of
 * saying "the source of truth is disk".
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
