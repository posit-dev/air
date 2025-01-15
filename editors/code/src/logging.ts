import * as util from "util";
import { Disposable, OutputChannel } from "vscode";

type Arguments = unknown[];
class OutputChannelLogger {
	constructor(private readonly channel: OutputChannel) {}

	public outputLog(...data: Arguments): void {
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

export function outputLog(...args: Arguments): void {
	if (process.env.CI === "true") {
		console.log(...args);
	}
	channel?.outputLog(...args);
}
