import * as cp from "child_process";

export interface CommandResult {
	type: "result";
	code: number | null;
	stdout: string;
	stderr: string;
}

export interface CommandError {
	type: "error";
	error: Error;
}

export function isResult(
	result: CommandResult | CommandError,
): result is CommandResult {
	return result.type == "result";
}

export function isError(
	result: CommandResult | CommandError,
): result is CommandError {
	return result.type == "error";
}

/**
 * Spawns a process and runs a command
 *
 * Collects stdout and stderr emitted along the way.
 *
 * @returns Returns a promise that results when the process exits or errors
 */
export async function runCommand(
	command: string,
	args?: readonly string[],
	options?: cp.SpawnOptionsWithoutStdio,
): Promise<CommandResult | CommandError> {
	return new Promise<CommandResult | CommandError>((resolve) => {
		let stdout = "";
		let stderr = "";

		// Use spawn instead of exec to avoid maxBufferExceeded error
		const p = cp.spawn(command, args, options);

		p.stdout.setEncoding("utf8");
		p.stdout.on("data", (data) => (stdout += data));

		p.stderr.setEncoding("utf8");
		p.stderr.on("data", (data) => (stderr += data));

		// `error` should fire before `close` in the event both fire.
		// `error` should also fire no matter what.
		p.on("error", (error) => {
			return resolve({
				type: "error",
				error: error,
			});
		});

		// Use `close` to wait for stdio streams to close too
		p.on("close", (code) => {
			return resolve({
				type: "result",
				code: code,
				stdout: stdout,
				stderr: stderr,
			});
		});
	});
}
