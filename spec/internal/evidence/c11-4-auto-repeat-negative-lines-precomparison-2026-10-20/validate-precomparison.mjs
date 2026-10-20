import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const observation = JSON.parse(readFileSync(resolve(here, "chrome155-observation.json"), "utf8"));
const expected = JSON.parse(readFileSync(resolve(here, "spec-derived-expectations.json"), "utf8"));

assert.match(observation.environment.userAgent, /HeadlessChrome\/155\./);
assert.equal(observation.environment.dpr, 1);
assert.deepEqual(observation.environment.viewport, { width: 1280, height: 633 });
assert.equal(observation.environment.colorScheme, "light");
assert.equal(observation.cases.length, expected.cases.length);

const observations = new Map(observation.cases.map(item => [item.id, item]));
assert.equal(observations.size, expected.cases.length, "관찰 case ID가 중복됐습니다");

function trackSizes(serialized) {
  const values = [...serialized.matchAll(/(-?\d+(?:\.\d+)?)px/g)].map(match => Number(match[1]));
  assert.ok(values.length > 0, `computed track list를 읽을 수 없습니다: ${serialized}`);
  return values;
}

function lineBoundary(cssLine, expandedExplicitTrackCount) {
  assert.notEqual(cssLine, 0, "CSS Grid line 0은 유효하지 않습니다");
  return cssLine > 0 ? cssLine - 1 : expandedExplicitTrackCount + 1 + cssLine;
}

function close(actual, expectedValue, label) {
  assert.ok(
    Math.abs(actual - expectedValue) <= 1 / 64,
    `${label}: actual=${actual}, expected=${expectedValue}`,
  );
}

for (const expectation of expected.cases) {
  const actual = observations.get(expectation.id);
  assert.ok(actual, `Chrome observation 누락: ${expectation.id}`);

  for (const axis of ["columns", "rows"]) {
    const axisExpected = expectation[axis];
    if (!axisExpected) continue;

    const model = expected.repeatCountModel[axis];
    const gapCountBeforeRepeat = axisExpected.prefix + axisExpected.suffix;
    const firstRepeatSize =
      axisExpected.prefix * model.prefixTrack +
      axisExpected.suffix * model.suffixTrack +
      model.minimumTrack +
      gapCountBeforeRepeat * model.gap;
    const perAdditionalRepeat = model.minimumTrack + model.gap;
    const derivedRepeatCount = Math.max(
      1,
      Math.floor((axisExpected.available - firstRepeatSize) / perAdditionalRepeat) + 1,
    );
    assert.equal(derivedRepeatCount, axisExpected.repeatCount, `${expectation.id} ${axis}: independent repeat formula`);

    const inputTemplate = actual.input[axis === "columns" ? "gridTemplateColumns" : "gridTemplateRows"];
    assert.ok(inputTemplate.includes(`repeat(${axisExpected.mode},`), `${expectation.id} ${axis}: mode/input mismatch`);

    const property = axis === "columns" ? "gridTemplateColumns" : "gridTemplateRows";
    const tracks = trackSizes(actual[property]);
    const explicitTrackCount = axisExpected.prefix + axisExpected.repeatCount + axisExpected.suffix;
    const expectedTrackCount = axisExpected.leading + explicitTrackCount + axisExpected.trailing;
    assert.equal(tracks.length, expectedTrackCount, `${expectation.id} ${axis}: expanded and implicit track count`);

    for (const [index, size] of axisExpected.leadingSizes.entries()) {
      close(tracks[index], size, `${expectation.id} ${axis}: leading track ${index}`);
    }
    for (const [index, size] of (axisExpected.trailingSizes ?? []).entries()) {
      close(tracks[tracks.length - axisExpected.trailing + index], size, `${expectation.id} ${axis}: trailing track ${index}`);
    }

    const repeatStart = axisExpected.leading + axisExpected.prefix;
    for (let index = 0; index < axisExpected.repeatCount; index += 1) {
      const size = tracks[repeatStart + index];
      if ((axisExpected.collapsed ?? []).includes(index)) {
        close(size, 0, `${expectation.id} ${axis}: collapsed auto-fit repeat ${index}`);
      } else {
        assert.ok(size >= model.minimumTrack, `${expectation.id} ${axis}: repeat ${index} below definite minimum`);
      }
    }

    const dimension = axis === "columns" ? "width" : "height";
    close(actual.grid[dimension], axisExpected.available, `${expectation.id} ${axis}: definite available size`);

    const lineProperty = axis === "columns" ? "gridColumn" : "gridRow";
    const frameProperty = axis === "columns" ? "x" : "y";
    for (const [index, line] of axisExpected.lines.entries()) {
      const [cssStart, cssEnd, expectedStart, expectedEnd] = line;
      const start = lineBoundary(cssStart, explicitTrackCount);
      const end = lineBoundary(cssEnd, explicitTrackCount);
      assert.equal(start, expectedStart, `${expectation.id} ${axis}: start line derived from expanded explicit grid`);
      assert.equal(end, expectedEnd, `${expectation.id} ${axis}: end line derived from expanded explicit grid`);

      const placement = actual.items[index];
      assert.ok(placement, `${expectation.id} ${axis}: item ${index} missing`);
      const serializedPlacement = placement[lineProperty];
      const placementMatch = serializedPlacement.match(/^\s*(-?\d+)\s*\/\s*(-?\d+)\s*$/);
      assert.ok(placementMatch, `${expectation.id} ${axis}: unexpected computed placement ${serializedPlacement}`);
      assert.equal(Number(placementMatch[1]), cssStart, `${expectation.id} ${axis}: computed start line`);
      assert.equal(Number(placementMatch[2]), cssEnd, `${expectation.id} ${axis}: computed end line`);

      // auto-fill은 gutter를 유지한다. auto-fit frame은 Chrome 관찰로만 보존하고 규범 기대값과 분리한다.
      if (axisExpected.mode === "auto-fill") {
        const normalizedStart = Math.min(start, end);
        const trackIndex = axisExpected.leading + normalizedStart;
        assert.ok(trackIndex >= 0 && trackIndex < tracks.length, `${expectation.id} ${axis}: start outside used tracks`);
        const offset = tracks.slice(0, trackIndex).reduce((sum, size) => sum + size, 0) + trackIndex * model.gap;
        close(placement.rect[frameProperty], actual.grid[frameProperty] + offset, `${expectation.id} ${axis}: item starts on resolved expanded-grid line`);
      }
    }
  }
}

console.log(`PASS: CSS Grid independent expectations and Chrome 155 observations match for ${expected.cases.length} cases`);
