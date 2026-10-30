import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const read = (path) => readFileSync(path, "utf8");
const expected = JSON.parse(read(resolve(evidenceDir, "expected-values.json")));
const observed = JSON.parse(read(resolve(evidenceDir, "chrome-observation-agent-browser.json")));
const readme = read(resolve(evidenceDir, "README.md"));

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function close(actual, wanted, label) {
  assert(Number.isFinite(actual) && Math.abs(actual - wanted) <= 1e-9, `${label}: ${actual} != ${wanted}`);
}

function pxList(value) {
  return value.split(/\s+/).map((part) => Number(part.replace(/px$/, "")));
}

function verifyGeometry(capture) {
  assert(capture.fontReady === true, "Ahem font was not ready");
  assert(capture.userAgent.includes("HeadlessChrome/156.0.0.0"), "unexpected Chrome version");
  assert(capture.results.length === expected.cases.length, "case count differs");

  for (const [index, target] of expected.cases.entries()) {
    const actual = capture.results[index];
    assert(actual?.id === target.id, `case order/id differs at index ${index}`);
    const columns = pxList(actual.columns);
    assert(columns.length === target.columns.length, `${target.id}: column count differs`);
    columns.forEach((size, track) => close(size, target.columns[track], `${target.id}: column ${track + 1}`));
    const rows = pxList(actual.rows);
    assert(rows.length === target.rows.length, `${target.id}: row count differs`);
    rows.forEach((size, row) => close(size, target.rows[row], `${target.id}: row ${row + 1}`));
    close(actual.grid.width, 100, `${target.id}: root width`);
    close(actual.item.x - actual.grid.x, target.item.x, `${target.id}: relative item x`);
    close(actual.item.y - actual.grid.y, target.item.y, `${target.id}: relative item y`);
    close(actual.item.width, target.item.width, `${target.id}: item width`);
    close(actual.item.height, target.item.height, `${target.id}: item height`);
    close(actual.text.width, target.text.width, `${target.id}: text width`);
    close(actual.text.height, target.text.height, `${target.id}: text height`);
    close(Math.max(0, actual.text.width - actual.item.width), target.text_inline_overflow, `${target.id}: text overflow`);
    assert(actual.computed.gridColumnStart === "1", `${target.id}: column start differs`);
    assert(actual.computed.gridColumnEnd === "span 2", `${target.id}: column span differs`);
    assert(actual.computed.aspectRatio === target.ratio, `${target.id}: ratio differs`);
    assert(actual.computed.font === "10px / 10px Ahem, sans-serif", `${target.id}: computed font differs`);

    const declarations = Object.fromEntries(
      target.constraints.split(";").filter(Boolean).map((declaration) => {
        const [property, value] = declaration.split(":");
        return [property, value];
      }),
    );
    const defaults = { "min-width": "auto", "max-width": "none", "min-height": "auto", "max-height": "none" };
    const styleNames = { "min-width": "minWidth", "max-width": "maxWidth", "min-height": "minHeight", "max-height": "maxHeight" };
    for (const [property, defaultValue] of Object.entries(defaults)) {
      if (declarations[property] !== undefined) {
        const expectedValue = declarations[property] === "0" ? "0px" : declarations[property];
        assert(actual.computed[styleNames[property]] === expectedValue, `${target.id}: ${property} differs`);
      } else {
        assert(actual.computed[styleNames[property]] === defaultValue, `${target.id}: ${property} default differs`);
      }
    }
  }
}

verifyGeometry(observed);

const expectedHashes = Object.fromEntries(
  [...readme.matchAll(/^\|\s*([^|]+)\s*\|\s*`([a-f0-9]{64})`\s*\|$/gm)].map((match) => [match[1].trim(), match[2]]),
);
const artifacts = {
  "Canonical HTML": resolve(repositoryRoot, "tests/fixtures/css/c11/grid-direct-text-aspect-ratio-multispan-precomparison.html"),
  "Ahem TTF": resolve(repositoryRoot, "tests/fixtures/resources/c15/Ahem.ttf"),
  "Agent Browser observation JSON": resolve(evidenceDir, "chrome-observation-agent-browser.json"),
  "Independent expected values": resolve(evidenceDir, "expected-values.json"),
  "Agent Browser screenshot": resolve(evidenceDir, "chrome-oracle-agent-browser.png"),
};
for (const [label, path] of Object.entries(artifacts)) {
  const actualHash = createHash("sha256").update(readFileSync(path)).digest("hex");
  assert(actualHash === expectedHashes[label], `${label}: SHA-256 differs`);
}

const negativeControl = structuredClone(observed);
negativeControl.results[0].item.height += 1;
let mutationRejected = false;
try {
  verifyGeometry(negativeControl);
} catch {
  mutationRejected = true;
}
assert(mutationRejected, "one-pixel geometry mutation was not rejected");

console.log(`Verified ${expected.cases.length}/${expected.cases.length} Agent Browser geometry cases, artifact hashes, and a rejected geometry mutation.`);
