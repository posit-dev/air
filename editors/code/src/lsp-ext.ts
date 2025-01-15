import * as lc from "vscode-languageclient/node";

export type ViewFileKind = "TreeSitter" | "SyntaxTree" | "FormatTree";

export const viewFile = new lc.RequestType<
	lc.TextDocumentPositionParams & { kind: ViewFileKind },
	string,
	void
>("air/viewFile");

export interface TomlSettingsParams {
	file_settings: TomlFileSettings[];
	[key: string]: any;
}

export interface TomlFileSettings {
	url: string;
	settings: TomlGlobalSettings;
	[key: string]: any;
}

export interface TomlGlobalSettings {
	format: TomlFormatSettings;
	[key: string]: any;
}

export interface TomlFormatSettings {
	indent_style: "tab" | "space";
	indent_width: number;
	line_width: number;
	[key: string]: any;
}

export const tomlSettings = new lc.NotificationType<TomlSettingsParams>(
	"air/tomlSettings",
);
