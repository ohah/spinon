import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const evidenceDirectory = new URL('.', import.meta.url);
const observation = JSON.parse(readFileSync(new URL('chrome156-observation.json', evidenceDirectory), 'utf8'));
const agentBrowserObservation = JSON.parse(readFileSync(new URL('agent-browser-pass02-observation.json', evidenceDirectory), 'utf8'));
const expected = JSON.parse(readFileSync(new URL('spec-derived-expectations.json', evidenceDirectory), 'utf8'));
const fixture = readFileSync(new URL('../../../../tests/fixtures/css/c11/grid-auto-repeat-zero-percent-minimum-c114.html', evidenceDirectory));
const fixtureSha256 = createHash('sha256').update(fixture).digest('hex');
assert.equal(fixtureSha256, observation.fixture.sha256, 'fixture source hash changed after browser observation');
assert.equal(agentBrowserObservation.fixturePath, observation.fixture.path, 'Agent Browser fixture path mismatch');
assert.equal(observation.browser.agentBrowser, '0.31.1', 'Chrome observation agent-browser version mismatch');
assert.match(observation.browser.userAgent, /HeadlessChrome\/156\.0\.0\.0/, 'Chrome observation browser version mismatch');
assert.equal(agentBrowserObservation.userAgent, observation.browser.userAgent, 'Agent Browser user agent mismatch');
assert.deepEqual(agentBrowserObservation.viewportCssPx, observation.browser.viewportCssPx, 'Agent Browser viewport mismatch');
assert.equal(agentBrowserObservation.devicePixelRatio, observation.browser.devicePixelRatio, 'Agent Browser device pixel ratio mismatch');
const expectedIds = expected.scenes.map(scene => scene.id);
assert.equal(new Set(expectedIds).size, expectedIds.length, 'duplicate expected scene id');

function indexScenesByUniqueId(scenes, label) {
  const scenesById = new Map();
  for (const scene of scenes) {
    assert.ok(typeof scene.id === 'string' && scene.id.length > 0, `${label}: missing scene id`);
    assert.ok(!scenesById.has(scene.id), `${label}: duplicate scene ${scene.id}`);
    scenesById.set(scene.id, scene);
  }
  assert.deepEqual([...scenesById.keys()].sort(), [...expectedIds].sort(), `${label}: scene ids`);
  return scenesById;
}

const observedById = indexScenesByUniqueId(observation.scenes, 'Chrome 156');
const agentBrowserById = indexScenesByUniqueId(agentBrowserObservation.scenes, 'Agent Browser pass02');

function assertMatchesSpecification(actual, scene, label) {
  assert.equal(actual.totalTrackCount, scene.totalTrackCount, `${label}: total track count`);
  assert.equal(Object.values(actual.trackHistogram).reduce((sum, count) => sum + Number(count), 0),
    scene.totalTrackCount, `${label}: track histogram count`);
  if (scene.repeatTrackCount !== undefined) {
    assert.equal(actual.repeatTrackCount ?? actual.totalTrackCount, scene.repeatTrackCount, `${label}: repeated track count`);
  }
  if (scene.contentAxisCssPx !== undefined) {
    assert.equal(actual.contentAxisCssPx, scene.contentAxisCssPx, `${label}: content basis`);
  }
  if (scene.expectedTracks) {
    assert.deepEqual(actual.trackHistogram, scene.expectedTracks, `${label}: exact track histogram`);
  }
  if (scene.expectedTrackSizeCssPx !== undefined) {
    assert.equal(Object.keys(actual.trackHistogram).length, 1, `${label}: expected uniform repeated tracks`);
    const [trackText, countText] = Object.entries(actual.trackHistogram)[0];
    const trackSize = Number.parseFloat(trackText);
    assert.equal(Number(countText), scene.repeatTrackCount, `${label}: uniform repeated count`);
    assert.ok(Math.abs(trackSize - scene.expectedTrackSizeCssPx) <= scene.actualChromeQuantizationToleranceCssPx,
      `${label}: track size ${trackSize} differs from independent value ${scene.expectedTrackSizeCssPx}`);
  }
  if (Object.hasOwn(scene, 'expectedItemSizeCssPx')) {
    if (scene.expectedItemSizeCssPx === null) {
      assert.equal(actual.itemRectCssPx, null, `${label}: unexpected item rect`);
      return;
    }
    assert.ok(Array.isArray(actual.itemRectCssPx), `${label}: item rect is missing`);
    assert.equal(actual.itemRectCssPx.length, 4, `${label}: item rect shape`);
    assert.deepEqual(actual.itemRectCssPx.slice(2), scene.expectedItemSizeCssPx, `${label}: item size`);
  }
}

for (const scene of expected.scenes) {
  assertMatchesSpecification(observedById.get(scene.id), scene, `Chrome 156 ${scene.id}`);
  assertMatchesSpecification(agentBrowserById.get(scene.id), scene, `Agent Browser pass02 ${scene.id}`);
}
console.log(JSON.stringify({
  chrome156Passed: expected.scenes.length,
  agentBrowserPass02Passed: expected.scenes.length,
  failed: 0,
  fixtureSha256,
}));
