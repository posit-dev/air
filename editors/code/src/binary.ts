import * as vscode from "vscode";
import which from "which";
import * as output from "./output";
import { AIR_BINARY_NAME, BUNDLED_AIR_EXECUTABLE } from "./constants";

export type ExecutableLocation = "environment" | "bundled";

export async function resolveAirBinaryPath(
	executableLocation: ExecutableLocation
): Promise<string> {
	if (!vscode.workspace.isTrusted) {
		output.log(
			`Workspace is not trusted, using bundled executable: ${BUNDLED_AIR_EXECUTABLE}`
		);
		return BUNDLED_AIR_EXECUTABLE;
	}

	// User requested the bundled air binary
	if (executableLocation === "bundled") {
		output.log(
			`Using bundled executable as requested by \`air.executableLocation\`: ${BUNDLED_AIR_EXECUTABLE}`
		);
		return BUNDLED_AIR_EXECUTABLE;
	}

	// First choice: the executable in the global environment.
	const environmentPath = await which(AIR_BINARY_NAME, { nothrow: true });
	if (environmentPath) {
		output.log(`Using environment executable: ${environmentPath}`);
		return environmentPath;
	}

	// Second choice: bundled executable.
	output.log(`Using bundled executable: ${BUNDLED_AIR_EXECUTABLE}`);
	return BUNDLED_AIR_EXECUTABLE;
}
