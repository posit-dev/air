import * as vscode from "vscode";
import * as fs from "fs";
import which from "which";

import * as output from "./output";
import { AIR_BINARY_NAME, BUNDLED_AIR_EXECUTABLE } from "./constants";

export type ExecutableLocation = "environment" | "bundled";

export async function resolveAirBinaryPath(
	executableLocation: ExecutableLocation,
): Promise<string> {
	if (!vscode.workspace.isTrusted) {
		output.log(
			`Workspace is not trusted, using bundled executable: ${BUNDLED_AIR_EXECUTABLE}`,
		);
		return BUNDLED_AIR_EXECUTABLE;
	}

	// User requested the `"bundled"` air binary
	if (executableLocation === "bundled") {
		if (fs.existsSync(BUNDLED_AIR_EXECUTABLE)) {
			output.log(
				`Using bundled executable as requested by \`air.executableLocation\`: ${BUNDLED_AIR_EXECUTABLE}`,
			);
			return BUNDLED_AIR_EXECUTABLE;
		}

		// Fallthrough
		output.log(`Bundled executable not found: ${BUNDLED_AIR_EXECUTABLE}`);
	}

	// User requested `"environment"` or there is no bundled binary.
	// First check the `PATH`.
	const environmentPath = await which(AIR_BINARY_NAME, { nothrow: true });

	if (environmentPath) {
		output.log(`Using environment executable: ${environmentPath}`);
		return environmentPath;
	}

	// We couldn't find a binary on the `PATH`, use the bundled
	// binary if it exists.
	if (fs.existsSync(BUNDLED_AIR_EXECUTABLE)) {
		output.log(`Using bundled executable: ${BUNDLED_AIR_EXECUTABLE}`);
		return BUNDLED_AIR_EXECUTABLE;
	}

	// Run away and go live in the woods
	output.log(`No suitable executable found`);
	throw new Error(
		"No suitable executable found in environment or bundled location",
	);
}
