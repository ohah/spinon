import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const target = process.argv[2];
const serial = process.argv[3] ?? 'emulator-5554';
const simulatorUDID = process.argv[3] ?? 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-4-auto-repeat-neighbors-simulator.mjs android-avd|ios-simulator [AVD serial|iOS Simulator UDID]');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const runPrefix = `c114-negative-implicit-${target}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-4-negative-implicit-track-sizing-${date}`, `${target}-${runPrefix}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

const scenes = [
  {
    id: 'auto-fill-fixed-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=10,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=10,height=40]',
    tracks: [20, 46, 46, 46, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fit-fixed-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=162,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=162,height=40]',
    tracks: [20, 162, 0, 0, 10],
    collapsed: [false, false, true, true, false],
  },
  {
    id: 'auto-fill-pattern-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=10,height=40',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=10,height=40]',
    tracks: [20, 45, 20, 45, 20, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fit-span-neighbors',
    axis: 'Columns',
    boxCount: 3,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=12,height=12;2:node=3,x=190,y=0,width=12,height=12]',
    tracks: [20, 77, 77, 0, 10],
    collapsed: [false, false, false, true, false],
  },
  {
    id: 'auto-fit-empty-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=200,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60]',
    tracks: [20, 0, 0, 0, 10],
    collapsed: [false, true, true, true, false],
  },
  {
    id: 'auto-fill-prefix-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=28,y=0,width=12,height=12]',
    tracks: [20, 52, 52, 52],
    collapsed: [false, false, false, false],
  },
  {
    id: 'auto-fill-suffix-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=200,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;1:node=2,x=190,y=0,width=12,height=12]',
    tracks: [55.328125, 55.328125, 55.328125, 10],
    collapsed: [false, false, false, false],
  },
  {
    id: 'auto-fill-indefinite-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=86,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=86,height=60;1:node=2,x=76,y=0,width=12,height=12]',
    tracks: [20, 40, 10],
    collapsed: [false, false, false],
  },
  {
    id: 'auto-fill-max-size-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=182,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=182,height=60]',
    tracks: [20, 40, 40, 40, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fill-min-size-neighbors',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=230,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=230,height=60]',
    tracks: [20, 40, 40, 40, 40, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-overflow-neighbors',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=60',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;1:node=2,x=86,y=0,width=12,height=12]',
    tracks: [30, 40, 20],
    collapsed: [false, false, false],
    pixelSamples: [
      { id: 'overflow-root-edge-inside', nodeId: 'c114-auto-fill-overflow-neighbors-grid', xCssPx: 39, yCssPx: 8, rgba: [34, 197, 94, 255] },
      { id: 'overflow-root-edge-outside', nodeId: 'c114-auto-fill-overflow-neighbors-grid', xCssPx: 40, yCssPx: 8, rgba: [0, 0, 0, 255] },
      { id: 'overflow-item-outside-root', nodeId: 'c114-auto-fill-overflow-neighbors-item', xCssPx: 91, yCssPx: 8, rgba: [229, 72, 77, 255] },
      { id: 'overflow-root-background', nodeId: 'c114-auto-fill-overflow-neighbors-grid', xCssPx: 10, yCssPx: 30, rgba: [34, 197, 94, 255] },
    ],
  },
  {
    id: 'auto-fill-row-neighbors',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=50,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=200;1:node=2,x=0,y=190,width=12,height=12]',
    tracks: [20, 46, 46, 46, 10],
    collapsed: [false, false, false, false, false],
  },
  {
    id: 'auto-fit-row-neighbors',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=50,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=200;1:node=2,x=0,y=28,width=12,height=12]',
    tracks: [20, 162, 0, 0, 10],
    collapsed: [false, false, true, true, false],
  },
  {
    id: 'auto-fill-two-track-columns-both-190',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=190,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=190,height=60]',
    tracks: [20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-columns-both-200',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=266,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=266,height=60]',
    tracks: [20, 40, 20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-columns-both-342',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=342,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=342,height=60]',
    tracks: [20, 40, 20, 40, 20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-columns-prefix-200',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=248,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=248,height=60]',
    tracks: [20, 40, 20, 40, 20, 40, 20],
    collapsed: [false, false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-columns-suffix-200',
    axis: 'Columns',
    boxCount: 1,
    rootFrame: 'width=238,height=60',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=238,height=60]',
    tracks: [40, 20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-rows-both-190',
    axis: 'Rows',
    boxCount: 1,
    rootFrame: 'width=50,height=190',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=190]',
    tracks: [20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-rows-both-200',
    axis: 'Rows',
    boxCount: 1,
    rootFrame: 'width=50,height=266',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=266]',
    tracks: [20, 40, 20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false, false, false],
  },
  {
    id: 'auto-fill-two-track-rows-both-342',
    axis: 'Rows',
    boxCount: 1,
    rootFrame: 'width=50,height=342',
    itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=342]',
    tracks: [20, 40, 20, 40, 20, 40, 20, 40, 20, 10],
    collapsed: [false, false, false, false, false, false, false, false, false, false],
  },
  {
    id: 'columns-fill-control', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=28,y=28,width=12,height=12]',
    axes: { Columns: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] } },
  },
  {
    id: 'rows-fill-control', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=28,y=28,width=12,height=12]',
    axes: { Rows: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] } },
  },
  {
    id: 'both-fill-square', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=28,y=28,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
      Rows: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
    },
  },
  {
    id: 'both-fill-asymmetric', boxCount: 2, rootFrame: 'width=200,height=266',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=266;1:node=2,x=28,y=28,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
      Rows: { tracks: [20, 40, 20, 40, 20, 40, 20, 10], collapsed: [false, false, false, false, false, false, false, false] },
    },
  },
  {
    id: 'both-fit-empty', boxCount: 1, rootFrame: 'width=200,height=200', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200]',
    axes: {
      Columns: { tracks: [20, 0, 0, 0, 0, 10], collapsed: [false, true, true, true, true, false] },
      Rows: { tracks: [20, 0, 0, 0, 0, 10], collapsed: [false, true, true, true, true, false] },
    },
  },
  {
    id: 'both-fit-single', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=28,y=28,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 162, 0, 0, 0, 10], collapsed: [false, false, true, true, true, false] },
      Rows: { tracks: [20, 162, 0, 0, 0, 10], collapsed: [false, false, true, true, true, false] },
    },
  },
  {
    id: 'both-fit-span-two', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=28,y=28,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 134, 20, 0, 0, 10], collapsed: [false, false, false, true, true, false] },
      Rows: { tracks: [20, 134, 20, 0, 0, 10], collapsed: [false, false, false, true, true, false] },
    },
  },
  {
    id: 'both-fit-second-repeat', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=20,y=20,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 0, 0, 170, 0, 10], collapsed: [false, true, true, false, true, false] },
      Rows: { tracks: [20, 0, 0, 170, 0, 10], collapsed: [false, true, true, false, true, false] },
    },
  },
  {
    id: 'columns-fit-rows-fill-empty', boxCount: 1, rootFrame: 'width=200,height=200', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200]',
    axes: {
      Columns: { tracks: [20, 0, 0, 0, 0, 10], collapsed: [false, true, true, true, true, false] },
      Rows: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
    },
  },
  {
    id: 'columns-fill-rows-fit-empty', boxCount: 1, rootFrame: 'width=200,height=200', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200]',
    axes: {
      Columns: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
      Rows: { tracks: [20, 0, 0, 0, 0, 10], collapsed: [false, true, true, true, true, false] },
    },
  },
  {
    id: 'columns-fit-rows-fill-second-repeat', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=20,y=109,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 0, 0, 170, 0, 10], collapsed: [false, true, true, false, true, false] },
      Rows: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
    },
  },
  {
    id: 'columns-fill-rows-fit-second-repeat', boxCount: 2, rootFrame: 'width=200,height=200',
    itemFrame: 'width=12,height=12',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;1:node=2,x=109,y=20,width=12,height=12]',
    axes: {
      Columns: { tracks: [20, 45, 20, 45, 20, 10], collapsed: [false, false, false, false, false, false] },
      Rows: { tracks: [20, 0, 0, 170, 0, 10], collapsed: [false, true, true, false, true, false] },
    },
  },
];
const baselineAutoRepeatSceneIDs = [
  'auto-fill-fixed-neighbors', 'auto-fit-fixed-neighbors', 'auto-fill-pattern-neighbors',
  'auto-fit-span-neighbors', 'auto-fit-empty-neighbors', 'auto-fill-prefix-neighbors',
  'auto-fill-suffix-neighbors', 'auto-fill-indefinite-neighbors', 'auto-fill-max-size-neighbors',
  'auto-fill-min-size-neighbors', 'auto-fill-overflow-neighbors', 'auto-fill-row-neighbors',
  'auto-fit-row-neighbors', 'auto-fill-two-track-columns-both-190',
  'auto-fill-two-track-columns-both-200', 'auto-fill-two-track-columns-both-342',
  'auto-fill-two-track-columns-prefix-200', 'auto-fill-two-track-columns-suffix-200',
  'auto-fill-two-track-rows-both-190', 'auto-fill-two-track-rows-both-200',
  'auto-fill-two-track-rows-both-342', 'columns-fill-control', 'rows-fill-control',
  'both-fill-square', 'both-fill-asymmetric', 'both-fit-empty', 'both-fit-single',
  'both-fit-span-two', 'both-fit-second-repeat', 'columns-fit-rows-fill-empty',
  'columns-fill-rows-fit-empty', 'columns-fit-rows-fill-second-repeat',
  'columns-fill-rows-fit-second-repeat',
];
assert.deepEqual(scenes.map(({ id }) => id), baselineAutoRepeatSceneIDs,
  'C11.4 기존 auto-repeat 33 scene ID가 변경됐습니다.');
