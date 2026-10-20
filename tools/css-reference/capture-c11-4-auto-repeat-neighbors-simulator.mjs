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
const requestedSceneIDs = process.argv.find((argument) => argument.startsWith('--scene-id='))
  ?.slice('--scene-id='.length).split(',').filter(Boolean);
const requestedRunKind = process.argv.find((argument) => argument.startsWith('--run-kind='))
  ?.slice('--run-kind='.length);
const evidenceGroup = process.argv.find((argument) => argument.startsWith('--evidence-group='))
  ?.slice('--evidence-group='.length);
const multiSpanSceneIDs = new Set([
  'competing-span2-forward', 'competing-span2-reversed',
  'span2-before-span3', 'span3-before-span2',
]);
const textMultiSpanSceneIDs = new Set([
  'text-span2-forward', 'text-span2-reversed',
  'text-span2-before-span3', 'text-span3-before-span2',
]);
const imageMultiSpanSceneIDs = new Set([
  'image-span2-forward', 'image-span2-reversed',
  'image-span2-before-span3', 'image-span3-before-span2',
]);
const isTextMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => textMultiSpanSceneIDs.has(sceneID));
const isImageMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => imageMultiSpanSceneIDs.has(sceneID));
const isMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => multiSpanSceneIDs.has(sceneID) || textMultiSpanSceneIDs.has(sceneID));
const runKind = requestedRunKind ?? (isImageMultiSpanRun ? 'image-multispan-contributions'
  : isTextMultiSpanRun ? 'text-multispan-contributions'
  : isMultiSpanRun ? 'multispan-ordering' : 'negative-implicit');
assert.match(runKind, /^[a-z0-9-]{1,80}$/, 'run-kind는 소문자·숫자·하이픈만 허용합니다.');
assert.ok(['android-avd', 'android-device', 'ios-simulator'].includes(target),
  '사용법: node capture-c11-4-auto-repeat-neighbors-simulator.mjs android-avd|android-device|ios-simulator [ADB serial|iOS Simulator UDID] [--scene-id=...] [--run-kind=...] [--evidence-group=...]');
