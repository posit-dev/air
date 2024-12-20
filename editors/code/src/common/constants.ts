// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// https://github.com/microsoft/vscode-python-tools-extension-template

import * as path from "path";

const folderName = path.basename(__dirname);

export const EXTENSION_ROOT_DIR =
	folderName === "common"
		? path.dirname(path.dirname(__dirname))
		: path.dirname(__dirname);
