import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";

const sourcePath =
  "tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html";
const source = readFileSync(sourcePath, "utf8");
const scenes = [
  {
    name: "zero-normal",
    alignment: "normal",
    rowGap: "0",
  },
  {
    name: "gap-2_5-space-between",
    alignment: "space-between",
    rowGap: "2.5",
  },
  {
    name: "gap-7_5-end",
    alignment: "end",
    rowGap: "7.5",
  },
  {
    name: "gap-7_5-center",
    alignment: "center",
    rowGap: "7.5",
  },
];

const outputPaths = scenes.map(
  (scene) =>
    `tests/fixtures/css/c11/root-html-auto-row-content-alignment-gap-runtime-${scene.name}.html`,
);
const existing = outputPaths.filter((path) => existsSync(path));
if (existing.length > 0) {
  throw new Error(`Refusing to overwrite existing fixture(s): ${existing.join(", ")}`);
}

for (const [index, scene] of scenes.entries()) {
  const outputPath = outputPaths[index];
  const replacements = [
    ["align-content: stretch;", `align-content: ${scene.alignment};`],
    ["gap: 0;", `row-gap: ${scene.rowGap}px; column-gap: 0;`],
  ];
  let output = source;
  for (const [before, after] of replacements) {
    if (output.split(before).length !== 2) {
      throw new Error(`Expected exactly one ${before} declaration in ${sourcePath}`);
    }
    output = output.replace(before, after);
  }
  writeFileSync(outputPath, output, { flag: "wx" });
  const sha256 = createHash("sha256").update(output).digest("hex");
  process.stdout.write(`${outputPath} ${sha256}\n`);
}
