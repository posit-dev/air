import * as vscode from "vscode";
import { ctx } from "./extension";
import { Cmd, Ctx } from "./context";
import { viewFileUsingTextDocumentContentProvider } from "./rust-analyzer/viewFileProvider";
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
	context.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewSyntaxTreeTs",
			viewSyntaxTreeTs(ctx),
		),
	);
	context.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewFormatTree",
			viewFormatTree(ctx),
		),
	);
}

function viewSyntaxTree(ctx: Ctx): Cmd {
	const uri = "air-syntax-tree://syntax/tree.rast";
	const scheme = "air-syntax-tree";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		ext.viewFile,
		"SyntaxTree",
		uri,
		scheme,
		true,
	);
}

function viewSyntaxTreeTs(ctx: Ctx): Cmd {
	const uri = "air-syntax-tree-ts://syntax/treesitter";
	const scheme = "air-syntax-tree-ts";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		ext.viewFile,
		"SyntaxTreeTs",
		uri,
		scheme,
		true,
	);
}

function viewFormatTree(ctx: Ctx): Cmd {
	const uri = "air-format-tree://format/biome.ir";
	const scheme = "air-format-tree";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		ext.viewFile,
		"FormatTree",
		uri,
		scheme,
		true,
	);
}
