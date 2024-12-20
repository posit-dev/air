// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// https://github.com/microsoft/vscode-python-tools-extension-template

import * as util from "util";
import { Disposable, OutputChannel } from "vscode";

type Arguments = unknown[];
class OutputChannelLogger {
	constructor(private readonly channel: OutputChannel) {}

	public traceLog(...data: Arguments): void {
		this.channel.appendLine(util.format(...data));
	}
}

let channel: OutputChannelLogger | undefined;
export function registerLogger(outputChannel: OutputChannel): Disposable {
	channel = new OutputChannelLogger(outputChannel);
	return {
		dispose: () => {
			channel = undefined;
		},
	};
}

export function traceLog(...args: Arguments): void {
	channel?.traceLog(...args);
}
