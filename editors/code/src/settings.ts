import { WorkspaceConfiguration, WorkspaceFolder } from "vscode";
import { getConfiguration, getWorkspaceFolders } from "./common/vscodeapi";

type LogLevel = "error" | "warn" | "info" | "debug" | "trace";

// One time settings that aren't ever refreshed within the extension's lifetime.
// They are read at the user (i.e. global) scope.
export interface IGlobalSettings {
	logLevel?: LogLevel;
	dependencyLogLevels?: string;
}

// Client representation of user level client settings.
// TODO: These are refreshed using a `Configuration` LSP request from the server.
// (It is possible we should ONLY get these through `Configuration` and not through
// initializationOptions)
export interface ISettings {}

// Client representation of workspace level client settings.
// Same as the user level settings, with the addition of the workspace path.
// TODO: These are refreshed using a `Configuration` LSP request from the server.
// (It is possible we should ONLY get these through `Configuration` and not through
// initializationOptions)
export interface IWorkspaceSettings {
	url: string;
	settings: ISettings;
}

// This is a direct representation of the Client settings sent to the Server in the
// `initializationOptions` field of `InitializeParams`
export type IInitializationOptions = {
	globalSettings: IGlobalSettings;
	userSettings: ISettings;
	workspaceSettings: IWorkspaceSettings[];
};

export async function getGlobalSettings(
	namespace: string
): Promise<IGlobalSettings> {
	const config = getConfiguration(namespace);

	return {
		logLevel: getOptionalUserValue<LogLevel>(config, "logLevel"),
		dependencyLogLevels: getOptionalUserValue<string>(
			config,
			"dependencyLogLevels"
		),
	};
}

export async function getUserSettings(namespace: string): Promise<ISettings> {
	const config = getConfiguration(namespace);

	return {};
}

export function getWorkspaceSettings(
	namespace: string
): Promise<IWorkspaceSettings[]> {
	return Promise.all(
		getWorkspaceFolders().map((workspaceFolder) =>
			getWorkspaceFolderSettings(namespace, workspaceFolder)
		)
	);
}

async function getWorkspaceFolderSettings(
	namespace: string,
	workspace: WorkspaceFolder
): Promise<IWorkspaceSettings> {
	const config = getConfiguration(namespace, workspace.uri);

	return {
		url: workspace.uri.toString(),
		settings: {},
	};
}

function getOptionalUserValue<T>(
	config: WorkspaceConfiguration,
	key: string
): T | undefined {
	const inspect = config.inspect<T>(key);
	return inspect?.globalValue;
}