if (target === 'android-device') {
  assert.ok(process.argv[3], 'android-device 모드에는 ADB serial이 필요합니다.');
}
if (evidenceGroup !== undefined) {
  assert.match(evidenceGroup, /^[a-z0-9][a-z0-9-]{0,79}$/,
    'evidence-group은 소문자·숫자·하이픈만 허용합니다.');
}

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const appPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const runPrefix = `c114-${runKind}-${target}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot, evidenceGroup
  ? `spec/internal/evidence/${evidenceGroup}-${date}/${target}-${runPrefix}`
  : isImageMultiSpanRun
    ? `spec/internal/evidence/c11-4-image-multispan-contributions-${date}/${target}-${runPrefix}`
  : isTextMultiSpanRun
    ? `spec/internal/evidence/c11-4-text-multispan-contributions-${date}/${target}-${runPrefix}`
  : isMultiSpanRun
    ? `spec/internal/evidence/c11-4-multispan-ordering-physical-android-${date}/${target}-${runPrefix}`
    : `spec/internal/evidence/c11-4-negative-implicit-track-sizing-${date}/${target}-${runPrefix}`);
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
scenes.push(...await autoRepeatTypedImplicitScenes());
assert.equal(scenes.length, 138,
  'C11.4 typed implicit auto-repeat simulator 행렬은 138 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
scenes.push(
  {
    id: 'competing-span2-forward', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=40', itemFrame: 'width=80,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;'
      + '1:node=2,x=0,y=0,width=80,height=10;2:node=3,x=40,y=10,width=120,height=10]',
    tracks: [40, 60, 60, 0], collapsed: [false, false, false, false], intrinsicSpans: [2],
  },
  {
    id: 'competing-span2-reversed', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=40', itemFrame: 'width=120,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;'
      + '1:node=2,x=40,y=0,width=120,height=10;2:node=3,x=0,y=10,width=80,height=10]',
    tracks: [40, 60, 60, 0], collapsed: [false, false, false, false], intrinsicSpans: [2],
  },
  {
    id: 'span2-before-span3', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=40', itemFrame: 'width=80,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;'
      + '1:node=2,x=0,y=0,width=80,height=10;'
      + '2:node=3,x=53.328125,y=10,width=120,height=10;'
      + '3:node=4,x=0,y=20,width=200,height=10]',
    tracks: [53.328125, 73.328125, 73.34375, 0],
    collapsed: [false, false, false, false], intrinsicSpans: [2, 3],
  },
  {
    id: 'span3-before-span2', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=40', itemFrame: 'width=200,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;'
      + '1:node=2,x=0,y=0,width=200,height=10;'
      + '2:node=3,x=53.328125,y=10,width=120,height=10;'
      + '3:node=4,x=0,y=20,width=80,height=10]',
    tracks: [53.328125, 73.328125, 73.34375, 0],
    collapsed: [false, false, false, false], intrinsicSpans: [2, 3],
  },
);
assert.equal(scenes.length, 142, 'C11.4 multi-span simulator 행렬은 142 scene이어야 합니다.');
scenes.push(
  {
    id: 'text-span2-forward', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=2,x=0,y=0,width=60,height=20;2:node=4,x=25,y=20,width=70,height=20]',
    tracks: [25, 35, 35, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], textIntrinsicContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 70, minContentHeight: 20, maxContent: 120, maxContentHeight: 10 },
    ],
  },
  {
    id: 'text-span2-reversed', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=2,x=25,y=20,width=70,height=20;2:node=4,x=0,y=0,width=60,height=20]',
    tracks: [25, 35, 35, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], textIntrinsicContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 70, minContentHeight: 20, maxContent: 120, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
    ],
  },
  {
    id: 'text-span2-before-span3', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=2,x=0,y=0,width=116.666667,height=10;'
      + '2:node=4,x=53.333333,y=20,width=126.666667,height=10;'
      + '3:node=6,x=0,y=40,width=180,height=10]',
    tracks: [53.333333, 63.333333, 63.333333, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], textIntrinsicContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 70, minContentHeight: 20, maxContent: 120, maxContentHeight: 10 },
      { grid: 1, node: 6, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 180, minContentHeight: 10, maxContent: 180, maxContentHeight: 10 },
    ],
  },
  {
    id: 'text-span3-before-span2', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=2,x=0,y=40,width=180,height=10;'
      + '2:node=4,x=53.333333,y=20,width=126.666667,height=10;'
      + '3:node=6,x=0,y=0,width=116.666667,height=10]',
    tracks: [53.333333, 63.333333, 63.333333, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], textIntrinsicContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 180, minContentHeight: 10, maxContent: 180, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 70, minContentHeight: 20, maxContent: 120, maxContentHeight: 10 },
      { grid: 1, node: 6, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
    ],
  },
);
assert.equal(scenes.length, 146, 'C11.4 text multi-span simulator 행렬은 146 scene이어야 합니다.');
scenes.push(
  {
    id: 'image-span2-forward', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=40,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;'
      + '1:node=2,x=1,y=1,width=24,height=16;2:node=3,x=13,y=21,width=48,height=32]',
    tracks: [12, 24, 24, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], imageIntrinsicContributions: true,
    imageItemSpans: [2, 2], imageMinContentWidths: [24, 48],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    pixelSamples: [
      { id: 'image-multispan-root-background', nodeId: 'c114-image-span2-forward-grid', xCssPx: 30, yCssPx: 10, rgba: [16, 24, 32, 255] },
      { id: 'image-multispan-outside-root', nodeId: 'c114-image-span2-forward-grid', xCssPx: 40, yCssPx: 10, rgba: [0, 0, 0, 255] },
      { id: 'image-multispan-item-overflow', nodeId: 'image-b-forward', xCssPx: 45, yCssPx: 30, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-after-item', nodeId: 'c114-image-span2-forward-grid', xCssPx: 62, yCssPx: 30, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'image-span2-reversed', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=40,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;'
      + '1:node=2,x=13,y=21,width=48,height=32;2:node=3,x=1,y=1,width=24,height=16]',
    tracks: [12, 24, 24, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], imageIntrinsicContributions: true,
    imageItemSpans: [2, 2], imageMinContentWidths: [48, 24],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    pixelSamples: [
      { id: 'image-multispan-root-background', nodeId: 'c114-image-span2-reversed-grid', xCssPx: 30, yCssPx: 10, rgba: [16, 24, 32, 255] },
      { id: 'image-multispan-outside-root', nodeId: 'c114-image-span2-reversed-grid', xCssPx: 40, yCssPx: 10, rgba: [0, 0, 0, 255] },
      { id: 'image-multispan-item-overflow', nodeId: 'image-b-reversed', xCssPx: 45, yCssPx: 30, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-after-item', nodeId: 'c114-image-span2-reversed-grid', xCssPx: 62, yCssPx: 30, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'image-span2-before-span3', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=40,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;'
      + '1:node=2,x=1,y=1,width=24,height=16;2:node=3,x=17,y=21,width=48,height=32;'
      + '3:node=4,x=1,y=41,width=72,height=48]',
    tracks: [16, 28, 28, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], imageIntrinsicContributions: true,
    imageItemSpans: [2, 2, 3], imageMinContentWidths: [24, 48, 72],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    pixelSamples: [
      { id: 'image-multispan-root-background', nodeId: 'c114-image-span2-before-span3-grid', xCssPx: 35, yCssPx: 10, rgba: [16, 24, 32, 255] },
      { id: 'image-multispan-outside-root', nodeId: 'c114-image-span2-before-span3-grid', xCssPx: 40, yCssPx: 10, rgba: [0, 0, 0, 255] },
      { id: 'image-multispan-item-overflow', nodeId: 'image-c-c', xCssPx: 65, yCssPx: 45, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-item-below-root', nodeId: 'image-c-c', xCssPx: 10, yCssPx: 70, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-after-item', nodeId: 'c114-image-span2-before-span3-grid', xCssPx: 75, yCssPx: 70, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'image-span3-before-span2', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=40,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;'
      + '1:node=2,x=1,y=41,width=72,height=48;2:node=3,x=17,y=21,width=48,height=32;'
      + '3:node=4,x=1,y=1,width=24,height=16]',
    tracks: [16, 28, 28, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], imageIntrinsicContributions: true,
    imageItemSpans: [3, 2, 2], imageMinContentWidths: [72, 48, 24],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    pixelSamples: [
      { id: 'image-multispan-root-background', nodeId: 'c114-image-span3-before-span2-grid', xCssPx: 35, yCssPx: 10, rgba: [16, 24, 32, 255] },
      { id: 'image-multispan-outside-root', nodeId: 'c114-image-span3-before-span2-grid', xCssPx: 40, yCssPx: 10, rgba: [0, 0, 0, 255] },
      { id: 'image-multispan-item-overflow', nodeId: 'image-r-c', xCssPx: 65, yCssPx: 45, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-item-below-root', nodeId: 'image-r-c', xCssPx: 10, yCssPx: 70, rgba: [229, 72, 77, 255] },
      { id: 'image-multispan-after-item', nodeId: 'c114-image-span3-before-span2-grid', xCssPx: 75, yCssPx: 70, rgba: [0, 0, 0, 255] },
    ],
  },
);
assert.equal(scenes.length, 150, 'C11.4 image multi-span simulator 행렬은 150 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
const requestedSceneIDSet = requestedSceneIDs ? new Set(requestedSceneIDs) : null;
const runScenes = requestedSceneIDSet
  ? scenes.filter(({ id }) => requestedSceneIDSet.has(id))
  : scenes;
assert.ok(runScenes.length > 0,
  requestedSceneIDs ? `C11.4 scene ID를 찾지 못했습니다: ${requestedSceneIDs.join(',')}` : '실행할 scene이 없습니다.');
if (requestedSceneIDs) {
  assert.equal(requestedSceneIDSet.size, requestedSceneIDs.length,
    'C11.4 선택 scene ID가 중복됐습니다.');
  assert.equal(runScenes.length, requestedSceneIDs.length,
    'C11.4 선택 scene ID 중 일부가 없습니다.');
}

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
  const [allFixtures, expectations, observations] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'spec-derived-expectations.json'), 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'chrome155-observation.json'), 'utf8').then(JSON.parse),
  ]);
  const fixtures = allFixtures.filter(({ id }) => !id.startsWith('typed-'));
  const expectationsByID = new Map(expectations.cases.map((entry) => [entry.id, entry]));
  const observationsByID = new Map(observations.cases.map((entry) => [entry.id, entry]));
  const fixtureIDs = fixtures.map(({ id }) => id);
  assert.equal(allFixtures.length, 32, 'auto-repeat fixture의 기존·typed case 수가 달라졌습니다.');
  assert.equal(fixtures.length, 18, '기존 auto-repeat negative line fixture는 18 case여야 합니다.');
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

async function autoRepeatTypedImplicitScenes() {
  const evidence = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-typed-implicit-precomparison-2026-10-20');
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_line_scenes.json');
  const [allFixtures, expectations, observations, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'spec-derived-expectations.json'), 'utf8').then(JSON.parse),
    readFile(resolve(evidence, 'chrome155-observation.json'), 'utf8').then(JSON.parse),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  const fixtures = allFixtures.filter(({ id }) => id.startsWith('typed-'));
  const expectationID = new Map([
    ['typed-mixed-cycle', 'mixed-typed-cycle'],
    ['typed-auto-fit-occupied', 'auto-fit-typed-occupied'],
    ['typed-auto-fit-empty', 'auto-fit-typed-empty'],
    ['typed-auto-fit-gutter', 'auto-fit-typed-gutter'],
  ]);
  const expectedByID = new Map(expectations.expectations.map((entry) => [entry.id, entry]));
  const observationByID = new Map(observations.cases.map((entry) => [entry.id, entry]));
  assert.equal(fixtures.length, 14, 'typed implicit auto-repeat fixture는 14 case여야 합니다.');
  assert.equal(expectedByID.size, 14, '규범-derived typed expectation은 14 case여야 합니다.');
  assert.equal(observationByID.size, 14, 'Chrome typed 관찰은 14 case여야 합니다.');

  const extractIDs = (source, name, opening, closing) => {
    const nameOffset = source.indexOf(name);
    assert.ok(nameOffset >= 0, `${name} allowlist가 없습니다.`);
    const openOffset = source.indexOf(opening, nameOffset + name.length);
    assert.ok(openOffset >= 0, `${name} allowlist 시작을 읽을 수 없습니다.`);
    const closeOffset = source.indexOf(closing, openOffset + opening.length);
    assert.ok(closeOffset >= 0, `${name} allowlist 끝을 읽을 수 없습니다.`);
    return new Set([...source.slice(openOffset + 1, closeOffset).matchAll(/"([^"]+)"/g)]
      .map((match) => match[1]));
  };
  const androidSceneIDs = extractIDs(androidSource, 'C114_GRID_TRACK_SIZING_SCENE_IDS', '{', '};');
  const androidAutoRepeatIDs = extractIDs(
    androidSource, 'C114_AUTO_REPEAT_NEGATIVE_LINE_SCENE_IDS', '{', '};');
  const iosSceneIDs = extractIDs(iosSource, 'c114GridTrackSizingSceneIDs', '[', ']');
  const iosAutoRepeatIDs = extractIDs(iosSource, 'c114AutoRepeatNegativeLineSceneIDs', '[', ']');
  for (const { id } of fixtures) {
    assert.ok(androidSceneIDs.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(androidAutoRepeatIDs.has(id), `Android auto-repeat allowlist에 ${id}가 없습니다.`);
    assert.ok(iosSceneIDs.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosAutoRepeatIDs.has(id), `iOS auto-repeat allowlist에 ${id}가 없습니다.`);
    const expectedID = expectationID.get(id) ?? id.slice('typed-'.length);
    assert.ok(expectedByID.has(expectedID), `${id}: 독립 규범 expectation이 없습니다.`);
    assert.ok(observationByID.has(expectedID), `${id}: Chrome 관찰이 없습니다.`);
  }

  const fixedCycle = expectedByID.get('fixed-cycle');
  const fixedThree = expectedByID.get('fixed-cycle-three-leading-trailing');
  const mixedCycle = expectedByID.get('mixed-typed-cycle');
  const autoFitEmpty = expectedByID.get('auto-fit-typed-empty');
  const ahem = { minContentInline: 48, maxContentInline: 108, minContentBlock: 36, maxContentBlock: 12 };
  const repeatCount = expectations.fixture.repeatCountPerAxis;
  const repeatTrackSizes = Array(repeatCount).fill(expectations.fixture.fixedTrackUsedSizeCssPx);
  const percentAt110 = 110 * 0.25;
  const percentAt132 = 132 * 0.25;
  const calcAt110 = 5 + 110 * 0.1;
  const calcAt132 = 5 + 132 * 0.1;
  const specs = {
    'typed-fixed-cycle': {
      Columns: { leading: fixedCycle.leadingTrackSizesCssPxLeftToRight, trailing: [] },
      Rows: { leading: fixedCycle.leadingTrackSizesCssPxLeftToRight, trailing: [] },
    },
    'typed-fixed-cycle-three-leading-trailing': {
      Columns: {
        leading: fixedThree.leadingTrackSizesCssPxLeftToRight,
        trailing: fixedThree.trailingTrackSizesCssPxLeftToRight,
      },
      Rows: {
        leading: fixedThree.leadingTrackSizesCssPxLeftToRight,
        trailing: fixedThree.trailingTrackSizesCssPxLeftToRight,
      },
    },
    'typed-mixed-cycle': {
      Columns: {
        leading: mixedCycle.leadingTrackSizesCssPxLeftToRight.columns,
        trailing: mixedCycle.trailingTrackSizesCssPxLeftToRight.columns,
      },
      Rows: {
        leading: mixedCycle.leadingTrackSizesCssPxLeftToRight.rows,
        trailing: mixedCycle.trailingTrackSizesCssPxLeftToRight.rows,
      },
    },
    'typed-auto': {
      Columns: { leading: [ahem.minContentInline, 0], trailing: [] },
      Rows: { leading: [ahem.minContentBlock, 0], trailing: [] },
    },
    'typed-min-content': {
      Columns: { leading: [ahem.minContentInline, 0], trailing: [] },
      Rows: { leading: [ahem.minContentBlock, 0], trailing: [] },
    },
    'typed-max-content': {
      Columns: { leading: [ahem.maxContentInline, 0], trailing: [] },
      Rows: { leading: [ahem.maxContentBlock, 0], trailing: [] },
    },
    'typed-minmax-max-content': {
      Columns: { leading: expectedByID.get('minmax-max-content').expectedTrackSizesCssPx, trailing: [] },
      Rows: { leading: expectedByID.get('minmax-max-content').expectedTrackSizesCssPx, trailing: [] },
    },
    'typed-fit-content': {
      Columns: {
        leading: [Math.max(ahem.minContentInline, Math.min(24, ahem.maxContentInline)), 0],
        trailing: [],
      },
      Rows: {
        leading: [Math.max(ahem.minContentBlock, Math.min(24, ahem.maxContentBlock)), 0],
        trailing: [],
      },
    },
    'typed-fraction': {
      Columns: { leading: expectedByID.get('fraction').expectedTrackSizesCssPx, trailing: [] },
      Rows: { leading: expectedByID.get('fraction').expectedTrackSizesCssPx, trailing: [] },
    },
    'typed-percentage': {
      Columns: { leading: expectedByID.get('percentage').expectedTrackSizesCssPx, trailing: [] },
      Rows: { leading: expectedByID.get('percentage').expectedTrackSizesCssPx, trailing: [] },
    },
    'typed-calc': {
      Columns: { leading: expectedByID.get('calc').expectedTrackSizesCssPx, trailing: [] },
      Rows: { leading: expectedByID.get('calc').expectedTrackSizesCssPx, trailing: [] },
    },
    'typed-auto-fit-occupied': {
      Columns: {
        leading: [ahem.minContentInline, percentAt110],
        trailing: mixedCycle.trailingTrackSizesCssPxLeftToRight.columns,
      },
      Rows: {
        leading: [ahem.minContentBlock, calcAt110],
        trailing: mixedCycle.trailingTrackSizesCssPxLeftToRight.rows,
      },
    },
    'typed-auto-fit-empty': {
      Columns: { leading: autoFitEmpty.expectedLeadingTrackSizesCssPx, trailing: [] },
      Rows: { leading: autoFitEmpty.expectedLeadingTrackSizesCssPx, trailing: [] },
    },
    'typed-auto-fit-gutter': {
      Columns: { leading: [ahem.minContentInline, percentAt132], trailing: [0, percentAt132, 0] },
      Rows: { leading: [ahem.minContentBlock, calcAt132], trailing: [0, calcAt132, 0] },
    },
  };
  const exactChromeFrames = new Set([
    'typed-fixed-cycle', 'typed-fixed-cycle-three-leading-trailing', 'typed-mixed-cycle',
    'typed-minmax-max-content', 'typed-fraction', 'typed-percentage', 'typed-calc',
    'typed-auto-fit-empty',
  ]);
  const fontSensitiveChromeGeometry = new Set([
    'typed-auto-fit-occupied', 'typed-auto-fit-gutter',
  ]);
  const chromeFontSensitiveTrackIndices = {
    'typed-auto': { Columns: [0], Rows: [0] },
    'typed-min-content': { Columns: [0], Rows: [0] },
    'typed-max-content': { Columns: [0], Rows: [0] },
    'typed-fit-content': { Columns: [0], Rows: [0] },
    'typed-auto-fit-occupied': { Columns: [0], Rows: [0] },
    'typed-auto-fit-gutter': { Columns: [0], Rows: [0] },
  };
  const close = (actual, expected, label) => assert.ok(
    Math.abs(actual - expected) <= 1 / 64,
    `${label}: Chrome=${actual}, 규범/Ahem 기대=${expected}`,
  );
  const parseLinePair = (value, label) => {
    const match = value?.match(/^\s*(-?\d+)\s*\/\s*(-?\d+)\s*$/);
    assert.ok(match, `${label}: numeric line placement가 없습니다.`);
    return [Number(match[1]), Number(match[2])];
  };
  const leadingCountFor = (items, key) => Math.max(0, ...items.flatMap((item) =>
    parseLinePair(item[key], key).filter((line) => line < 0).map((line) => -line - 6)));
  const trailingCountFor = (items, key) => Math.max(0, ...items.flatMap((item) =>
    parseLinePair(item[key], key).filter((line) => line > 6).map((line) => line - 6)));
  const boundaryFor = (line, leading) => leading + (line > 0 ? line - 1 : 6 + line);

  return fixtures.map((fixture) => {
    const expectedID = expectationID.get(fixture.id) ?? fixture.id.slice('typed-'.length);
    const expectation = expectedByID.get(expectedID);
    const observation = observationByID.get(expectedID);
    const mode = fixture.columns.includes('auto-fit') ? 'auto-fit' : 'auto-fill';
    const cssLines = { Columns: 'c', Rows: 'r' };
    const gaps = { Columns: Number.parseFloat(fixture.columnGap), Rows: Number.parseFloat(fixture.rowGap) };
    const axes = {};
    const itemFrames = fixture.items.map(() => ({ x: 0, y: 0, width: 0, height: 0 }));
    let nextItemNodeId = 2;
    const itemNodeIds = fixture.items.map((item) => {
      const nodeId = nextItemNodeId;
      nextItemNodeId += 1 + (item.text ? 1 : 0);
      return nodeId;
    });

    for (const axis of ['Columns', 'Rows']) {
      const key = cssLines[axis];
      const leading = leadingCountFor(fixture.items, key);
      const trailing = trailingCountFor(fixture.items, key);
      const spec = specs[fixture.id][axis];
      assert.equal(spec.leading.length, leading, `${fixture.id} ${axis}: leading cycle 길이가 다릅니다.`);
      assert.equal(spec.trailing.length, trailing, `${fixture.id} ${axis}: trailing cycle 길이가 다릅니다.`);
      let tracks = [...spec.leading, ...repeatTrackSizes, ...spec.trailing];
      const collapsed = tracks.map(() => false);
      if (mode === 'auto-fit') {
        const repeatedStart = leading;
        const repeatedEnd = repeatedStart + repeatCount;
        const occupied = new Set();
        for (const item of fixture.items) {
          const [first, second] = parseLinePair(item[key], `${fixture.id} ${axis}`);
          const start = Math.min(boundaryFor(first, leading), boundaryFor(second, leading));
          const end = Math.max(boundaryFor(first, leading), boundaryFor(second, leading));
          for (let index = Math.max(start, repeatedStart); index < Math.min(end, repeatedEnd); index += 1) {
            occupied.add(index);
          }
        }
        for (let index = repeatedStart; index < repeatedEnd; index += 1) {
          if (!occupied.has(index)) {
            collapsed[index] = true;
            tracks[index] = 0;
          }
        }
      }
      axes[axis] = { tracks, collapsed };

      for (const [itemIndex, item] of fixture.items.entries()) {
        const [first, second] = parseLinePair(item[key], `${fixture.id} ${axis}`);
        const start = Math.min(boundaryFor(first, leading), boundaryFor(second, leading));
        const end = Math.max(boundaryFor(first, leading), boundaryFor(second, leading));
        assert.ok(start >= 0 && end <= tracks.length,
          `${fixture.id} ${axis}: placement가 사용 track 범위를 벗어납니다.`);
        let offset = 0;
        for (let index = 0; index < start; index += 1) {
          offset += tracks[index];
          if (index + 1 < tracks.length) {
            offset += gaps[axis] * (!collapsed[index] && !collapsed[index + 1] ? 1 : 0);
          }
        }
        let extent = 0;
        for (let index = start; index < end; index += 1) {
          extent += tracks[index];
          if (index + 1 < end) {
            extent += gaps[axis] * (!collapsed[index] && !collapsed[index + 1] ? 1 : 0);
          }
        }
        itemFrames[itemIndex][axis === 'Columns' ? 'x' : 'y'] = offset;
        itemFrames[itemIndex][axis === 'Columns' ? 'width' : 'height'] = extent;
      }

      const computed = observation[axis === 'Columns' ? 'computedColumns' : 'computedRows'];
      const chromeTracks = [...computed.matchAll(/(-?\d+(?:\.\d+)?)px/g)]
        .map((match) => Number(match[1]));
      assert.equal(chromeTracks.length, tracks.length,
        `${fixture.id} ${axis}: Chrome과 규범 기대의 expanded track 수가 다릅니다.`);
      const fontSensitive = new Set(chromeFontSensitiveTrackIndices[fixture.id]?.[axis] ?? []);
      tracks.forEach((size, index) => {
        if (!fontSensitive.has(index)) close(chromeTracks[index], size,
          `${fixture.id} ${axis} track ${index + 1}`);
      });
      assert.deepEqual(collapsed, chromeTracks.map((size, index) => mode === 'auto-fit'
        && index >= leading && index < leading + repeatCount && size === 0),
      `${fixture.id} ${axis}: auto-fit collapse 상태가 Chrome 관찰과 다릅니다.`);
    }

    const rootWidth = Number.parseFloat(fixture.width);
    const rootHeight = Number.parseFloat(fixture.height);
    const rootFrame = `width=${rootWidth},height=${rootHeight}`;
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};`
      + itemFrames.map((frame, index) => `${index + 1}:node=${itemNodeIds[index]},x=${frame.x},y=${frame.y},`
        + `width=${frame.width},height=${frame.height}`).join(';') + ']';
    const boxCount = itemFrames.filter((frame) => frame.width > 0 && frame.height > 0).length + 1;
    const placedIndex = fixture.items.findIndex((item) =>
      parseLinePair(item.c, `${fixture.id} placed item`).some((line) => line < 0)
        && parseLinePair(item.r, `${fixture.id} placed item`).some((line) => line < 0));
    assert.ok(placedIndex >= 0, `${fixture.id}: negative-line placed item이 없습니다.`);
    const placedFrame = itemFrames[placedIndex];

    if (exactChromeFrames.has(fixture.id)
        || fixture.id === 'typed-auto-fit-occupied' || fixture.id === 'typed-auto-fit-gutter') {
      close(observation.item.x - observation.grid.x, placedFrame.x,
        `${fixture.id}: Chrome item x`);
      close(observation.item.y - observation.grid.y, placedFrame.y,
        `${fixture.id}: Chrome item y`);
      if (exactChromeFrames.has(fixture.id)) {
        close(observation.item.width, placedFrame.width, `${fixture.id}: Chrome item width`);
        close(observation.item.height, placedFrame.height, `${fixture.id}: Chrome item height`);
      }
      if (fixture.items.length > 1 && observation.control) {
        const controlIndex = fixture.items.findIndex((item) => item.c === '1 / 2' && item.r === '1 / 2');
        if (controlIndex >= 0) {
          const control = itemFrames[controlIndex];
          if (!fontSensitiveChromeGeometry.has(fixture.id)) {
            close(observation.control.x - observation.grid.x, control.x,
              `${fixture.id}: Chrome control x`);
            close(observation.control.y - observation.grid.y, control.y,
              `${fixture.id}: Chrome control y`);
          }
          close(observation.control.width, control.width, `${fixture.id}: Chrome control width`);
          close(observation.control.height, control.height, `${fixture.id}: Chrome control height`);
        }
      }
      if (fixture.items.length > 1 && observation.trailing) {
        const trailingIndex = fixture.items.findIndex((item) => item.c === '8 / 9' && item.r === '8 / 9');
        if (trailingIndex >= 0) {
          const trailingFrame = itemFrames[trailingIndex];
          if (!fontSensitiveChromeGeometry.has(fixture.id)) {
            close(observation.trailing.x - observation.grid.x, trailingFrame.x,
              `${fixture.id}: Chrome trailing x`);
            close(observation.trailing.y - observation.grid.y, trailingFrame.y,
              `${fixture.id}: Chrome trailing y`);
          }
          close(observation.trailing.width, trailingFrame.width,
            `${fixture.id}: Chrome trailing width`);
          close(observation.trailing.height, trailingFrame.height,
            `${fixture.id}: Chrome trailing height`);
        }
      }
    }

    const intrinsicItemIndex = fixture.items.findIndex(({ text }) => Boolean(text));
    const intrinsicInputs = intrinsicItemIndex < 0 ? undefined : [{
      grid: 1,
      node: intrinsicItemIndex + 2,
      sourceOrder: intrinsicItemIndex,
      kind: 'text',
      fontRegistryRevision: 1,
      ...ahem,
    }].map(({ grid, node, sourceOrder, kind, fontRegistryRevision }) => ({
      grid, node, sourceOrder, kind, fontRegistryRevision,
      minContent: ahem.minContentInline,
      minContentHeight: ahem.minContentBlock,
      maxContent: ahem.maxContentInline,
      maxContentHeight: ahem.maxContentBlock,
    }));
    const expectedRepeatCount = mode === 'auto-fit'
      ? expectations.fixture.autoFitRepeatCountBeforeCollapsePerAxis : repeatCount;
    assert.equal(expectedRepeatCount, repeatCount,
      `${fixture.id}: 독립 repeat count 기대와 고정 fixture가 다릅니다.`);

    return {
      id: fixture.id,
      boxCount,
      rootFrame,
      itemFrame: null,
      nodeFrames,
      axes,
      gaps,
      ...(intrinsicInputs ? { intrinsicInputs } : {}),
    };
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
  assert.equal(connected.length, 1, `Android 대상 기기가 연결되어 있지 않습니다: ${serial}`);
  const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
  const qemu = adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim();
  const model = adb(['shell', 'getprop', 'ro.product.model']).stdout.trim();
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  const abi = adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim();
  if (target === 'android-avd') {
    assert.equal(qemu, '1', 'android-avd 모드에는 Android Emulator가 필요합니다.');
    assert.equal(apiLevel, 37, `고정 Android API 37 AVD가 아닙니다: ${apiLevel}`);
  } else {
    assert.equal(qemu, '0', 'android-device 모드에는 실기기가 필요합니다.');
    assert.equal(model, 'SM-S731N', `검증 대상 Samsung SM-S731N이 아닙니다: ${model}`);
    assert.ok(apiLevel >= 36, `Android API 36 이상이 필요합니다: ${apiLevel}`);
  }
  assert.equal(abi, 'arm64-v8a', `ARM64 Android 대상이 아닙니다: ${abi}`);
  let deviceMetadata = {};
  if (target === 'android-device') {
    const surfaceFlinger = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.replaceAll('\0', '');
    const gpuRenderer = surfaceFlinger.match(/(?:^|\r?\n)GLES:\s*([^\r\n]+)/)?.[1]?.trim() ?? '';
    assert.ok(gpuRenderer, 'SurfaceFlinger에서 실기기 GLES renderer를 읽지 못했습니다.');
    deviceMetadata = {
      manufacturer: adb(['shell', 'getprop', 'ro.product.manufacturer']).stdout.trim(),
      androidRelease: adb(['shell', 'getprop', 'ro.build.version.release']).stdout.trim(),
      boardPlatform: adb(['shell', 'getprop', 'ro.board.platform']).stdout.trim(),
      physicalSize: adb(['shell', 'wm', 'size']).stdout.trim(),
      physicalDensity: adb(['shell', 'wm', 'density']).stdout.trim(),
      gpuRenderer,
    };
  }
  return {
    adb,
    metadata: {
      serial,
      target,
      qemu,
      apiLevel,
      abi,
      model,
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
      ...deviceMetadata,
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
  const expectedNodeFramesMatch = parseNodeFrames(scene.nodeFrames, scene.id);
  const actualNodeFrames = log.split(/\r?\n/).flatMap((line) => {
    const marker = 'SPINON_C114_GRID_NODE_FRAME ';
    const markerOffset = line.indexOf(marker);
    return markerOffset < 0 ? [] : [line.slice(markerOffset + marker.length)];
  }).map((frame) => parseNodeFrame(frame, scene.id));
  assert.equal(actualNodeFrames.length, expectedNodeFramesMatch.length,
    `${scene.id}: 전체 node frame 개수가 다릅니다.`);
  for (const [index, expected] of expectedNodeFramesMatch.entries()) {
    const actual = actualNodeFrames[index];
    assert.equal(actual.order, expected.order, `${scene.id}: node frame ${index} 순서가 다릅니다.`);
    assert.equal(actual.node, expected.node, `${scene.id}: node frame ${index} ID가 다릅니다.`);
    for (const property of ['x', 'y', 'width', 'height']) {
      assert.ok(Math.abs(actual[property] - expected[property]) <= 1 / 64,
        `${scene.id}: node ${actual.node} ${property} 예상 ${expected[property]}, 실제 ${actual[property]}`);
    }
  }
  assert.ok(log.includes(scene.rootFrame)
    && (scene.itemFrame === null || log.includes(scene.itemFrame)),
    `${scene.id}: root/item frame이 예상과 다릅니다.`);
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
  if (scene.intrinsicInputs && scene.imageIntrinsicContributions) {
    const intrinsicInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      kind: input.source?.kind,
      aspectRatio: input.source?.natural?.aspectRatio ?? null,
      width: input.source?.natural?.width,
      height: input.source?.natural?.height,
    }));
    assert.deepEqual(intrinsicInputs, scene.intrinsicInputs,
      `${scene.id}: replaced image intrinsic input matrix differs from PNG oracle`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: intrinsic resource revision is not the fixed fixture revision`);
  } else if (scene.intrinsicInputs) {
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
  let observedIntrinsicSpans = null;
  for (const [axis, expected] of Object.entries(axisSpecs)) {
    const axisPass = diagnostics.trackSizingTraces[0].passes
      .filter((pass) => pass.axis === axis).at(-1);
    assert.ok(axisPass, `${scene.id}: ${axis} track-sizing pass가 없습니다.`);
    const intrinsicSpans = axisPass.stages.flatMap(({ stage }) => {
      const match = stage.match(/^IntrinsicItemBatch \{ span: (\d+),/);
      return match ? [Number(match[1])] : [];
    });
    if (scene.intrinsicSpans) {
      assert.deepEqual(intrinsicSpans, scene.intrinsicSpans,
        `${scene.id}: intrinsic span stage 순서가 다릅니다.`);
    }
    if (scene.textIntrinsicContributions) {
      const expectedContributions = [];
      for (const span of scene.intrinsicSpans) {
        const spanItems = scene.intrinsicInputs.filter((_, itemIndex) => {
          const itemSpan = scene.id === 'text-span2-forward'
            || scene.id === 'text-span2-reversed' ? 2
            : scene.id === 'text-span2-before-span3'
              ? [2, 2, 3][itemIndex] : [3, 2, 2][itemIndex];
          return itemSpan === span;
        });
        for (const kind of ['Minimum', 'MinContent']) {
          const repetitions = kind === 'MinContent' ? 2 : 1;
          for (let repetition = 0; repetition < repetitions; repetition++) {
            for (const input of spanItems) {
            expectedContributions.push({
              item: input.node,
              kind,
              span,
              value: kind === 'Minimum' ? 0 : input.minContent,
            });
            }
          }
        }
      }
      const observedContributions = axisPass.contributions.map((contribution) => ({
        item: contribution.item,
        kind: contribution.kind,
        span: contribution.span,
        value: contribution.value,
      }));
      assert.deepEqual(observedContributions, expectedContributions,
        `${scene.id}: Minimum/MinContent trace contribution mapping differs`);
    }
    if (scene.imageIntrinsicContributions) {
      const expectedContributions = [];
      for (const span of scene.intrinsicSpans) {
        const spanItems = scene.intrinsicInputs.flatMap((input, itemIndex) => (
          scene.imageItemSpans[itemIndex] === span ? [{ input, itemIndex }] : []
        ));
        for (const kind of ['Minimum', 'MinContent']) {
          const repetitions = kind === 'MinContent' ? 2 : 1;
          for (let repetition = 0; repetition < repetitions; repetition++) {
            for (const { input, itemIndex } of spanItems) {
              expectedContributions.push({
                item: input.node,
                kind,
                span,
                value: kind === 'Minimum' ? 0 : scene.imageMinContentWidths[itemIndex],
              });
            }
          }
        }
      }
      const observedContributions = axisPass.contributions.map((contribution) => ({
        item: contribution.item,
        kind: contribution.kind,
        span: contribution.span,
        value: contribution.value,
      }));
      assert.deepEqual(observedContributions, expectedContributions,
        `${scene.id}: replaced Minimum/MinContent contribution mapping differs`);
    }
    if (axis === scene.axis) observedIntrinsicSpans = intrinsicSpans;
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
    observedNodeFrames: actualNodeFrames,
    tracks: scene.axes ? tracksByAxis : tracksByAxis[scene.axis],
    ...(scene.intrinsicSpans ? { intrinsicSpans: observedIntrinsicSpans } : {}),
    trackTraceChunks: traceParts.length,
    drawPresented: true,
  };
}

function parseNodeFrames(value, sceneID) {
  const match = value.match(/node_frames_css_px=\[([^\]]+)\]/);
  assert.ok(match, `${sceneID}: 독립 node frame 기대값이 없습니다.`);
  return match[1].split(';').map((frame) => parseNodeFrame(frame, sceneID));
}

function parseNodeFrame(value, sceneID) {
  const match = value.match(/^(\d+):node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)$/);
  assert.ok(match, `${sceneID}: node frame을 읽을 수 없습니다: ${value}`);
  return {
    order: Number(match[1]),
    node: Number(match[2]),
    x: Number(match[3]),
    y: Number(match[4]),
    width: Number(match[5]),
    height: Number(match[6]),
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
  for (const [index, scene] of runScenes.entries()) {
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
    console.log(`${target} C11.4 scene ${index + 1}/${runScenes.length}: ${scene.id} passed`);
    await saveLog(`screenshot-${scene.id}.log`, command('xcrun', ['simctl', 'io', simulatorUDID,
      'screenshot', resolve(evidenceDirectory, `${scene.id}.png`)]));
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target, udid: simulatorUDID, name: simulator[0].name, runtime: simulator[0].runtime, appSha256,
  }, null, 2)}\n`);
} else {
  const { adb, metadata } = currentAndroidDevice();
  const apkBytes = await readFile(apkPath);
  const localApkSha256 = sha256(apkBytes);
  await saveLog('install.log', adb(['install', '-r', apkPath]));
  const packagePaths = adb(['shell', 'pm', 'path', packageName]).stdout
    .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim()).filter(Boolean);
  const installedBaseApk = packagePaths.find((path) => path.endsWith('/base.apk'));
  assert.ok(installedBaseApk, `설치된 ${packageName}의 base.apk 경로를 찾지 못했습니다.`);
  const installedApkSha256 = adb(['shell', 'sha256sum', installedBaseApk]).stdout
    .trim().split(/\s+/)[0];
  assert.equal(installedApkSha256, localApkSha256,
    '실기기에 설치된 base.apk와 로컬 APK의 SHA-256이 다릅니다.');
  metadata.apkSha256 = localApkSha256;
  metadata.installedApkSha256 = installedApkSha256;
  await writeFile(resolve(evidenceDirectory,
    target === 'android-device' ? 'device.json' : 'simulator.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'installed-apk-verification.json'), `${JSON.stringify({
    packageName,
    installedBaseApk,
    localApkSha256,
    installedApkSha256,
    matched: installedApkSha256 === localApkSha256,
  }, null, 2)}\n`);
  for (const [index, scene] of runScenes.entries()) {
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
    console.log(`${target} C11.4 scene ${index + 1}/${runScenes.length}: ${scene.id} passed`);
    const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
  }
}

await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, sceneCount: runScenes.length, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
