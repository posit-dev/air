import AdmZip from "adm-zip";
import path from "path";
import * as vscode from "vscode";

import { workspaceFolderFormattingCallback } from "./command/workspace-folder-formatting";
import { workspaceFolderInitializationCallback } from "./command/workspace-folder-initialization";
import { Cmd, Ctx } from "./context";
import {
	VIEW_FILE,
	viewFileUsingTextDocumentContentProvider,
} from "./request/viewFile";

export function registerCommands(ctx: Ctx) {
	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.restart",
			async () => await ctx.lsp.restart(),
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.workspaceFolderInitialization",
			workspaceFolderInitializationCallback(ctx),
		),
	);

	ctx.extension.subscriptions.push(
		vscode.commands.registerCommand(
			"air.workspaceFolderFormatting",
			workspaceFolderFormattingCallback(ctx),
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
