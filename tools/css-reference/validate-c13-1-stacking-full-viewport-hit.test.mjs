import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { validateCapture } from "./validate-c13-1-stacking-full-viewport-hit.mjs";

const evidence = "spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18";
const [oracleText, fixtureBytes, screenshotBytes] = await Promise.all([
  readFile(`${evidence}/chrome-full-viewport-hit.json`, "utf8"),
  readFile("tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html"),
  readFile(`${evidence}/chrome155-stacking-full-viewport.png`),
]);
const oracle = JSON.parse(oracleText);

function expectRejected(mutator, pattern) {
  const changed = structuredClone(oracle);
  mutator(changed);
  assert.throws(
    () => validateCapture({ oracle: changed, fixtureBytes, screenshotBytes }),
    pattern,
  );
}

test("pinned Chrome full viewport capture has complete canonical RLE coverage", () => {
  const summary = validateCapture({ oracle, fixtureBytes, screenshotBytes });
  assert.equal(summary.pointCount, 69_120);
  assert.equal(summary.rowCount, 128);
  assert.equal(summary.runCount, 1_136);
  assert.equal(summary.expandedNodeIndexSha256, oracle.expandedNodeIndexSha256);
});

test("rejects changed fixture hash", () => {
  expectRejected((changed) => { changed.fixtureSha256 = "0".repeat(64); }, /fixture hash mismatch/);
});

test("rejects changed fixture source bytes", () => {
  const changedFixture = Buffer.from(fixtureBytes);
  changedFixture[0] ^= 1;
  assert.throws(
    () => validateCapture({ oracle, fixtureBytes: changedFixture, screenshotBytes }),
    /fixture hash mismatch/,
  );
});

test("rejects a different Chrome build", () => {
  expectRejected((changed) => {
    changed.browser.fullVersionList[0].version = "155.0.8059.84";
  }, /Chrome full version changed/);
});

test("rejects a different browser platform", () => {
  expectRejected((changed) => { changed.browser.platformVersion = "26.5.0"; }, /macOS version changed/);
});

test("rejects a changed device scale factor", () => {
  expectRejected((changed) => { changed.viewport.devicePixelRatio = 2; }, /viewport or DPR changed/);
});

test("rejects a scrolled capture", () => {
  expectRejected((changed) => { changed.viewport.scrollY = 1; }, /page scroll changed/);
});

test("rejects a pixel-center convention substitution", () => {
  expectRejected((changed) => {
    changed.queryCoordinateConvention = "CSS pixel centers";
  }, /coordinate convention changed/);
});

test("rejects a reordered or duplicated node registry", () => {
  expectRejected((changed) => { changed.nodeIds.reverse(); }, /node ID order changed/);
  expectRejected((changed) => { changed.nodeIds[2] = changed.nodeIds[1]; }, /node ID order changed/);
});

test("rejects a changed total point count", () => {
  expectRejected((changed) => { changed.pointCount -= 1; }, /point count changed/);
});

test("rejects a missing viewport row", () => {
  expectRejected((changed) => { changed.rows.pop(); }, /row count changed/);
});

test("rejects an under-covered row", () => {
  expectRejected((changed) => { changed.rows[20][0][1] -= 1; }, /gap or overlap/);
});

test("rejects an over-covered row", () => {
  expectRejected((changed) => { changed.rows[20][0][1] += 1; }, /exceeds viewport width|gap or overlap/);
});

test("rejects a zero-width run", () => {
  expectRejected((changed) => { changed.rows[20][0][1] = 0; }, /invalid run width/);
});

test("rejects an unknown node index", () => {
  expectRejected((changed) => { changed.rows[20][0][0] = 13; }, /unregistered node index/);
});

test("rejects noncanonical adjacent runs for one identity", () => {
  expectRejected((changed) => {
    const [nodeIndex, width] = changed.rows[20][0];
    changed.rows[20].splice(0, 1, [nodeIndex, 1], [nodeIndex, width - 1]);
  }, /noncanonical adjacent run/);
});

test("rejects a modified expanded identity digest", () => {
  expectRejected((changed) => { changed.expandedNodeIndexSha256 = "0".repeat(64); }, /digest mismatch/);
});

test("rejects a missing or changed right-edge positive control", () => {
  expectRejected((changed) => { changed.edgeControls.splice(1, 1); }, /edge controls changed/);
  expectRejected((changed) => { changed.edgeControls[1].hitId = null; }, /edge controls changed/);
});

test("rejects a missing or changed fractional-edge null control", () => {
  expectRejected((changed) => { changed.edgeControls.splice(4, 1); }, /edge controls changed/);
  expectRejected((changed) => { changed.edgeControls[4].hitId = "root"; }, /edge controls changed/);
});

test("rejects incorrect screenshot dimensions or bytes", () => {
  expectRejected((changed) => { changed.screenshot.width = 539; }, /screenshot dimensions changed/);
  const changedScreenshot = Buffer.from(screenshotBytes);
  changedScreenshot[0] ^= 1;
  assert.throws(
    () => validateCapture({ oracle, fixtureBytes, screenshotBytes: changedScreenshot }),
    /screenshot hash mismatch/,
  );
});
