import * as vscode from "vscode";
import path from "path";
import AdmZip from "adm-zip";

import { Cmd, Ctx } from "./context";
import { viewFileUsingTextDocumentContentProvider } from "./request/viewFile";
import { VIEW_FILE } from "./request/viewFile";
import { workspaceFolderFormatting } from "./commands/workspace-folder-formatting";

export function registerCommands(ctx: Ctx) {
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.restart",
			async () => await ctx.lsp.restart(),
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.workspaceFolderFormatting",
			workspaceFolderFormatting(ctx),
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewTreeSitter",
			viewTreeSitter(ctx),
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
			"air.viewFormatTree",
			viewFormatTree(ctx),
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.viewFileRepresentations",
			async () => {
				await vscode.commands.executeCommand("air.viewTreeSitter");
				await vscode.commands.executeCommand("air.viewSyntaxTree");
				await vscode.commands.executeCommand("air.viewFormatTree");
			},
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.saveFileRepresentations",
			async () => {
				await vscode.commands.executeCommand(
					"air.viewFileRepresentations",
				);
				zipFileRepresentations();
			},
		),
	);
}

function viewTreeSitter(ctx: Ctx): Cmd {
	const uri = "air-tree-sitter://syntax/treesitter";
	const scheme = "air-tree-sitter";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		VIEW_FILE,
		"TreeSitter",
		uri,
		scheme,
		true,
	);
}

function viewSyntaxTree(ctx: Ctx): Cmd {
	const uri = "air-syntax-tree://syntax/rowan.rast";
	const scheme = "air-syntax-tree";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		VIEW_FILE,
		"SyntaxTree",
		uri,
		scheme,
		true,
	);
}

function viewFormatTree(ctx: Ctx): Cmd {
	const uri = "air-format-tree://format/biome";
	const scheme = "air-format-tree";

	return viewFileUsingTextDocumentContentProvider(
		ctx,
		VIEW_FILE,
		"FormatTree",
		uri,
		scheme,
		true,
	);
}

async function zipFileRepresentations() {
	const docs = vscode.workspace.textDocuments.filter((doc) => {
		switch (doc.uri.scheme) {
			case "air-tree-sitter":
			case "air-syntax-tree":
			case "air-format-tree":
				return true;
			default:
				return false;
		}
	});

	const zip = new AdmZip();

	for (const doc of docs) {
		const name = path.basename(doc.fileName);
		const content = doc.getText();
		zip.addFile(name, Buffer.from(content, "utf8"));
	}

	const uri = await vscode.window.showSaveDialog({
		filters: {
			"Zip files": ["zip"],
		},
		defaultUri: vscode.Uri.file(
			path.join(
				vscode.workspace.rootPath || __dirname,
				"file-representations.zip",
			),
		),
	});

	if (uri) {
		zip.writeZip(uri.fsPath);
	}
}
