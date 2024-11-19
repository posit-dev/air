import * as vscode from "vscode";

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from "vscode-languageclient/node";

// The LSP is stored in this global singleton
let client: LanguageClient | undefined;

// We use the same output channel for all LSP instances (e.g. a new instance
// after a restart) to avoid having multiple channels in the Output viewpane.
let channel: vscode.OutputChannel;

// Simple flags to manage the state of the LSP
let restartInProgress = false;
let stopInProgress = false;

export function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(
		vscode.commands.registerCommand("air.restart", async () => {
			await restartAirLsp();
		}),
	);

	channel = vscode.window.createOutputChannel("Air Language Server");
	context.subscriptions.push(channel);

	startAirLsp();
}

export function deactivate() {
	stopAirLsp();
}

export async function startAirLsp() {
	if (client) {
		throw new Error("Air is already running");
	}

	let options: ServerOptions = {
		command: "air",
		args: ["lsp"],
	};

	let clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "r" }],
		synchronize: {
			// Notify the server about file changes to R files contained in the workspace
			fileEvents: vscode.workspace.createFileSystemWatcher("**/*.[Rr]"),
		},
		outputChannel: channel,
	};

	client = new LanguageClient(
		"airLanguageServer",
		"Air Language Server",
		options,
		clientOptions,
	);
	await client.start();
}

export async function stopAirLsp() {
	if (!client) {
		throw new Error("Air is not running");
	}

	if (stopInProgress) {
		throw new Error("Air is already restarting");
	}
	stopInProgress = true;

	try {
		await client.stop();
	} finally {
		client = undefined;
		stopInProgress = false;
	}
}

export async function restartAirLsp() {
	if (restartInProgress) {
		throw new Error("Air is already restarting");
	}
	restartInProgress = true;

	try {
		if (client) {
			await stopAirLsp();
		}
		await startAirLsp();
	} finally {
		restartInProgress = false;
	}
}
