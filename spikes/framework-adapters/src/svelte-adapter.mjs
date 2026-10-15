import { readFile, mkdtemp, writeFile, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { compile } from "svelte/compiler";
import { mount, tick, unmount } from "svelte";
import { installSvelteDomFacade } from "./svelte-dom-facade.mjs";

const fixturePath = new URL("../fixtures/svelte-counter.svelte", import.meta.url);
const moduleDirectory = dirname(fileURLToPath(import.meta.url));

export async function mountSvelteCounter(recorder, ownerId = "svelte-root") {
  const source = await readFile(fixturePath, "utf8");
  const compiled = compile(source, {
    filename: "svelte-counter.svelte",
    generate: "client",
    dev: true,
  });
  const generatedDirectory = await mkdtemp(join(moduleDirectory, ".generated-svelte-"));
  const generatedPath = join(generatedDirectory, "svelte-counter.mjs");
  await writeFile(generatedPath, compiled.js.code, "utf8");
  const restoreGlobals = installSvelteDomFacade(recorder, ownerId);

  try {
    const { default: component } = await import(`${pathToFileURL(generatedPath).href}?r07=${Date.now()}`);
    const instance = mount(component, { target: recorder.root, intro: false });
    await tick();
    return {
      async settle() {
        await tick();
      },
      async unmount() {
        await unmount(instance, { outro: false });
      },
      dispose() {
        restoreGlobals();
        return rm(generatedDirectory, { recursive: true, force: true });
      },
      compileMetadata: {
        runes: compiled.metadata?.runes ?? null,
        warningCodes: compiled.warnings.map((warning) => warning.code),
        generatedBytes: Buffer.byteLength(compiled.js.code),
      },
    };
  } catch (error) {
    restoreGlobals();
    await rm(generatedDirectory, { recursive: true, force: true });
    throw error;
  }
}
