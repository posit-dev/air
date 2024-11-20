import * as vscode from "vscode";
import { Lsp } from "./lsp";
import { registerCommands } from "./commands";

export let lsp: Lsp;

export function activate(context: vscode.ExtensionContext) {
	registerCommands(context);

	lsp = new Lsp(context);
	lsp.start();
}

export function deactivate() {
	lsp.stop();
}
