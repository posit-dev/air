import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

export class Lsp {
	private client: lc.LanguageClient | null = null;

	// We use the same output channel for all LSP instances (e.g. a new instance
	// after a restart) to avoid having multiple channels in the Output viewpane.
	private channel: vscode.OutputChannel;

	// Simple flags to manage the state of the LSP
	private restartInProgress = false;
	private stopInProgress = false;

	constructor(context: vscode.ExtensionContext) {
		this.channel = vscode.window.createOutputChannel("Air Language Server");
		context.subscriptions.push(this.channel);
	}

	public async start() {
		if (this.client) {
			throw new Error("Air is already running");
		}

		let options: lc.ServerOptions = {
			command: "air",
			args: ["lsp"],
		};

		let clientOptions: lc.LanguageClientOptions = {
			documentSelector: [{ scheme: "file", language: "r" }],
			synchronize: {
				// Notify the server about file changes to R files contained in the workspace
				fileEvents:
					vscode.workspace.createFileSystemWatcher("**/*.[Rr]"),
			},
			outputChannel: this.channel,
		};

		this.client = new lc.LanguageClient(
			"airLanguageServer",
			"Air Language Server",
			options,
			clientOptions,
		);
		await this.client.start();
	}

	public async stop() {
		if (!this.client) {
			throw new Error("Air is not running");
		}

		if (this.stopInProgress) {
			throw new Error("Air is already restarting");
		}
		this.stopInProgress = true;

		try {
			await this.client.stop();
		} finally {
			this.client = null;
			this.stopInProgress = false;
		}
	}

	public async restart() {
		if (this.restartInProgress) {
			throw new Error("Air is already restarting");
		}
		this.restartInProgress = true;

		try {
			if (this.client) {
				await this.stop();
			}
			await this.start();
		} finally {
			this.restartInProgress = false;
		}
	}
}
