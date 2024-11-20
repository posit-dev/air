import * as vscode from "vscode";
import { Ctx } from "./context";
import { Lsp } from "./lsp";
import { registerCommands } from "./commands";

export let ctx: Ctx;

export function activate(context: vscode.ExtensionContext) {
	let lsp = new Lsp(context);
	lsp.start();

	ctx = new Ctx(context, lsp);
	registerCommands(context);
}

export function deactivate() {
	ctx.lsp.stop();
}
