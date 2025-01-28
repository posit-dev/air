import { defineConfig } from "@vscode/test-cli";

export default defineConfig({
	files: "out/test/**/*.test.js",
	workspaceFolder: "./src/test",
	mocha: {
		color: true,
		parallel: false,
		timeout: 30000,
	},
});
