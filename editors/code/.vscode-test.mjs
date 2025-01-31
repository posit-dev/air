import { defineConfig } from "@vscode/test-cli";

export default defineConfig({
	files: "out/test/**/*.test.js",
	// Changing the workspace folder dynamically causes the VS Code host to
	// silently crash. So we set it only once here.
	workspaceFolder: "./src/test",
	mocha: {
		color: true,
		// This is the default but just in case. We rely on sequentiality for
		// proper setup and teardown of global state (e.g. presence of an
		// air.toml file in the workspace).
		parallel: false,
		timeout: 30000,
	},
});
