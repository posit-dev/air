import path from "path";
import * as vscode from "vscode";
import * as fs from "fs-extra";

export async function getRootWorkspaceFolder(): Promise<vscode.WorkspaceFolder> {
	const workspaces: readonly vscode.WorkspaceFolder[] = getWorkspaceFolders();

	if (workspaces.length === 0) {
		// No workspaces open, use current working directory
		return {
			uri: vscode.Uri.file(process.cwd()),
			name: path.basename(process.cwd()),
			index: 0,
		};
	} else if (workspaces.length === 1) {
		// One workspace open, return it
		return workspaces[0];
	} else {
		// Multiple workspaces open, use the one with the shortest path,
		// i.e. the most "root" one
		let rootWorkspace = workspaces[0];
		let root = undefined;

		// Find first existing workspace path
		for (const w of workspaces) {
			if (await fs.pathExists(w.uri.fsPath)) {
				root = w.uri.fsPath;
				rootWorkspace = w;
				break;
			}
		}

		// Update root workspace if we find a shorter path
		for (const w of workspaces) {
			if (
				root &&
				root.length > w.uri.fsPath.length &&
				(await fs.pathExists(w.uri.fsPath))
			) {
				root = w.uri.fsPath;
				rootWorkspace = w;
			}
		}

		return rootWorkspace;
	}
}

function getWorkspaceFolders(): readonly vscode.WorkspaceFolder[] {
	return vscode.workspace.workspaceFolders ?? [];
}