scenes.push(...autoFitMinimumSizeScenes());
assert.equal(scenes.length, 53, 'C11.4 auto-fit minimum-size simulator 행렬은 53 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
scenes.push(...implicitIntrinsicTrackScenes());
assert.equal(scenes.length, 73, 'C11.4 intrinsic implicit track simulator 행렬은 73 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
scenes.push(...await negativeImplicitTrackScenes());
assert.equal(scenes.length, 106, 'C11.4 negative implicit track simulator 행렬은 106 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
scenes.push(...await autoRepeatNegativeLineScenes());
assert.equal(scenes.length, 124, 'C11.4 auto-repeat negative line simulator 행렬은 124 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');

async function negativeImplicitTrackScenes() {
  const evidence = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-negative-implicit-track-sizing-precomparison-2026-10-20');
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/negative_implicit_track_scenes.json');
  const [fixtures, expectations, observations] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'spec-derived-expectations.json'), 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'chrome155-observation.json'), 'utf8').then(JSON.parse),
  ]);
  const tracksByID = new Map(expectations.trackExpectations.map((entry) => [entry.id, entry]));
  const observationsByID = new Map(observations.map((entry) => [entry.id, entry]));
  const fixtureIDs = fixtures.map(({ id }) => id);
  assert.equal(fixtures.length, 33, 'negative implicit track fixture는 33 case여야 합니다.');
  assert.equal(tracksByID.size, 33, 'negative implicit track expectation은 33 case여야 합니다.');
  assert.equal(observationsByID.size, 33, 'Chrome 관찰은 33 case여야 합니다.');
  assert.deepEqual(new Set(fixtureIDs), new Set(tracksByID.keys()),
    'runtime fixture와 규범-derived track expectation scene이 다릅니다.');
  assert.deepEqual(new Set(fixtureIDs), new Set(observationsByID.keys()),
    'runtime fixture와 Chrome 관찰 scene이 다릅니다.');

  return fixtures.map(({ id, items: fixtureItems }) => {
    const trackExpectation = tracksByID.get(id);
    const observation = observationsByID.get(id);
    const visibleItems = observation.items.filter((item) => item.display !== 'none');
    const firstVisibleItem = visibleItems[0];
    assert.ok(firstVisibleItem, `${id}: visible first item이 없습니다.`);
    const rootFrame = `width=${observation.grid.width},height=${observation.grid.height}`;
    const itemFrame = `width=${firstVisibleItem.rect.width},height=${firstVisibleItem.rect.height}`;
    const itemNodeFrames = observation.items.map((item, itemIndex) => {
      const sourceIndex = observation.items.indexOf(item);
      assert.ok(sourceIndex >= 0, `${id}: Chrome item ${item.id}를 runtime fixture에서 못 찾았습니다.`);
      const precedingTextNodes = fixtureItems.slice(0, sourceIndex)
        .filter((fixtureItem, index) => Boolean(fixtureItem.text)
          && observation.items[index]?.display !== 'none').length;
      const x = item.display === 'none' ? 0 : item.rect.x - observation.grid.x;
      const y = item.display === 'none' ? 0 : item.rect.y - observation.grid.y;
      return `${itemIndex + 1}:node=${sourceIndex + precedingTextNodes + 2},x=${x},y=${y},`
        + `width=${item.rect.width},height=${item.rect.height}`;
    });
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};`
      + `${itemNodeFrames.join(';')}]`;
    const scene = {
      id,
      boxCount: visibleItems.filter((item) => item.rect.width > 0 && item.rect.height > 0).length + 1,
      rootFrame,
      itemFrame,
      nodeFrames,
      axes: {
        Columns: {
          tracks: trackExpectation.columns,
          collapsed: trackExpectation.columns.map(() => false),
        },
        Rows: {
          tracks: trackExpectation.rows,
          collapsed: trackExpectation.rows.map(() => false),
        },
      },
      gaps: {
        Columns: Number.parseFloat(observation.columnGap),
        Rows: Number.parseFloat(observation.rowGap),
      },
    };
    if (id === 'columns-negative-fixed-root-overflow') {
      scene.pixelSamples = [
        { id: 'negative-root-interior', nodeId: `c114-${id}-grid`, xCssPx: 33, yCssPx: 10, rgba: [226, 232, 240, 255] },
        { id: 'negative-root-edge', nodeId: `c114-${id}-grid`, xCssPx: 0, yCssPx: 10, rgba: [226, 232, 240, 255] },
        { id: 'negative-root-exterior', nodeId: `c114-${id}-grid`, xCssPx: 150, yCssPx: 10, rgba: [0, 0, 0, 255] },
        { id: 'negative-overflow-item', nodeId: `c114-${id}-item-${observation.items.length}`, xCssPx: 250, yCssPx: 10, rgba: [141, 95, 211, 255] },
        { id: 'negative-overflow-tail', nodeId: `c114-${id}-grid`, xCssPx: 273, yCssPx: 10, rgba: [0, 0, 0, 255] },
      ];
    } else if (id === 'rows-negative-fixed-root-overflow') {
      scene.pixelSamples = [
        { id: 'negative-root-interior', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 19, rgba: [226, 232, 240, 255] },
        { id: 'negative-root-edge', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 0, rgba: [226, 232, 240, 255] },
        { id: 'negative-root-exterior', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 100, rgba: [0, 0, 0, 255] },
        { id: 'negative-overflow-item', nodeId: `c114-${id}-item-${observation.items.length}`, xCssPx: 10, yCssPx: 195, rgba: [141, 95, 211, 255] },
        { id: 'negative-overflow-tail', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 209, rgba: [0, 0, 0, 255] },
      ];
    }
    return scene;
  });
}

async function autoRepeatNegativeLineScenes() {
  const evidence = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-negative-lines-precomparison-2026-10-20');
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_line_scenes.json');
  const [fixtures, expectations, observations] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'spec-derived-expectations.json'), 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'chrome155-observation.json'), 'utf8').then(JSON.parse),
  ]);
  const expectationsByID = new Map(expectations.cases.map((entry) => [entry.id, entry]));
  const observationsByID = new Map(observations.cases.map((entry) => [entry.id, entry]));
  const fixtureIDs = fixtures.map(({ id }) => id);
  assert.equal(fixtures.length, 18, 'auto-repeat negative line fixture는 18 case여야 합니다.');
  assert.equal(expectationsByID.size, 18, '독립 auto-repeat 기대값은 18 case여야 합니다.');
  assert.equal(observationsByID.size, 18, 'Chrome 관찰은 18 case여야 합니다.');
  assert.deepEqual(new Set(fixtureIDs), new Set(expectationsByID.keys()),
    'runtime fixture와 규범-derived expectation scene이 다릅니다.');
  assert.deepEqual(new Set(fixtureIDs), new Set(observationsByID.keys()),
    'runtime fixture와 Chrome 관찰 scene이 다릅니다.');

  const parseTrackSizes = (serialized) => {
    const tracks = [...serialized.matchAll(/(-?\d+(?:\.\d+)?)px/g)]
      .map((match) => Number(match[1]));
    assert.ok(tracks.length > 0, `computed track list를 읽을 수 없습니다: ${serialized}`);
    return tracks;
  };
  const gapBetween = (left, right, leftCollapsed, rightCollapsed, gap) =>
    leftCollapsed || rightCollapsed ? 0 : gap;
  const close = (actual, expected, label) => assert.ok(
    Math.abs(actual - expected) <= 1 / 64,
    `${label}: Chrome=${actual}, 규범 계산=${expected}`,
  );

  return fixtures.map(({ id, items: fixtureItems }) => {
    const expectation = expectationsByID.get(id);
    const observation = observationsByID.get(id);
    assert.equal(observation.items.length, fixtureItems.length,
      `${id}: runtime fixture와 Chrome item 수가 다릅니다.`);
    const axes = {};
    const itemFrames = observation.items.map((item) => ({
      x: item.rect.x - observation.grid.x,
      y: item.rect.y - observation.grid.y,
      width: item.rect.width,
      height: item.rect.height,
    }));
    const gaps = {
      Columns: Number.parseFloat(observation.columnGap),
      Rows: Number.parseFloat(observation.rowGap),
    };

    for (const [axis, key] of [['Columns', 'columns'], ['Rows', 'rows']]) {
      const axisExpectation = expectation[key] ?? null;
      const templateProperty = key === 'columns' ? 'gridTemplateColumns' : 'gridTemplateRows';
      const trackSizes = parseTrackSizes(observation[templateProperty]);
      const collapsed = trackSizes.map(() => false);
      if (axisExpectation) {
        const expectedTrackCount = axisExpectation.leading + axisExpectation.prefix
          + axisExpectation.repeatCount + axisExpectation.suffix + axisExpectation.trailing;
        assert.equal(trackSizes.length, expectedTrackCount,
          `${id} ${axis}: explicit/implicit track count differs from the independent model`);
        const repeatStart = axisExpectation.leading + axisExpectation.prefix;
        for (const repeatIndex of axisExpectation.collapsed ?? []) {
          collapsed[repeatStart + repeatIndex] = true;
        }
        for (const [index, size] of (axisExpectation.leadingSizes ?? []).entries()) {
          close(trackSizes[index], size, `${id} ${axis}: leading implicit track ${index + 1}`);
        }
        for (const [index, size] of (axisExpectation.trailingSizes ?? []).entries()) {
          close(trackSizes[trackSizes.length - axisExpectation.trailing + index], size,
            `${id} ${axis}: trailing implicit track ${index + 1}`);
        }

        if (axisExpectation.mode === 'auto-fit') {
          const flexibleRepeats = Array.from(
            { length: axisExpectation.repeatCount }, (_, index) => repeatStart + index,
          ).filter((index) => !collapsed[index]);
          if (flexibleRepeats.length > 0) {
            const minimumTrack = expectations.repeatCountModel[key].minimumTrack;
            const fixedTrackTotal = trackSizes.reduce((sum, size, index) =>
              sum + (flexibleRepeats.includes(index) ? 0 : size), 0);
            let nonCollapsedGapTotal = 0;
            for (let index = 0; index + 1 < trackSizes.length; index += 1) {
              nonCollapsedGapTotal += gapBetween(trackSizes[index], trackSizes[index + 1],
                collapsed[index], collapsed[index + 1], gaps[axis]);
            }
            const flexSpace = Math.max(0, axisExpectation.available - fixedTrackTotal
              - nonCollapsedGapTotal - minimumTrack * flexibleRepeats.length);
            const flexTrackSize = minimumTrack + flexSpace / flexibleRepeats.length;
            for (const index of flexibleRepeats) trackSizes[index] = flexTrackSize;
          }
        }
      }
      axes[axis] = { tracks: trackSizes, collapsed };

      if (!axisExpectation) continue;
      const lineProperty = key === 'columns' ? 'c' : 'r';
      const frameProperty = key === 'columns' ? 'x' : 'y';
      const extentProperty = key === 'columns' ? 'width' : 'height';
      for (const [itemIndex, placement] of fixtureItems.entries()) {
        const lineMapping = axisExpectation.lines[itemIndex];
        if (!lineMapping) continue;
        const cssLines = placement[lineProperty]?.match(/^\s*(-?\d+)\s*\/\s*(-?\d+)\s*$/);
        assert.ok(cssLines, `${id} ${axis}: fixture item ${itemIndex + 1} has no numeric lines`);
        assert.deepEqual([Number(cssLines[1]), Number(cssLines[2])], lineMapping.slice(0, 2),
          `${id} ${axis}: fixture lines differ from the independent case`);
        const startBoundary = axisExpectation.leading + Math.min(lineMapping[2], lineMapping[3]);
        const endBoundary = axisExpectation.leading + Math.max(lineMapping[2], lineMapping[3]);
        assert.ok(startBoundary >= 0 && endBoundary <= trackSizes.length,
          `${id} ${axis}: resolved placement lies outside the used tracks`);
        let offset = 0;
        for (let index = 0; index < startBoundary; index += 1) {
          offset += trackSizes[index];
          if (index + 1 < trackSizes.length) {
            offset += gapBetween(trackSizes[index], trackSizes[index + 1],
              collapsed[index], collapsed[index + 1], gaps[axis]);
          }
        }
        let extent = 0;
        for (let index = startBoundary; index < endBoundary; index += 1) {
          extent += trackSizes[index];
          if (index + 1 < endBoundary) {
            extent += gapBetween(trackSizes[index], trackSizes[index + 1],
              collapsed[index], collapsed[index + 1], gaps[axis]);
          }
        }
        itemFrames[itemIndex][frameProperty] = offset;
        itemFrames[itemIndex][extentProperty] = extent;
        if (axisExpectation.mode === 'auto-fill') {
          const chromeItem = observation.items[itemIndex];
          close(chromeItem.rect[frameProperty] - observation.grid[frameProperty], offset,
            `${id} ${axis}: auto-fill item start`);
          close(chromeItem.rect[extentProperty], extent, `${id} ${axis}: auto-fill item size`);
        }
      }
    }

    const itemNodeFrames = itemFrames.map((frame, index) =>
      `${index + 1}:node=${index + 2},x=${frame.x},y=${frame.y},width=${frame.width},height=${frame.height}`);
    const rootFrame = `width=${observation.grid.width},height=${observation.grid.height}`;
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};`
      + `${itemNodeFrames.join(';')}]`;
    const firstItem = itemFrames[0];
    const scene = {
      id,
      boxCount: itemFrames.filter((frame) => frame.width > 0 && frame.height > 0).length + 1,
      rootFrame,
      itemFrame: `width=${firstItem.width},height=${firstItem.height}`,
      nodeFrames,
      axes,
      gaps,
      // The pre-comparison's auto-fit geometry is kept as a browser observation. This scene
      // uses collapsed-gutter placement derived from CSS Grid rather than Blink's boundary gap.
      boundaryGutterOverrides: { Columns: null, Rows: null },
    };
    if (id === 'columns-fill-negative-positive-cycle') {
      scene.pixelSamples = [
        { id: 'negative-cycle-root-interior', nodeId: `c114-${id}-grid`, xCssPx: 100, yCssPx: 10, rgba: [226, 232, 240, 255] },
        { id: 'negative-cycle-root-edge', nodeId: `c114-${id}-grid`, xCssPx: 199, yCssPx: 10, rgba: [226, 232, 240, 255] },
        { id: 'negative-cycle-root-exterior', nodeId: `c114-${id}-grid`, xCssPx: 200, yCssPx: 10, rgba: [0, 0, 0, 255] },
        { id: 'negative-cycle-overflow-item', nodeId: `c114-${id}-item-2`, xCssPx: 290, yCssPx: 10, rgba: [42, 157, 143, 255] },
        { id: 'negative-cycle-overflow-tail', nodeId: `c114-${id}-grid`, xCssPx: 319, yCssPx: 10, rgba: [0, 0, 0, 255] },
      ];
    }
    return scene;
  });
}

