import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { isREditor, REditor } from "./r-files";

// Parts of this file are adapted from
// https://github.com/rust-lang/rust-analyzer/blob/master/editors/code/src/ctx.ts

export class Ctx {
	constructor(
		readonly extension: vscode.ExtensionContext,
		public client: lc.LanguageClient,
	) {}
}

export type Cmd = (...args: any[]) => unknown;

export interface Disposable {
	dispose(): void;
}
