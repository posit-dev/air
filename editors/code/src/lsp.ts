import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import { default as PQueue } from "p-queue";

// All session management operations are put on a queue. They can't run
// concurrently and either result in a started or stopped state. Starting when
// started is a noop, same for stopping when stopped. On the other hand
// restarting is always scheduled.
enum State {
	Started = "started",
	Stopped = "stopped",
}

export class Lsp {
	private client: lc.LanguageClient | null = null;

	// We use the same output channel for all LSP instances (e.g. a new instance
	// after a restart) to avoid having multiple channels in the Output viewpane.
	private channel: vscode.OutputChannel;

	private state = State.Stopped;
	private stateQueue: PQueue;

	constructor(context: vscode.ExtensionContext) {
		this.channel = vscode.window.createOutputChannel("Air Language Server");
		context.subscriptions.push(this.channel);
		this.stateQueue = new PQueue({ concurrency: 1 });
	}

	public async start() {
		await this.stateQueue.add(async () => await this.startImpl());
	}

	public async restart() {
		await this.stateQueue.add(async () => await this.restartImpl());
	}

	public async stop() {
		await this.stateQueue.add(async () => await this.stopImpl());
	}

	private async startImpl() {
		// Noop if already started
		if (this.state === State.Started) {
			return;
		}

		let options: lc.ServerOptions = {
			command: "air",
			args: ["lsp"],
		};

		let clientOptions: lc.LanguageClientOptions = {
			// Look for unnamed scheme
			documentSelector: [
				{ language: "r", scheme: "untitled" },
				{ language: "r", scheme: "file" },
				{ language: "r", pattern: "**/*.{r,R}" },
				{ language: "r", pattern: "**/*.{rprofile,Rprofile}" },
			],
			synchronize: {
				// Notify the server about file changes to R files contained in the workspace
				fileEvents:
					vscode.workspace.createFileSystemWatcher("**/*.[Rr]"),
			},
			outputChannel: this.channel,
		};

		const client = new lc.LanguageClient(
			"airLanguageServer",
			"Air Language Server",
			options,
			clientOptions,
		);
		await client.start();

		// Only update state if no error occurred
		this.client = client;
		this.state = State.Started;
	}

	private async stopImpl() {
		// Noop if already stopped
		if (this.state === State.Stopped) {
			return;
		}

		try {
			await this.client?.stop();
		} finally {
			// We're always stopped even if an error happens. Hard to do better
			// in that case, we just drop the client and hope an eventual restart
			// will put us back in a good place.
			this.state = State.Stopped;
			this.client = null;
		}
	}

	private async restartImpl() {
		if (this.state === State.Started) {
			await this.stopImpl();
		}
		await this.startImpl();
	}
}
