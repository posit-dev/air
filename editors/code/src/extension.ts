import * as vscode from "vscode";
import { Ctx } from "./context";
import { Lsp } from "./lsp";
import { registerCommands } from "./commands";

let ctx: Ctx;

export function activate(context: vscode.ExtensionContext) {
	let lsp = new Lsp(context);
	lsp.start();

	ctx = new Ctx(context, lsp);
	registerCommands(ctx);

	return {
		// For unit tests
		__private: {
			ctx: ctx,
		},
	};
}

export function deactivate() {
	ctx.lsp.stop();
}
