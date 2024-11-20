import * as vscode from "vscode";
import { lsp } from "./extension";

export function registerCommands(context: vscode.ExtensionContext) {
	context.subscriptions.push(
		vscode.commands.registerCommand("air.restart", async () => {
			await lsp.restart();
		}),
	);
}
