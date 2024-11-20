import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { Lsp } from "./lsp";

// Parts of this file are adapted from
// https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/src/ctx.ts

export class Ctx {
	public client: lc.LanguageClient | null;

	constructor(
		readonly extension: vscode.ExtensionContext,
		public lsp: Lsp,
	) {
		this.client = lsp.client;
	}

	public getClient(): lc.LanguageClient {
		if (!this.client) {
			throw new Error("LSP must be started");
		}
		return this.client;
	}
}

export type Cmd = (...args: any[]) => unknown;

export interface Disposable {
	dispose(): void;
}