function autoFitMinimumSizeScenes() {
  const result = [];
  const repeatCount = (minSize, position) => {
    const prefix = position === 'both' || position === 'prefix' ? 20 : 0;
    const suffix = position === 'both' || position === 'suffix' ? 10 : 0;
    let count = 1;
    const usedSpace = (repetitions) => {
      const tracks = 2 * repetitions + Number(prefix > 0) + Number(suffix > 0);
      return prefix + suffix + 60 * repetitions + Math.max(0, tracks - 1) * 8;
    };
    while (usedSpace(count) < minSize) count += 1;
    return count;
  };
  const makeScene = ({ axis, minSize, position = 'both', placement = 'empty', pattern = 'minmax' }) => {
    const label = pattern === 'fixed' ? 'fixed-pattern'
      : placement === 'empty' ? position : placement;
    const id = `auto-fit-min-${axis.toLowerCase()}-${label}-${minSize}`;
    const repetitions = repeatCount(minSize, position);
    const repeatedTrackCount = repetitions * 2;
    const tracks = [];
    const collapsed = [];
    const prefix = position === 'both' || position === 'prefix';
    const suffix = position === 'both' || position === 'suffix';
    if (prefix) {
      tracks.push(20);
      collapsed.push(false);
    }
    for (let index = 0; index < repeatedTrackCount; index += 1) {
      if (placement === 'single' && index === 0) {
        tracks.push(162);
        collapsed.push(false);
      } else if (placement === 'span-two' && index === 0) {
        tracks.push(134);
        collapsed.push(false);
      } else if (placement === 'span-two' && index === 1) {
        tracks.push(20);
        collapsed.push(false);
      } else if (placement === 'second-repeat' && index === 2) {
        tracks.push(170);
        collapsed.push(false);
      } else {
        tracks.push(0);
        collapsed.push(true);
      }
    }
    if (suffix) {
      tracks.push(10);
      collapsed.push(false);
    }
    const rootFrame = axis === 'Columns'
      ? `width=${minSize},height=60`
      : `width=50,height=${minSize}`;
    const hasItem = ['single', 'span-two', 'second-repeat'].includes(placement);
    const itemFrame = hasItem ? 'width=12,height=12' : null;
    const itemPosition = placement === 'second-repeat'
      ? axis === 'Columns' ? 'x=20,y=0' : 'x=0,y=20'
      : axis === 'Columns' ? 'x=28,y=0' : 'x=0,y=28';
    const nodeFrames = hasItem
      ? `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};1:node=2,${itemPosition},width=12,height=12]`
      : `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame}]`;
    return {
      id,
      axis,
      boxCount: hasItem ? 2 : 1,
      rootFrame,
      itemFrame,
      nodeFrames,
      tracks,
      collapsed,
      pattern,
    };
  };

  for (const axis of ['Columns', 'Rows']) {
    for (const minSize of [190, 200, 342]) {
      result.push(makeScene({ axis, minSize }));
    }
    for (const position of ['prefix', 'suffix', 'none']) {
      result.push(makeScene({ axis, minSize: 200, position }));
    }
    for (const placement of ['single', 'span-two', 'second-repeat']) {
      result.push(makeScene({ axis, minSize: 200, placement }));
    }
    result.push(makeScene({ axis, minSize: 200, pattern: 'fixed' }));
  }
  return result;
}

