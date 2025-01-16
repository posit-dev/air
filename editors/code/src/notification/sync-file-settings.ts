import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { normalizePath } from "../utils";

export interface SyncFileSettingsParams {
	file_settings: FileSettings[];
}

export interface FileSettings {
	url: string;
	format: FileFormatSettings;
}

export interface FileFormatSettings {
	indent_style: "tab" | "space";
	indent_width: number;
	line_width: number;
}

export const SYNC_FILE_SETTINGS =
	new lc.NotificationType<SyncFileSettingsParams>("air/syncFileSettings");

export class FileSettingsState {
	// There is no way to propagate settings to all editors from the
	// notification handler as we can only change visible editors. To properly
	// handle background editors (e.g. editors in other groups), we need to
	// maintain a map of file settings and query it when the visibility state of
	// an editor changes.
	private settings = new Map<string, FileFormatSettings>();

	constructor(context: vscode.ExtensionContext) {
		context.subscriptions.push(
			vscode.window.onDidChangeActiveTextEditor(async (editor) => {
				if (editor) {
					this.apply(editor);
				}
			}),
		);
	}

	public handleSettingsNotification(params: SyncFileSettingsParams) {
		// Reset map of settings
		this.settings.clear();

		for (const fileSettings of params.file_settings) {
			const path = normalizePath(fileSettings.url);
			this.settings.set(path, fileSettings.format);
		}

		// Apply right away the active text editors. We also have a handler
		// for the case where visible editors change. We could apply to all
		// visible text editors but that would not be useful AFAICT.
		if (vscode.window.activeTextEditor) {
			this.apply(vscode.window.activeTextEditor);
		}
	}

	public apply(editor: vscode.TextEditor) {
		const settings = this.settings.get(editor.document.uri.fsPath);

		if (settings) {
			const insertSpaces = settings.indent_style === "space";
			const indentSize = settings.indent_width;

			// If inserting spaces, keep tab size in sync. If inserting tabs,
			// allow them to diverge so user can configure the visual aspect of
			// tabs without affecting the formatting (we'll use `indentSize` to
			// decide the width of a tab and figure out where does code overflow
			// the line width).
			const tabSize = insertSpaces ? indentSize : editor.options.tabSize;

			editor.options = {
				...editor.options,
				insertSpaces,
				indentSize,
				tabSize,
			};
		}
	}
}
