import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const directory = dirname(fileURLToPath(import.meta.url));
const observationPath = resolve(directory, "chrome155-target-frame-hit-identity-2d.json");
const observationBytes = readFileSync(observationPath);
const observation = JSON.parse(observationBytes.toString("utf8"));
const repositoryRoot = resolve(directory, "../../../..");
const fixturePath = resolve(repositoryRoot, "tests/fixtures/css/c13/overflow-clip-c13-1.html");
const parentObservationPath = resolve(
  repositoryRoot,
  "spec/internal/evidence/c13-1-overflow-clip-hit-identity-2026-10-14/chrome155-element-hit-identity.json",
);
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const expectedIds = [
  "visible",
  "clip-x",
  "clip-y",
  "clip-both",
  "clip-margin-padding",
  "clip-margin-content",
  "clip-margin-border",
  "clip-margin-hidden",
  "mixed-visible-hidden",
  "mixed-clip-auto",
  "positioned-contained",
  "positioned-escape",
  "nested-clip",
];
const knownLabels = new Set(["target", "box", "inner", "root", "main", "body", "html"]);

assert.equal(observation.browser.product, "Chrome for Testing");
assert.equal(observation.browser.version, "155.0.8059.40");
assert.equal(
  observation.browser.revision,
  "cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}",
);
assert.equal(
  observation.browser.userAgent,
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36",
);
assert.deepEqual(observation.browser.viewport, { width: 1280, height: 1100, dpr: 1 });
assert.equal(observation.fixture.sha256, sha256(readFileSync(fixturePath)));
assert.equal(observation.fixture.parentHitIdentitySha256, sha256(readFileSync(parentObservationPath)));
assert.equal(observation.scan.mode, "target-frame-integer-grid-2d");
assert.equal(observation.scan.caseCount, expectedIds.length);
assert.equal(observation.cases.length, expectedIds.length);
assert.deepEqual(observation.cases.map((row) => row.id), expectedIds);

const observedLabels = new Set();
let pointCount = 0;
for (const row of observation.cases) {
  assert.equal(row.targetId, `${row.id}-target`);
  assert.equal(row.xStart, Math.floor(row.target.x));
  assert.equal(row.xEnd, Math.ceil(row.target.right) - 1);
  assert.equal(row.yStart, Math.floor(row.target.y));
  assert.equal(row.yEnd, Math.ceil(row.target.bottom) - 1);
  assert.ok(row.xEnd >= row.xStart, `${row.id} x extent must be nonempty`);
  assert.ok(row.yEnd >= row.yStart, `${row.id} y extent must be nonempty`);
  assert.equal(row.rows.length, row.yEnd - row.yStart + 1);

  row.rows.forEach((scanline, index) => {
    assert.equal(scanline.y, row.yStart + index, `${row.id} y rows must be continuous`);
    assert.ok(scanline.runs.length > 0, `${row.id} y=${scanline.y} must contain a hit`);
    let nextX = row.xStart;
    for (const run of scanline.runs) {
      assert.equal(run.from, nextX, `${row.id} y=${scanline.y} x runs must be continuous`);
      assert.ok(run.to >= run.from, `${row.id} y=${scanline.y} run must be nonempty`);
      assert.ok(knownLabels.has(run.hit), `${row.id} unknown identity ${run.hit}`);
      observedLabels.add(run.hit);
      pointCount += run.to - run.from + 1;
      nextX = run.to + 1;
    }
    assert.equal(nextX, row.xEnd + 1, `${row.id} y=${scanline.y} must cover the target frame`);
  });
  assert.equal(
    row.rows.length * (row.xEnd - row.xStart + 1),
    row.rows.reduce(
      (sum, scanline) => sum + scanline.runs.reduce((lineSum, run) => lineSum + run.to - run.from + 1, 0),
      0,
    ),
    `${row.id} RLE sample count must equal its integer frame area`,
  );
}

assert.equal(pointCount, 187_320);
assert.equal(observation.scan.pointCount, pointCount);
assert.equal(observation.scan.expectedPointCount, pointCount);
assert.deepEqual(observation.scan.uniqueIdentities, [...observedLabels].sort());
console.log(
  `PASS: ${expectedIds.length} rows, ${pointCount} integer points, identities=${[...observedLabels].sort().join(",")}, observation_sha256=${sha256(observationBytes)}`,
);
