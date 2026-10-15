import { readFile } from "node:fs/promises";
import { mountVueCounter } from "./vue-adapter.mjs";
import { mountSvelteCounter } from "./svelte-adapter.mjs";
import { runAdapterScenario } from "./run-scenario.mjs";

const packageJson = JSON.parse(await readFile(new URL("../package.json", import.meta.url), "utf8"));
const vue = await runAdapterScenario("Vue", "vue-root", mountVueCounter);
const svelte = await runAdapterScenario("Svelte", "svelte-root", mountSvelteCounter);
const report = {
  schema: "spinon-r07-framework-adapter-observation@1",
  environment: {
    node: process.version,
    platform: process.platform,
    architecture: process.arch,
  },
  dependencies: packageJson.dependencies,
  adapters: [vue, svelte].map(({ recorder, button, ...result }) => result),
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
