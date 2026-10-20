import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';

const toleranceCssPx = 1 / 64;
const baselineDirectory = resolve(
  'spec/internal/evidence/c11-4-auto-fit-min-size-axis-2026-10-20/'
    + 'android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f',
);
const baseline = JSON.parse(await readFile(resolve(baselineDirectory, 'result.json'), 'utf8'));
const physicalResultPath = process.argv[2];
assert.ok(physicalResultPath, '사용법: node compare-avd-physical.mjs <physical result.json>');
const physicalResultAbsolute = resolve(physicalResultPath);
const physicalDirectory = dirname(physicalResultAbsolute);
const physical = JSON.parse(await readFile(physicalResultAbsolute, 'utf8'));

function trackEntries(tracks) {
  const entries = new Map();
  const visit = (value, prefix) => {
    if (Array.isArray(value)) {
      value.forEach((track, index) => visit(track, [...prefix, String(index)]));
    } else if (value && typeof value === 'object' && Object.hasOwn(value, 'baseSize')) {
      entries.set(prefix.join('.'), value);
    } else if (value && typeof value === 'object') {
      Object.entries(value).forEach(([key, nested]) => visit(nested, [...prefix, key]));
    }
  };
  visit(tracks, []);
  return entries;
}

function parseNodeFrames(log) {
  const marker = 'SPINON_C114_GRID_NODE_FRAME ';
  return log.split(/\r?\n/).flatMap((line) => {
    const markerOffset = line.indexOf(marker);
    if (markerOffset < 0) return [];
    const frame = line.slice(markerOffset + marker.length).trim();
    const match = frame.match(/^(\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,]+)$/);
    assert.ok(match, `node frame을 읽지 못했습니다: ${frame}`);
    return [{
      order: Number(match[1]),
      node: Number(match[2]),
      x: Number(match[3]),
      y: Number(match[4]),
      width: Number(match[5]),
      height: Number(match[6]),
    }];
  });
}

assert.equal(baseline.observations.length, 53, '기준 행렬 scene 수가 53이 아닙니다.');
assert.equal(physical.observations.length, 53, '실기기 행렬 scene 수가 53이 아닙니다.');
const baselineByID = new Map(baseline.observations.map((observation) => [observation.sceneId, observation]));
const physicalByID = new Map(physical.observations.map((observation) => [observation.sceneId, observation]));
assert.equal(new Set(physical.observations.map(({ sceneId }) => sceneId)).size, 53,
  '실기기 결과에 scene ID 중복이 있습니다.');
assert.deepEqual([...physicalByID.keys()].sort(), [...baselineByID.keys()].sort(),
  '실기기와 AVD scene ID 집합이 다릅니다.');

let maxTrackDeltaCssPx = 0;
let maxFrameDeltaCssPx = 0;
const scenes = [];
let pixelSampleCount = 0;
for (const sceneId of baselineByID.keys()) {
  const avd = baselineByID.get(sceneId);
  const device = physicalByID.get(sceneId);
  const avdTracks = trackEntries(avd.tracks);
  const deviceTracks = trackEntries(device.tracks);
  assert.deepEqual([...deviceTracks.keys()].sort(), [...avdTracks.keys()].sort(),
    `${sceneId}: AVD와 실기기 track 목록이 다릅니다.`);
  let sceneTrackDelta = 0;
  for (const [key, avdTrack] of avdTracks) {
    const deviceTrack = deviceTracks.get(key);
    assert.equal(deviceTrack.collapsed, avdTrack.collapsed,
      `${sceneId} ${key}: collapse 상태가 AVD와 다릅니다.`);
    const delta = Math.abs(deviceTrack.baseSize - avdTrack.baseSize);
    sceneTrackDelta = Math.max(sceneTrackDelta, delta);
  }
  assert.ok(sceneTrackDelta <= toleranceCssPx,
    `${sceneId}: track 차이 ${sceneTrackDelta} CSS px가 허용치를 넘습니다.`);
  maxTrackDeltaCssPx = Math.max(maxTrackDeltaCssPx, sceneTrackDelta);

  const avdLog = await readFile(resolve(baselineDirectory, `runtime-${sceneId}.log`), 'utf8');
  const deviceLog = await readFile(resolve(physicalDirectory, `runtime-${sceneId}.log`), 'utf8');
  const avdFrames = parseNodeFrames(avdLog);
  const deviceFrames = device.observedNodeFrames;
  assert.ok(Array.isArray(deviceFrames), `${sceneId}: 관측된 실기기 frame이 결과 JSON에 없습니다.`);
  assert.equal(deviceFrames.length, avdFrames.length, `${sceneId}: node frame 수가 다릅니다.`);
  let sceneFrameDelta = 0;
  for (const [index, avdFrame] of avdFrames.entries()) {
    const deviceFrame = deviceFrames[index];
    assert.equal(deviceFrame.order, avdFrame.order, `${sceneId}: node frame 순서가 다릅니다.`);
    assert.equal(deviceFrame.node, avdFrame.node, `${sceneId}: node ID가 다릅니다.`);
    for (const property of ['x', 'y', 'width', 'height']) {
      sceneFrameDelta = Math.max(sceneFrameDelta,
        Math.abs(deviceFrame[property] - avdFrame[property]));
    }
  }
  assert.ok(sceneFrameDelta <= toleranceCssPx,
    `${sceneId}: frame 차이 ${sceneFrameDelta} CSS px가 허용치를 넘습니다.`);
  maxFrameDeltaCssPx = Math.max(maxFrameDeltaCssPx, sceneFrameDelta);

  const pixelSamples = deviceLog.split(/\r?\n/).filter((line) =>
    line.includes('SPINON_C114_GRID_PIXEL_MATRIX_SAMPLE '));
  const pixelSummaries = deviceLog.split(/\r?\n/).filter((line) =>
    line.includes('SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY '));
  assert.ok(pixelSamples.every((line) => line.includes('exact=true')),
    `${sceneId}: 표면 pixel 표본에 불일치가 있습니다.`);
  assert.ok(pixelSummaries.every((line) => line.includes('status=matched')),
    `${sceneId}: 표면 pixel summary가 불일치입니다.`);
  pixelSampleCount += pixelSamples.length;
  scenes.push({ sceneId, trackCount: deviceTracks.size, frameCount: deviceFrames.length,
    maxTrackDeltaCssPx: sceneTrackDelta, maxFrameDeltaCssPx: sceneFrameDelta,
    pixelSampleCount: pixelSamples.length });
}

const report = {
  schemaVersion: 1,
  baseline: baselineDirectory,
  physicalResult: resolve(physicalResultPath),
  device: JSON.parse(await readFile(resolve(physicalDirectory, 'device.json'), 'utf8')),
  sceneCount: scenes.length,
  toleranceCssPx,
  maxTrackDeltaCssPx,
  maxFrameDeltaCssPx,
  surfacePixelSampleCount: pixelSampleCount,
  surfacePixelSamplesExact: pixelSampleCount > 0,
  scenes,
};
assert.equal(report.device.qemu, '0', '결과 기기가 실기기가 아닙니다.');
assert.equal(report.device.model, 'SM-S731N');
assert.equal(report.surfacePixelSamplesExact, true, 'surface pixel 표본이 없습니다.');
await writeFile(resolve(physicalDirectory, 'physical-vs-avd-comparison.json'),
  `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({
  sceneCount: report.sceneCount,
  maxTrackDeltaCssPx,
  maxFrameDeltaCssPx,
  toleranceCssPx,
  surfacePixelSampleCount: pixelSampleCount,
}, null, 2));
