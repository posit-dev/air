import * as lc from "vscode-languageclient/node";

export type ViewFileKind = "SyntaxTree" | "SyntaxTreeTs" | "FormatTree";

export const viewFile = new lc.RequestType<
	lc.TextDocumentPositionParams & { kind: ViewFileKind },
	string,
	void
>("air/viewFile");
