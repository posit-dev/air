// From testUtils.ts in the typescript-language-feature extension
// https://github.com/posit-dev/positron/blob/main/extensions/typescript-language-features/src/test/testUtils.ts

/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// See also https://github.com/posit-dev/positron/blob/main/extensions/positron-r/src/test/editor-utils.ts

import * as fs from "fs";
import * as os from "os";
import { join } from "path";
import * as vscode from "vscode";

export function rndName() {
	let name = "";
	const possible =
		"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	for (let i = 0; i < 10; i++) {
		name += possible.charAt(Math.floor(Math.random() * possible.length));
	}
	return name;
}

export function createRandomFile(
	contents = "",
	fileExtension = "txt",
): Thenable<vscode.Uri> {
	return new Promise((resolve, reject) => {
		const tmpFile = join(os.tmpdir(), rndName() + "." + fileExtension);
		fs.writeFile(tmpFile, contents, (error) => {
			if (error) {
				return reject(error);
			}

			resolve(vscode.Uri.file(tmpFile));
		});
	});
}

export async function withFileEditor(
	file: string,
	run: (editor: vscode.TextEditor, doc: vscode.TextDocument) => Promise<void>,
): Promise<void> {
	const doc = await vscode.workspace.openTextDocument(file);

	try {
		const editor = await vscode.window.showTextDocument(doc);
		await run(editor, doc);
	} finally {
		await vscode.commands.executeCommand(
			"workbench.action.closeActiveEditor",
		);
	}
}

export async function withUntitledEditor(
	content: string,
	language: string,
	run: (editor: vscode.TextEditor, doc: vscode.TextDocument) => Promise<void>,
): Promise<void> {
	const doc = await vscode.workspace.openTextDocument({
		language,
		content,
	});

	try {
		const editor = await vscode.window.showTextDocument(doc);
		await run(editor, doc);
	} finally {
		await vscode.commands.executeCommand(
			"workbench.action.closeActiveEditor",
		);
	}
}

export const onDocumentChange = (
	doc: vscode.TextDocument,
): Promise<vscode.TextDocument> => {
	return new Promise<vscode.TextDocument>((resolve) => {
		const sub = vscode.workspace.onDidChangeTextDocument((e) => {
			if (e.document !== doc) {
				return;
			}
			sub.dispose();
			resolve(e.document);
		});
	});
};

export const type = async (
	document: vscode.TextDocument,
	text: string,
): Promise<vscode.TextDocument> => {
	const onChange = onDocumentChange(document);
	await vscode.commands.executeCommand("type", { text });
	await onChange;
	return document;
};

export async function stripWhitespace(
	editor: vscode.TextEditor,
	doc: vscode.TextDocument,
) {
	const stripped = doc.getText().replace(/ +/g, "");
	await editor.edit((editBuilder) => {
		editBuilder.replace(
			new vscode.Range(
				doc.positionAt(0),
				doc.positionAt(doc.getText().length),
			),
			stripped,
		);
	});
}