function implicitIntrinsicTrackScenes() {
  const metrics = {
    XX: { minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
    XXXX: { minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
    'XX XXXXXXX': { minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10 },
  };
  const scene = ({ id, axis, width, height, tracks, items, texts, pixelSamples }) => {
    const rootFrame = `width=${width},height=${height}`;
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};`
      + items.map((item, index) => `${index + 1}:node=${index * 2 + 2},x=${item.x},y=${item.y},`
        + `width=${item.width},height=${item.height}`).join(';') + ']';
    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame,
      itemFrame: null,
      nodeFrames,
      tracks,
      collapsed: tracks.map(() => false),
      intrinsicInputs: texts.map((text, index) => ({
        grid: 1,
        node: index * 2 + 2,
        sourceOrder: index,
        kind: 'text',
        fontRegistryRevision: 1,
        ...metrics[text],
      })),
      ...(pixelSamples ? { pixelSamples } : {}),
    };
  };
  const overflowRoot = 'c114-implicit-min-content-overflow-border-grid';
  const overflowSamples = [
    { id: 'overflow-root-border-inside', nodeId: overflowRoot,
      xCssPx: 0, yCssPx: 8, rgba: [16, 24, 32, 255] },
    { id: 'overflow-root-border-covered', nodeId: overflowRoot,
      xCssPx: 43, yCssPx: 8, rgba: [229, 72, 77, 255] },
    { id: 'overflow-root-border-outside', nodeId: overflowRoot,
      xCssPx: 44, yCssPx: 8, rgba: [229, 72, 77, 255] },
    { id: 'overflow-item-outside-root', nodeId: 'c114-implicit-min-content-overflow-border-item-2',
      xCssPx: 91, yCssPx: 8, rgba: [229, 72, 77, 255] },
    { id: 'overflow-after-item', nodeId: overflowRoot,
      xCssPx: 92, yCssPx: 8, rgba: [0, 0, 0, 255] },
  ];
  return [
    scene({ id: 'row-min-content', axis: 'Rows', width: 40, height: 80,
      tracks: [12, 20], texts: ['XX XXXXXXX'],
      items: [{ x: 0, y: 12, width: 40, height: 20 }] }),
    scene({ id: 'row-max-content', axis: 'Rows', width: 40, height: 80,
      tracks: [12, 20], texts: ['XX XXXXXXX'],
      items: [{ x: 0, y: 12, width: 40, height: 20 }] }),
    scene({ id: 'column-min-content', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 70], texts: ['XX XXXXXXX'],
      items: [{ x: 24, y: 0, width: 70, height: 40 }] }),
    scene({ id: 'column-max-content', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 100], texts: ['XX XXXXXXX'],
      items: [{ x: 24, y: 0, width: 100, height: 40 }] }),
    scene({ id: 'row-auto-placement-min-content', axis: 'Rows', width: 40, height: 80,
      tracks: [12, 20], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 0, y: 0, width: 40, height: 12 }, { x: 0, y: 12, width: 40, height: 20 }] }),
    scene({ id: 'column-auto-placement-max-content', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 100], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 100, height: 40 }] }),
    scene({ id: 'row-auto-placement-auto', axis: 'Rows', width: 40, height: 80,
      tracks: [12, 20], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 0, y: 0, width: 40, height: 12 }, { x: 0, y: 12, width: 40, height: 20 }] }),
    scene({ id: 'column-auto-placement-auto', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 100], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 100, height: 40 }] }),
    scene({ id: 'two-implicit-columns-min-content', axis: 'Columns', width: 240, height: 40,
      tracks: [24, 20, 40], texts: ['XX', 'XXXX'],
      items: [{ x: 24, y: 0, width: 20, height: 40 }, { x: 44, y: 0, width: 40, height: 40 }] }),
    scene({ id: 'cycled-intrinsic-columns', axis: 'Columns', width: 240, height: 40,
      tracks: [24, 20, 100, 40], texts: ['XX', 'XX', 'XX XXXXXXX', 'XXXX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 20, height: 40 },
        { x: 44, y: 0, width: 100, height: 40 }, { x: 144, y: 0, width: 40, height: 40 }] }),
    scene({ id: 'cycled-fixed-max-rows', axis: 'Rows', width: 40, height: 100,
      tracks: [12, 10, 20, 10], texts: ['XX', 'XX', 'XX XXXXXXX', 'XXXX'],
      items: [{ x: 0, y: 0, width: 40, height: 12 }, { x: 0, y: 12, width: 40, height: 10 },
        { x: 0, y: 22, width: 40, height: 20 }, { x: 0, y: 42, width: 40, height: 10 }] }),
    scene({ id: 'implicit-percentage-column', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 40], texts: ['XX', 'XX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 40, height: 40 }] }),
    scene({ id: 'implicit-fraction-column', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 136], texts: ['XX', 'XX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 136, height: 40 }] }),
    scene({ id: 'implicit-minmax-column', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 40], texts: ['XX', 'XX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 40, height: 40 }] }),
    scene({ id: 'implicit-fit-content-column', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 70], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 70, height: 40 }] }),
    scene({ id: 'implicit-min-content-overflow-border', axis: 'Columns', width: 44, height: 44,
      tracks: [20, 70], texts: ['XX', 'XX XXXXXXX'],
      items: [{ x: 2, y: 2, width: 20, height: 40 }, { x: 22, y: 2, width: 70, height: 40 }],
      pixelSamples: overflowSamples }),
    scene({ id: 'implicit-calc-column', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 21], texts: ['XX', 'XX'],
      items: [{ x: 0, y: 0, width: 24, height: 40 }, { x: 24, y: 0, width: 21, height: 40 }] }),
    scene({ id: 'explicit-track-control', axis: 'Columns', width: 40, height: 24,
      tracks: [40], texts: ['XX'],
      items: [{ x: 0, y: 0, width: 40, height: 24 }] }),
    scene({ id: 'row-positive-implicit-locked-item', axis: 'Rows', width: 40, height: 80,
      tracks: [12, 20], texts: ['XX XXXXXXX'],
      items: [{ x: 0, y: 12, width: 40, height: 20 }] }),
    scene({ id: 'column-positive-implicit-locked-item', axis: 'Columns', width: 160, height: 40,
      tracks: [24, 70], texts: ['XX XXXXXXX'],
      items: [{ x: 24, y: 0, width: 70, height: 40 }] }),
  ];
}

function command(binary, args, { allowFailure = false, timeout = 120_000, encoding = 'utf8' } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 96 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    const error = new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
    error.result = result;
    throw error;
  }
  return result;
}

function autoFitBoundaryGutters(scene, axis) {
  if (scene.boundaryGutterOverrides
      && Object.hasOwn(scene.boundaryGutterOverrides, axis)) {
    return scene.boundaryGutterOverrides[axis];
  }
  return null;
}

async function saveLog(path, result) {
  await writeFile(resolve(evidenceDirectory, path), `${result.stdout ?? ''}${result.stderr ?? ''}`);
}

function currentAndroidDevice() {
  const connected = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(connected.length, 1, `Android AVD가 연결되어 있지 않습니다: ${serial}`);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  assert.equal(adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(), '1',
    '대상이 Android Emulator가 아닙니다. 실기기는 사용하지 않습니다.');
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  assert.equal(apiLevel, 37, `고정 Android API 37 AVD가 아닙니다: ${apiLevel}`);
  const abi = adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim();
  assert.equal(abi, 'arm64-v8a', `ARM64 Android AVD가 아닙니다: ${abi}`);
  return {
    adb,
    metadata: {
      serial,
      apiLevel,
      abi,
      model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    },
  };
}

function androidLog(adb, pid) {
  const output = adb([
    'logcat', '-d', '-T', '1000', '-v', 'threadtime', `--pid=${pid}`,
    'SpinonBootstrap:V', '*:S',
  ], { timeout: 30_000 }).stdout;
  return output.split(/\r?\n/).filter((line) => {
    const match = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return match && Number(match[1]) === pid;
  }).join('\n');
}

function iosProcessLog(pid) {
  const predicate = 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C114_GRID" '
    + 'OR eventMessage CONTAINS[c] "SPINON_C114_METAL_PIXEL" '
    + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW" '
    + 'OR eventMessage CONTAINS[c] "node_frames_css_px=")';
  const output = command('xcrun', ['simctl', 'spawn', simulatorUDID, 'log', 'show', '--last', '30s',
    '--style', 'syslog', '--predicate', predicate], { timeout: 45_000 }).stdout;
  const processMarker = new RegExp('SpinonBootstrap\\[' + pid + '(?::|\\])');
  return output.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
}

function waitForLog(readLog, scene, runID) {
  const deadline = Date.now() + 150_000;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    const summary = latest.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
    const trace = latest.includes('SPINON_C114_GRID_TRACE_CHUNK');
    const draw = latest.includes('SPINON_C0410_DRAW')
      && latest.includes(`presented boxes=${scene.boxCount}`);
    const pixelSummary = !scene.pixelSamples
      || (target === 'ios-simulator'
        ? latest.includes(`SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`)
        : latest.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`));
    if ((summary && trace && draw && pixelSummary)
        || latest.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || latest.includes('SPINON_C114_GRID_ARGUMENT_ERROR')
        || latest.includes(`SPINON_C114_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
        || latest.includes(`SPINON_C114_METAL_PIXEL_ERROR run_id=${runID}`)
        || latest.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`)) return latest;
    sleep(1_000);
  }
  throw new Error(`${scene.id}: Android/iOS runtime 결과 대기 시간이 초과되었습니다.\n${latest}`);
}

