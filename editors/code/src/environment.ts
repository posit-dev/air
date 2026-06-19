import * as vscode from "vscode";
import * as path from "path";

import * as output from "./output";
import { getWorkspaceSettings } from "./settings";
import { State, StateChange } from "./lsp";

// Manages the `PATH` environment variable contribution we make to integrated
// terminals.
export class PathEnvironmentVariableManager implements vscode.Disposable {
	private executableDirectory: string | null = null;
	private workspaceFolder: vscode.WorkspaceFolder | null = null;

	private subscriptions: vscode.Disposable[] = [];

	constructor(
		private collection: vscode.EnvironmentVariableCollection,
		onStateChange: vscode.Event<StateChange>,
	) {
		// When the LSP fully starts up, sync our shared state and update `PATH` contribution.
		// When the LSP shuts down (restart or deactivation), clear shared state and unset `PATH` contribution.
		this.subscriptions.push(
			onStateChange((change) => {
				switch (change.state) {
					case State.Started: {
						this.executableDirectory = path.dirname(
							change.binaryPath,
						);
						this.workspaceFolder = change.workspaceFolder;
						this.update();
						break;
					}
					case State.Stopped: {
						this.executableDirectory = null;
						this.workspaceFolder = null;
						this.delete();
						break;
					}
				}
			}),
		);

		// When the user changes `air.addExecutableToTerminalPath`, reflect this immediately in new terminals
		this.subscriptions.push(
			vscode.workspace.onDidChangeConfiguration((event) => {
				if (
					event.affectsConfiguration(
						"air.addExecutableToTerminalPath",
					)
				) {
					this.update();
				}
			}),
		);
	}

	private update(): void {
		if (!this.workspaceFolder) {
			return;
		}
		if (!this.executableDirectory) {
			return;
		}

		const workspaceSettings = getWorkspaceSettings(
			"air",
			this.workspaceFolder,
		);

		if (workspaceSettings.addExecutableToPath) {
			this.prepend(this.executableDirectory);
		} else {
			this.delete();
		}
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
	private prepend(directory: string): void {
		this.collection.prepend("PATH", directory + path.delimiter, {
			applyAtProcessCreation: false,
			applyAtShellIntegration: true,
		});
		output.log(`Added \`${directory}\` to the terminal PATH.`);
	}

	private delete(): void {
		this.collection.delete("PATH");
		output.log(`Removed terminal PATH mutation.`);
	}

	dispose() {
		for (const subscription of this.subscriptions) {
			subscription.dispose();
		}
	}
}
