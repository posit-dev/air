import * as vscode from "vscode";
import { ctx } from "./extension";
import { Cmd, Ctx } from "./context";
import { viewFileUsingTextDocumentContentProvider } from "./rust-analyzer/virtualViewer";
import * as ext from "./lsp-ext";

export function registerCommands(context: vscode.ExtensionContext) {
	context.subscriptions.push(
		vscode.commands.registerCommand("air.restart", async () => {
			await ctx.lsp.restart();
		}),
	);
	context.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewSyntaxTree",
			viewSyntaxTree(ctx),
		),
	);
}

function viewSyntaxTree(ctx: Ctx): Cmd {
	const uri = "air-syntax-tree://syntaxtree/tree.rast";
	const scheme = "air-syntax-tree";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		ext.viewFile,
		uri,
		scheme,
		true,
	);
}
