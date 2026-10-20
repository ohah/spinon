import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

const here = new URL("./", import.meta.url);
const readJson = async name => JSON.parse(await readFile(new URL(name, here), "utf8"));
const [expected, observed, environment] = await Promise.all([
  readJson("spec-derived-expectations.json"),
  readJson("chrome155-observation.json"),
  readJson("browser-environment.json"),
]);
const runtimeScenes = JSON.parse(await readFile(new URL(
  "../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/negative_implicit_track_scenes.json",
  import.meta.url,
), "utf8"));
const tolerance = expected.cssPixelTolerance;
const actualById = new Map(observed.map(row => [row.id, row]));

assert.equal(observed.length, expected.trackExpectations.length, "관찰 case 수가 기대 case 수와 다릅니다.");
assert.equal(actualById.size, observed.length, "관찰 결과에 중복 case id가 있습니다.");
assert.equal(runtimeScenes.length, expected.trackExpectations.length, "runtime scene 수가 기대 case 수와 다릅니다.");
assert.deepEqual(new Set(runtimeScenes.map(({ id }) => id)), new Set(expected.trackExpectations.map(({ id }) => id)),
  "runtime scene과 track expectation ID가 다릅니다.");
assert.deepEqual(new Set(runtimeScenes.map(({ id }) => id)), new Set(observed.map(({ id }) => id)),
  "runtime scene과 Chrome 관찰 ID가 다릅니다.");
assert.equal(environment.caseCount, expected.trackExpectations.length, "브라우저에 열린 fixture case 수가 다릅니다.");
assert.match(environment.userAgent, /HeadlessChrome\/155\./, "Chrome 155 외 환경의 관찰값입니다.");
assert.equal(environment.devicePixelRatio, 1, "DPR 1 기준 관찰이 아닙니다.");
assert.equal(environment.ahemLoaded, true, "Ahem 글꼴이 로드되지 않았습니다.");

function parseTrackList(value) {
  return value.split(/\s+/).map(token => {
    assert.match(token, /^-?(?:\d+\.?\d*|\.\d+)px$/, `지원하지 않는 computed track 값: ${token}`);
    return Number.parseFloat(token);
  });
}

function equalNumbers(actual, wanted, label) {
  assert.equal(actual.length, wanted.length, `${label}: track/좌표 수가 다릅니다 (${actual.length} != ${wanted.length}).`);
  actual.forEach((value, index) => assert.ok(
    Math.abs(value - wanted[index]) <= tolerance,
    `${label}[${index}]: ${value} != ${wanted[index]} CSS px (허용 오차 ${tolerance}).`,
  ));
}

for (const want of expected.trackExpectations) {
  const got = actualById.get(want.id);
  assert.ok(got, `관찰 결과에서 case를 찾지 못했습니다: ${want.id}`);
  equalNumbers(parseTrackList(got.gridTemplateColumns), want.columns, `${want.id} columns`);
  equalNumbers(parseTrackList(got.gridTemplateRows), want.rows, `${want.id} rows`);
}

for (const want of expected.geometryExpectations) {
  const got = actualById.get(want.id);
  assert.ok(got, `geometry case를 찾지 못했습니다: ${want.id}`);
  if (want.items) {
    for (const item of want.items) {
      const rect = got.items[item.index].rect;
      equalNumbers(
        [rect.x - got.grid.x, rect.y - got.grid.y, rect.width, rect.height],
        item.relativeRect,
        `${want.id} item ${item.index + 1} relative rect`,
      );
    }
  }
  if (want.fixedGridWidth !== undefined) {
    assert.equal(got.grid.width, want.fixedGridWidth, `${want.id} root width`);
    const item = got.items.at(-1).rect;
    const beyond = item.x + item.width - got.grid.x - got.grid.width;
    assert.ok(Math.abs(beyond - want.lastItemRightBeyondGrid) <= tolerance, `${want.id} right overflow ${beyond}px`);
  }
  if (want.fixedGridHeight !== undefined) {
    assert.equal(got.grid.height, want.fixedGridHeight, `${want.id} root height`);
    const item = got.items.at(-1).rect;
    const beyond = item.y + item.height - got.grid.y - got.grid.height;
    assert.ok(Math.abs(beyond - want.lastItemBottomBeyondGrid) <= tolerance, `${want.id} bottom overflow ${beyond}px`);
  }
  if (want.display) assert.equal(got.items[want.hiddenItemIndex].display, want.display, `${want.id} hidden item`);
}

console.log(`통과: Chrome 155 관찰/runtime scene ${observed.length}/${expected.trackExpectations.length}, 트랙·자동배치·overflow 기대값 일치 (±${tolerance} CSS px).`);
