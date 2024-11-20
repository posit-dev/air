import * as lc from "vscode-languageclient/node";

export const viewFile = new lc.RequestType<
	lc.TextDocumentPositionParams,
	string,
	void
>("air/viewFile");
