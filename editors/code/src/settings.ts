import * as vscode from "vscode";
import * as url from "url";
import * as path from "path";
import { FileFormatSettings, SyncFileSettingsParams } from "./lsp-ext";

type LogLevel = "error" | "warn" | "info" | "debug" | "trace";

// This is a direct representation of the Client settings sent to the Server in the
// `initializationOptions` field of `InitializeParams`. These are only pulled at the
// user level since they are global settings on the server side (and are scoped to
// `"scope": "application"` in `package.json` so they can't even be set at workspace level).
export type InitializationOptions = {
	logLevel?: LogLevel;
	dependencyLogLevels?: string;
};

export function getInitializationOptions(
	namespace: string,
): InitializationOptions {
	const config = getConfiguration(namespace);

	return {
		logLevel: getOptionalUserValue<LogLevel>(config, "logLevel"),
		dependencyLogLevels: getOptionalUserValue<string>(
			config,
			"dependencyLogLevels",
		),
	};
}

function getOptionalUserValue<T>(
	config: vscode.WorkspaceConfiguration,
	key: string,
): T | undefined {
	const inspect = config.inspect<T>(key);
	return inspect?.globalValue;
}

function getConfiguration(
	config: string,
	scope?: vscode.ConfigurationScope,
): vscode.WorkspaceConfiguration {
	return vscode.workspace.getConfiguration(config, scope);
}

export class TomlSettings {
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

function normalizePath(file: string): string {
	if (file.startsWith("file:///")) {
		file = url.fileURLToPath(file);
	}
	return path.normalize(file);
}
