import * as vscode from "vscode";
import { Cmd, Ctx } from "./context";
import { viewFileUsingTextDocumentContentProvider } from "./rust-analyzer/viewFileProvider";
import * as ext from "./lsp-ext";

export function registerCommands(ctx: Ctx) {
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.restart",
			async () => await ctx.lsp.restart(),
		),
	);
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewSyntaxTree",
			viewSyntaxTree(ctx),
		),
	);
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewSyntaxTreeTs",
			viewSyntaxTreeTs(ctx),
		),
	);
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewFormatTree",
			viewFormatTree(ctx),
		),
	);
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewFileRepresentations",
			async () => {
				await vscode.commands.executeCommand("air.viewSyntaxTree");
				await vscode.commands.executeCommand("air.viewSyntaxTreeTs");
				await vscode.commands.executeCommand("air.viewFormatTree");
			},
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
