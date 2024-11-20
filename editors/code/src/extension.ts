import * as vscode from "vscode";
import { Lsp } from "./lsp";

let lsp: Lsp;

export function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(
		vscode.commands.registerCommand("air.restart", async () => {
			await lsp.restart();
		}),
	);

	lsp = new Lsp(context);
	lsp.start();
}

export function deactivate() {
	lsp.stop();
}
