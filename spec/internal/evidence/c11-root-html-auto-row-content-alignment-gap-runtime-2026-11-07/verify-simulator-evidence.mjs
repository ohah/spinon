import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { gunzipSync } from "node:zlib";

const repoRoot = resolve(new URL("../../../../", import.meta.url).pathname);
const evidenceRoot = join(
  repoRoot,
  "spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-runtime-2026-11-07",
);
const expectedScenes = [
  ["pass01", "zero-normal", "normal", "0px"],
  ["pass02", "gap-2_5-space-between", "space-between", "2.5px"],
  ["pass03", "gap-7_5-end", "end", "7.5px"],
  ["pass04", "gap-7_5-center", "center", "7.5px"],
];

function readKeyValues(path) {
  return Object.fromEntries(
    readFileSync(path, "utf8")
      .trim()
      .split("\n")
      .map((line) => {
        const separator = line.indexOf("=");
        return [line.slice(0, separator), line.slice(separator + 1)];
      }),
  );
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

for (const [passName, fixtureName, alignment, rowGap] of expectedScenes) {
  const runRoot = join(evidenceRoot, passName);
  const environment = readKeyValues(join(runRoot, "environment.txt"));
  const fixturePath = join(
    repoRoot,
    `tests/fixtures/css/c11/root-html-auto-row-content-alignment-gap-runtime-${fixtureName}.html`,
  );
  const fixtureBytes = readFileSync(fixturePath);
  const fixtureHash = sha256(fixtureBytes);
  assert.equal(environment.align_content, alignment, `${passName}: alignment`);
  assert.equal(environment.row_gap, rowGap, `${passName}: row-gap`);
  assert.equal(environment.fixture_sha256, fixtureHash, `${passName}: fixture hash`);
  assert.equal(environment.android_result?.startsWith("PASS"), true, `${passName}: Android result`);
  assert.equal(environment.ios_result?.startsWith("PASS"), true, `${passName}: iOS result`);
  assert.equal(environment.asset_and_simulator_restoration, "PASS", `${passName}: restoration`);

  for (const platform of ["android", "ios"]) {
    const platformRoot = join(runRoot, platform);
    const artifact = readKeyValues(join(platformRoot, "artifact-sha256.txt"));
    const platformFixtureHash =
      artifact.android_fixture_sha256 ?? artifact.ios_fixture_sha256;
    assert.equal(platformFixtureHash, fixtureHash, `${passName}/${platform}: embedded fixture`);
    const runtimeLog = gunzipSync(
      readFileSync(join(platformRoot, "runtime.log.gz")),
    ).toString("utf8");
    assert.match(runtimeLog, /SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready/);
    assert.match(runtimeLog, /SPINON_C0410_DRAW .*presented .*runtime_text_count=2/);

    const screenshotPath = join(platformRoot, "simulator.png");
    const screenshot = readFileSync(screenshotPath);
    assert.equal(screenshot.subarray(0, 8).toString("hex"), "89504e470d0a1a0a");
    const expectedDimensions = platform === "android" ? [1080, 2400] : [1206, 2622];
    assert.deepEqual(
      [screenshot.readUInt32BE(16), screenshot.readUInt32BE(20)],
      expectedDimensions,
      `${passName}/${platform}: screenshot dimensions`,
    );
    const screenshotDigest = sha256(screenshot);
    const screenshotRecord = readFileSync(
      join(platformRoot, "screenshot-sha256.txt"),
      "utf8",
    );
    assert.ok(screenshotRecord.startsWith(`${screenshotDigest}  `));
  }

  process.stdout.write(`${passName} ${alignment} row-gap ${rowGap}: PASS ${fixtureHash}\n`);
}
