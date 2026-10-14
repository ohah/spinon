import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const dir = "spec/internal/evidence/c13-1-overflow-clip-full-viewport-hit-identity-2026-10-14";
const readJson = (path) => JSON.parse(readFileSync(path, "utf8"));
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const ids = [
  "visible", "clip-x", "clip-y", "clip-both", "clip-margin-padding",
  "clip-margin-content", "clip-margin-border", "clip-margin-hidden",
  "mixed-visible-hidden", "mixed-clip-auto", "positioned-contained",
  "positioned-escape", "nested-clip",
];
const fixture = readFileSync("tests/fixtures/css/c13/overflow-clip-c13-1.html");
const targetFrameObservation = readJson("spec/internal/evidence/c13-1-overflow-clip-hit-identity-2d-2026-10-14/chrome155-target-frame-hit-identity-2d.json");
const observationBytes = readFileSync(`${dir}/chrome155-full-viewport-hit-identity.json`);
const observation = JSON.parse(observationBytes);
assert.equal(sha256(fixture), "b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a");
assert.equal(observation.fixture.sha256, "b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a");
assert.equal(observation.fixture.targetFrameHitIdentity2dSha256, "248c9d15a56ef332428b0c1e341bac1b1ebe5ce6ea3ac7d01c95a95bd787fe5b");
assert.equal(observation.browser.version, "155.0.8059.40");
assert.equal(observation.browser.revision, "cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}");
assert.equal(observation.browser.userAgent, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36");
assert.deepEqual(observation.browser.viewport, { width: 1280, height: 1100, dpr: 1, scrollX: 0, scrollY: 0 });
assert.equal(observation.scan.mode, "full-viewport-integer-grid-2d-combined-c131-page");
assert.equal(observation.scan.identityProjection, "each valid case section, box, target and nested inner keeps its case id; invalid/float rows and other page elements map to runtime viewport root; viewport-exterior null controls are preserved separately");
assert.equal(observation.scan.caseCount, ids.length);
assert.equal(observation.scan.pointCount, 1280 * 1100);
assert.equal(observation.cases.length, ids.length);
assert.equal(observation.rows.length, 1100);
assert.deepEqual(observation.cases.map((row) => row.id), ids);
assert.deepEqual(targetFrameObservation.cases.map((row) => row.id), ids);

const actualIdentityCounts = Object.create(null);
let pointCount = 0;
for (const [rowIndex, row] of observation.rows.entries()) {
  assert.equal(row.y, rowIndex, `y row ${rowIndex}`);
  let nextX = 0;
  for (const run of row.runs) {
    assert.equal(run.from, nextX, `x run gap at y=${rowIndex}`);
    assert.ok(run.to >= run.from && run.to < 1280, `invalid run at y=${rowIndex}: ${JSON.stringify(run)}`);
    const identity = run.identity;
    assert.ok(identity === "root" || identity === "null" || ids.some((id) => ["case", "box", "target", "inner"].some((kind) => identity === `${id}:${kind}`)), `unknown identity ${identity}`);
    const length = run.to - run.from + 1;
    pointCount += length;
    actualIdentityCounts[identity] = (actualIdentityCounts[identity] || 0) + length;
    nextX = run.to + 1;
  }
  assert.equal(nextX, 1280, `x runs do not cover viewport row ${rowIndex}`);
}
assert.equal(pointCount, 1280 * 1100);
assert.deepEqual(JSON.parse(JSON.stringify(actualIdentityCounts)), observation.scan.identityCounts);
assert.deepEqual([...new Set(Object.keys(actualIdentityCounts))].sort(), observation.scan.uniqueIdentities);

const nullControls = [
  ["viewport-left-outside", -1, 20],
  ["viewport-top-outside", 100, -1],
  ["viewport-right-outside", 1280, 20],
  ["viewport-bottom-outside", 100, 1100],
];
assert.equal(observation.offViewportNullControls.length, nullControls.length);
for (const [index, [id, x, y]] of nullControls.entries()) {
  assert.deepEqual(observation.offViewportNullControls[index], {
    id,
    xCssPx: x,
    yCssPx: y,
    identity: "null",
    elementId: null,
    tagName: null,
  });
}

for (const id of ids) {
  const captured = observation.cases.find((row) => row.id === id);
  const prior = targetFrameObservation.cases.find((row) => row.id === id);
  assert.equal(captured.targetId, prior.targetId, `${id} target id`);
  for (const field of ["x", "y", "width", "height", "right", "bottom"]) {
    assert.equal(captured.target[field], prior.target[field], `${id} target ${field}`);
  }
  if (id === "nested-clip") assert.ok(captured.inner, "nested inner frame");
  else assert.equal(captured.inner, null, `${id} unexpected inner frame`);
}

process.stdout.write(JSON.stringify({
  status: "matched",
  browser: observation.browser.version,
  fixtureSha256: sha256(fixture),
  caseCount: ids.length,
  pointCount,
  rowCount: observation.rows.length,
  runCount: observation.rows.reduce((sum, row) => sum + row.runs.length, 0),
  identities: Object.keys(actualIdentityCounts).sort(),
  observationSha256: sha256(observationBytes),
}) + "\n");