function supersededC114Result(log, runID) {
  const assertion = log.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID} `));
  return assertion?.includes('report=status=-12 ')
    && assertion.includes('op=eval status=0 ');
}

function verifyRuntimeLog(log, scene, runID) {
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  assert.ok(log.includes(summary), `${scene.id}: runtime summary가 없습니다.`);
  assert.ok(log.includes(`layout=ready boxes=${scene.boxCount}`),
    `${scene.id}: visible box 수가 예상과 다릅니다.`);
  assert.ok(log.includes(scene.rootFrame)
    && (scene.itemFrame === null || log.includes(scene.itemFrame)),
    `${scene.id}: root/item frame이 예상과 다릅니다.`);
  assert.ok(log.includes(scene.nodeFrames), `${scene.id}: 전체 node frame이 예상과 다릅니다.`);
  assert.ok(log.includes('SPINON_C114_GRID_TRACE_CHUNK'), `${scene.id}: track-sizing trace가 없습니다.`);
  assert.ok(log.includes('SPINON_C0410_DRAW')
    && log.includes(`presented boxes=${scene.boxCount}`),
    `${scene.id}: 화면 표시 결과가 없습니다.`);
  assert.ok(!log.includes('SPINON_C114_GRID_ASSERTION_ERROR'), `${scene.id}: frame assertion이 실패했습니다.`);
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: 실행 인자가 거부됐습니다.`);
  const traceParts = log.split(/\r?\n/).flatMap((line) => {
    if (!line.includes(`SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `)) return [];
    const match = line.match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  }).sort((left, right) => left.index - right.index);
  assert.ok(traceParts.length > 0 && traceParts.length === traceParts[0].total,
    `${scene.id}: track-sizing trace chunk이 누락됐습니다.`);
  assert.ok(traceParts.every(({ total }) => total === traceParts[0].total),
    `${scene.id}: track-sizing trace chunk 수가 일치하지 않습니다.`);
  const tracePayload = traceParts.map(({ payload }) => payload).join('')
    .replace(/^grid_track_sizing_diagnostics=/, '');
  const diagnostics = JSON.parse(tracePayload);
  if (scene.intrinsicInputs) {
    const intrinsicInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      kind: input.source?.kind,
      fontRegistryRevision: input.source?.fontRegistryRevision,
      minContent: input.source?.minContent,
      minContentHeight: input.source?.minContentHeight,
      maxContent: input.source?.maxContent,
      maxContentHeight: input.source?.maxContentHeight,
    }));
    assert.deepEqual(intrinsicInputs, scene.intrinsicInputs,
      `${scene.id}: text intrinsic input matrix differs from Ahem oracle`);
  }
  const axisSpecs = scene.axes ?? {
    [scene.axis]: { tracks: scene.tracks, collapsed: scene.collapsed },
  };
  const tracksByAxis = {};
  for (const [axis, expected] of Object.entries(axisSpecs)) {
    const axisPass = diagnostics.trackSizingTraces[0].passes
      .filter((pass) => pass.axis === axis).at(-1);
    assert.ok(axisPass, `${scene.id}: ${axis} track-sizing pass가 없습니다.`);
    const finalStageTracks = axisPass.stages.at(-1).tracks;
    const finalTracks = finalStageTracks.filter(({ gutter }) => !gutter);
    const boundaryGutters = autoFitBoundaryGutters(scene, axis);
    assert.deepEqual(finalTracks.map(({ collapsed }) => collapsed), expected.collapsed,
      `${scene.id}: ${axis} collapsed track 상태가 예상과 다릅니다.`);
    assert.equal(finalTracks.length, expected.tracks.length,
      `${scene.id}: ${axis} track 수가 다릅니다.`);
    finalTracks.forEach(({ baseSize }, index) => assert.ok(
      Math.abs(baseSize - expected.tracks[index]) <= 1 / 64,
      `${scene.id}: ${axis} track ${index + 1} 크기 예상 ${expected.tracks[index]}, 실제 ${baseSize}`,
    ));
    if (scene.axes || scene.id.startsWith('auto-fill-two-track-')
        || scene.id.startsWith('auto-fit-min-') || boundaryGutters) {
      const allGutters = finalStageTracks.filter(({ gutter }) => gutter);
      const internalGutters = allGutters.slice(1, -1);
      assert.equal(internalGutters.length, finalTracks.length - 1,
        `${scene.id}: ${axis} 내부 gap 수가 track 사이 개수와 다릅니다.`);
      // A collapsed track collapses the gutters on both of its sides.
      internalGutters.forEach(({ baseSize }, index) => {
        const preservedBoundary = (index === 0 && boundaryGutters?.leading)
          || (index === internalGutters.length - 1 && boundaryGutters?.trailing);
        const expectedSize = preservedBoundary
          ? (scene.gaps?.[axis] ?? 8)
          : expected.collapsed[index] || expected.collapsed[index + 1]
            ? 0 : (scene.gaps?.[axis] ?? 8);
        assert.ok(Math.abs(baseSize - expectedSize) <= 1 / 64,
          `${scene.id}: ${axis} gap ${index + 1} 크기 예상 ${expectedSize}, 실제 ${baseSize}`);
      });
    }
    tracksByAxis[axis] = finalTracks.map(({ baseSize, collapsed }) => ({ baseSize, collapsed }));
  }
  if (scene.pixelSamples) {
    const summaryMarker = target === 'ios-simulator'
      ? `SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`
      : `SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`;
    assert.ok(log.includes(summaryMarker), `${scene.id}: overflow pixel 요약이 통과하지 않았습니다.`);
    for (const sample of scene.pixelSamples) {
      const sampleLine = log.split(/\r?\n/)
        .find((line) => line.includes(`sample=${sample.id}`));
      assert.ok(sampleLine?.includes('exact=true'),
        `${scene.id}: ${sample.id} surface 표본이 정확히 일치하지 않습니다.`);
    }
  }
  return {
    sceneId: scene.id,
    summary: log.split(/\r?\n/).find((line) => line.includes(summary)),
    nodeFrames: scene.nodeFrames,
    tracks: scene.axes ? tracksByAxis : tracksByAxis[scene.axis],
    trackTraceChunks: traceParts.length,
    drawPresented: true,
  };
}

