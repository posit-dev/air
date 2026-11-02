const fs = require("node:fs");
const path = require("node:path");

const esbuild = require("esbuild");

const production = process.argv.includes("--production");
const watch = process.argv.includes("--watch");

const TEST_ENTRY = "test-entry";
const TEST_DIR = path.join(__dirname, "src", "test");

async function main() {
	const entryPoints = [{ in: "src/extension.ts", out: "dist/extension" }];
	if (!production) {
		entryPoints.push({ in: TEST_ENTRY, out: "out/test" });
	}

	const ctx = await esbuild.context({
		entryPoints,
		outdir: ".",
		bundle: true,
		format: "cjs",
		minify: production,
		sourcemap: !production,
		sourcesContent: false,
		platform: "node",
		// `mocha` must come from the test runner: it only fills in the exports
		// of `import { before } from "mocha"` on the instance that is running
		// the tests, so a bundled copy would export `before === undefined`. The
		// extension doesn't import `mocha`, so sharing this list is harmless.
		external: ["vscode", "mocha"],
		logLevel: "warning",
		plugins: [
			testEntryPlugin,
			/* add to the end of plugins array */
			esbuildProblemMatcherPlugin,
		],
	});
	if (watch) {
		await ctx.watch();
	} else {
		await ctx.rebuild();
		await ctx.dispose();
	}
}

/**
 * Virtual entry point that imports every `*.test.ts` file under `src/test`, so
 * the extension host tests are a single bundle (same approach as
 * vscode-copilot-chat). The directory is listed on every build, so new test
 * files are picked up without maintaining an index.
 *
 * @type {import('esbuild').Plugin}
 */
const testEntryPlugin = {
	name: TEST_ENTRY,

	setup(build) {
		build.onResolve({ filter: new RegExp(`^${TEST_ENTRY}$`) }, (args) => {
			if (args.kind !== "entry-point") {
				return;
			}
			return { path: TEST_ENTRY, namespace: TEST_ENTRY };
		});

		build.onLoad({ filter: /.*/, namespace: TEST_ENTRY }, () => {
			const entries = fs.readdirSync(TEST_DIR, { recursive: true });

			const files = entries
				.filter((entry) => entry.endsWith(".test.ts"))
				.sort();
			if (files.length === 0) {
				throw new Error(`No test files found in ${TEST_DIR}`);
			}

			// `watchDirs` isn't recursive, so watch every subdirectory too in
			// case a test file is added there
			const dirs = entries
				.map((entry) => path.join(TEST_DIR, entry))
				.filter((entry) => fs.statSync(entry).isDirectory());

			const contents = files
				.map((file) => file.replaceAll(path.sep, "/"))
				.map((file) => `import "./${file}";`)
				.join("\n");

			return {
				contents,
				loader: "ts",
				resolveDir: TEST_DIR,
				watchDirs: [TEST_DIR, ...dirs],
			};
		});
	},
};

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildProblemMatcherPlugin = {
	name: "esbuild-problem-matcher",

	setup(build) {
		build.onStart(() => {
			console.log("[watch] build started");
		});
		build.onEnd((result) => {
			result.errors.forEach(({ text, location }) => {
				console.error(`✘ [ERROR] ${text}`);
				if (!location) {
					return;
				}
				console.error(
					`    ${location.file}:${location.line}:${location.column}:`,
				);
			});
			console.log("[watch] build finished");
		});
	},
};

main().catch((e) => {
	console.error(e);
	process.exit(1);
});
