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
		// API for our own unit tests.
		//
		// It's important to access state this way and not by simply importing
		// files in `src` from the `test` folder (as suggested in the test file
		// template) because the tests live in a different environment than the
		// one where VS Code instantiates extensions. So if you try to activate
		// the extension in this way you'll just end up with a duplicate
		// instance and get in trouble.
		__private: {
			ctx: ctx,
		},
	};
}

export function deactivate() {
	ctx.lsp.stop();
}
