import * as vscode from "vscode";
import * as path from "path";

import * as output from "./output";

// Manages the contributions we make to the environment variables of integrated
// terminals.
export class EnvironmentVariableManager {
	constructor(private collection: vscode.EnvironmentVariableCollection) {
		// `PATH` should mirror the running server's resolved binary, recomputed
		// on each start, so don't cache it across IDE sessions. In particular,
		// it would be bad if a collection was restored after an Air extension
		// update, in which case there would be a short period where terminal
		// `PATH`s point to an outdated location where an `air` executable no
		// longer exists (until the next extension start updates it).
		this.collection.persistent = false;
	}

	// Prepend a directory to the `PATH`
	//
	// `applyAtProcessCreation` and `applyAtShellIntegration` are really
	// mutually exclusive:
	//
	// - `applyAtProcessCreation` would modify the `PATH` as early as possible,
	//   before any shell startup scripts run (like zshrc, for example). This
	//   could allow shell scripts to overwrite our modifications.
	// - `applyAtShellIntegration` is the newer mechanism and applies the
	//   modification via a VS Code managed shell integration script after any
	//   shell startup scripts have run.
	public prependToPATH(directory: string): void {
		this.collection.prepend("PATH", directory + path.delimiter, {
			applyAtProcessCreation: false,
			applyAtShellIntegration: true,
		});
		output.log(`Added \`${directory}\` to the terminal PATH.`);
	}

	// Clear any contributions we've made
	public clear(): void {
		this.collection.clear();
	}
}
