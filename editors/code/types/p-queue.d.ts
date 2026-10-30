// `@esm2cjs/p-queue` only ships types next to its ESM build, which a CommonJS
// project can't import directly. Load them in import mode and re-export them
// as a CommonJS-compatible module.
import type ESMPQueue from "../node_modules/p-queue/build/esm/index.js" with { "resolution-mode": "import" };

declare const PQueue: typeof ESMPQueue;
type PQueue = ESMPQueue;
export default PQueue;
