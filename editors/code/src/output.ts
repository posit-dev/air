import * as util from "util";
import { Disposable, OutputChannel } from "vscode";

type Arguments = unknown[];
class OutputChannelLogger {
	constructor(private readonly channel: OutputChannel) {}

	public log(...data: Arguments): void {
		this.channel.appendLine(util.format(...data));
	}
}

let channel: OutputChannelLogger | undefined;
export function registerLogger(logChannel: OutputChannel): Disposable {
	channel = new OutputChannelLogger(logChannel);
	return {
		dispose: () => {
			channel = undefined;
		},
	};
}

/*
 * Free function for logging to the global output channel shared with the server
 *
 * Adapted from:
 * https://github.com/microsoft/vscode-python-tools-extension-template/blob/main/src/common/log/logging.ts
 */
export function log(...args: Arguments): void {
	if (process.env.CI === "true") {
		console.log(...args);
	}
	channel?.log(...args);
}
