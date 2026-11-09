import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDirectory, "../../../..");
const fixturePath = resolve(
  repoRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-fit-content-positions-runtime.html",
);
const fixture = await readFile(fixturePath, "utf8");
const tail = ["min-content", "min-content", "max-content", "max-content", "auto"];
const scenes = [
  { name: "scene-01-p01-start-20px", fitPosition: 1, cap: "20px", align: "start" },
  { name: "scene-02-p02-start-20px", fitPosition: 2, cap: "20px", align: "start" },
  { name: "scene-03-p03-start-20px", fitPosition: 3, cap: "20px", align: "start" },
  { name: "scene-04-p04-start-20pct", fitPosition: 4, cap: "20%", align: "start" },
  { name: "scene-05-p05-start-20px", fitPosition: 5, cap: "20px", align: "start" },
  { name: "scene-06-p06-start-20px", fitPosition: 6, cap: "20px", align: "start" },
  { name: "scene-07-p01-stretch-0pct-overflow", fitPosition: 1, cap: "0%", align: "stretch" },
  { name: "scene-08-p04-stretch-20pct", fitPosition: 4, cap: "20%", align: "stretch" },
];

const manifestScenes = [];
for (const scene of scenes) {
  const tracks = [...tail];
  tracks.splice(scene.fitPosition - 1, 0, `fit-content(${scene.cap})`);
  const source = fixture
    .replace(
      "--tracks: fit-content(20px) min-content min-content max-content max-content auto;",
      `--tracks: ${tracks.join(" ")};`,
    )
    .replace("--align: start;", `--align: ${scene.align};`);
  if (!source.includes(`--tracks: ${tracks.join(" ")};`)) {
    throw new Error(`Could not specialize runtime fixture for ${scene.name}`);
  }
  const fileName = `${scene.name}.html`;
  await writeFile(resolve(scriptDirectory, fileName), source);
  manifestScenes.push({
    ...scene,
    fileName,
    authoredAutoRows: tracks,
    sha256: createHash("sha256").update(source).digest("hex"),
  });
}

await writeFile(
  resolve(scriptDirectory, "sentinel-manifest.json"),
  `${JSON.stringify({ fixture: "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-fit-content-positions-runtime.html", scenes: manifestScenes }, null, 2)}\n`,
);
