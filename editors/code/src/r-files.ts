import * as vscode from "vscode";

// Parts of this file are adapted from
// https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/src/util.ts

export type RDocument = vscode.TextDocument & { languageId: "r" };
export type REditor = vscode.TextEditor & { document: RDocument };

export function isRDocument(
	document: vscode.TextDocument,
): document is RDocument {
	return document.languageId === "r" && document.uri.scheme === "file";
}

export function isREditor(editor: vscode.TextEditor): editor is REditor {
	return isRDocument(editor.document);
}

export function activeREditor(): REditor | undefined {
	const editor = vscode.window.activeTextEditor;
	return editor && isREditor(editor) ? editor : undefined;
}
