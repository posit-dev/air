import * as vscode from "vscode";
import * as url from "url";
import * as path from "path";

export function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export function normalizePath(file: string): string {
	if (file.startsWith("file:///")) {
		file = url.fileURLToPath(file);
	}
	return path.normalize(file);
}

/**
 * Determine whether or not a file exists
 *
 * `vscode.workspace.fs.*` does not provide a way to check if a Uri exists or not,
 * so this is supposedly the next best way to do so.
 *
 * This also works on directories.
 */
export async function fileExists(uri: vscode.Uri): Promise<boolean> {
	try {
		await vscode.workspace.fs.stat(uri);
		return true;
	} catch {
		return false;
	}
}
