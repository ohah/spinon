import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { test } from "node:test";

const manifestUrl = new URL("../../spec/internal/evidence/c13-1-overflow-clip-wpt-classification-2026-10-14/classification.json", import.meta.url);
const manifest = JSON.parse(readFileSync(manifestUrl, "utf8"));
const expectedSource = {
  commit: "a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7",
  commitDateUtc: "2026-10-14T10:07:55Z",
  commitUrl: "https://github.com/web-platform-tests/wpt/tree/a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7",
  directoryUrl: "https://github.com/web-platform-tests/wpt/tree/a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7/css/css-overflow",
  inventorySha256: "de618ed3e3d8bc67afc86528c33950bd7a31a6d7c9981ebf9ddc599c5365ff4a",
  classificationSha256: "b271e4e1fb7330d3ff9aa7c1f9b7cfa24f7a49364b04c3d257b1b1722ed641f0",
};
const expectedCounts = {
  spinon_fixture_candidate: 13,
  chrome_oracle_only: 17,
  unsupported: 28,
};

function sha256(value) {
  return createHash("sha256").update(JSON.stringify(value)).digest("hex");
}

function validateClassification(value) {
  assert.equal(value.source.commit, expectedSource.commit);
  assert.equal(value.source.commitDateUtc, expectedSource.commitDateUtc);
  assert.equal(value.source.commitUrl, expectedSource.commitUrl);
  assert.equal(value.source.directoryUrl, expectedSource.directoryUrl);
  assert.equal(value.source.inventorySha256, expectedSource.inventorySha256);
  assert.equal(value.source.classificationSha256, expectedSource.classificationSha256);
  assert.equal(value.wptExecution, "not-run");
  assert.equal(value.classifications.length, 58);
  const paths = value.classifications.map(item => item.path);
  assert.equal(value.sourceInventory.length, 58);
  assert.equal(new Set(value.sourceInventory).size, value.sourceInventory.length);
  assert.equal(new Set(paths).size, paths.length);
  assert.deepEqual([...paths].sort(), [...value.sourceInventory].sort());
  assert.equal(sha256([...value.sourceInventory].sort()), expectedSource.inventorySha256);
  const rows = value.classifications
    .map(item => [item.path, item.category, item.reason])
    .sort((left, right) => left[0] < right[0] ? -1 : left[0] > right[0] ? 1 : 0);
  assert.equal(sha256(rows), expectedSource.classificationSha256);
  assert.ok(paths.every(path => path.startsWith("css/css-overflow/") && path.endsWith(".html")));
  assert.ok(paths.every(path => !path.includes("-ref.html")));
  const counts = Object.fromEntries(Object.keys(expectedCounts).map(category => [
    category,
    value.classifications.filter(item => item.category === category).length,
  ]));
  assert.deepEqual(counts, expectedCounts);
  assert.deepEqual(counts, value.counts);
  assert.ok(value.classifications.every(item => item.reason.trim().length >= 24));
}

test("C13.1 WPT source, labels, digests, and unrun status are pinned", () => {
  validateClassification(manifest);
});

test("inventory 검증은 중복, 누락, 미등록 경로와 개수 보존 재분류를 거부한다", () => {
  const duplicate = structuredClone(manifest);
  duplicate.classifications.push(duplicate.classifications[0]);
  assert.throws(() => validateClassification(duplicate));

  const missing = structuredClone(manifest);
  missing.classifications.pop();
  assert.throws(() => validateClassification(missing));

  const invalidCategory = structuredClone(manifest);
  invalidCategory.classifications[0].category = "pass";
  assert.throws(() => validateClassification(invalidCategory));

  const unlistedReplacement = structuredClone(manifest);
  unlistedReplacement.classifications[0].path = "css/css-overflow/unlisted.html";
  assert.throws(() => validateClassification(unlistedReplacement));

  const balancedRelabel = structuredClone(manifest);
  const candidate = balancedRelabel.classifications.find(item => item.category === "spinon_fixture_candidate");
  const oracle = balancedRelabel.classifications.find(item => item.category === "chrome_oracle_only");
  [candidate.category, oracle.category] = [oracle.category, candidate.category];
  assert.throws(() => validateClassification(balancedRelabel));

  const changedInventory = structuredClone(manifest);
  changedInventory.sourceInventory[0] = "css/css-overflow/unlisted.html";
  changedInventory.classifications[0].path = "css/css-overflow/unlisted.html";
  assert.throws(() => validateClassification(changedInventory));
});

test("inventory 검증은 WPT 실행 허위 표기를 거부한다", () => {
  const claimedPass = structuredClone(manifest);
  claimedPass.wptExecution = "passed";
  assert.throws(() => validateClassification(claimedPass));
});
