import * as vscode from "vscode";
import * as fs from "fs";
import which from "which";

import * as output from "./output";
import { AIR_BINARY_NAME, BUNDLED_AIR_EXECUTABLE } from "./constants";

export type ExecutableStrategy = "bundled" | "environment" | "path";

export async function resolveAirBinaryPath(
	executableStrategy: ExecutableStrategy,
	executablePath?: string,
): Promise<string> {
	if (!vscode.workspace.isTrusted) {
		output.log(
			`Workspace is not trusted, using bundled executable: ${BUNDLED_AIR_EXECUTABLE}`,
		);

		const bundledPath = airBinaryFromBundled();

		if (bundledPath) {
			output.log(`Using bundled executable: ${bundledPath}`);
			return bundledPath;
		}

		throw new Error(
			"Workspace is not trusted and failed to find executable in bundled location",
		);
	} else if (executableStrategy === "bundled") {
		const bundledPath = airBinaryFromBundled();

		if (bundledPath) {
			output.log(`Using bundled executable: ${bundledPath}`);
			return bundledPath;
		}

		output.log(
			"Bundled executable not found, falling back to environment executable",
		);
		const environmentPath = await airBinaryFromEnvironment();

		if (environmentPath) {
			output.log(`Using environment executable: ${environmentPath}`);
			return environmentPath;
		}

		throw new Error(
			"Failed to find bundled executable and fallback environment executable",
		);
	} else if (executableStrategy === "environment") {
		const environmentPath = await airBinaryFromEnvironment();

		if (environmentPath) {
			output.log(`Using environment executable: ${environmentPath}`);
			return environmentPath;
		}

		output.log(
			"Environment executable not found, falling back to bundled executable",
		);
		const bundledPath = airBinaryFromBundled();

		if (bundledPath) {
			output.log(`Using bundled executable: ${bundledPath}`);
			return bundledPath;
		}

		throw new Error(
			"Failed to find environment executable and fallback bundled executable",
		);
	} else if (executableStrategy === "path") {
		const path = airBinaryFromPath(executablePath);

		if (path) {
			output.log(`Using executable from \`air.executablePath\`: ${path}`);
			return path;
		}

		throw new Error("Failed to find executable at `air.executablePath`");
	} else {
		throw new Error("Unreachable");
	}
}

function airBinaryFromBundled(): string | undefined {
	if (!fs.existsSync(BUNDLED_AIR_EXECUTABLE)) {
		output.log(
			`Failed to find bundled executable: ${BUNDLED_AIR_EXECUTABLE}`,
		);
		return undefined;
	}

	output.log(`Found bundled executable: ${BUNDLED_AIR_EXECUTABLE}`);
	return BUNDLED_AIR_EXECUTABLE;
}

async function airBinaryFromEnvironment(): Promise<string | undefined> {
	const environmentPath = await which(AIR_BINARY_NAME, { nothrow: true });

	if (!environmentPath) {
		output.log("Failed to find environment executable");
		return undefined;
	}

	output.log(`Found environment executable: ${environmentPath}`);
	return environmentPath;
}

function airBinaryFromPath(executablePath?: string): string | undefined {
	if (!executablePath) {
		output.log(
			"Failed to find executable from path, no `air.executablePath` provided",
		);
		return undefined;
	}

	if (!fs.existsSync(executablePath)) {
		output.log(
			"Failed to find executable from path, provided `air.executablePath` does not exist",
		);
		return undefined;
	}

	output.log(
		`Found executable from \`air.executablePath\`: ${executablePath}`,
	);
	return executablePath;
}
