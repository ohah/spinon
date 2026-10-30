import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, "../../../..");

async function readJson(file) {
  return JSON.parse(await readFile(path.join(evidenceDir, file), "utf8"));
}

async function sha256(file) {
  return createHash("sha256")
    .update(await readFile(path.resolve(repoRoot, file)))
    .digest("hex");
}

function decodeAgentBrowserRaw(value) {
  const decoded = JSON.parse(value);
  return typeof decoded === "string" ? JSON.parse(decoded) : decoded;
}

function assertObservationMatches(observation, expected) {
  assert.equal(observation.id, expected.id, `${expected.id}: id`);
  assert.equal(observation.columns, expected.columns, `${expected.id}: columns`);
  assert.equal(observation.rows, expected.rows, `${expected.id}: rows`);
  assert.deepEqual(
    [observation.ratioItem.width, observation.ratioItem.height],
    expected.item,
    `${expected.id}: item frame`,
  );
  assert.equal(
    observation.computed.aspectRatio,
    expected.aspectRatio,
    `${expected.id}: computed aspect-ratio`,
  );
  assert.equal(observation.grid.width, 100, `${expected.id}: grid width`);
  assert.match(observation.computed.font, /10px \/ 10px Ahem/);
}

const expected = await readJson("expected-values.json");
const canonical = await readJson("chrome-observation-agent-browser-auto-track.json");
const embedded = await readJson("chrome-observation-embedded-auto-track.json");
const sourceComparison = await readJson("capture-source-comparison.json");
const evidenceRelative = "spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30";
const canonicalRaw = decodeAgentBrowserRaw(
  await readFile(path.join(evidenceDir, "chrome-observation-agent-browser-auto-track.raw.json"), "utf8"),
);
const embeddedRaw = decodeAgentBrowserRaw(
  await readFile(path.join(evidenceDir, "chrome-observation-embedded-auto-track.raw.json"), "utf8"),
);

assert.equal(expected.caseCount, 17);
assert.equal(expected.cases.length, expected.caseCount);
assert.equal(canonical.caseCount, expected.caseCount);
assert.equal(embedded.caseCount, expected.caseCount);
assert.equal(canonical.fontReady, true);
assert.equal(embedded.fontReady, true);
assert.match(canonical.userAgent, /HeadlessChrome\/156\./);
assert.equal(embedded.userAgent, canonical.userAgent);
assert.deepEqual(canonical.results, embedded.results, "local-font and embedded-font results");
assert.deepEqual(canonicalRaw.results, canonical.results, "canonical raw capture");
assert.deepEqual(embeddedRaw.results, embedded.results, "embedded raw capture");
assert.equal(sourceComparison.browserResultsIdentical, true);
assert.equal(sourceComparison.caseCount, expected.caseCount);
assert.equal(sourceComparison.fontReadyCanonical, true);
assert.equal(sourceComparison.fontReadyEmbedded, true);

for (const [index, expectedCase] of expected.cases.entries()) {
  assertObservationMatches(canonical.results[index], expectedCase);
  assertObservationMatches(embedded.results[index], expectedCase);
}

assert.equal(
  sourceComparison.canonicalFixtureSha256,
  await sha256("tests/fixtures/css/c11/grid-direct-text-aspect-ratio-min-max-precomparison.html"),
  "canonical fixture SHA-256",
);
assert.equal(
  sourceComparison.embeddedCaptureSha256,
  await sha256("spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/chrome-oracle.html"),
  "embedded capture SHA-256",
);
assert.equal(
  sourceComparison.ahemSha256,
  await sha256("tests/fixtures/resources/c15/Ahem.ttf"),
  "Ahem font SHA-256",
);
for (const [recordedKey, file] of [
  ["canonicalObservationSha256", `${evidenceRelative}/chrome-observation-agent-browser-auto-track.json`],
  ["embeddedObservationSha256", `${evidenceRelative}/chrome-observation-embedded-auto-track.json`],
  ["canonicalRawCaptureSha256", `${evidenceRelative}/chrome-observation-agent-browser-auto-track.raw.json`],
  ["embeddedRawCaptureSha256", `${evidenceRelative}/chrome-observation-embedded-auto-track.raw.json`],
  ["expectedValuesSha256", `${evidenceRelative}/expected-values.json`],
  ["screenshotSha256", `${evidenceRelative}/chrome-oracle-agent-browser-auto-track.png`],
]) {
  assert.equal(sourceComparison[recordedKey], await sha256(file), `${recordedKey} SHA-256`);
}

const mutated = structuredClone(canonical.results[0]);
mutated.ratioItem.height += 1;
assert.throws(
  () => assertObservationMatches(mutated, expected.cases[0]),
  assert.AssertionError,
  "one-pixel geometry mutation must be rejected",
);

console.log(`verified ${expected.caseCount} exact Chrome 156 cases, both font sources, and one geometry negative control`);
