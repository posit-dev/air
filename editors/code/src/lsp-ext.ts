import * as lc from "vscode-languageclient/node";

export type ViewFileKind = "TreeSitter" | "SyntaxTree" | "FormatTree";

export const viewFile = new lc.RequestType<
	lc.TextDocumentPositionParams & { kind: ViewFileKind },
	string,
	void
>("air/viewFile");

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
