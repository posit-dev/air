// --- source
// authors = ["rust-analyzer team"]
// license = "MIT OR Apache-2.0"
// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/src/commands.ts"
// ---

import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { Ctx, Cmd } from "../context";
import { activeREditor, isRDocument, isREditor } from "../r-files";
import { sleep } from "../utils";

export type ViewFileKind = "TreeSitter" | "SyntaxTree" | "FormatTree";

export const VIEW_FILE = new lc.RequestType<
	lc.TextDocumentPositionParams & { kind: ViewFileKind },
	string,
	void
>("air/viewFile");

export function viewFileUsingTextDocumentContentProvider(
	ctx: Ctx,
	requestType: lc.RequestType<lc.TextDocumentPositionParams, string, void>,
	kind: ViewFileKind,
	uri: string,
	scheme: string,
	shouldUpdate: boolean,
): Cmd {
	const tdcp = new (class implements vscode.TextDocumentContentProvider {
		readonly uri = vscode.Uri.parse(uri);
		readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();
		constructor() {
			vscode.workspace.onDidChangeTextDocument(
				this.onDidChangeTextDocument,
				this,
				ctx.extension.subscriptions,
			);
			vscode.window.onDidChangeActiveTextEditor(
				this.onDidChangeActiveTextEditor,
				this,
				ctx.extension.subscriptions,
			);
		}

		private onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
			if (isRDocument(event.document) && shouldUpdate) {
				// We need to order this after language server updates, but there's no API for that.
				// Hence, good old sleep().
				void sleep(10).then(() => this.eventEmitter.fire(this.uri));
			}
		}
		private onDidChangeActiveTextEditor(
			editor: vscode.TextEditor | undefined,
		) {
			if (editor && isREditor(editor) && shouldUpdate) {
				this.eventEmitter.fire(this.uri);
			}
		}

		async provideTextDocumentContent(
			_uri: vscode.Uri,
			ct: vscode.CancellationToken,
		): Promise<string> {
			const rEditor = activeREditor();
			if (!rEditor) {
				return "";
			}

			const client = ctx.getClient();

			const params = {
				textDocument:
					client.code2ProtocolConverter.asTextDocumentIdentifier(
						rEditor.document,
					),
				position: client.code2ProtocolConverter.asPosition(
					rEditor.selection.active,
				),
				kind: kind,
			};
			return client.sendRequest(requestType, params, ct);
		}

		get onDidChange(): vscode.Event<vscode.Uri> {
			return this.eventEmitter.event;
		}
	})();

	ctx.extension.subscriptions.push(
		vscode.workspace.registerTextDocumentContentProvider(scheme, tdcp),
	);

	return async () => {
		const document = await vscode.workspace.openTextDocument(tdcp.uri);
		tdcp.eventEmitter.fire(tdcp.uri);
		void (await vscode.window.showTextDocument(document, {
			viewColumn: vscode.ViewColumn.Two,
			preserveFocus: true,
			preview: false,
		}));
	};
}
