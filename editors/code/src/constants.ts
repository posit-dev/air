import * as path from "path";

const folderName = path.basename(__dirname);

/**
 * Path to the root directory of this extension.
 * https://github.com/microsoft/vscode-python-tools-extension-template/blob/main/src/common/constants.ts
 */
export const EXTENSION_ROOT_DIR =
	folderName === "common"
		? path.dirname(path.dirname(__dirname))
		: path.dirname(__dirname);

/**
 * Name of the `air` binary based on the current platform.
 */
export const AIR_BINARY_NAME = process.platform === "win32" ? "air.exe" : "air";

/**
 * Path to the `air` executable that is bundled with the extension.
 * The GitHub Action is in charge of placing the executable here.
 */
export const BUNDLED_AIR_EXECUTABLE = path.join(
	EXTENSION_ROOT_DIR,
	"bundled",
	"bin",
	AIR_BINARY_NAME
);
