import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

import { Cmd, Ctx } from "../context";
import * as output from "../output";

interface WorkspaceFolderFormattingParams {
	workspaceFolder: lc.WorkspaceFolder;
}

interface WorkspaceFolderFormattingResult {
	workspaceEdit: lc.WorkspaceEdit | undefined;
}

const WORKSPACE_FOLDER_FORMATTING = new lc.RequestType<
	WorkspaceFolderFormattingParams,
	WorkspaceFolderFormattingResult,
	void
>("air/workspaceFolderFormatting");

export function workspaceFolderFormatting(ctx: Ctx): Cmd {
	return async () => {
		const client = ctx.getClient();

		const workspaceFolders = vscode.workspace.workspaceFolders;

		if (!workspaceFolders || workspaceFolders.length === 0) {
			vscode.window.showErrorMessage(
				"You must be inside a workspace to format a workspace folder.",
			);
			return;
		}

		// Let the user select a workspace folder if >1 are open
		const workspaceFolder =
			workspaceFolders.length === 1
				? workspaceFolders[0]
				: await workspaceFolderFromQuickPick(workspaceFolders);

		if (!workspaceFolder) {
			return;
		}

		const protocolWorkspaceFolder = asProtocol(workspaceFolder, client);

		const result = await client.sendRequest(WORKSPACE_FOLDER_FORMATTING, {
			workspaceFolder: protocolWorkspaceFolder,
		});

		const protocolWorkspaceEdit = result.workspaceEdit;

		if (protocolWorkspaceEdit) {
			const workspaceEdit =
				await client.protocol2CodeConverter.asWorkspaceEdit(
					protocolWorkspaceEdit,
				);

			// `isRefactoring: true` is very important for a good user experience.
			// Without this, VS Code will open all changed files in an editor for
			// the user to manually save them. This ensures they are saved
			// automatically (as long as `files.refactoring.autoSave: false` is not
			// set) and the files won't be opened.
			//
			// Works well unless exactly 1 file in the project gets an edit, in
			// which case for some reason VS Code will open it unconditionally:
			// https://github.com/microsoft/vscode/issues/246720
			//
			// Ideally, we'd send an LSP `workspace/applyEdit` request from the server
			// to the client, but `WorkspaceEditMetadata` is a proposed feature in the
			// LSP 3.18 version, so we can't use it yet, but we should switch to that
			// when it feels reasonable to do so.
			// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_applyEdit
			// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#workspaceEditMetadata
			const metadata = {
				isRefactoring: true,
			};

			const success = await vscode.workspace.applyEdit(
				workspaceEdit,
				metadata,
			);

			if (!success) {
				vscode.window.showErrorMessage(
					`Failed to apply workspace edit for workspace folder: ${workspaceFolder.name}.`,
				);
				return;
			}
		}

		// Either there was no workspace edit to apply, or we applied it successfully.
		// Let the user know we are done.
		vscode.window.showInformationMessage(
			`Successfully formatted workspace folder: ${workspaceFolder.name}.`,
		);

		return;
	};
}

async function workspaceFolderFromQuickPick(
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
			title: "Which workspace should be formatted?",
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
		`Matched a workspace name, but unexpectedly can't find corresponding workspace folder. Folder path: ${workspaceFolderName}.`,
	);
	return undefined;
}

/* Converts a `vscode.WorkspaceFolder` to an LSP `WorkspaceFolder`
 *
 * Follows vscode-languageserver-node as a guide:
 * https://github.com/microsoft/vscode-languageserver-node/blob/df05883f34b39255d40d68cef55caf2e93cff35f/client/src/common/workspaceFolder.ts#L172
 */
function asProtocol(
	workspaceFolder: vscode.WorkspaceFolder,
	client: lc.LanguageClient,
): lc.WorkspaceFolder {
	return {
		uri: client.code2ProtocolConverter.asUri(workspaceFolder.uri),
		name: workspaceFolder.name,
	};
}
