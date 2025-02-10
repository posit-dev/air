import * as vscode from "vscode";
import * as assert from "assert";
import * as fs from "fs";
import { before } from "mocha";

import { stripWhitespace, withFileEditor } from "./editor-utils";
import { snapshotPath, testPath, waitLsp, withToml } from "./extension";

before(async () => {
	await waitLsp();

	// Check for an `air.toml` file that is not properly cleaned up as it would
	// interfere with tests
	const airTomlPath = testPath("air.toml");
	if (fs.existsSync(airTomlPath)) {
		console.log(
			"Deleting `air.toml` file that was not properly cleaned up",
		);
		fs.unlinkSync(airTomlPath);
	}
});

suite("Format Test Suite", () => {
	test("Format document with forward propagation of settings", async () => {
		await withFileEditor(snapshotPath("format.R"), async (editor, doc) => {
			const old = doc.getText();

			// To ensure something happens (e.g. add whitespace back)
			await stripWhitespace(editor, doc);

			assert.strictEqual(editor.options.insertSpaces, true);
			assert.strictEqual(editor.options.indentSize, 8);
			assert.strictEqual(editor.options.tabSize, 8);

			await vscode.commands.executeCommand(
				"editor.action.formatDocument",
			);
			await doc.save();

			assert.strictEqual(doc.getText(), old);
		});
	});

	test("Format document with backward propagation of settings", async () => {
		const toml = `
[format]
indent-style = "tab"
indent-width = 6
`;
		await withToml(toml, async () => {
			await withFileEditor(
				snapshotPath("format-toml.R"),
				async (editor, doc) => {
					const old = doc.getText();

					// To ensure something happens (e.g. add whitespace back)
					await stripWhitespace(editor, doc);

					assert.strictEqual(editor.options.insertSpaces, false);
					assert.strictEqual(editor.options.indentSize, 6);
					assert.strictEqual(editor.options.tabSize, 8);

					await vscode.commands.executeCommand(
						"editor.action.formatDocument",
					);
					await doc.save();

					assert.strictEqual(doc.getText(), old);
				},
			);
		});
	});
});
