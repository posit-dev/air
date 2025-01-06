import { ConfigurationScope, workspace, WorkspaceConfiguration } from "vscode";

type LogLevel = "error" | "warn" | "info" | "debug" | "trace";

// This is a direct representation of the Client settings sent to the Server in the
// `initializationOptions` field of `InitializeParams`. These are only pulled at the
// user level since they are global settings on the server side (and are scoped to
// `"scope": "application"` in `package.json` so they can't even be set at workspace level).
export type IInitializationOptions = {
	logLevel?: LogLevel;
	dependencyLogLevels?: string;
};

export async function getInitializationOptions(
	namespace: string
): Promise<IInitializationOptions> {
	const config = getConfiguration(namespace);

	return {
		logLevel: getOptionalUserValue<LogLevel>(config, "logLevel"),
		dependencyLogLevels: getOptionalUserValue<string>(
			config,
			"dependencyLogLevels"
		),
	};
}

function getOptionalUserValue<T>(
	config: WorkspaceConfiguration,
	key: string
): T | undefined {
	const inspect = config.inspect<T>(key);
	return inspect?.globalValue;
}

function getConfiguration(
	config: string,
	scope?: ConfigurationScope
): WorkspaceConfiguration {
	return workspace.getConfiguration(config, scope);
}
