import * as url from "url";
import * as path from "path";

export function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export function normalizePath(file: string): string {
	if (file.startsWith("file:///")) {
		file = url.fileURLToPath(file);
	}
	return path.normalize(file);
}
