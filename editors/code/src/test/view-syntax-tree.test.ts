import * as vscode from "vscode";
import * as assert from "assert";
import { before } from "mocha";

import { withUntitledEditor } from "./editor-utils";
import { waitLsp } from "./extension";

before(async () => {
	await waitLsp();
});

suite("View Syntax Tree Test Suite", () => {
	// https://github.com/posit-dev/air/issues/532
	test("View syntax tree works on unsaved files", async () => {
		await withUntitledEditor("x <- 1\n", "r", async (_editor, doc) => {
			assert.strictEqual(doc.uri.scheme, "untitled");

			await vscode.commands.executeCommand("air.viewSyntaxTree");

			const treeDoc = vscode.workspace.textDocuments.find(
				(d) => d.uri.scheme === "air-syntax-tree",
			);
			assert.ok(treeDoc, "Syntax tree document should be open");

			// The provider returns "" when there is no active R editor
			assert.ok(
				treeDoc.getText().includes("R_BINARY_EXPRESSION"),
				`Unexpected syntax tree contents: ${treeDoc.getText()}`,
			);
		});
	});
});
