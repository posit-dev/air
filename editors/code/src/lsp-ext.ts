import * as lc from "vscode-languageclient/node";

export type ViewFileKind = "TreeSitter" | "SyntaxTree" | "FormatTree";

export const viewFile = new lc.RequestType<
	lc.TextDocumentPositionParams & { kind: ViewFileKind },
	string,
	void
>("air/viewFile");
