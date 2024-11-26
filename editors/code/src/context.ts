import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { Lsp } from "./lsp";

// Parts of this file are adapted from
// https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/src/ctx.ts

export class Ctx {
	constructor(
		readonly extension: vscode.ExtensionContext,
		public lsp: Lsp,
	) {}

	public getClient(): lc.LanguageClient {
		return this.lsp.getClient();
	}
}

export type Cmd = (...args: any[]) => unknown;