function pixelCaptureArguments(scene, runID, platform) {
  if (!scene.pixelSamples) return [];
  const json = JSON.stringify(scene.pixelSamples);
  if (platform === 'ios-simulator') {
    const encoded = Buffer.from(json).toString('base64');
    return [
      `--spinon-c11-grid-metal-surface-pixels=${runID}`,
      `--spinon-c11-grid-metal-surface-samples=${encoded}`,
    ];
  }
  const encoded = Buffer.from(json).toString('base64url');
  return [
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', encoded,
  ];
}

if (process.argv.includes('--list-scenes')) {
  console.log(JSON.stringify({ sceneCount: scenes.length, sceneIDs: scenes.map(({ id }) => id) }, null, 2));
  process.exit(0);
}

await mkdir(evidenceDirectory, { recursive: true });
const observations = [];

if (target === 'ios-simulator') {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator를 찾지 못했습니다: ${simulatorUDID}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  assert.equal(simulator[0].name, 'iPhone 17 Pro');
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulatorUDID, appPath]));
  const appSha256 = sha256(await readFile(resolve(appPath, 'SpinonBootstrap')));
  for (const [index, scene] of scenes.entries()) {
    let runID = `${runPrefix}-${index}`;
    let log = '';
    let supersededAttempts = 0;
    for (let attempt = 0; attempt <= 3; attempt++) {
      runID = attempt === 0 ? `${runPrefix}-${index}` : `${runPrefix}-${index}-retry-${attempt}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process', simulatorUDID,
        packageName, '--spinon-c114-grid-track-sizing', `--spinon-c114-grid-run-id=${runID}`,
        `--spinon-c114-grid-scene=${scene.id}`,
        ...pixelCaptureArguments(scene, runID, target)]);
      await saveLog(`launch-${scene.id}${attempt === 0 ? '' : `-retry-${attempt}`}.log`, launch);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: launch PID가 없습니다.`);
      log = waitForLog(() => iosProcessLog(pid), scene, runID);
      if (!supersededC114Result(log, runID)) break;
      supersededAttempts++;
      await writeFile(resolve(evidenceDirectory,
        `runtime-${scene.id}-superseded-${attempt + 1}.log`), `${log}\n`);
    }
    assert.ok(!supersededC114Result(log, runID),
      `${scene.id}: 새 host 재시도 후에도 eval 결과가 superseded 상태입니다.`);
    await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
    const observation = verifyRuntimeLog(log, scene, runID);
    observation.supersededRetries = supersededAttempts;
    observations.push(observation);
    console.log(`${target} C11.4 scene ${index + 1}/${scenes.length}: ${scene.id} passed`);
    await saveLog(`screenshot-${scene.id}.log`, command('xcrun', ['simctl', 'io', simulatorUDID,
      'screenshot', resolve(evidenceDirectory, `${scene.id}.png`)]));
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target, udid: simulatorUDID, name: simulator[0].name, runtime: simulator[0].runtime, appSha256,
  }, null, 2)}\n`);
} else {
  const { adb, metadata } = currentAndroidDevice();
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  const apkBytes = await readFile(apkPath);
  await saveLog('install.log', adb(['install', '-r', apkPath]));
  for (const [index, scene] of scenes.entries()) {
    let runID = `${runPrefix}-${index}`;
    let log = '';
    let supersededAttempts = 0;
    for (let attempt = 0; attempt <= 3; attempt++) {
      runID = attempt === 0 ? `${runPrefix}-${index}` : `${runPrefix}-${index}-retry-${attempt}`;
      const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n', component,
        '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.id,
        '--es', 'spinon_c114_grid_run_id', runID,
        ...pixelCaptureArguments(scene, runID, target)]);
      await saveLog(`launch-${scene.id}${attempt === 0 ? '' : `-retry-${attempt}`}.log`, launch);
      const deadline = Date.now() + 150_000;
      let pid = null;
      log = '';
      while (Date.now() < deadline) {
        pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
          .stdout.trim().split(/\s+/)[0]) || null;
        if (pid) log = androidLog(adb, pid);
        const summary = log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
        const trace = log.includes('SPINON_C114_GRID_TRACE_CHUNK');
        const draw = log.includes('SPINON_C0410_DRAW')
          && log.includes(`presented boxes=${scene.boxCount}`);
        const pixelSummary = !scene.pixelSamples
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`);
        const pixelError = log.includes(`SPINON_C114_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`);
        if ((summary && trace && draw && pixelSummary)
            || pixelError
            || log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        sleep(300);
      }
      if (!supersededC114Result(log, runID)) break;
      supersededAttempts++;
      await writeFile(resolve(evidenceDirectory,
        `runtime-${scene.id}-superseded-${attempt + 1}.log`), `${log}\n`);
    }
    assert.ok(!supersededC114Result(log, runID),
      `${scene.id}: 새 host 재시도 후에도 eval 결과가 superseded 상태입니다.`);
    await writeFile(resolve(evidenceDirectory, `runtime-${scene.id}.log`), `${log}\n`);
    const observation = verifyRuntimeLog(log, scene, runID);
    observation.supersededRetries = supersededAttempts;
    observations.push(observation);
    console.log(`${target} C11.4 scene ${index + 1}/${scenes.length}: ${scene.id} passed`);
    const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    ...metadata, target, apkSha256: sha256(apkBytes),
  }, null, 2)}\n`);
}

await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, sceneCount: scenes.length, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
