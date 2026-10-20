import assert from 'node:assert/strict';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = dirname(fileURLToPath(import.meta.url));
const androidDirectory = join(
  evidenceDirectory,
  'android-avd-c114-negative-implicit-android-avd-20261020T125416760Z-ef57f92f',
);
const iosDirectory = join(
  evidenceDirectory,
  'ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712',
);
const android = JSON.parse(readFileSync(join(androidDirectory, 'result.json'), 'utf8'));
const ios = JSON.parse(readFileSync(join(iosDirectory, 'result.json'), 'utf8'));
const toleranceCssPx = 1 / 64;

assert.equal(android.target, 'android-avd');
assert.equal(ios.target, 'ios-simulator');
assert.equal(android.sceneCount, 138);
assert.equal(ios.sceneCount, 138);
assert.deepEqual(
  android.observations.map(({ sceneId }) => sceneId),
  ios.observations.map(({ sceneId }) => sceneId),
);

const framePattern = /(?:^|;)(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g;

function nodeFrames(value) {
  return [...value.matchAll(framePattern)].map((match) => ({
    order: Number(match[1]),
    node: Number(match[2]),
    x: Number(match[3]),
    y: Number(match[4]),
    width: Number(match[5]),
    height: Number(match[6]),
  }));
}

function usedTracks(value, output = []) {
  if (Array.isArray(value)) {
    for (const item of value) usedTracks(item, output);
  } else if (value && typeof value === 'object') {
    if (typeof value.baseSize === 'number' && typeof value.collapsed === 'boolean') {
      output.push(value);
    } else {
      for (const item of Object.values(value)) usedTracks(item, output);
    }
  }
  return output;
}

let framesCompared = 0;
let tracksCompared = 0;
let maxFrameDeltaCssPx = 0;
let maxTrackDeltaCssPx = 0;

for (let index = 0; index < android.observations.length; index += 1) {
  const androidScene = android.observations[index];
  const iosScene = ios.observations[index];
  assert.equal(androidScene.drawPresented, true, `${androidScene.sceneId}: Android draw`);
  assert.equal(iosScene.drawPresented, true, `${iosScene.sceneId}: iOS draw`);

  const androidFrames = nodeFrames(androidScene.nodeFrames);
  const iosFrames = nodeFrames(iosScene.nodeFrames);
  assert.equal(androidFrames.length, iosFrames.length, `${androidScene.sceneId}: frame count`);
  for (let frameIndex = 0; frameIndex < androidFrames.length; frameIndex += 1) {
    const androidFrame = androidFrames[frameIndex];
    const iosFrame = iosFrames[frameIndex];
    assert.equal(androidFrame.order, iosFrame.order, `${androidScene.sceneId}: frame order`);
    assert.equal(androidFrame.node, iosFrame.node, `${androidScene.sceneId}: frame node`);
    for (const property of ['x', 'y', 'width', 'height']) {
      const delta = Math.abs(androidFrame[property] - iosFrame[property]);
      maxFrameDeltaCssPx = Math.max(maxFrameDeltaCssPx, delta);
      assert.ok(delta <= toleranceCssPx, `${androidScene.sceneId}: ${property} delta ${delta}`);
      framesCompared += 1;
    }
  }

  const androidTracks = usedTracks(androidScene.tracks);
  const iosTracks = usedTracks(iosScene.tracks);
  assert.equal(androidTracks.length, iosTracks.length, `${androidScene.sceneId}: track count`);
  for (let trackIndex = 0; trackIndex < androidTracks.length; trackIndex += 1) {
    const androidTrack = androidTracks[trackIndex];
    const iosTrack = iosTracks[trackIndex];
    assert.equal(androidTrack.collapsed, iosTrack.collapsed, `${androidScene.sceneId}: collapse`);
    const delta = Math.abs(androidTrack.baseSize - iosTrack.baseSize);
    maxTrackDeltaCssPx = Math.max(maxTrackDeltaCssPx, delta);
    assert.ok(delta <= toleranceCssPx, `${androidScene.sceneId}: track delta ${delta}`);
    tracksCompared += 1;
  }
}

function pixelScenes(directory, marker) {
  return readdirSync(directory)
    .filter((name) => /^runtime-.*\.log$/.test(name))
    .flatMap((name) => {
      const lines = readFileSync(join(directory, name), 'utf8').split(/\r?\n/);
      const line = lines.find((value) => value.includes(marker) && value.includes('status=matched'));
      if (!line) return [];
      const scene = line.match(/scene=([^ ]+)/)?.[1];
      const expected = Number(line.match(/expected_samples=(\d+)/)?.[1]);
      const observed = Number(line.match(/observed_samples=(\d+)/)?.[1]);
      const matched = Number(line.match(/matched_samples=(\d+)/)?.[1]);
      assert.ok(expected === 4 || expected === 5, `${name}: unexpected sample count`);
      assert.equal(observed, expected, `${name}: observed sample count`);
      assert.equal(matched, expected, `${name}: matched sample count`);
      return [{ scene, samples: matched }];
    })
    .sort((left, right) => left.scene.localeCompare(right.scene));
}

const androidPixelScenes = pixelScenes(androidDirectory, 'SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY');
const iosPixelScenes = pixelScenes(iosDirectory, 'SPINON_C114_METAL_PIXEL_SUMMARY');
assert.deepEqual(androidPixelScenes, iosPixelScenes);
assert.equal(androidPixelScenes.length, 5);

const comparison = {
  schemaVersion: 1,
  androidRun: androidDirectory.split('/').at(-1),
  iosRun: iosDirectory.split('/').at(-1),
  scenes: 138,
  sharedSceneIDs: true,
  framesCompared,
  maxFrameDeltaCssPx,
  frameToleranceCssPx: toleranceCssPx,
  tracksCompared,
  maxTrackDeltaCssPx,
  trackToleranceCssPx: toleranceCssPx,
  autoFitCollapseStatesMatched: true,
  drawPresented: '138/138 per platform',
  pixelScenes: androidPixelScenes,
};

const outputPath = join(evidenceDirectory, 'cross-platform-comparison-20261020.json');
writeFileSync(outputPath, `${JSON.stringify(comparison, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(comparison, null, 2)}\n`);
