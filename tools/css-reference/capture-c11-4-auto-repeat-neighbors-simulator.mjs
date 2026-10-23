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
const cyclicPercentSceneIDs = new Set([
  'cyclic-percent-column', 'cyclic-percent-row', 'definite-percent-column',
  'cyclic-minmax-column', 'cyclic-minmax-row', 'definite-minmax-column',
  'cyclic-fit-content-column', 'cyclic-fit-content-row', 'definite-fit-content-column',
  'cyclic-fit-content-calc-column', 'cyclic-fit-content-calc-row',
  'definite-fit-content-calc-column', 'definite-fit-content-calc-small-limit',
  'cyclic-minmax-calc-column', 'cyclic-minmax-calc-row',
  'definite-minmax-calc-column', 'definite-minmax-calc-small-limit',
  'cyclic-minmax-calc-min-column', 'cyclic-minmax-calc-min-row',
  'definite-minmax-calc-min-column', 'definite-minmax-calc-min-row',
  'cyclic-minmax-calc-min-both-axes',
  'cyclic-minmax-calc-min-column-definite-row',
  'definite-column-cyclic-minmax-calc-min-row',
  'definite-minmax-calc-min-both-axes',
  'cyclic-minmax-calc-min-span-columns',
  'definite-minmax-calc-min-span-columns',
  'cyclic-minmax-calc-min-span-rows',
  'definite-minmax-calc-min-span-rows',
  'cyclic-minmax-calc-two-axis-span-text-min-content',
  'cyclic-minmax-calc-two-axis-span-text-max-content',
  'definite-minmax-calc-two-axis-span-text-min-content',
  'definite-minmax-calc-two-axis-span-text-max-content',
  'cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward',
  'cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse',
  'definite-minmax-calc-mixed-text-image-span-forward',
  'definite-minmax-calc-mixed-text-image-span-reverse',
  'max-content-forward', 'max-content-reverse', 'definite-forward', 'definite-reverse',
  'cyclic-minmax-calc-overlap-forward', 'cyclic-minmax-calc-overlap-reverse',
  'definite-minmax-calc-overlap-forward', 'definite-minmax-calc-overlap-reverse',
  'definite-minmax-calc-overlap-small-forward',
  'definite-minmax-calc-overlap-small-reverse',
  'cyclic-minmax-calc-overlap-rows-forward', 'cyclic-minmax-calc-overlap-rows-reverse',
  'definite-minmax-calc-overlap-rows-forward', 'definite-minmax-calc-overlap-rows-reverse',
  'definite-minmax-calc-overlap-rows-small-forward',
  'definite-minmax-calc-overlap-rows-small-reverse',
  'cyclic-minmax-cross-axis-max-content-span-groups-forward',
  'cyclic-minmax-cross-axis-max-content-span-groups-reverse',
  'definite-minmax-cross-axis-max-content-span-groups-forward',
  'definite-minmax-cross-axis-max-content-span-groups-reverse',
  'cyclic-percent-two-axis-multitrack', 'cyclic-percent-columns-multitrack',
  'cyclic-percent-rows-multitrack', 'definite-percent-two-axis-multitrack',
  'cyclic-calc-column', 'cyclic-calc-row', 'cyclic-calc-two-axis',
  'definite-calc-control',
]);
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
const mixedMultiSpanSceneIDs = new Set([
  'mixed-text-image-span2-forward', 'mixed-text-image-span2-reversed',
  'mixed-text-image-span2-before-span3', 'mixed-text-image-span3-before-span2',
]);
const anonymousTextSceneIDs = new Set([
  'anonymous-min-content-wrap', 'anonymous-comment-continuation',
  'anonymous-runs-around-block', 'anonymous-hidden-child-continuation',
  'whitespace-does-not-create-item',
]);
const anonymousTextImageSceneIDs = new Set([
  'anonymous-text-image-text', 'image-before-comment-group',
  'comment-group-before-image', 'whitespace-around-image',
  'hidden-child-around-image',
]);
const anonymousTextMultiImageSceneIDs = new Set([
  'anonymous-text-two-images-forward', 'two-images-anonymous-text-reversed',
  'anonymous-text-three-images-forward', 'three-images-anonymous-text-reversed',
]);
const anonymousGridBorderOverflowSceneIDs = new Set([
  'anonymous-runs-two-images-forward', 'two-images-anonymous-runs-reversed',
  'two-anonymous-runs-three-images-forward', 'three-images-two-anonymous-runs-reversed',
]);
const nestedGridImplicitRowSceneIDs = new Set([
  'nested-grid-implicit-row-span-two', 'nested-grid-implicit-row-one-row',
]);
const nestedGridDepthSceneIDs = new Set([
  'nested-depth-one-min-content', 'nested-depth-two-min-content',
  'nested-depth-one-max-content', 'nested-depth-two-max-content',
  'nested-depth-one-min-content-two-axis', 'nested-depth-two-min-content-two-axis',
  'nested-depth-one-max-content-two-axis', 'nested-depth-two-max-content-two-axis',
  'nested-depth-three-min-content-two-axis', 'nested-depth-three-max-content-two-axis',
]);
const nestedGridTextMultiSpanSceneIDs = new Set([
  'nested-text-span2-forward', 'nested-text-span2-reversed',
  'nested-text-span2-before-span3', 'nested-text-span3-before-span2',
]);
const nestedGridDepthAnonymousImageSceneIDs = new Set([
  'depth-three-min-content-text-before-image',
  'depth-three-min-content-image-before-text',
  'depth-three-max-content-text-before-image',
  'depth-three-max-content-image-before-text',
]);
const isTextMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => textMultiSpanSceneIDs.has(sceneID));
const isImageMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => imageMultiSpanSceneIDs.has(sceneID));
const isMixedMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => mixedMultiSpanSceneIDs.has(sceneID));
const isAnonymousTextRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => anonymousTextSceneIDs.has(sceneID));
const isAnonymousTextImageRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => anonymousTextImageSceneIDs.has(sceneID));
const isAnonymousTextMultiImageRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => anonymousTextMultiImageSceneIDs.has(sceneID));
const isAnonymousGridBorderOverflowRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => anonymousGridBorderOverflowSceneIDs.has(sceneID));
const isNestedGridImplicitRowRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => nestedGridImplicitRowSceneIDs.has(sceneID));
const isNestedGridDepthRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => nestedGridDepthSceneIDs.has(sceneID));
const isNestedGridTextMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => nestedGridTextMultiSpanSceneIDs.has(sceneID));
const isNestedGridDepthAnonymousImageRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => nestedGridDepthAnonymousImageSceneIDs.has(sceneID));
const isCyclicPercentRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => cyclicPercentSceneIDs.has(sceneID));
const isMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => multiSpanSceneIDs.has(sceneID)
    || textMultiSpanSceneIDs.has(sceneID) || mixedMultiSpanSceneIDs.has(sceneID));
const runKind = requestedRunKind ?? (isCyclicPercentRun ? 'cyclic-percent-tracks'
  : isMixedMultiSpanRun ? 'mixed-multispan-contributions'
  : isNestedGridTextMultiSpanRun ? 'nested-grid-text-multispan'
  : isNestedGridDepthAnonymousImageRun ? 'nested-depth-three-anonymous-image'
  : isNestedGridDepthRun ? 'nested-grid-depth'
  : isNestedGridImplicitRowRun ? 'nested-implicit-rows'
  : isAnonymousGridBorderOverflowRun ? 'anonymous-grid-border-overflow'
  : isAnonymousTextMultiImageRun ? 'anonymous-text-multi-image'
  : isAnonymousTextImageRun ? 'anonymous-grid-text-image'
  : isAnonymousTextRun ? 'anonymous-grid-text'
  : isImageMultiSpanRun ? 'image-multispan-contributions'
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
const runTimestamp = new Date().toISOString().replaceAll(/[^0-9TZ]/g, '');
const runNonce = randomUUID().slice(0, 8);
const runPrefix = `c114-${runKind}-${target}-${runTimestamp}-${runNonce}`;
const runIDPrefix = `c114-${target}-${runTimestamp}-${runNonce}`;
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const evidenceDirectory = resolve(repositoryRoot, evidenceGroup
  ? `spec/internal/evidence/${evidenceGroup}-${date}/${target}-${runPrefix}`
  : isCyclicPercentRun
    ? `spec/internal/evidence/c11-4-cyclic-percentage-track-runtime-${date}/${target}-${runPrefix}`
  : isNestedGridTextMultiSpanRun
    ? `spec/internal/evidence/c11-4-nested-grid-text-multispan-runtime-${date}/${target}-${runPrefix}`
  : isNestedGridDepthAnonymousImageRun
    ? `spec/internal/evidence/c11-4-nested-depth-three-anonymous-image-runtime-${date}/${target}-${runPrefix}`
  : isAnonymousGridBorderOverflowRun
    ? `spec/internal/evidence/c11-4-anonymous-grid-border-overflow-${date}/${target}-${runPrefix}`
  : isAnonymousTextMultiImageRun
    ? `spec/internal/evidence/c11-4-anonymous-text-multi-image-${date}/${target}-${runPrefix}`
  : isMixedMultiSpanRun
    ? `spec/internal/evidence/c11-4-mixed-text-image-multispan-${date}/${target}-${runPrefix}`
  : isAnonymousTextRun
    ? `spec/internal/evidence/c11-4-anonymous-grid-text-${date}/${target}-${runPrefix}`
  : isImageMultiSpanRun
    ? `spec/internal/evidence/c11-4-image-multispan-contributions-${date}/${target}-${runPrefix}`
  : isTextMultiSpanRun
    ? `spec/internal/evidence/c11-4-text-multispan-contributions-${date}/${target}-${runPrefix}`
  : isMultiSpanRun
    ? `spec/internal/evidence/c11-4-multispan-ordering-physical-android-${date}/${target}-${runPrefix}`
    : `spec/internal/evidence/c11-4-negative-implicit-track-sizing-${date}/${target}-${runPrefix}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);

function sourceRevision() {
  const commit = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const tree = command('git', ['rev-parse', 'HEAD^{tree}']).stdout.trim();
  const trackedDiff = command('git', ['diff', '--binary', 'HEAD']).stdout;
  const dirtyPaths = command('git', ['diff', '--name-only', 'HEAD']).stdout
    .split(/\r?\n/).filter(Boolean);
  return {
    commit,
    tree,
    trackedDiffSha256: sha256(trackedDiff),
    trackedDirtyPaths: dirtyPaths,
  };
}

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
scenes.push(
  {
    id: 'mixed-text-image-span2-forward', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=0,y=0,width=61,height=20;2:node=4,x=25,y=60,width=72,height=48]',
    tracks: [25, 36, 36, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], mixedIntrinsicContributions: true,
    itemSpans: [2, 2], imageMinContentWidths: [null, 72],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
  },
  {
    id: 'mixed-text-image-span2-reversed', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=25,y=60,width=72,height=48;2:node=3,x=0,y=0,width=61,height=20]',
    tracks: [25, 36, 36, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2], mixedIntrinsicContributions: true,
    itemSpans: [2, 2], imageMinContentWidths: [72, null],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
    ],
  },
  {
    id: 'mixed-text-image-span2-before-span3', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=0,y=0,width=116.333333,height=10;'
      + '2:node=4,x=52.666667,y=60,width=72,height=48;'
      + '3:node=5,x=0,y=120,width=180,height=10]',
    tracks: [158 / 3, 191 / 3, 191 / 3, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], mixedIntrinsicContributions: true,
    itemSpans: [2, 2, 3], imageMinContentWidths: [null, 72, null],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 5, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 180, minContentHeight: 10, maxContent: 180, maxContentHeight: 10 },
    ],
  },
  {
    id: 'mixed-text-image-span3-before-span2', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=0,y=120,width=180,height=10;'
      + '2:node=4,x=52.666667,y=60,width=72,height=48;'
      + '3:node=5,x=0,y=0,width=116.333333,height=10]',
    tracks: [158 / 3, 191 / 3, 191 / 3, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [2, 3], mixedIntrinsicContributions: true,
    itemSpans: [3, 2, 2], imageMinContentWidths: [null, 72, null],
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 180, minContentHeight: 10, maxContent: 180, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 5, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 50, minContentHeight: 20, maxContent: 80, maxContentHeight: 10 },
    ],
  },
);
assert.equal(scenes.length, 154, 'C11.4 mixed text/image multi-span simulator 행렬은 154 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 simulator scene ID가 중복됐습니다.');
scenes.push({
  id: 'spanning-auto-min-flex-tracks', axis: 'Columns', boxCount: 2,
  rootFrame: 'width=100,height=30', itemFrame: 'width=100,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=30;'
    + '1:node=2,x=0,y=0,width=100,height=10]',
  tracks: [48, 48], collapsed: [false, false],
  intrinsicInputs: [
    { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 200, minContentHeight: 10, maxContent: 200, maxContentHeight: 10 },
  ],
});
assert.equal(scenes.length, 155, 'C11.4 auto-minimum flexible span 후속은 155 scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 auto-minimum flexible span scene ID는 중복되지 않아야 합니다.');
scenes.push(
  {
    id: 'anonymous-min-content-wrap', axis: 'Columns', boxCount: 1,
    rootFrame: 'width=120,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20]',
    tracks: [20, 100], collapsed: [false, false],
    anonymousTextItems: [{
      grid: 1, node: 2, sourceOrder: 0,
      source: { minContent: 20, minContentHeight: 20, maxContent: 50, maxContentHeight: 10 },
      textSources: [{ node: 2, text: 'XX XX' }],
      frame: { node: 2, x: 0, y: 0, width: 20, height: 20 },
    }],
  },
  {
    id: 'anonymous-comment-continuation', axis: 'Columns', boxCount: 1,
    rootFrame: 'width=120,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10]',
    tracks: [40, 80], collapsed: [false, false],
    anonymousTextItems: [{
      grid: 1, node: 2, sourceOrder: 0,
      source: { minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
      textSources: [{ node: 2, text: 'XX' }, { node: 3, text: 'XX' }],
      frame: { node: 2, x: 0, y: 0, width: 40, height: 10 },
    }],
  },
  {
    id: 'anonymous-runs-around-block', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=120,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;'
      + '1:node=3,x=20,y=0,width=30,height=10]',
    tracks: [20, 30, 40], collapsed: [false, false, false],
    anonymousTextItems: [
      {
        grid: 1, node: 2, sourceOrder: 0,
        source: { minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }],
        frame: { node: 2, x: 0, y: 0, width: 20, height: 10 },
      },
      {
        grid: 1, node: 4, sourceOrder: 2,
        source: { minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
        textSources: [{ node: 4, text: 'XXXX' }],
        frame: { node: 4, x: 50, y: 0, width: 40, height: 10 },
      },
    ],
  },
  {
    id: 'anonymous-hidden-child-continuation', axis: 'Columns', boxCount: 1,
    rootFrame: 'width=120,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10]',
    tracks: [40, 80], collapsed: [false, false],
    anonymousTextItems: [{
      grid: 1, node: 2, sourceOrder: 0,
      source: { minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
      textSources: [{ node: 2, text: 'XX' }, { node: 5, text: 'XX' }],
      frame: { node: 2, x: 0, y: 0, width: 40, height: 10 },
    }],
  },
  {
    id: 'whitespace-does-not-create-item', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=120,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;'
      + '1:node=2,x=0,y=0,width=10,height=10;2:node=4,x=20,y=0,width=10,height=10]',
    tracks: [20, 30, 40], collapsed: [false, false, false],
    anonymousTextItems: [],
  },
);
assert.equal(scenes.length, 160, 'C11.4 anonymous Grid text slice adds five scene이어야 합니다.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 anonymous Grid text scene ID는 중복되지 않아야 합니다.');
scenes.push(
  {
    id: 'anonymous-text-image-text', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=3,x=20,y=0,width=72,height=48]',
    tracks: [20, 36, 36, 20], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [1, 2, 1], imageMinContentWidths: [null, 72, null],
    mixedIntrinsicContributions: true, anonymousTextImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }],
        frame: { node: 2, x: 0, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 3, x: 20, y: 0, width: 72, height: 48 } },
      { grid: 1, node: 4, sourceOrder: 2,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 4, text: 'XX' }],
        frame: { node: 4, x: 92, y: 0, width: 20, height: 10 } },
    ],
  },
  {
    id: 'image-before-comment-group', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=2,x=0,y=0,width=72,height=48]',
    tracks: [36, 36, 40, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [2, 1], imageMinContentWidths: [72, null],
    mixedIntrinsicContributions: true, anonymousTextImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 2, x: 0, y: 0, width: 72, height: 48 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
        textSources: [{ node: 3, text: 'XX' }, { node: 4, text: 'XX' }],
        frame: { node: 3, x: 72, y: 0, width: 40, height: 10 } },
    ],
  },
  {
    id: 'comment-group-before-image', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=4,x=40,y=0,width=72,height=48]',
    tracks: [40, 36, 36, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [1, 2], imageMinContentWidths: [null, 72],
    mixedIntrinsicContributions: true, anonymousTextImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }, { node: 3, text: 'XX' }],
        frame: { node: 2, x: 0, y: 0, width: 40, height: 10 } },
      { grid: 1, node: 4, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 4, x: 40, y: 0, width: 72, height: 48 } },
    ],
  },
  {
    id: 'whitespace-around-image', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=3,x=0,y=0,width=72,height=48]',
    tracks: [36, 36, 20, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [2, 1], imageMinContentWidths: [72, null],
    mixedIntrinsicContributions: true, anonymousTextImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 3, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 3, sourceOrder: 0,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 3, x: 0, y: 0, width: 72, height: 48 } },
      { grid: 1, node: 4, sourceOrder: 1,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 4, text: '\n\tXX' }],
        frame: { node: 4, x: 72, y: 0, width: 20, height: 10 } },
    ],
  },
  {
    id: 'hidden-child-around-image', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=220,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;'
      + '1:node=5,x=20,y=0,width=72,height=48]',
    tracks: [20, 36, 36, 20], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [1, 2, 1], imageMinContentWidths: [null, 72, null],
    mixedIntrinsicContributions: true, anonymousTextImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 5, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 6, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }],
        frame: { node: 2, x: 0, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 5, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 5, x: 20, y: 0, width: 72, height: 48 } },
      { grid: 1, node: 6, sourceOrder: 2,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 6, text: 'XX' }],
        frame: { node: 6, x: 92, y: 0, width: 20, height: 10 } },
    ],
  },
  {
    id: 'anonymous-text-two-images-forward', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=3,x=0,y=0,width=24,height=16;'
      + '2:node=4,x=12,y=60,width=48,height=32]',
    tracks: [12, 28, 20, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [1, 2, 2],
    imageMinContentWidths: [null, 24, 48],
    mixedIntrinsicContributions: true, anonymousTextMultiImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }],
        frame: { node: 2, x: 40, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 3, x: 0, y: 0, width: 24, height: 16 } },
      { grid: 1, node: 4, sourceOrder: 2,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 4, x: 12, y: 60, width: 48, height: 32 } },
    ],
  },
  {
    id: 'two-images-anonymous-text-reversed', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=12,y=60,width=48,height=32;'
      + '2:node=4,x=0,y=0,width=24,height=16]',
    tracks: [12, 28, 20, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2], itemSpans: [2, 1, 2],
    imageMinContentWidths: [48, null, 24],
    mixedIntrinsicContributions: true, anonymousTextMultiImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 2, x: 12, y: 60, width: 48, height: 32 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 3, text: 'XX' }],
        frame: { node: 3, x: 40, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 4, sourceOrder: 2,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 4, x: 0, y: 0, width: 24, height: 16 } },
    ],
  },
  {
    id: 'anonymous-text-three-images-forward', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=3,x=0,y=0,width=24,height=16;'
      + '2:node=4,x=18,y=60,width=48,height=32;'
      + '3:node=5,x=0,y=120,width=72,height=48]',
    tracks: [18, 34, 20, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2, 3], itemSpans: [1, 2, 2, 3],
    imageMinContentWidths: [null, 24, 48, 72],
    mixedIntrinsicContributions: true, anonymousTextMultiImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 5, sourceOrder: 3, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 2, text: 'XX' }],
        frame: { node: 2, x: 52, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 3, x: 0, y: 0, width: 24, height: 16 } },
      { grid: 1, node: 4, sourceOrder: 2,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 4, x: 18, y: 60, width: 48, height: 32 } },
      { grid: 1, node: 5, sourceOrder: 3,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 5, x: 0, y: 120, width: 72, height: 48 } },
    ],
  },
  {
    id: 'three-images-anonymous-text-reversed', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=220,height=180', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;'
      + '1:node=2,x=0,y=120,width=72,height=48;'
      + '2:node=3,x=18,y=60,width=48,height=32;'
      + '3:node=5,x=0,y=0,width=24,height=16]',
    tracks: [18, 34, 20, 0], collapsed: [false, false, false, false],
    intrinsicSpans: [1, 2, 3], itemSpans: [3, 2, 1, 2],
    imageMinContentWidths: [72, 48, null, 24],
    mixedIntrinsicContributions: true, anonymousTextMultiImageContributions: true,
    intrinsicInputs: [
      { grid: 1, node: 2, sourceOrder: 0, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 3, sourceOrder: 1, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
      { grid: 1, node: 4, sourceOrder: 2, kind: 'text', fontRegistryRevision: 1,
        minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
      { grid: 1, node: 5, sourceOrder: 3, kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    ],
    anonymousTextImageItems: [
      { grid: 1, node: 2, sourceOrder: 0,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 2, x: 0, y: 120, width: 72, height: 48 } },
      { grid: 1, node: 3, sourceOrder: 1,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 3, x: 18, y: 60, width: 48, height: 32 } },
      { grid: 1, node: 4, sourceOrder: 2,
        source: { kind: 'text', fontRegistryRevision: 1, minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10 },
        textSources: [{ node: 4, text: 'XX' }],
        frame: { node: 4, x: 52, y: 0, width: 20, height: 10 } },
      { grid: 1, node: 5, sourceOrder: 3,
        source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 }, textSources: [],
        frame: { node: 5, x: 0, y: 0, width: 24, height: 16 } },
    ],
  },
);
scenes.push(...anonymousGridBorderOverflowScenes());
scenes.push(
  {
    id: 'nested-grid-span-two', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=134,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=134,height=20;'
      + '1:node=2,x=0,y=0,width=134,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=60,height=20]',
    tracks: [65, 65], collapsed: [false, false],
    axes: { Columns: { tracks: [65, 65], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [2],
    nestedGridIntrinsic: {
      outerNode: 1, nestedNode: 2, leafNodes: [3, 5],
      innerTracks: [70, 60], contributionSpan: 2, contributionValue: 134,
    },
  },
  {
    id: 'nested-grid-one-track', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=138,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=138,height=20;'
      + '1:node=2,x=0,y=0,width=134,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=60,height=20]',
    tracks: [134, 0], collapsed: [false, false],
    axes: { Columns: { tracks: [134, 0], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [1],
    nestedGridIntrinsic: {
      outerNode: 1, nestedNode: 2, leafNodes: [3, 5],
      innerTracks: [70, 60], contributionSpan: 1, contributionValue: 134,
    },
  },
  {
    id: 'nested-grid-definite-flex-control', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=120,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;'
      + '1:node=2,x=0,y=0,width=120,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=60,height=20]',
    tracks: [58, 58], collapsed: [false, false],
    axes: { Columns: { tracks: [58, 58], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [2],
    nestedGridIntrinsic: {
      outerNode: 1, nestedNode: 2, leafNodes: [3, 5], innerTracks: [70, 60],
      contributionSpan: 2, minimumContributionValue: 0,
      maximumContributionValue: 120,
    },
    pixelSamples: [
      { id: 'nested-grid-internal-gap', nodeId: 'c114-nested-grid-definite-flex-control-nested',
        xCssPx: 72, yCssPx: 10, rgba: [231, 246, 237, 255] },
      { id: 'nested-grid-inside-outer-edge', nodeId: 'c114-nested-grid-definite-flex-control-item-b',
        xCssPx: 119, yCssPx: 10, rgba: [229, 72, 77, 255] },
      { id: 'nested-grid-outside-outer-edge', nodeId: 'c114-nested-grid-definite-flex-control-item-b',
        xCssPx: 120, yCssPx: 10, rgba: [229, 72, 77, 255] },
      { id: 'nested-grid-overflow-tail', nodeId: 'c114-nested-grid-definite-flex-control-item-b',
        xCssPx: 133, yCssPx: 10, rgba: [229, 72, 77, 255] },
      { id: 'nested-grid-after-overflow', nodeId: 'c114-nested-grid-definite-flex-control-grid',
        xCssPx: 134, yCssPx: 10, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'nested-grid-text-image-span-two', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=98,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=20;'
      + '1:node=2,x=0,y=0,width=98,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=24,height=16]',
    tracks: [47, 47], collapsed: [false, false],
    axes: { Columns: { tracks: [47, 47], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [2],
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerTracks: [70, 24], contributionSpan: 2, contributionValue: 98,
    },
  },
  {
    id: 'nested-grid-text-image-one-track', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=102,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=102,height=20;'
      + '1:node=2,x=0,y=0,width=98,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=24,height=16]',
    tracks: [98, 0], collapsed: [false, false],
    axes: { Columns: { tracks: [98, 0], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [1],
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerTracks: [70, 24], contributionSpan: 1, contributionValue: 98,
    },
  },
  {
    id: 'nested-grid-text-image-definite-flex-control', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=120,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;'
      + '1:node=2,x=0,y=0,width=120,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=0,width=24,height=16]',
    tracks: [58, 58], collapsed: [false, false],
    axes: { Columns: { tracks: [58, 58], collapsed: [false, false] } },
    gaps: { Columns: 4 }, intrinsicSpans: [2],
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerTracks: [70, 24], contributionSpan: 2,
      minimumContributionValue: 0, maximumContributionValue: 120,
    },
  },
  {
    id: 'nested-grid-text-image-row-span-two', axis: 'Rows', boxCount: 4,
    rootFrame: 'width=98,height=40', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=40;'
      + '1:node=2,x=0,y=0,width=98,height=40;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=24,width=24,height=16]',
    tracks: [18, 18], collapsed: [false, false],
    axes: { Rows: { tracks: [18, 18], collapsed: [false, false] } },
    gaps: { Rows: 4 }, intrinsicSpansByAxis: { Rows: [2] },
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerAxis: 'Rows', contributionAxis: 'Rows', innerTracks: [20, 16],
      contributionSpan: 2, contributionValue: 40,
    },
  },
  {
    id: 'nested-grid-text-image-row-one-track', axis: 'Rows', boxCount: 4,
    rootFrame: 'width=98,height=44', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=44;'
      + '1:node=2,x=0,y=0,width=98,height=40;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=24,width=24,height=16]',
    tracks: [40, 0], collapsed: [false, false],
    axes: { Rows: { tracks: [40, 0], collapsed: [false, false] } },
    gaps: { Rows: 4 }, intrinsicSpansByAxis: { Rows: [1] },
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerAxis: 'Rows', contributionAxis: 'Rows', innerTracks: [20, 16],
      contributionSpan: 1, contributionValue: 40,
    },
  },
  {
    id: 'nested-grid-text-image-row-definite-flex-control', axis: 'Rows', boxCount: 4,
    rootFrame: 'width=98,height=60', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=60;'
      + '1:node=2,x=0,y=0,width=98,height=60;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=5,x=74,y=24,width=24,height=16]',
    tracks: [28, 28], collapsed: [false, false],
    axes: { Rows: { tracks: [28, 28], collapsed: [false, false] } },
    gaps: { Rows: 4 }, intrinsicSpansByAxis: { Rows: [2] },
    nestedGridTextImageIntrinsic: {
      outerNode: 1, nestedNode: 2, textNode: 3, imageNode: 5,
      innerAxis: 'Rows', contributionAxis: 'Rows', innerTracks: [20, 16],
      contributionSpan: 2, minimumContributionValue: 0, maximumContributionValue: 60,
    },
  },
  {
    id: 'nested-grid-implicit-row-span-two', axis: 'Rows', boxCount: 5,
    rootFrame: 'width=24,height=40', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=40;'
      + '1:node=2,x=0,y=0,width=24,height=40;'
      + '2:node=3,x=0,y=0,width=24,height=8;'
      + '3:node=4,x=0,y=12,width=4,height=8;'
      + '4:node=5,x=0,y=24,width=24,height=16]',
    tracks: [18, 18], collapsed: [false, false],
    axes: { Rows: { tracks: [18, 18], collapsed: [false, false] } },
    gaps: { Rows: 4 }, intrinsicSpansByAxis: { Rows: [2] },
    nestedGridImplicitRows: {
      outerNode: 1, nestedNode: 2, imageNode: 5,
      outerRows: [18, 18], innerRows: [8, 8, 16], contributionSpan: 2,
      contributionValue: 40,
    },
  },
  {
    id: 'nested-grid-implicit-row-one-row', axis: 'Rows', boxCount: 5,
    rootFrame: 'width=24,height=44', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=44;'
      + '1:node=2,x=0,y=0,width=24,height=40;'
      + '2:node=3,x=0,y=0,width=24,height=8;'
      + '3:node=4,x=0,y=12,width=4,height=8;'
      + '4:node=5,x=0,y=24,width=24,height=16]',
    tracks: [40, 0], collapsed: [false, false],
    axes: { Rows: { tracks: [40, 0], collapsed: [false, false] } },
    gaps: { Rows: 4 }, intrinsicSpansByAxis: { Rows: [1] },
    nestedGridImplicitRows: {
      outerNode: 1, nestedNode: 2, imageNode: 5,
      outerRows: [40, 0], innerRows: [8, 8, 16], contributionSpan: 1,
      contributionValue: 40,
    },
  },
  {
    id: 'nested-depth-one-min-content', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=70,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;'
      + '1:node=2,x=0,y=0,width=70,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20]',
    tracks: [70], collapsed: [false],
    axes: {
      Columns: { tracks: [70], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 2, node: 3, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2], leafNode: 3,
      width: 70, height: 20, contributionKind: 'MinContent',
    },
  },
  {
    id: 'nested-depth-two-min-content', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=70,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;'
      + '1:node=2,x=0,y=0,width=70,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=4,x=0,y=0,width=70,height=20]',
    tracks: [70], collapsed: [false],
    axes: {
      Columns: { tracks: [70], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 3, node: 4, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3], leafNode: 4,
      width: 70, height: 20, contributionKind: 'MinContent',
    },
  },
  {
    id: 'nested-depth-one-max-content', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=100,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;'
      + '1:node=2,x=0,y=0,width=100,height=10;'
      + '2:node=3,x=0,y=0,width=100,height=10]',
    tracks: [100], collapsed: [false],
    axes: {
      Columns: { tracks: [100], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 2, node: 3, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2], leafNode: 3,
      width: 100, height: 10, contributionKind: 'MaxContent',
    },
  },
  {
    id: 'nested-depth-two-max-content', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=100,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;'
      + '1:node=2,x=0,y=0,width=100,height=10;'
      + '2:node=3,x=0,y=0,width=100,height=10;'
      + '3:node=4,x=0,y=0,width=100,height=10]',
    tracks: [100], collapsed: [false],
    axes: {
      Columns: { tracks: [100], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 3, node: 4, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3], leafNode: 4,
      width: 100, height: 10, contributionKind: 'MaxContent',
    },
  },
  {
    id: 'nested-depth-one-min-content-two-axis', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=70,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;'
      + '1:node=2,x=0,y=0,width=70,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20]',
    tracks: [70], collapsed: [false],
    axes: {
      Columns: { tracks: [70], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 2, node: 3, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2], leafNode: 3,
      width: 70, height: 20, contributionKind: 'MinContent', twoAxis: true,
    },
  },
  {
    id: 'nested-depth-two-min-content-two-axis', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=70,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;'
      + '1:node=2,x=0,y=0,width=70,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=4,x=0,y=0,width=70,height=20]',
    tracks: [70], collapsed: [false],
    axes: {
      Columns: { tracks: [70], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 3, node: 4, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3], leafNode: 4,
      width: 70, height: 20, contributionKind: 'MinContent', twoAxis: true,
    },
  },
  {
    id: 'nested-depth-one-max-content-two-axis', axis: 'Columns', boxCount: 3,
    rootFrame: 'width=100,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;'
      + '1:node=2,x=0,y=0,width=100,height=10;'
      + '2:node=3,x=0,y=0,width=100,height=10]',
    tracks: [100], collapsed: [false],
    axes: {
      Columns: { tracks: [100], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 2, node: 3, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2], leafNode: 3,
      width: 100, height: 10, contributionKind: 'MaxContent', twoAxis: true,
    },
  },
  {
    id: 'nested-depth-two-max-content-two-axis', axis: 'Columns', boxCount: 4,
    rootFrame: 'width=100,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;'
      + '1:node=2,x=0,y=0,width=100,height=10;'
      + '2:node=3,x=0,y=0,width=100,height=10;'
      + '3:node=4,x=0,y=0,width=100,height=10]',
    tracks: [100], collapsed: [false],
    axes: {
      Columns: { tracks: [100], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 3, node: 4, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3], leafNode: 4,
      width: 100, height: 10, contributionKind: 'MaxContent', twoAxis: true,
    },
  },
  {
    id: 'nested-depth-three-min-content-two-axis', axis: 'Columns', boxCount: 5,
    rootFrame: 'width=70,height=20', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;'
      + '1:node=2,x=0,y=0,width=70,height=20;'
      + '2:node=3,x=0,y=0,width=70,height=20;'
      + '3:node=4,x=0,y=0,width=70,height=20;'
      + '4:node=5,x=0,y=0,width=70,height=20]',
    tracks: [70], collapsed: [false],
    axes: {
      Columns: { tracks: [70], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 4, node: 5, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3, 4], leafNode: 5,
      width: 70, height: 20, contributionKind: 'MinContent', twoAxis: true,
    },
  },
  {
    id: 'nested-depth-three-max-content-two-axis', axis: 'Columns', boxCount: 5,
    rootFrame: 'width=100,height=10', itemFrame: null,
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;'
      + '1:node=2,x=0,y=0,width=100,height=10;'
      + '2:node=3,x=0,y=0,width=100,height=10;'
      + '3:node=4,x=0,y=0,width=100,height=10;'
      + '4:node=5,x=0,y=0,width=100,height=10]',
    tracks: [100], collapsed: [false],
    axes: {
      Columns: { tracks: [100], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: [{
      grid: 4, node: 5, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20, maxContent: 100, maxContentHeight: 10,
    }],
    nestedGridDepth: {
      gridNodes: [1, 2, 3, 4], leafNode: 5,
      width: 100, height: 10, contributionKind: 'MaxContent', twoAxis: true,
    },
  },
);
for (const imageBeforeText of [false, true]) {
  for (const minContent of [true, false]) {
    const intrinsic = minContent ? 'min-content' : 'max-content';
    const id = `depth-three-${intrinsic}-${imageBeforeText ? 'image-before-text' : 'text-before-image'}`;
    const width = minContent ? 94 : 124;
    const height = minContent ? 20 : 16;
    const textWidth = minContent ? 70 : 100;
    const textHeight = minContent ? 20 : 10;
    const textNode = 6;
    const imageNode = 5;
    const textSourceOrder = imageBeforeText ? 1 : 0;
    const imageSourceOrder = imageBeforeText ? 0 : 1;
    const textX = imageBeforeText ? 24 : 0;
    const imageX = imageBeforeText ? 0 : textWidth;
    const gridNodes = [1, 2, 3, 4];
    const textSource = {
      kind: 'text', fontRegistryRevision: 1,
      minContent: 70, minContentHeight: 20,
      maxContent: 100, maxContentHeight: 10,
    };
    const imageSource = { kind: 'replaced', aspectRatio: null, width: 24, height: 16 };
    const textFrame = { node: textNode, x: textX, y: 0, width: textWidth, height: textHeight };
    const imageFrame = { node: imageNode, x: imageX, y: 0, width: 24, height: 16 };
    const textItem = {
      grid: 4, node: textNode, sourceOrder: textSourceOrder,
      source: textSource,
      textSources: [{ node: textNode, text: 'XX XXXXXXX' }],
      frame: textFrame,
    };
    const imageItem = {
      grid: 4, node: imageNode, sourceOrder: imageSourceOrder,
      source: imageSource, textSources: [], frame: imageFrame,
    };
    const orderedItems = imageBeforeText ? [imageItem, textItem] : [textItem, imageItem];
    const gridFrame = `width=${width},height=${height}`;
    const allGridFrames = gridNodes.map((node, index) => (
      `${index}:node=${node},x=0,y=0,width=${width},height=${height}`
    ));
    allGridFrames.push(`4:node=${imageNode},x=${imageX},y=0,width=24,height=16`);
    const contributionKind = minContent ? 'MinContent' : 'MaxContent';
    scenes.push({
      id,
      axis: 'Columns',
      boxCount: 5,
      rootFrame: gridFrame,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${allGridFrames.join(';')}]`,
      tracks: [width],
      collapsed: [false],
      axes: {
        Columns: { tracks: [width], collapsed: [false] },
        Rows: { tracks: [height], collapsed: [false] },
      },
      gaps: { Columns: 0, Rows: 0 },
      nestedGridAnonymousTextImage: true,
      intrinsicInputs: orderedItems.map((item) => ({
        grid: item.grid,
        node: item.node,
        sourceOrder: item.sourceOrder,
        kind: item.source.kind,
        ...(item.source.kind === 'text'
          ? {
            fontRegistryRevision: item.source.fontRegistryRevision,
            minContent: item.source.minContent,
            minContentHeight: item.source.minContentHeight,
            maxContent: item.source.maxContent,
            maxContentHeight: item.source.maxContentHeight,
          }
          : {
            aspectRatio: item.source.aspectRatio,
            width: item.source.width,
            height: item.source.height,
          }),
      })),
      anonymousTextImageItems: orderedItems,
      nestedGridDepth: {
        gridNodes,
        leafNode: textNode,
        width,
        height,
        contributionKind,
        twoAxis: true,
        innerColumns: imageBeforeText ? [24, textWidth] : [textWidth, 24],
        directItemContributions: [
          { node: textNode, width: minContent ? 70 : 100, height: minContent ? 20 : 10 },
          { node: imageNode, width: 24, height: 16 },
        ],
      },
    });
  }
}
const nestedTextMultiSpanItems = {
  a: { suffix: 'a', text: 'XX XXXXX', column: 1, span: 2, row: 1, minContent: 50, maxContent: 80, minContentHeight: 20, maxContentHeight: 10 },
  b: { suffix: 'b', text: 'XXXX XXXXXXX', column: 2, span: 2, row: 2, minContent: 70, maxContent: 120, minContentHeight: 20, maxContentHeight: 10 },
  c: { suffix: 'c', text: 'XXXXXXXXXXXXXXXXXX', column: 1, span: 3, row: 3, minContent: 180, maxContent: 180, minContentHeight: 10, maxContentHeight: 10 },
};
const nestedTextMultiSpanCases = [
  { id: 'nested-text-span2-forward', order: ['a', 'b'], columns: [25, 35, 35, 0], width: 95 },
  { id: 'nested-text-span2-reversed', order: ['b', 'a'], columns: [25, 35, 35, 0], width: 95 },
  { id: 'nested-text-span2-before-span3', order: ['a', 'b', 'c'], columns: [160 / 3, 190 / 3, 190 / 3, 0], width: 180 },
  { id: 'nested-text-span3-before-span2', order: ['c', 'b', 'a'], columns: [160 / 3, 190 / 3, 190 / 3, 0], width: 180 },
];
for (const spec of nestedTextMultiSpanCases) {
  const gridNodes = [1, 2, 3, 4];
  const items = spec.order.map((suffix, index) => ({
    ...nestedTextMultiSpanItems[suffix],
    node: 5 + index * 2,
    sourceOrder: index,
  }));
  const frames = gridNodes.map((node, order) => (
    `${order}:node=${node},x=0,y=0,width=${spec.width},height=60`
  ));
  for (const item of items) {
    const x = spec.columns.slice(0, item.column - 1).reduce((sum, track) => sum + track, 0);
    const width = spec.columns.slice(item.column - 1, item.column + item.span - 1)
      .reduce((sum, track) => sum + track, 0);
    frames.push(`${frames.length}:node=${item.node},x=${x},y=${(item.row - 1) * 20},width=${width},height=${spec.width === 95 ? 20 : 10}`);
  }
  scenes.push({
    id: spec.id,
    axis: 'Columns',
    boxCount: gridNodes.length + items.length,
    rootFrame: `width=${spec.width},height=60`,
    itemFrame: null,
    nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
    tracks: [spec.width],
    collapsed: [false],
    axes: {
      Columns: { tracks: [spec.width], collapsed: [false] },
      Rows: { tracks: [60], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    intrinsicInputs: items.map((item) => ({
      grid: 4,
      node: item.node,
      sourceOrder: item.sourceOrder,
      kind: 'text',
      fontRegistryRevision: 1,
      minContent: item.minContent,
      minContentHeight: item.minContentHeight,
      maxContent: item.maxContent,
      maxContentHeight: item.maxContentHeight,
    })),
    nestedGridTextMultiSpan: {
      gridNodes,
      width: spec.width,
      innerColumns: spec.columns,
      items: items.map(({ node, span, minContent }) => ({ node, span, minContent })),
    },
  });
}
scenes.push(
  {
    id: 'cyclic-percent-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=24,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [12],
    collapsed: [false],
    axes: {
      Columns: { tracks: [12], collapsed: [false] },
      Rows: { tracks: [16], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-column-inside-track', nodeId: 'c114-cyclic-percent-column-item',
        xCssPx: 11, yCssPx: 8, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-column-past-track', nodeId: 'c114-cyclic-percent-column-item',
        xCssPx: 12, yCssPx: 8, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'cyclic-percent-row',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=24,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [8],
    collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [8], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-row-inside-track', nodeId: 'c114-cyclic-percent-row-item',
        xCssPx: 12, yCssPx: 7, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-row-past-track', nodeId: 'c114-cyclic-percent-row-item',
        xCssPx: 12, yCssPx: 8, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'definite-percent-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [20],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [16], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-column-inside-track', nodeId: 'c114-definite-percent-column-item',
        xCssPx: 19, yCssPx: 8, rgba: [229, 72, 77, 255] },
      { id: 'definite-column-past-track', nodeId: 'c114-definite-percent-column-item',
        xCssPx: 20, yCssPx: 8, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'cyclic-minmax-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=24,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [12],
    collapsed: [false],
    axes: {
      Columns: { tracks: [12], collapsed: [false] },
      Rows: { tracks: [16], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-minmax-column-inside-track', nodeId: 'c114-cyclic-minmax-column-item',
        xCssPx: 11, yCssPx: 8, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-column-past-track', nodeId: 'c114-cyclic-minmax-column-item',
        xCssPx: 18, yCssPx: 8, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'cyclic-minmax-row',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=24,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [8],
    collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [8], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-minmax-row-inside-track', nodeId: 'c114-cyclic-minmax-row-item',
        xCssPx: 12, yCssPx: 7, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-row-past-track', nodeId: 'c114-cyclic-minmax-row-item',
        xCssPx: 12, yCssPx: 12, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'definite-minmax-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=16',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=16;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [20],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [16], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-minmax-column-inside-track', nodeId: 'c114-definite-minmax-column-item',
        xCssPx: 19, yCssPx: 8, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-column-past-track', nodeId: 'c114-definite-minmax-column-item',
        xCssPx: 22, yCssPx: 8, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'cyclic-fit-content-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=50,height=20',
    itemFrame: 'width=25,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;'
      + '1:node=2,x=0,y=0,width=25,height=20]',
    tracks: [25],
    collapsed: [false],
    axes: {
      Columns: { tracks: [25], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-fit-content-column-inside-item', nodeId: 'c114-cyclic-fit-content-column-item',
        xCssPx: 24, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-fit-content-column-past-item', nodeId: 'c114-cyclic-fit-content-column-item',
        xCssPx: 26, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'cyclic-fit-content-row',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=20,height=40',
    itemFrame: 'width=20,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;'
      + '1:node=2,x=0,y=0,width=20,height=20]',
    tracks: [20],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-fit-content-row-inside-item', nodeId: 'c114-cyclic-fit-content-row-item',
        xCssPx: 19, yCssPx: 19, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-fit-content-row-past-item-no-text', nodeId: 'c114-cyclic-fit-content-row-item',
        xCssPx: 19, yCssPx: 39, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'definite-fit-content-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=20',
    itemFrame: 'width=20,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;'
      + '1:node=2,x=0,y=0,width=20,height=20]',
    tracks: [20],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-fit-content-column-inside-item', nodeId: 'c114-definite-fit-content-column-item',
        xCssPx: 19, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-fit-content-column-past-item', nodeId: 'c114-definite-fit-content-column-item',
        xCssPx: 21, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'cyclic-fit-content-calc-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=50,height=20',
    itemFrame: 'width=29,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;'
      + '1:node=2,x=0,y=0,width=29,height=20]',
    tracks: [29],
    collapsed: [false],
    axes: {
      Columns: { tracks: [29], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-fit-content-calc-column-inside-item',
        nodeId: 'c114-cyclic-fit-content-calc-column-item',
        xCssPx: 28, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-fit-content-calc-column-past-item',
        nodeId: 'c114-cyclic-fit-content-calc-column-item',
        xCssPx: 30, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'cyclic-fit-content-calc-row',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=20,height=40',
    itemFrame: 'width=20,height=22',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;'
      + '1:node=2,x=0,y=0,width=20,height=22]',
    tracks: [22],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [22], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-fit-content-calc-row-inside-item',
        nodeId: 'c114-cyclic-fit-content-calc-row-item',
        xCssPx: 19, yCssPx: 21, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-fit-content-calc-row-past-item',
        nodeId: 'c114-cyclic-fit-content-calc-row-item',
        xCssPx: 19, yCssPx: 23, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'definite-fit-content-calc-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=20',
    itemFrame: 'width=24,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;'
      + '1:node=2,x=0,y=0,width=24,height=20]',
    tracks: [24],
    collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-fit-content-calc-column-inside-item',
        nodeId: 'c114-definite-fit-content-calc-column-item',
        xCssPx: 23, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-fit-content-calc-column-past-item',
        nodeId: 'c114-definite-fit-content-calc-column-item',
        xCssPx: 25, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'definite-fit-content-calc-small-limit',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=10,height=20',
    itemFrame: 'width=9,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=10,height=20;'
      + '1:node=2,x=0,y=0,width=9,height=20]',
    tracks: [9],
    collapsed: [false],
    axes: {
      Columns: { tracks: [9], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-fit-content-calc-small-limit-inside-item',
        nodeId: 'c114-definite-fit-content-calc-small-limit-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-fit-content-calc-small-limit-text-overflow',
        nodeId: 'c114-definite-fit-content-calc-small-limit-item',
        xCssPx: 15, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-fit-content-calc-small-limit-past-text',
        nodeId: 'c114-definite-fit-content-calc-small-limit-item',
        xCssPx: 25, yCssPx: 5, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'cyclic-minmax-calc-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=50,height=20',
    itemFrame: 'width=29,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;'
      + '1:node=2,x=0,y=0,width=29,height=20]',
    tracks: [29],
    collapsed: [false],
    axes: {
      Columns: { tracks: [29], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    pixelSamples: [
      { id: 'cyclic-minmax-calc-column-inside-item',
        nodeId: 'c114-cyclic-minmax-calc-column-item',
        xCssPx: 28, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-column-past-item',
        nodeId: 'c114-cyclic-minmax-calc-column-item',
        xCssPx: 30, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'cyclic-minmax-calc-row',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=20,height=40',
    itemFrame: 'width=20,height=22',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;'
      + '1:node=2,x=0,y=0,width=20,height=22]',
    tracks: [22],
    collapsed: [false],
    axes: {
      Columns: { tracks: [20], collapsed: [false] },
      Rows: { tracks: [22], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    pixelSamples: [
      { id: 'cyclic-minmax-calc-row-inside-item',
        nodeId: 'c114-cyclic-minmax-calc-row-item',
        xCssPx: 19, yCssPx: 21, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-row-past-item',
        nodeId: 'c114-cyclic-minmax-calc-row-item',
        xCssPx: 19, yCssPx: 23, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'definite-minmax-calc-column',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=40,height=20',
    itemFrame: 'width=24,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;'
      + '1:node=2,x=0,y=0,width=24,height=20]',
    tracks: [24],
    collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    pixelSamples: [
      { id: 'definite-minmax-calc-column-inside-item',
        nodeId: 'c114-definite-minmax-calc-column-item',
        xCssPx: 23, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-column-past-item',
        nodeId: 'c114-definite-minmax-calc-column-item',
        xCssPx: 25, yCssPx: 5, rgba: [16, 24, 32, 255] },
    ],
  },
  {
    id: 'definite-minmax-calc-small-limit',
    axis: 'Columns',
    boxCount: 2,
    rootFrame: 'width=10,height=20',
    itemFrame: 'width=9,height=20',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=10,height=20;'
      + '1:node=2,x=0,y=0,width=9,height=20]',
    tracks: [9],
    collapsed: [false],
    axes: {
      Columns: { tracks: [9], collapsed: [false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    pixelSamples: [
      { id: 'definite-minmax-calc-small-limit-inside-item',
        nodeId: 'c114-definite-minmax-calc-small-limit-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-small-limit-text-overflow',
        nodeId: 'c114-definite-minmax-calc-small-limit-item',
        xCssPx: 15, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-small-limit-past-text',
        nodeId: 'c114-definite-minmax-calc-small-limit-item',
        xCssPx: 25, yCssPx: 5, rgba: [0, 0, 0, 255] },
    ],
  },
  ...[
    {
      id: 'cyclic-calc-column',
      boxCount: 2,
      rootFrame: 'width=24,height=16',
      itemFrame: 'width=24,height=16',
      nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
        + '1:node=2,x=0,y=0,width=24,height=16]',
      axes: {
        Columns: { tracks: [16], collapsed: [false] },
        Rows: { tracks: [16], collapsed: [false] },
      },
      gaps: { Columns: 0, Rows: 0 },
      pixelSamples: [
        { id: 'cyclic-calc-column-inside-track', nodeId: 'c114-cyclic-calc-column-item',
          xCssPx: 15, yCssPx: 8, rgba: [229, 72, 77, 255] },
        { id: 'cyclic-calc-column-past-track', nodeId: 'c114-cyclic-calc-column-item',
          xCssPx: 16, yCssPx: 8, rgba: [229, 72, 77, 255] },
      ],
    },
    {
      id: 'cyclic-calc-row',
      boxCount: 2,
      rootFrame: 'width=24,height=16',
      itemFrame: 'width=24,height=16',
      nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
        + '1:node=2,x=0,y=0,width=24,height=16]',
      axes: {
        Columns: { tracks: [24], collapsed: [false] },
        Rows: { tracks: [10], collapsed: [false] },
      },
      gaps: { Columns: 0, Rows: 0 },
      pixelSamples: [
        { id: 'cyclic-calc-row-inside-track', nodeId: 'c114-cyclic-calc-row-item',
          xCssPx: 12, yCssPx: 9, rgba: [229, 72, 77, 255] },
        { id: 'cyclic-calc-row-past-track', nodeId: 'c114-cyclic-calc-row-item',
          xCssPx: 12, yCssPx: 10, rgba: [229, 72, 77, 255] },
      ],
    },
    {
      id: 'cyclic-calc-two-axis',
      boxCount: 2,
      rootFrame: 'width=24,height=16',
      itemFrame: 'width=24,height=16',
      nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;'
        + '1:node=2,x=0,y=0,width=24,height=16]',
      axes: {
        Columns: { tracks: [16], collapsed: [false] },
        Rows: { tracks: [10], collapsed: [false] },
      },
      gaps: { Columns: 0, Rows: 0 },
      pixelSamples: [
        { id: 'cyclic-calc-two-axis-inside-tracks', nodeId: 'c114-cyclic-calc-two-axis-item',
          xCssPx: 15, yCssPx: 9, rgba: [229, 72, 77, 255] },
        { id: 'cyclic-calc-two-axis-past-both', nodeId: 'c114-cyclic-calc-two-axis-item',
          xCssPx: 16, yCssPx: 10, rgba: [229, 72, 77, 255] },
      ],
    },
    {
      id: 'definite-calc-control',
      boxCount: 2,
      rootFrame: 'width=40,height=20',
      itemFrame: 'width=24,height=16',
      nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;'
        + '1:node=2,x=0,y=0,width=24,height=16]',
      axes: {
        Columns: { tracks: [24], collapsed: [false] },
        Rows: { tracks: [12], collapsed: [false] },
      },
      gaps: { Columns: 0, Rows: 0 },
      pixelSamples: [
        { id: 'definite-calc-control-inside-track', nodeId: 'c114-definite-calc-control-item',
          xCssPx: 23, yCssPx: 11, rgba: [229, 72, 77, 255] },
        { id: 'definite-calc-control-past-row', nodeId: 'c114-definite-calc-control-item',
          xCssPx: 5, yCssPx: 12, rgba: [229, 72, 77, 255] },
      ],
    },
  ],
  ...[
    'cyclic-percent-two-axis-multitrack',
    'cyclic-percent-columns-multitrack',
    'cyclic-percent-rows-multitrack',
    'definite-percent-two-axis-multitrack',
  ].map((id) => ({
    id,
    boxCount: 3,
    rootFrame: 'width=40,height=20',
    itemFrame: 'width=20,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;'
      + '1:node=2,x=0,y=0,width=20,height=10;'
      + '2:node=4,x=30,y=15,width=20,height=10]',
    axes: {
      Columns: {
        tracks: id === 'cyclic-percent-rows-multitrack' ? [20, 10, 10] : [20, 10, 20],
        collapsed: [false, false, false],
      },
      Rows: {
        tracks: id === 'cyclic-percent-columns-multitrack' ? [10, 5, 5] : [10, 5, 10],
        collapsed: [false, false, false],
      },
    },
    gaps: { Columns: 0, Rows: 0 },
    pixelSamples: [
      { id: `${id}-text-red`, nodeId: `c114-${id}-text`,
        xCssPx: 10, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: `${id}-overflow-inside-grid`, nodeId: `c114-${id}-overflow`,
        xCssPx: 35, yCssPx: 17, rgba: [32, 191, 107, 255] },
      { id: `${id}-overflow-past-right`, nodeId: `c114-${id}-overflow`,
        xCssPx: 45, yCssPx: 18, rgba: [32, 191, 107, 255] },
      { id: `${id}-overflow-past-bottom`, nodeId: `c114-${id}-overflow`,
        xCssPx: 35, yCssPx: 22, rgba: [32, 191, 107, 255] },
      { id: `${id}-overflow-past-both`, nodeId: `c114-${id}-overflow`,
        xCssPx: 45, yCssPx: 22, rgba: [32, 191, 107, 255] },
    ],
  })),
);
scenes.push(
  {
    id: 'cyclic-minmax-calc-min-column', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=14,height=14', itemFrame: 'width=24,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;'
      + '1:node=2,x=2,y=2,width=24,height=10]',
    tracks: [24], collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-minmax-calc-min-column-left-border',
        nodeId: 'c114-cyclic-minmax-calc-min-column-grid',
        xCssPx: 1, yCssPx: 5, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-min-column-item-inside',
        nodeId: 'c114-cyclic-minmax-calc-min-column-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-column-right-border-overpaint',
        nodeId: 'c114-cyclic-minmax-calc-min-column-item',
        xCssPx: 13, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-column-outside-root',
        nodeId: 'c114-cyclic-minmax-calc-min-column-item',
        xCssPx: 16, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-column-past-item',
        nodeId: 'c114-cyclic-minmax-calc-min-column-grid',
        xCssPx: 27, yCssPx: 5, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'cyclic-minmax-calc-min-row', axis: 'Rows', boxCount: 2,
    rootFrame: 'width=14,height=14', itemFrame: 'width=10,height=22',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;'
      + '1:node=2,x=2,y=2,width=10,height=22]',
    tracks: [22], collapsed: [false],
    axes: {
      Columns: { tracks: [10], collapsed: [false] },
      Rows: { tracks: [22], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'cyclic-minmax-calc-min-row-left-border',
        nodeId: 'c114-cyclic-minmax-calc-min-row-grid',
        xCssPx: 1, yCssPx: 5, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-min-row-item-inside',
        nodeId: 'c114-cyclic-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-row-bottom-border-overpaint',
        nodeId: 'c114-cyclic-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 13, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-row-outside-root',
        nodeId: 'c114-cyclic-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 16, rgba: [229, 72, 77, 255] },
      { id: 'cyclic-minmax-calc-min-row-past-item',
        nodeId: 'c114-cyclic-minmax-calc-min-row-grid',
        xCssPx: 5, yCssPx: 25, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'definite-minmax-calc-min-column', axis: 'Columns', boxCount: 2,
    rootFrame: 'width=14,height=14', itemFrame: 'width=24,height=10',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;'
      + '1:node=2,x=2,y=2,width=24,height=10]',
    tracks: [24], collapsed: [false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-minmax-calc-min-column-left-border',
        nodeId: 'c114-definite-minmax-calc-min-column-grid',
        xCssPx: 1, yCssPx: 5, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-min-column-item-inside',
        nodeId: 'c114-definite-minmax-calc-min-column-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-column-right-border-overpaint',
        nodeId: 'c114-definite-minmax-calc-min-column-item',
        xCssPx: 13, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-column-outside-root',
        nodeId: 'c114-definite-minmax-calc-min-column-item',
        xCssPx: 16, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-column-past-item',
        nodeId: 'c114-definite-minmax-calc-min-column-grid',
        xCssPx: 27, yCssPx: 5, rgba: [0, 0, 0, 255] },
    ],
  },
  {
    id: 'definite-minmax-calc-min-row', axis: 'Rows', boxCount: 2,
    rootFrame: 'width=14,height=14', itemFrame: 'width=10,height=22',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;'
      + '1:node=2,x=2,y=2,width=10,height=22]',
    tracks: [22], collapsed: [false],
    axes: {
      Columns: { tracks: [10], collapsed: [false] },
      Rows: { tracks: [22], collapsed: [false] },
    },
    pixelSamples: [
      { id: 'definite-minmax-calc-min-row-left-border',
        nodeId: 'c114-definite-minmax-calc-min-row-grid',
        xCssPx: 1, yCssPx: 5, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-min-row-item-inside',
        nodeId: 'c114-definite-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 5, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-row-bottom-border-overpaint',
        nodeId: 'c114-definite-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 13, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-row-outside-root',
        nodeId: 'c114-definite-minmax-calc-min-row-item',
        xCssPx: 5, yCssPx: 16, rgba: [229, 72, 77, 255] },
      { id: 'definite-minmax-calc-min-row-past-item',
        nodeId: 'c114-definite-minmax-calc-min-row-grid',
        xCssPx: 5, yCssPx: 25, rgba: [0, 0, 0, 255] },
    ],
  },
);
const minimumTextInput = [{
  grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
  minContent: 20, minContentHeight: 20, maxContent: 50, maxContentHeight: 10,
}];
const minimumPixelSamples = (id, points) => points.map(([suffix, node, x, y, rgba]) => ({
  id: `${id}-${suffix}`,
  nodeId: `c114-${id}-${node}`,
  xCssPx: x,
  yCssPx: y,
  rgba,
}));
const minmaxCalcMinimumScene = ({ id, root, item, columns, rows, points }) => ({
  id,
  boxCount: 2,
  rootFrame: `width=${root[0]},height=${root[1]}`,
  itemFrame: `width=${item[2]},height=${item[3]}`,
  nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};`
    + `1:node=2,x=2,y=2,width=${item[2]},height=${item[3]}]`,
  axes: {
    Columns: { tracks: [columns], collapsed: [false] },
    Rows: { tracks: [rows], collapsed: [false] },
  },
  gaps: { Columns: 0, Rows: 0 },
  intrinsicInputs: minimumTextInput,
  pixelSamples: minimumPixelSamples(id, points),
});
const spanRowsTextInput = [{
  grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
  minContent: 10, minContentHeight: 20, maxContent: 30, maxContentHeight: 10,
}];
const minmaxCalcMinimumSpanScene = ({
  id, root, item, columns, rows, gaps, points, axis, intrinsicInputs,
}) => ({
  id,
  axis,
  boxCount: 2,
  rootFrame: `width=${root[0]},height=${root[1]}`,
  itemFrame: `width=${item[2]},height=${item[3]}`,
  nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};`
    + `1:node=2,x=2,y=2,width=${item[2]},height=${item[3]}]`,
  axes: {
    Columns: { tracks: columns, collapsed: columns.map(() => false) },
    Rows: { tracks: rows, collapsed: rows.map(() => false) },
  },
  gaps,
  intrinsicInputs,
  intrinsicSpans: [2],
  intrinsicSpansByAxis: axis === 'Columns'
    ? { Columns: [2], Rows: [] }
    : { Columns: [], Rows: [2] },
  pixelSamples: minimumPixelSamples(id, points),
});
const twoAxisSpanTextInput = (minContentHeight) => [{
  grid: 1, node: 2, sourceOrder: 0, kind: 'text', fontRegistryRevision: 1,
  minContent: 10, minContentHeight, maxContent: 60, maxContentHeight: 10,
}];
const twoAxisSpanTextScene = ({ id, root, item, columns, rows, minContentHeight, points }) => ({
  id,
  axis: 'Rows',
  boxCount: 2,
  rootFrame: `width=${root[0]},height=${root[1]}`,
  itemFrame: `width=${item[2]},height=${item[3]}`,
  nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};`
    + `1:node=2,x=2,y=2,width=${item[2]},height=${item[3]}]`,
  axes: {
    Columns: { tracks: columns, collapsed: [false, false] },
    Rows: { tracks: rows, collapsed: [false, false] },
  },
  gaps: { Columns: 2, Rows: 2 },
  intrinsicInputs: twoAxisSpanTextInput(minContentHeight),
  intrinsicSpans: [2],
  intrinsicSpansByAxis: { Columns: [2], Rows: [2] },
  twoAxisSpanningTextContributions: {
    Columns: { minContent: 10, maxContent: 60 },
    Rows: { minContent: item[3], maxContent: item[3] },
  },
  columnSizingFeedsRows: true,
  pixelSamples: [
    { id: `${id}-left-border`, nodeId: `c114-${id}-grid`,
      xCssPx: 1, yCssPx: 6, rgba: [240, 198, 116, 255] },
    { id: `${id}-item-inside`, nodeId: `c114-${id}-item`,
      xCssPx: 5, yCssPx: 6, rgba: [229, 72, 77, 255] },
    ...points,
  ],
});
const crossAxisSpanGroupScene = ({ id, root, columns, rows, reverse, intrinsic, itemA, itemB, itemAHeight }) => {
  const order = reverse
    ? [{ key: 'b', frame: itemB }, { key: 'a', frame: itemA }]
    : [{ key: 'a', frame: itemA }, { key: 'b', frame: itemB }];
  const itemNodes = { a: reverse ? 4 : 2, b: reverse ? 2 : 4 };
  const itemMetrics = {
    a: { text: 'AA A A', minContent: 20, minContentHeight: 30, maxContent: 60, maxContentHeight: 10 },
    b: { text: 'XXXX', minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
  };
  const nodeFrames = order.map(({ key, frame }, index) => {
    const [x, y, width, height] = frame;
    return `${index + 1}:node=${2 + index * 2},x=${x},y=${y},width=${width},height=${height}`;
  });
  const intrinsicInputs = order.map(({ key }, sourceOrder) => ({
    grid: 1,
    node: itemNodes[key],
    sourceOrder,
    kind: 'text',
    fontRegistryRevision: 1,
    minContent: itemMetrics[key].minContent,
    minContentHeight: itemMetrics[key].minContentHeight,
    maxContent: itemMetrics[key].maxContent,
    maxContentHeight: itemMetrics[key].maxContentHeight,
  }));
  const scene = {
    id,
    boxCount: 3,
    rootFrame: `width=${root[0]},height=${root[1]}`,
    itemFrame: null,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${nodeFrames.join(';')}]`,
    axes: {
      Columns: { tracks: columns, collapsed: columns.map(() => false) },
      Rows: { tracks: rows, collapsed: rows.map(() => false) },
    },
    gaps: { Columns: 2, Rows: 2 },
    intrinsicInputs,
    intrinsicSpans: [2],
    intrinsicSpansByAxis: { Columns: [2], Rows: [2] },
    columnSizingFeedsRows: true,
    crossAxisSpanGroups: {
      itemNodes,
      items: ['a', 'b'].map((key) => ({ key, ...itemMetrics[key] })),
      minimum: {
        Columns: { a: intrinsic ? 20 : 0, b: 40 },
        Rows: { a: intrinsic ? 20 : 0, b: 10 },
      },
      minContent: { Columns: { a: 20, b: 40 }, Rows: { a: itemAHeight, b: 10 } },
      maxContent: { Columns: { a: 60, b: 40 }, Rows: { a: itemAHeight, b: 10 } },
      itemAHeight,
      ...(intrinsic ? {
        initialColumns: [2, 2, 2],
        minContentColumns: [9, 19, 19],
        perTrackPlannedIncreases: [7, 17, 17],
        perItemPlannedIncreases: { a: [7, 7], b: [17, 17] },
      } : {}),
    },
  };
  if (!intrinsic) {
    scene.pixelSamples = [
      { id: `${id}-overflow-item-b`, nodeId: `c114-${id}-item-b`,
        xCssPx: 50, yCssPx: 4, rgba: [229, 72, 77, 255] },
      { id: `${id}-clear-past-item-b`, nodeId: `c114-${id}-grid`,
        xCssPx: 60, yCssPx: 4, rgba: [0, 0, 0, 255] },
    ];
  }
  return scene;
};
const mixedCrossAxisSpanGroupScene = ({
  id, root, columns, rows, reverse, intrinsic, itemA, itemB, itemC,
  itemAHeight, itemCHeight, minimumRows, pixelSamples,
}) => {
  const itemFrames = { a: itemA, b: itemB, c: itemC };
  const order = reverse ? ['c', 'b', 'a'] : ['a', 'b', 'c'];
  const itemNodes = { a: reverse ? 6 : 2, b: 4, c: reverse ? 2 : 6 };
  const itemMetrics = {
    a: { text: 'AA A A', minContent: 20, minContentHeight: 30, maxContent: 60, maxContentHeight: 10 },
    b: { text: 'XXXX', minContent: 40, minContentHeight: 10, maxContent: 40, maxContentHeight: 10 },
    c: { text: 'AA AA AA AA', minContent: 20, minContentHeight: 40, maxContent: 110, maxContentHeight: 10 },
  };
  const nodeFrames = order.map((key, index) => {
    const [x, y, width, height] = itemFrames[key];
    return `${index + 1}:node=${2 + index * 2},x=${x},y=${y},width=${width},height=${height}`;
  });
  const intrinsicInputs = order.map((key, sourceOrder) => ({
    grid: 1,
    node: itemNodes[key],
    sourceOrder,
    kind: 'text',
    fontRegistryRevision: 1,
    minContent: itemMetrics[key].minContent,
    minContentHeight: itemMetrics[key].minContentHeight,
    maxContent: itemMetrics[key].maxContent,
    maxContentHeight: itemMetrics[key].maxContentHeight,
  }));
  const scene = {
    id,
    boxCount: 4,
    rootFrame: `width=${root[0]},height=${root[1]}`,
    itemFrame: null,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${nodeFrames.join(';')}]`,
    axes: {
      Columns: { tracks: columns, collapsed: columns.map(() => false) },
      Rows: { tracks: rows, collapsed: rows.map(() => false) },
    },
    gaps: { Columns: 2, Rows: 2 },
    intrinsicInputs,
    intrinsicSpans: [2, 3],
    intrinsicSpansByAxis: { Columns: [2, 3], Rows: [2] },
    columnSizingFeedsRows: true,
    mixedCrossAxisSpanGroups: {
      itemNodes,
      items: [
        { key: 'a', columnSpan: 2, rowSpan: 2, ...itemMetrics.a },
        { key: 'b', columnSpan: 2, rowSpan: 2, ...itemMetrics.b },
        { key: 'c', columnSpan: 3, rowSpan: 2, ...itemMetrics.c },
      ],
      minimum: {
        Columns: { a: intrinsic ? 20 : 0, b: 40, c: intrinsic ? 20 : 0 },
        Rows: minimumRows ?? { a: intrinsic ? 20 : 0, b: 10, c: intrinsic ? 20 : 0 },
      },
      minContent: {
        Columns: { a: 20, b: 40, c: 20 },
        Rows: { a: itemAHeight, b: 10, c: itemCHeight },
      },
      maxContent: {
        Columns: { a: 60, b: 40, c: 110 },
        Rows: { a: itemAHeight, b: 10, c: itemCHeight },
      },
      ...(intrinsic ? {
        initialColumns: [2, 2, 2],
        span2Columns: [9, 19, 19],
        span3Columns: [9, 19, 19],
        perTrackPlannedIncreases: [7, 17, 17],
        perItemPlannedIncreases: { a: [7, 7], b: [17, 17], c: [0, 0, 0] },
      } : {}),
    },
  };
  if (pixelSamples) {
    scene.pixelSamples = pixelSamples;
  } else if (!intrinsic) {
    scene.pixelSamples = [
      { id: `${id}-overflow-item-b`, nodeId: `c114-${id}-item-b`,
        xCssPx: 50, yCssPx: 4, rgba: [229, 72, 77, 255] },
      { id: `${id}-clear-past-item-b`, nodeId: `c114-${id}-grid`,
        xCssPx: 60, yCssPx: 4, rgba: [0, 0, 0, 255] },
    ];
  }
  return scene;
};
const mixedTextImageSpanScene = ({ id, intrinsic, reverse, maxContent = false }) => {
  const order = reverse ? ['c', 'b', 'a'] : ['a', 'b', 'c'];
  const itemNodes = { a: reverse ? 5 : 2, b: 4, c: reverse ? 2 : 5 };
  const itemFrames = maxContent
    ? { a: [2, 2, 238 / 3, 80 / 3], b: [119 / 3, 2, 40, 80 / 3], c: [2, 2, 110, 80 / 3] }
    : intrinsic
    ? { a: [2, 2, 33.34375, 80 / 3], b: [19.671875, 2, 40, 80 / 3], c: [2, 2, 51, 80 / 3] }
    : { a: [2, 2, 26, 40], b: [16, 2, 40, 80 / 3], c: [2, 2, 40, 40] };
  const metrics = {
    a: { kind: 'text', text: 'AA A A', minContent: 20, minContentHeight: 30, maxContent: 60, maxContentHeight: 10 },
    b: { kind: 'replaced', natural: { width: 24, height: 16, aspectRatio: null }, minContent: 40, minContentHeight: 80 / 3, maxContent: 40, maxContentHeight: 80 / 3 },
    c: { kind: 'text', text: 'AA AA AA AA', minContent: 20, minContentHeight: 40, maxContent: 110, maxContentHeight: 10 },
  };
  const intrinsicInputs = order.map((key, sourceOrder) => {
    const input = {
      grid: 1,
      node: itemNodes[key],
      sourceOrder,
      kind: metrics[key].kind,
    };
    if (metrics[key].kind === 'text') {
      Object.assign(input, {
        fontRegistryRevision: 1,
        minContent: metrics[key].minContent,
        minContentHeight: metrics[key].minContentHeight,
        maxContent: metrics[key].maxContent,
        maxContentHeight: metrics[key].maxContentHeight,
      });
    } else {
      input.natural = metrics[key].natural;
    }
    return input;
  });
  const root = maxContent ? [114, 92 / 3] : intrinsic ? [55, 92 / 3] : [44, 44];
  const columns = maxContent ? [107 / 3, 125 / 3, 86 / 3]
    : intrinsic ? [15.671875, 15.671875, 15.65625] : [12, 12, 12];
  const rows = intrinsic ? [37 / 3, 37 / 3] : [19, 19];
  const nodeFrames = order.map((key, index) => {
    const [x, y, width, height] = itemFrames[key];
    return `${index + 1}:node=${itemNodes[key]},x=${x},y=${y},width=${width},height=${height}`;
  });
  const items = ['a', 'b', 'c'].map((key) => ({
    key, node: itemNodes[key], columnSpan: key === 'c' ? 3 : 2, rowSpan: 2,
    ...metrics[key], frame: itemFrames[key],
  }));
  const scene = {
    id,
    boxCount: 4,
    runtimeImageCount: 1,
    // Runtime summary rounds fractional frame strings; compare parsed node frames at 1/64px below.
    rootFrame: null,
    itemFrame: null,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${nodeFrames.join(';')}]`,
    axes: {
      Columns: { tracks: columns, collapsed: columns.map(() => false) },
      Rows: { tracks: rows, collapsed: rows.map(() => false) },
    },
    gaps: { Columns: 2, Rows: 2 },
    intrinsicInputs,
    intrinsicSpans: [2, 3],
    intrinsicSpansByAxis: { Columns: [2, 3], Rows: [2] },
    columnSizingFeedsRows: true,
    mixedTextImageSpan: {
      intrinsic, maxContentWidth: maxContent, reverse, order, itemNodes, items,
      minimum: {
        Columns: { a: intrinsic ? 20 : 0, b: 40, c: intrinsic ? 20 : 0 },
        Rows: { a: maxContent ? 10 : intrinsic ? 20 : 0, b: 80 / 3, c: maxContent ? 10 : intrinsic ? 20 : 0 },
      },
      minContent: {
        Columns: { a: 20, b: 40, c: 20 },
        Rows: {
          a: maxContent ? 10 : intrinsic ? 20 : 30,
          b: 80 / 3,
          c: maxContent ? 10 : intrinsic ? 20 : 40,
        },
      },
      maxContent: {
        Columns: { a: 60, b: 40, c: 110 },
        Rows: {
          a: maxContent ? 10 : intrinsic ? 20 : 30,
          b: 80 / 3,
          c: maxContent ? 10 : intrinsic ? 20 : 40,
        },
      },
    },
  };
  if (!intrinsic) {
    const rootID = `c114-${id}-grid`;
    scene.pixelSamples = [
      { id: `${id}-top-border`, nodeId: rootID, xCssPx: 8, yCssPx: 0, rgba: [240, 198, 116, 255] },
      { id: `${id}-image-b-overflow`, nodeId: `c114-${id}-item-b`, xCssPx: 50, yCssPx: 6, rgba: [0, 255, 64, 255] },
      { id: `${id}-clear-past-image-b`, nodeId: rootID, xCssPx: 58, yCssPx: 6, rgba: [0, 0, 0, 255] },
    ];
  }
  return scene;
};
scenes.push(
  minmaxCalcMinimumScene({
    id: 'cyclic-minmax-calc-min-both-axes', root: [54, 14], item: [2, 2, 104, 22],
    columns: 104, rows: 22,
    points: [
      ['left-border', 'grid', 1, 5, [240, 198, 116, 255]],
      ['item-inside', 'item', 5, 5, [229, 72, 77, 255]],
      ['right-border-overpaint', 'item', 53, 5, [229, 72, 77, 255]],
      ['outside-root', 'item', 60, 5, [229, 72, 77, 255]],
      ['past-item-right', 'grid', 106, 5, [0, 0, 0, 255]],
      ['bottom-border-overpaint', 'item', 5, 13, [229, 72, 77, 255]],
      ['outside-root-bottom', 'item', 5, 16, [229, 72, 77, 255]],
      ['past-item-bottom', 'grid', 5, 25, [0, 0, 0, 255]],
    ],
  }),
  minmaxCalcMinimumScene({
    id: 'cyclic-minmax-calc-min-column-definite-row', root: [54, 14],
    item: [2, 2, 104, 10], columns: 104, rows: 10,
    points: [
      ['left-border', 'grid', 1, 5, [240, 198, 116, 255]],
      ['item-inside', 'item', 5, 5, [229, 72, 77, 255]],
      ['right-border-overpaint', 'item', 53, 5, [229, 72, 77, 255]],
      ['outside-root', 'item', 60, 5, [229, 72, 77, 255]],
      ['last-item-column', 'item', 105, 5, [229, 72, 77, 255]],
      ['past-item-right', 'grid', 106, 5, [0, 0, 0, 255]],
    ],
  }),
  minmaxCalcMinimumScene({
    id: 'definite-column-cyclic-minmax-calc-min-row', root: [14, 24],
    item: [2, 2, 10, 42], columns: 10, rows: 42,
    points: [
      ['left-border', 'grid', 1, 5, [240, 198, 116, 255]],
      ['item-inside', 'item', 5, 5, [229, 72, 77, 255]],
      ['bottom-border-overpaint', 'item', 5, 23, [229, 72, 77, 255]],
      ['outside-root-bottom', 'item', 5, 25, [229, 72, 77, 255]],
      ['last-item-row', 'item', 5, 43, [229, 72, 77, 255]],
      ['past-item-bottom', 'grid', 5, 45, [0, 0, 0, 255]],
    ],
  }),
  minmaxCalcMinimumScene({
    id: 'definite-minmax-calc-min-both-axes', root: [14, 14],
    item: [2, 2, 24, 22], columns: 24, rows: 22,
    points: [
      ['left-border', 'grid', 1, 5, [240, 198, 116, 255]],
      ['item-inside', 'item', 5, 5, [229, 72, 77, 255]],
      ['right-border-overpaint', 'item', 13, 5, [229, 72, 77, 255]],
      ['outside-root', 'item', 16, 5, [229, 72, 77, 255]],
      ['past-item-right', 'grid', 27, 5, [0, 0, 0, 255]],
      ['bottom-border-overpaint', 'item', 5, 13, [229, 72, 77, 255]],
      ['outside-root-bottom', 'item', 5, 16, [229, 72, 77, 255]],
      ['past-item-bottom', 'grid', 5, 25, [0, 0, 0, 255]],
    ],
  }),
);
scenes.push(
  twoAxisSpanTextScene({
    id: 'cyclic-minmax-calc-two-axis-span-text-min-content',
    root: [14, 64], item: [2, 2, 10, 60], columns: [4, 4], rows: [29, 29],
    minContentHeight: 60,
    points: [
      { id: 'cyclic-minmax-calc-two-axis-span-text-min-content-right-border',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 13, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-min-content-outside-root',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 15, yCssPx: 6, rgba: [0, 0, 0, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-min-content-bottom-border',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 6, yCssPx: 63, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-min-content-past-root',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 6, yCssPx: 65, rgba: [0, 0, 0, 255] },
    ],
  }),
  twoAxisSpanTextScene({
    id: 'cyclic-minmax-calc-two-axis-span-text-max-content',
    root: [64, 14], item: [2, 2, 60, 10], columns: [29, 29], rows: [4, 4],
    minContentHeight: 60,
    points: [
      { id: 'cyclic-minmax-calc-two-axis-span-text-max-content-right-border',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 63, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-max-content-outside-root',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 65, yCssPx: 6, rgba: [0, 0, 0, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-max-content-bottom-border',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 6, yCssPx: 13, rgba: [240, 198, 116, 255] },
      { id: 'cyclic-minmax-calc-two-axis-span-text-max-content-past-root',
        nodeId: 'c114-cyclic-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 6, yCssPx: 15, rgba: [0, 0, 0, 255] },
    ],
  }),
  twoAxisSpanTextScene({
    id: 'definite-minmax-calc-two-axis-span-text-min-content',
    root: [14, 64], item: [2, 2, 10, 60], columns: [4, 4], rows: [29, 29],
    minContentHeight: 60,
    points: [
      { id: 'definite-minmax-calc-two-axis-span-text-min-content-right-border',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 13, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-min-content-outside-root',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 15, yCssPx: 6, rgba: [0, 0, 0, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-min-content-bottom-border',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 6, yCssPx: 63, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-min-content-past-root',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-min-content-grid',
        xCssPx: 6, yCssPx: 65, rgba: [0, 0, 0, 255] },
    ],
  }),
  twoAxisSpanTextScene({
    id: 'definite-minmax-calc-two-axis-span-text-max-content',
    root: [64, 14], item: [2, 2, 60, 10], columns: [29, 29], rows: [4, 4],
    minContentHeight: 60,
    points: [
      { id: 'definite-minmax-calc-two-axis-span-text-max-content-right-border',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 63, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-max-content-outside-root',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 65, yCssPx: 6, rgba: [0, 0, 0, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-max-content-bottom-border',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 6, yCssPx: 13, rgba: [240, 198, 116, 255] },
      { id: 'definite-minmax-calc-two-axis-span-text-max-content-past-root',
        nodeId: 'c114-definite-minmax-calc-two-axis-span-text-max-content-grid',
        xCssPx: 6, yCssPx: 15, rgba: [0, 0, 0, 255] },
    ],
  }),
);
scenes.push(
  crossAxisSpanGroupScene({
    id: 'cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward',
    root: [55, 24], columns: [15.671875, 15.671875, 15.65625], rows: [9, 9],
    reverse: false, intrinsic: true,
    itemA: [2, 2, 33.34375, 20], itemB: [19.671875, 2, 40, 10], itemAHeight: 20,
  }),
  crossAxisSpanGroupScene({
    id: 'cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse',
    root: [55, 24], columns: [15.671875, 15.671875, 15.65625], rows: [9, 9],
    reverse: true, intrinsic: true,
    itemA: [2, 2, 33.34375, 20], itemB: [19.671875, 2, 40, 10], itemAHeight: 20,
  }),
  crossAxisSpanGroupScene({
    id: 'definite-minmax-calc-cross-axis-span-groups-forward',
    root: [44, 34], columns: [12, 12, 12], rows: [14, 14],
    reverse: false, intrinsic: false,
    itemA: [2, 2, 26, 30], itemB: [16, 2, 40, 10], itemAHeight: 30,
  }),
  crossAxisSpanGroupScene({
    id: 'definite-minmax-calc-cross-axis-span-groups-reverse',
    root: [44, 34], columns: [12, 12, 12], rows: [14, 14],
    reverse: true, intrinsic: false,
    itemA: [2, 2, 26, 30], itemB: [16, 2, 40, 10], itemAHeight: 30,
  }),
);
scenes.push(
  mixedCrossAxisSpanGroupScene({
    id: 'cyclic-minmax-cross-axis-mixed-span-groups-intrinsic-forward',
    root: [55, 24], columns: [15.671875, 15.671875, 15.65625], rows: [9, 9],
    reverse: false, intrinsic: true,
    itemA: [2, 2, 33.34375, 20], itemB: [19.671875, 2, 40, 10], itemC: [2, 2, 51, 20],
    itemAHeight: 20, itemCHeight: 20,
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'cyclic-minmax-cross-axis-mixed-span-groups-intrinsic-reverse',
    root: [55, 24], columns: [15.671875, 15.671875, 15.65625], rows: [9, 9],
    reverse: true, intrinsic: true,
    itemA: [2, 2, 33.34375, 20], itemB: [19.671875, 2, 40, 10], itemC: [2, 2, 51, 20],
    itemAHeight: 20, itemCHeight: 20,
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'definite-minmax-cross-axis-mixed-span-groups-forward',
    root: [44, 44], columns: [12, 12, 12], rows: [19, 19],
    reverse: false, intrinsic: false,
    itemA: [2, 2, 26, 40], itemB: [16, 2, 40, 10], itemC: [2, 2, 40, 40],
    itemAHeight: 30, itemCHeight: 40,
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'definite-minmax-cross-axis-mixed-span-groups-reverse',
    root: [44, 44], columns: [12, 12, 12], rows: [19, 19],
    reverse: true, intrinsic: false,
    itemA: [2, 2, 26, 40], itemB: [16, 2, 40, 10], itemC: [2, 2, 40, 40],
    itemAHeight: 30, itemCHeight: 40,
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'cyclic-minmax-cross-axis-max-content-span-groups-forward',
    root: [114, 14], columns: [107 / 3, 125 / 3, 86 / 3], rows: [4, 4],
    reverse: false, intrinsic: true,
    itemA: [2, 2, 238 / 3, 10], itemB: [119 / 3, 2, 40, 10], itemC: [2, 2, 110, 10],
    itemAHeight: 10, itemCHeight: 10,
    minimumRows: { a: 10, b: 10, c: 10 },
    pixelSamples: [
      { id: 'max-content-forward-right-border', nodeId: 'c114-cyclic-minmax-cross-axis-max-content-span-groups-forward-grid',
        xCssPx: 113, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'max-content-forward-clear-past-root', nodeId: 'c114-cyclic-minmax-cross-axis-max-content-span-groups-forward-grid',
        xCssPx: 116, yCssPx: 6, rgba: [0, 0, 0, 255] },
    ],
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'cyclic-minmax-cross-axis-max-content-span-groups-reverse',
    root: [114, 14], columns: [107 / 3, 125 / 3, 86 / 3], rows: [4, 4],
    reverse: true, intrinsic: true,
    itemA: [2, 2, 238 / 3, 10], itemB: [119 / 3, 2, 40, 10], itemC: [2, 2, 110, 10],
    itemAHeight: 10, itemCHeight: 10,
    minimumRows: { a: 10, b: 10, c: 10 },
    pixelSamples: [
      { id: 'max-content-reverse-right-border', nodeId: 'c114-cyclic-minmax-cross-axis-max-content-span-groups-reverse-grid',
        xCssPx: 113, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'max-content-reverse-clear-past-root', nodeId: 'c114-cyclic-minmax-cross-axis-max-content-span-groups-reverse-grid',
        xCssPx: 116, yCssPx: 6, rgba: [0, 0, 0, 255] },
    ],
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'definite-minmax-cross-axis-max-content-span-groups-forward',
    root: [84, 24], columns: [76 / 3, 76 / 3, 76 / 3], rows: [9, 9],
    reverse: false, intrinsic: false,
    itemA: [2, 2, 158 / 3, 20], itemB: [88 / 3, 2, 40, 10], itemC: [2, 2, 80, 20],
    itemAHeight: 20, itemCHeight: 20,
    pixelSamples: [
      { id: 'definite-80-forward-right-border', nodeId: 'c114-definite-minmax-cross-axis-max-content-span-groups-forward-grid',
        xCssPx: 83, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'definite-80-forward-clear-past-root', nodeId: 'c114-definite-minmax-cross-axis-max-content-span-groups-forward-grid',
        xCssPx: 86, yCssPx: 6, rgba: [0, 0, 0, 255] },
    ],
  }),
  mixedCrossAxisSpanGroupScene({
    id: 'definite-minmax-cross-axis-max-content-span-groups-reverse',
    root: [84, 24], columns: [76 / 3, 76 / 3, 76 / 3], rows: [9, 9],
    reverse: true, intrinsic: false,
    itemA: [2, 2, 158 / 3, 20], itemB: [88 / 3, 2, 40, 10], itemC: [2, 2, 80, 20],
    itemAHeight: 20, itemCHeight: 20,
    pixelSamples: [
      { id: 'definite-80-reverse-right-border', nodeId: 'c114-definite-minmax-cross-axis-max-content-span-groups-reverse-grid',
        xCssPx: 83, yCssPx: 6, rgba: [240, 198, 116, 255] },
      { id: 'definite-80-reverse-clear-past-root', nodeId: 'c114-definite-minmax-cross-axis-max-content-span-groups-reverse-grid',
        xCssPx: 86, yCssPx: 6, rgba: [0, 0, 0, 255] },
    ],
  }),
);
scenes.push(
  mixedTextImageSpanScene({ id: 'cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward', intrinsic: true, reverse: false }),
  mixedTextImageSpanScene({ id: 'cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse', intrinsic: true, reverse: true }),
  mixedTextImageSpanScene({ id: 'definite-minmax-calc-mixed-text-image-span-forward', intrinsic: false, reverse: false }),
  mixedTextImageSpanScene({ id: 'definite-minmax-calc-mixed-text-image-span-reverse', intrinsic: false, reverse: true }),
  mixedTextImageSpanScene({ id: 'max-content-forward', intrinsic: true, maxContent: true, reverse: false }),
  mixedTextImageSpanScene({ id: 'max-content-reverse', intrinsic: true, maxContent: true, reverse: true }),
  mixedTextImageSpanScene({ id: 'definite-forward', intrinsic: false, reverse: false }),
  mixedTextImageSpanScene({ id: 'definite-reverse', intrinsic: false, reverse: true }),
);
const spanColumnSamples = () => [
  ['outside-root-item', 'item', 58, 6, [229, 72, 77, 255]],
  ['past-item', 'grid', 66, 6, [0, 0, 0, 255]],
];
const spanRowSamples = () => [
  ['outside-root-item', 'item', 8, 27, [229, 72, 77, 255]],
  ['past-item', 'grid', 8, 31, [0, 0, 0, 255]],
];
scenes.push(
  minmaxCalcMinimumSpanScene({
    id: 'cyclic-minmax-calc-min-span-columns', root: [54, 14],
    item: [2, 2, 62, 10], columns: [29, 29], rows: [10],
    gaps: { Columns: 4, Rows: 0 }, points: spanColumnSamples(),
    axis: 'Columns', intrinsicInputs: minimumTextInput,
  }),
  minmaxCalcMinimumSpanScene({
    id: 'definite-minmax-calc-min-span-columns', root: [54, 14],
    item: [2, 2, 62, 10], columns: [29, 29], rows: [10],
    gaps: { Columns: 4, Rows: 0 }, points: spanColumnSamples(),
    axis: 'Columns', intrinsicInputs: minimumTextInput,
  }),
  minmaxCalcMinimumSpanScene({
    id: 'cyclic-minmax-calc-min-span-rows', root: [24, 24],
    item: [2, 2, 20, 28], columns: [20], rows: [12, 12],
    gaps: { Columns: 0, Rows: 4 }, points: spanRowSamples(),
    axis: 'Rows', intrinsicInputs: spanRowsTextInput,
  }),
  minmaxCalcMinimumSpanScene({
    id: 'definite-minmax-calc-min-span-rows', root: [24, 24],
    item: [2, 2, 20, 28], columns: [20], rows: [12, 12],
    gaps: { Columns: 0, Rows: 4 }, points: spanRowSamples(),
    axis: 'Rows', intrinsicInputs: spanRowsTextInput,
  }),
);
for (const id of [
  'cyclic-minmax-calc-overlap-forward',
  'cyclic-minmax-calc-overlap-reverse',
  'definite-minmax-calc-overlap-forward',
  'definite-minmax-calc-overlap-reverse',
  'definite-minmax-calc-overlap-small-forward',
  'definite-minmax-calc-overlap-small-reverse',
]) {
  const small = id.includes('-small-');
  const reversed = id.endsWith('-reverse');
  const root = small ? [104, 14] : [144, 14];
  const columns = small ? [32, 32, 32] : [42, 47, 47];
  const order = reversed ? ['span3', 'span2-right', 'span2-left']
    : ['span2-left', 'span2-right', 'span3'];
  const framesByKind = small
    ? {
      'span2-left': [2, 2, 66, 10],
      'span2-right': [36, 2, 66, 10],
      span3: [2, 2, 100, 10],
    }
    : {
      'span2-left': [2, 2, 91, 10],
      'span2-right': [46, 2, 96, 10],
      span3: [2, 2, 140, 10],
    };
  const itemFrames = order.map((kind, index) => {
    const [x, y, width, height] = framesByKind[kind];
    return `${index + 1}:node=${2 + index * 2},x=${x},y=${y},width=${width},height=${height}`;
  });
  const firstFrame = framesByKind[order[0]];
  const textWidths = { 'span2-left': 70, 'span2-right': 80, span3: 140 };
  const intrinsicInputs = order.map((kind, index) => ({
    grid: 1,
    node: 2 + index * 2,
    sourceOrder: index,
    kind: 'text',
    fontRegistryRevision: 1,
    minContent: textWidths[kind],
    minContentHeight: 10,
    maxContent: textWidths[kind],
    maxContentHeight: 10,
  }));
  const scene = {
    id,
    axis: 'Columns',
    boxCount: 4,
    rootFrame: `width=${root[0]},height=${root[1]}`,
    itemFrame: `width=${firstFrame[2]},height=${firstFrame[3]}`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${itemFrames.join(';')}]`,
    axes: {
      Columns: { tracks: columns, collapsed: [false, false, false] },
      Rows: { tracks: [10], collapsed: [false] },
    },
    gaps: { Columns: 2, Rows: 0 },
    intrinsicInputs,
    intrinsicSpans: [2, 3],
    intrinsicSpansByAxis: { Columns: [2, 3], Rows: [] },
  };
  if (small) {
    scene.pixelSamples = [
      {
        id: `${id}-inside-root-text`,
        nodeId: `c114-${id}-item-span2-left`,
        xCssPx: 4,
        yCssPx: 7,
        rgba: [229, 72, 77, 255],
      },
      {
        id: `${id}-red-text-outside-root`,
        nodeId: `c114-${id}-item-span3`,
        xCssPx: 126,
        yCssPx: 7,
        rgba: [229, 72, 77, 255],
      },
      {
        id: `${id}-clear-after-text`,
        nodeId: `c114-${id}-grid`,
        xCssPx: 150,
        yCssPx: 7,
        rgba: [0, 0, 0, 255],
      },
    ];
  }
  scenes.push(scene);
}
for (const id of [
  'cyclic-minmax-calc-overlap-rows-forward',
  'cyclic-minmax-calc-overlap-rows-reverse',
  'definite-minmax-calc-overlap-rows-forward',
  'definite-minmax-calc-overlap-rows-reverse',
  'definite-minmax-calc-overlap-rows-small-forward',
  'definite-minmax-calc-overlap-rows-small-reverse',
]) {
  const small = id.includes('-small-');
  const reversed = id.endsWith('-reverse');
  const root = [14, small ? 104 : 144];
  const rows = small ? [32, 32, 32] : [42, 47, 47];
  const order = reversed ? ['span3', 'span2-right', 'span2-left']
    : ['span2-left', 'span2-right', 'span3'];
  const framesByKind = small
    ? {
      'span2-left': [2, 2, 10, 66],
      'span2-right': [2, 36, 10, 66],
      span3: [2, 2, 10, 100],
    }
    : {
      'span2-left': [2, 2, 10, 91],
      'span2-right': [2, 46, 10, 96],
      span3: [2, 2, 10, 140],
    };
  const itemFrames = order.map((kind, index) => {
    const [x, y, width, height] = framesByKind[kind];
    return `${index + 1}:node=${2 + index * 2},x=${x},y=${y},width=${width},height=${height}`;
  });
  const firstFrame = framesByKind[order[0]];
  const textHeights = { 'span2-left': 70, 'span2-right': 80, span3: 140 };
  const intrinsicInputs = order.map((kind, index) => ({
    grid: 1,
    node: 2 + index * 2,
    sourceOrder: index,
    kind: 'text',
    fontRegistryRevision: 1,
    minContent: 10,
    minContentHeight: textHeights[kind],
    maxContent: textHeights[kind],
    maxContentHeight: 10,
  }));
  const scene = {
    id,
    axis: 'Rows',
    boxCount: 4,
    rootFrame: `width=${root[0]},height=${root[1]}`,
    itemFrame: `width=${firstFrame[2]},height=${firstFrame[3]}`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${itemFrames.join(';')}]`,
    axes: {
      Columns: { tracks: [10], collapsed: [false] },
      Rows: { tracks: rows, collapsed: [false, false, false] },
    },
    gaps: { Columns: 0, Rows: 2 },
    intrinsicInputs,
    intrinsicSpans: [2, 3],
    intrinsicSpansByAxis: { Columns: [], Rows: [2, 3] },
    overlappingSpanTextContributions: true,
  };
  if (small) {
    scene.pixelSamples = [
      {
        id: `${id}-inside-root-text`,
        nodeId: `c114-${id}-item-span2-left`,
        xCssPx: 7,
        yCssPx: 7,
        rgba: [229, 72, 77, 255],
      },
      {
        id: `${id}-red-text-outside-root`,
        nodeId: `c114-${id}-item-span3`,
        xCssPx: 7,
        yCssPx: 126,
        rgba: [229, 72, 77, 255],
      },
      {
        id: `${id}-clear-after-text`,
        nodeId: `c114-${id}-grid`,
        xCssPx: 7,
        yCssPx: 150,
        rgba: [0, 0, 0, 255],
      },
    ];
  }
  scenes.push(scene);
}
function cyclicMinmaxReplacedImageSpanScene(id, order, intrinsic) {
  const rootID = `c114-${id}-grid`;
  const root = intrinsic ? [29, 20] : [84, 84];
  const columns = intrinsic ? [5, 8, 8] : [10, 10, 10];
  const rows = intrinsic ? [7, 7] : [10, 10];
  const itemSpecs = {
    a: { frame: [2, 2, 12, 8], columnSpan: 2 },
    b: { frame: [intrinsic ? 9 : 14, 2, 18, 12], columnSpan: 2 },
    c: { frame: [2, 2, 24, 16], columnSpan: 3 },
  };
  const nodeByKey = Object.fromEntries(order.map((key, index) => [key, index + 2]));
  const itemFrames = order.map((key, index) => {
    const [x, y, width, height] = itemSpecs[key].frame;
    return `${index + 1}:node=${nodeByKey[key]},x=${x},y=${y},width=${width},height=${height}`;
  });
  const itemSizes = { a: [12, 8], b: [18, 12], c: [24, 16] };
  const borderY = Math.floor(root[1] / 2);
  return {
    id,
    axis: 'Columns',
    boxCount: 4,
    rootFrame: `width=${root[0]},height=${root[1]}`,
    itemFrame: null,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${root[0]},height=${root[1]};${itemFrames.join(';')}]`,
    axes: {
      Columns: { tracks: columns, collapsed: [false, false, false] },
      Rows: { tracks: rows, collapsed: [false, false] },
    },
    gaps: { Columns: 2, Rows: 2 },
    intrinsicInputs: order.map((key, sourceOrder) => ({
      grid: 1,
      node: nodeByKey[key],
      sourceOrder,
      kind: 'replaced',
      aspectRatio: null,
      width: 24,
      height: 16,
    })),
    intrinsicSpansByAxis: { Columns: [2, 3], Rows: [2] },
    columnSizingFeedsRows: true,
    replacedCyclicMinmaxImageSpan: { intrinsic, order, nodeByKey, itemSpecs, itemSizes },
    pixelSamples: [
      {
        id: `${id}-image-interior`,
        nodeId: rootID,
        xCssPx: 4,
        yCssPx: 4,
        rgba: [255, 255, 64, 255],
      },
      {
        id: `${id}-right-border`,
        nodeId: rootID,
        xCssPx: root[0] - 1,
        yCssPx: borderY,
        rgba: [240, 198, 116, 255],
      },
      {
        id: `${id}-after-root`,
        nodeId: rootID,
        xCssPx: root[0],
        yCssPx: borderY,
        rgba: [0, 0, 0, 255],
      },
    ],
  };
}
scenes.push(
  cyclicMinmaxReplacedImageSpanScene(
    'cyclic-minmax-replaced-image-span-intrinsic-forward', ['a', 'b', 'c'], true,
  ),
  cyclicMinmaxReplacedImageSpanScene(
    'cyclic-minmax-replaced-image-span-intrinsic-reverse', ['c', 'b', 'a'], true,
  ),
  cyclicMinmaxReplacedImageSpanScene(
    'cyclic-minmax-replaced-image-span-definite-forward', ['a', 'b', 'c'], false,
  ),
  cyclicMinmaxReplacedImageSpanScene(
    'cyclic-minmax-replaced-image-span-definite-reverse', ['c', 'b', 'a'], false,
  ),
);
async function autoRepeatNamedLineScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_named_line_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 28, 'named auto-repeat 고정 fixture는 28 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const androidNamed = allowlist(androidSource,
    /C114_AUTO_REPEAT_NEGATIVE_LINE_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android auto-repeat');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');
  const iosNamed = allowlist(iosSource,
    /c114AutoRepeatNegativeLineSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS auto-repeat');

  return fixtures.map(({ id, expect }) => {
    for (const [label, ids] of [
      ['Android Grid', androidScenes], ['Android auto-repeat', androidNamed],
      ['iOS Grid', iosScenes], ['iOS auto-repeat', iosNamed],
    ]) assert.ok(ids.has(id), `${label} allowlist에 ${id}가 없습니다.`);
    const [itemX, itemY, itemWidth, itemHeight] = expect.item;
    const [rootWidth, rootHeight] = expect.root;
    const rootFrame = `width=${rootWidth},height=${rootHeight}`;
    const itemFrame = `width=${itemWidth},height=${itemHeight}`;
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};`
      + `1:node=2,x=${itemX},y=${itemY},width=${itemWidth},height=${itemHeight}]`;
    const scene = {
      id,
      axis: expect.tracks.Columns.length > 1 ? 'Columns' : 'Rows',
      boxCount: 2,
      rootFrame,
      itemFrame,
      nodeFrames,
      axes: Object.fromEntries(Object.entries(expect.tracks).map(([axis, tracks]) => [axis, {
        tracks,
        collapsed: expect.collapsed[axis],
      }])),
      gaps: expect.gaps,
    };
    const rootNodeId = `c114-${id}-grid`;
    const itemNodeId = `c114-${id}-item`;
    const background = [226, 232, 240, 255];
    const itemColor = [57, 165, 219, 255];
    if (id === 'auto-repeat-named-row') {
      scene.pixelSamples = [
        { id: `${id}-before-item`, nodeId: rootNodeId, xCssPx: 20, yCssPx: itemY - 1, rgba: background },
        { id: `${id}-item-start-edge`, nodeId: itemNodeId, xCssPx: 20, yCssPx: itemY, rgba: itemColor },
        { id: `${id}-item-end-edge`, nodeId: itemNodeId, xCssPx: 20, yCssPx: itemY + itemHeight - 1, rgba: itemColor },
        { id: `${id}-after-item`, nodeId: rootNodeId, xCssPx: 20, yCssPx: itemY + itemHeight, rgba: background },
      ];
    } else if (id.startsWith('auto-repeat-named-implicit-')
      || id.startsWith('auto-fit-named-implicit-')) {
      const rowAxis = id.includes('-row-');
      const [rootWidth, rootHeight] = expect.root;
      const itemExtendsPastRoot = rowAxis
        ? itemY + itemHeight > rootHeight
        : itemX + itemWidth > rootWidth;
      scene.pixelSamples = [
        {
          id: `${id}-root-interior`, nodeId: rootNodeId,
          xCssPx: rowAxis ? 4 : 4,
          yCssPx: rowAxis ? rootHeight - 1 : rootHeight - 5,
          rgba: background,
        },
        {
          id: `${id}-item-start-edge`, nodeId: itemNodeId,
          xCssPx: rowAxis ? itemX + 4 : itemX,
          yCssPx: rowAxis ? itemY : itemY + 4,
          rgba: itemColor,
        },
        {
          id: `${id}-${itemExtendsPastRoot ? 'item-end-outside-root' : 'item-end-edge'}`,
          nodeId: itemNodeId,
          xCssPx: rowAxis ? itemX + 4 : itemX + itemWidth - 1,
          yCssPx: rowAxis ? itemY + itemHeight - 1 : itemY + 4,
          rgba: itemColor,
        },
      ];
    } else {
      scene.pixelSamples = [
        ...(itemX > 0 ? [{ id: `${id}-before-item`, nodeId: rootNodeId, xCssPx: itemX - 1, yCssPx: 20, rgba: background }] : []),
        { id: `${id}-item-start-edge`, nodeId: itemNodeId, xCssPx: itemX, yCssPx: 20, rgba: itemColor },
        { id: `${id}-item-end-edge`, nodeId: itemNodeId, xCssPx: itemX + itemWidth - 1, yCssPx: 20, rgba: itemColor },
        { id: `${id}-after-item`, nodeId: rootNodeId, xCssPx: itemX + itemWidth, yCssPx: 20, rgba: background },
      ];
    }
    return scene;
  });
}

async function templateAreaRepeatScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/template_area_auto_repeat_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 29, 'template-area auto-repeat fixture는 29 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');

  return fixtures.map(({ id, expect, rootColor, items: fixtureItems }) => {
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const [rootWidth, rootHeight] = expect.root;
    const items = expect.items;
    if (id.startsWith('template-area-repeat-two-axis-')) {
      const rootNodeId = `c114-${id}-grid`;
      const frameList = items.map(({ name, frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      });
      const nodeFrames = `node_frames_css_px=[${[
        `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
        ...frameList,
      ].join(';')}]`;
      const rgba = (color) => [
        Number.parseInt(color.slice(1, 3), 16),
        Number.parseInt(color.slice(3, 5), 16),
        Number.parseInt(color.slice(5, 7), 16),
        255,
      ];
      const expandedSpan = id.endsWith('-fit-fit-area-span');
      const pixelSamples = [{
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: rootWidth - 2, yCssPx: rootHeight - 2,
        rgba: expandedSpan ? [229, 72, 77, 255] : rgba(rootColor),
      }];
      if (id.endsWith('-area-added-both')) {
        pixelSamples.push(
          {
            id: `${id}-hero-overflow-outside-root`,
            nodeId: `c114-${id}-hero`, xCssPx: 47, yCssPx: 43,
            rgba: [229, 72, 77, 255],
          },
          {
            id: `${id}-after-root`, nodeId: rootNodeId, xCssPx: 53, yCssPx: 43,
            rgba: [0, 0, 0, 255],
          },
          {
            id: `${id}-after-root-bottom`, nodeId: rootNodeId, xCssPx: 47, yCssPx: 49,
            rgba: [0, 0, 0, 255],
          },
        );
      } else if (expandedSpan) {
        pixelSamples.push(
          {
            id: `${id}-hero-overflow-outside-root`,
            nodeId: `c114-${id}-hero`, xCssPx: 60, yCssPx: 50,
            rgba: [229, 72, 77, 255],
          },
          {
            id: `${id}-after-root`, nodeId: rootNodeId, xCssPx: 64, yCssPx: 50,
            rgba: [0, 0, 0, 255],
          },
          {
            id: `${id}-after-root-bottom`, nodeId: rootNodeId, xCssPx: 50, yCssPx: 60,
            rgba: [0, 0, 0, 255],
          },
        );
      } else if (id.endsWith('-fit-fill-area-item')) {
        pixelSamples.push(
          {
            id: `${id}-hero-overflow-outside-root`,
            nodeId: `c114-${id}-hero`, xCssPx: 8, yCssPx: 56,
            rgba: [229, 72, 77, 255],
          },
          {
            id: `${id}-after-root`, nodeId: rootNodeId, xCssPx: 8, yCssPx: 60,
            rgba: [0, 0, 0, 255],
          },
          {
            id: `${id}-after-root-bottom`, nodeId: rootNodeId, xCssPx: 50, yCssPx: 56,
            rgba: [0, 0, 0, 255],
          },
        );
      } else if (id.endsWith('-fill-fit-area-item')) {
        pixelSamples.push(
          {
            id: `${id}-hero-overflow-outside-root`,
            nodeId: `c114-${id}-hero`, xCssPx: 56, yCssPx: 8,
            rgba: [229, 72, 77, 255],
          },
          {
            id: `${id}-after-root`, nodeId: rootNodeId, xCssPx: 64, yCssPx: 8,
            rgba: [0, 0, 0, 255],
          },
          {
            id: `${id}-after-root-bottom`, nodeId: rootNodeId, xCssPx: 56, yCssPx: 50,
            rgba: [0, 0, 0, 255],
          },
        );
      } else {
        for (const { name, frame } of items) {
          const [x, y, width, height] = frame;
          const item = fixtureItems.find(({ name: itemName }) => itemName === name);
          pixelSamples.push({
            id: `${id}-${name}-interior`, nodeId: `c114-${id}-${name}`,
            xCssPx: x + Math.floor(width / 2), yCssPx: y + Math.floor(height / 2),
            rgba: rgba(item.color),
          });
        }
        const furthestX = Math.max(rootWidth, ...items.map(({ frame: [x, , width] }) => x + width));
        pixelSamples.push({
          id: `${id}-after-root`, nodeId: rootNodeId,
          xCssPx: furthestX, yCssPx: 10, rgba: [0, 0, 0, 255],
        });
      }
      return {
        id,
        axis: 'Both',
        boxCount: items.length + 1,
        rootFrame: `width=${rootWidth},height=${rootHeight}`,
        itemFrame: null,
        nodeFrames,
        axes: {
          Columns: {
            tracks: expect.tracks.columns,
            collapsed: expect.tracks.columns.map((track) => track === 0),
          },
          Rows: {
            tracks: expect.tracks.rows,
            collapsed: expect.tracks.rows.map((track) => track === 0),
          },
        },
        gaps: { Columns: 4, Rows: 4 },
        pixelSamples,
      };
    }
    const axis = id.includes('-columns-') ? 'Columns' : 'Rows';
    const tracks = expect.tracks;
    const rootFrame = `width=${rootWidth},height=${rootHeight}`;
    const rootNodeId = `c114-${id}-grid`;
    const itemFrames = items.map(({ name, frame }, index) => {
      const [x, y, width, height] = frame;
      return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
    });
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,${rootFrame};${itemFrames.join(';')}]`;
    const allFrames = items.map(({ frame }) => frame);
    const furthestX = Math.max(rootWidth, ...allFrames.map(([x, , width]) => x + width));
    const furthestY = Math.max(rootHeight, ...allFrames.map(([, y, , height]) => y + height));
    const rootSamplePoint = id.endsWith('-line-collision') ? [94, 10]
      : id.endsWith('-fit-empty-area') ? (axis === 'Rows' ? [2, 60] : [60, 2])
        : axis === 'Rows' ? [2, 22] : [22, 2];
    const pixelSamples = [
      {
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: rootSamplePoint[0], yCssPx: rootSamplePoint[1], rgba: [16, 24, 32, 255],
      },
      ...items.map(({ name, frame }) => {
        const [x, y, width, height] = frame;
        const rgb = name === 'suffix' ? [48, 164, 108]
          : name === 'tail' ? [114, 87, 255] : [229, 72, 77];
        return {
          id: `${id}-${name}-interior`,
          nodeId: `c114-${id}-${name}`,
          xCssPx: x + Math.floor(width / 2),
          yCssPx: y + Math.floor(height / 2),
          rgba: [...rgb, 255],
        };
      }),
      {
        id: `${id}-after-root`,
        nodeId: rootNodeId,
        xCssPx: axis === 'Rows' ? 10 : furthestX,
        yCssPx: axis === 'Rows' ? furthestY : 10,
        rgba: [0, 0, 0, 255],
      },
    ];

    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame,
      itemFrame: null,
      nodeFrames,
      axes: {
        [axis]: { tracks, collapsed: tracks.map((track) => track === 0) },
      },
      gaps: { Columns: 4, Rows: 4 },
      pixelSamples,
    };
  });
}

async function autoRepeatCalcMinimumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_minimum_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 12,
    'absolute 및 percentage calc auto-repeat fixture는 12 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');

  return fixtures.map(({ id, expect, columnGap, rowGap, items }) => {
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const [rootWidth, rootHeight] = expect.root;
    const axis = id.startsWith('columns-') ? 'Columns' : 'Rows';
    const nodeFrames = [
      `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
      ...expect.items.map(({ frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      }),
    ].join(';');
    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${nodeFrames}]`,
      axes: Object.fromEntries(Object.entries(expect.tracks).map(([trackAxis, tracks]) => [
        trackAxis === 'columns' ? 'Columns' : 'Rows', {
          tracks,
          collapsed: expect.collapsed[trackAxis],
        },
      ])),
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
    };
  });
}

async function autoRepeatPositivePercentMinimumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_positive_percent_minimum_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 16,
    'positive percentage auto-repeat fixture는 16 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');

  return fixtures.map(({ id, expect, columnGap, rowGap, items }) => {
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const [rootWidth, rootHeight] = expect.root;
    const axis = id.includes('-columns-') ? 'Columns' : 'Rows';
    const nodeFrames = [
      `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
      ...expect.items.map(({ frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      }),
    ].join(';');
    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${nodeFrames}]`,
      axes: Object.fromEntries(Object.entries(expect.tracks).map(([trackAxis, tracks]) => [
        trackAxis === 'columns' ? 'Columns' : 'Rows', {
          tracks,
          collapsed: expect.collapsed[trackAxis],
        },
      ])),
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
    };
  });
}

async function autoRepeatZeroPercentMinimumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_percent_minimum_scenes.json');
  const oraclePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/spec-derived-expectations.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, oracle, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(oraclePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 13,
    'zero-percent auto-repeat fixture는 13 scene이어야 합니다.');
  assert.equal(oracle.scenes.length, 13,
    'zero-percent Chrome oracle은 13 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');
  const oracleById = new Map(oracle.scenes.map((scene) => [scene.id, scene]));
  const expandRuns = (runs) => {
    const tracks = [];
    const collapsed = [];
    for (const [size, count, isCollapsed] of runs) {
      tracks.push(...Array(count).fill(size));
      collapsed.push(...Array(count).fill(isCollapsed));
    }
    return { tracks, collapsed };
  };
  const expectedHistogram = (runs) => {
    const histogram = new Map();
    for (const [size, count] of runs) {
      const key = `${Number(size.toFixed(6))}px`;
      histogram.set(key, (histogram.get(key) ?? 0) + count);
    }
    return Object.fromEntries([...histogram].sort(([left], [right]) => left.localeCompare(right)));
  };

  return fixtures.map(({ id, expect, columnGap, rowGap, items }) => {
    const oracleScene = oracleById.get(id);
    assert.ok(oracleScene, `독립 Chrome oracle에 ${id}가 없습니다.`);
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const [rootWidth, rootHeight] = expect.root;
    const axis = id.startsWith('fill-row') || id.startsWith('fit-row')
      ? 'Rows' : 'Columns';
    const axes = Object.fromEntries(Object.entries(expect.trackRuns).map(([trackAxis, runs]) => [
      trackAxis === 'columns' ? 'Columns' : 'Rows', expandRuns(runs),
    ]));
    const activeTrackRuns = expect.trackRuns[axis.toLowerCase()];
    const actualTotalTrackCount = activeTrackRuns.reduce((sum, [, count]) => sum + count, 0);
    assert.equal(actualTotalTrackCount, oracleScene.totalTrackCount,
      `${id} total track count differs from independent oracle`);
    assert.equal(oracleScene.repeatTrackCount, oracleScene.totalTrackCount
      - (id === 'fill-col-zero-neighbors-subpixel' ? 2 : 0),
    `${id} repeat count differs from independent oracle`);
    if (oracleScene.expectedTracks) {
      assert.deepEqual(expectedHistogram(activeTrackRuns.map(([size, count]) => [size, count])),
        oracleScene.expectedTracks, `${id} track histogram differs from independent oracle`);
    }
    if (oracleScene.expectedTrackSizeCssPx !== undefined) {
      const tolerance = oracleScene.actualChromeQuantizationToleranceCssPx ?? 1 / 64;
      for (const [size, count] of activeTrackRuns) {
        assert.ok(Math.abs(size - oracleScene.expectedTrackSizeCssPx) <= tolerance,
          `${id} expected repeated track ${size} to match ${oracleScene.expectedTrackSizeCssPx}`);
        assert.ok(count > 0, `${id} has an empty expected track run`);
      }
    }
    const nodeFrames = [
      `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
      ...expect.items.map(({ frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      }),
    ].join(';');
    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${nodeFrames}]`,
      axes,
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
    };
  });
}

async function autoRepeatPercentageMaximumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_percentage_maximum_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 28,
    'percentage auto-repeat maximum fixture는 28 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid track sizing');
  const iosScenes = allowlist(iosSource,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid track sizing');
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];

  return fixtures.map(({ id, expect, columnGap, rowGap, items }) => {
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const [rootWidth, rootHeight] = expect.root;
    const axis = id.startsWith('both-')
      ? 'Both'
      : id.startsWith('columns-') ? 'Columns' : 'Rows';
    const rootNodeId = `c114-${id}-grid`;
    const itemFrames = expect.items.map(({ frame }, index) => {
      const [x, y, width, height] = frame;
      return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
    });
    const nodeFrames =
      `node_frames_css_px=[0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight};${itemFrames.join(';')}]`;
    const pixelSamples = [
      {
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: 180, yCssPx: 180, rgba: [226, 232, 240, 255],
      },
      ...expect.items.map(({ name, frame }, index) => {
        const [x, y, width, height] = frame;
        return {
          id: `${id}-${name}-interior`,
          nodeId: `c114-${id}-${name}`,
          xCssPx: Math.floor(x + width / 2),
          yCssPx: Math.floor(y + height / 2),
          rgba: colorRgba(items[index].color),
        };
      }),
    ];

    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames,
      axes: Object.fromEntries(Object.entries(expect.tracks).map(([trackAxis, tracks]) => [
        trackAxis === 'columns' ? 'Columns' : 'Rows', {
          tracks,
          collapsed: expect.collapsed[trackAxis],
        },
      ])),
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      pixelSamples,
    };
  });
}

scenes.push(...await templateAreaRepeatScenes());
scenes.push(...await autoRepeatNamedLineScenes());
scenes.push(...await autoRepeatCalcMinimumScenes());
scenes.push(...await autoRepeatPositivePercentMinimumScenes());
scenes.push(...await autoRepeatZeroPercentMinimumScenes());
scenes.push(...await autoRepeatPercentageMaximumScenes());
assert.equal(scenes.length, 405, 'C11.4 fixture scene count differs.');
assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length,
  'C11.4 fixture scene IDs must be unique.');
const requestedSceneIDSet = requestedSceneIDs ? new Set(requestedSceneIDs) : null;
const runScenes = requestedSceneIDSet
  ? scenes.filter(({ id }) => requestedSceneIDSet.has(id))
  : scenes;
assert.ok(runScenes.length > 0,
  requestedSceneIDs ? `C11.4 scene ID를 찾지 못했습니다: ${requestedSceneIDs.join(',')}` : '실행할 scene이 없습니다.');
const longestRunID = `${runIDPrefix}-${runScenes.length - 1}-retry-3`;
assert.ok(longestRunID.length <= 80,
  `C11.4 run ID가 플랫폼 인수 제한 80자를 넘습니다: ${longestRunID.length}`);
if (requestedSceneIDs) {
  assert.equal(requestedSceneIDSet.size, requestedSceneIDs.length,
    'C11.4 선택 scene ID가 중복됐습니다.');
  assert.equal(runScenes.length, requestedSceneIDs.length,
    'C11.4 선택 scene ID 중 일부가 없습니다.');
}

function anonymousGridBorderOverflowScenes() {
  const text = (node, x) => ({
    node,
    kind: 'text',
    frame: { node, x, y: 2, width: 20, height: 10 },
    source: {
      kind: 'text', fontRegistryRevision: 1,
      minContent: 20, minContentHeight: 10, maxContent: 20, maxContentHeight: 10,
    },
    textSources: [{ node, text: 'XX' }],
  });
  const image = (node, x, y, width, height, columnSpan = 2) => ({
    node,
    kind: 'replaced',
    frame: { node, x, y, width, height },
    source: { kind: 'replaced', aspectRatio: null, width: 24, height: 16 },
    columnSpan,
    textSources: [],
  });
  const makeScene = (id, inputs, options = {}) => {
    const tracks = options.tracks ?? [12, 28, 20, 20];
    const intrinsicSpans = options.intrinsicSpans ?? [1, 2];
    const overflowSample = options.overflowSample ?? {
      id: 'image-overflow-png-quadrant', xCssPx: 51, yCssPx: 71,
    };
    const rootNodeId = `c114-${id}-grid`;
    const imageFrames = inputs.filter((input) => input.kind === 'replaced');
    const nodeFrames = ['0:node=1,x=0,y=0,width=40,height=184',
      ...imageFrames.map((input, index) => `${index + 1}:node=${input.node},x=${input.frame.x},y=${input.frame.y},`
        + `width=${input.frame.width},height=${input.frame.height}`)];
    return {
      id,
      axis: 'Columns',
      boxCount: imageFrames.length + 1,
      rootFrame: 'width=40,height=184',
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${nodeFrames.join(';')}]`,
      tracks,
      collapsed: [false, false, false, false],
      gaps: { Columns: 0, Rows: 0 },
      axes: {
        Columns: { tracks, collapsed: [false, false, false, false] },
        Rows: { tracks: [60, 60, 60], collapsed: [false, false, false] },
      },
      intrinsicSpans,
      intrinsicSpansByAxis: { Columns: intrinsicSpans, Rows: [] },
      itemSpans: inputs.map((input) => input.kind === 'text' ? 1 : input.columnSpan),
      imageMinContentWidths: inputs.map((input) => input.kind === 'text' ? null : input.frame.width),
      mixedIntrinsicContributions: true,
      anonymousGridBorderOverflowContributions: true,
      anonymousTextMultiImageContributions: true,
      runtimeImageCount: imageFrames.length,
      intrinsicInputs: inputs.map((input, sourceOrder) => input.kind === 'text'
        ? {
          grid: 1, node: input.node, sourceOrder, kind: 'text',
          fontRegistryRevision: 1, minContent: 20, minContentHeight: 10,
          maxContent: 20, maxContentHeight: 10,
        }
        : {
          grid: 1, node: input.node, sourceOrder, kind: 'replaced',
          aspectRatio: null, width: 24, height: 16,
        }),
      anonymousTextImageItems: inputs.map((input, sourceOrder) => ({
        grid: 1,
        node: input.node,
        sourceOrder,
        source: input.source,
        textSources: input.textSources,
        frame: input.frame,
      })),
      pixelSamples: [
        { id: 'top-border-color', nodeId: rootNodeId,
          xCssPx: 10, yCssPx: 0, rgba: [240, 198, 116, 255] },
        { id: overflowSample.id, nodeId: rootNodeId,
          xCssPx: overflowSample.xCssPx, yCssPx: overflowSample.yCssPx,
          rgba: [0, 255, 64, 255] },
      ],
    };
  };
  return [
    makeScene('anonymous-runs-two-images-forward', [
      text(2, 42), image(3, 2, 2, 24, 16), text(4, 62), image(5, 14, 62, 48, 32),
    ]),
    makeScene('two-images-anonymous-runs-reversed', [
      image(2, 14, 62, 48, 32), text(3, 42), image(4, 2, 2, 24, 16), text(5, 62),
    ]),
    makeScene('two-anonymous-runs-three-images-forward', [
      text(2, 54), image(3, 2, 2, 24, 16), text(4, 74),
      image(5, 20, 62, 48, 32), image(6, 2, 122, 72, 48, 3),
    ], {
      tracks: [18, 34, 20, 20],
      intrinsicSpans: [1, 2, 3],
      overflowSample: { id: 'image-c-overflow-green-png-texel', xCssPx: 62, yCssPx: 128 },
    }),
    makeScene('three-images-two-anonymous-runs-reversed', [
      image(2, 2, 122, 72, 48, 3), text(3, 54), image(4, 20, 62, 48, 32),
      text(5, 74), image(6, 2, 2, 24, 16),
    ], {
      tracks: [18, 34, 20, 20],
      intrinsicSpans: [1, 2, 3],
      overflowSample: { id: 'image-c-overflow-green-png-texel', xCssPx: 62, yCssPx: 128 },
    }),
  ];
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
        { id: 'negative-root-edge', nodeId: `c114-${id}-grid`, xCssPx: 0, yCssPx: 10, rgba: [17, 24, 39, 255] },
        { id: 'negative-root-exterior', nodeId: `c114-${id}-grid`, xCssPx: 150, yCssPx: 10, rgba: [0, 0, 0, 255] },
        { id: 'negative-overflow-item', nodeId: `c114-${id}-item-${observation.items.length}`, xCssPx: 250, yCssPx: 10, rgba: [141, 95, 211, 255] },
        { id: 'negative-overflow-tail', nodeId: `c114-${id}-grid`, xCssPx: 273, yCssPx: 10, rgba: [0, 0, 0, 255] },
      ];
    } else if (id === 'rows-negative-fixed-root-overflow') {
      scene.pixelSamples = [
        { id: 'negative-root-interior', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 19, rgba: [226, 232, 240, 255] },
        { id: 'negative-root-edge', nodeId: `c114-${id}-grid`, xCssPx: 10, yCssPx: 0, rgba: [17, 24, 39, 255] },
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
      xCssPx: 0, yCssPx: 8, rgba: [17, 204, 136, 255] },
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

function c114TraceChunkState(log, runID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  const parts = log.split(/\r?\n/).flatMap((line) => {
    const markerOffset = line.indexOf(marker);
    if (markerOffset < 0) return [];
    const match = line.slice(markerOffset + marker.length).match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
  const total = parts[0]?.total ?? null;
  const byIndex = new Map();
  let consistent = total !== null;
  for (const part of parts) {
    if (part.total !== total || part.index < 1 || part.index > part.total) consistent = false;
    byIndex.set(part.index, part);
  }
  const missing = total === null
    ? []
    : Array.from({ length: total }, (_, index) => index + 1).filter((index) => !byIndex.has(index));
  const orderedParts = [...byIndex.values()].sort((left, right) => left.index - right.index);
  return {
    complete: consistent && missing.length === 0,
    total,
    count: byIndex.size,
    missing,
    parts: orderedParts,
  };
}

function waitForLog(readLog, scene, runID) {
  const deadline = Date.now() + 150_000;
  let latest = '';
  while (Date.now() < deadline) {
    latest = readLog();
    const summary = latest.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
    const traceComplete = c114TraceChunkState(latest, runID).complete;
    const draw = latest.includes('SPINON_C0410_DRAW')
      && latest.includes(`presented boxes=${scene.boxCount}`);
    const pixelSummary = !scene.pixelSamples
      || (target === 'ios-simulator'
        ? latest.includes(`SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`)
        : latest.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`));
    if ((summary && traceComplete && draw && pixelSummary)
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
  const assertionError = log.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID} `));
  assert.ok(!assertionError,
    `${scene.id}: native runtime assertion failed: ${assertionError ?? ''}`);
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
  assert.ok((scene.rootFrame === null || log.includes(scene.rootFrame))
    && (scene.itemFrame === null || log.includes(scene.itemFrame)),
    `${scene.id}: root/item frame이 예상과 다릅니다.`);
  const traceState = c114TraceChunkState(log, runID);
  assert.ok(traceState.complete,
    `${scene.id}: track-sizing trace chunk이 누락됐습니다 (${traceState.count}/${traceState.total ?? '?'}, missing: ${traceState.missing.join(', ')}).`);
  assert.ok(log.includes('SPINON_C0410_DRAW')
    && log.includes(`presented boxes=${scene.boxCount}`),
    `${scene.id}: 화면 표시 결과가 없습니다.`);
  if (scene.runtimeImageCount !== undefined) {
    assert.ok(log.split(/\r?\n/).some((line) => line.includes('SPINON_C0410_DRAW')
      && line.includes(`runtime_image_count=${scene.runtimeImageCount} image_draw_count=${scene.runtimeImageCount}`)),
    `${scene.id}: 고정 PNG 전체가 runtime scene과 실제 draw에 포함되지 않았습니다.`);
  }
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: 실행 인자가 거부됐습니다.`);
  const tracePayload = traceState.parts.map(({ payload }) => payload).join('')
    .replace(/^grid_track_sizing_diagnostics=/, '');
  const diagnostics = JSON.parse(tracePayload);
  if (scene.nestedGridIntrinsic) {
    const expected = scene.nestedGridIntrinsic;
    const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      kind: input.source?.kind,
      minContent: input.source?.minContent,
      maxContent: input.source?.maxContent,
    }));
    const expectedInputs = expected.leafNodes.map((node, sourceOrder) => ({
      grid: expected.nestedNode,
      node,
      sourceOrder,
      kind: 'text',
      minContent: sourceOrder === 0 ? 70 : 60,
      maxContent: sourceOrder === 0 ? 100 : 60,
    }));
    assert.deepEqual(actualInputs, expectedInputs,
      `${scene.id}: nested grid leaf text input mapping differs`);
    assert.ok(actualInputs.every(({ node }) => node !== expected.nestedNode),
      `${scene.id}: nested Grid must not be represented as a leaf intrinsic input`);
    const outerTrace = diagnostics.trackSizingTraces
      .find((trace) => trace.grid === expected.outerNode);
    const nestedTrace = diagnostics.trackSizingTraces
      .find((trace) => trace.grid === expected.nestedNode);
    assert.ok(outerTrace, `${scene.id}: outer Grid trace is missing`);
    assert.ok(nestedTrace, `${scene.id}: nested Grid trace is missing`);
    const nestedColumns = nestedTrace.passes.filter((pass) => pass.axis === 'Columns').at(-1);
    const nestedTracks = nestedColumns.stages.at(-1).tracks
      .filter(({ gutter }) => !gutter).map(({ baseSize }) => baseSize);
    assert.deepEqual(nestedTracks, expected.innerTracks,
      `${scene.id}: nested Grid final columns differ`);
    const outerColumns = outerTrace.passes.filter((pass) => pass.axis === 'Columns').at(-1);
    const nestedContributions = outerColumns.contributions.filter(({ item }) => item === expected.nestedNode);
    if (expected.contributionValue !== undefined) {
      assert.ok(nestedContributions.some((contribution) => contribution.kind === 'MinContent'
        && contribution.span === expected.contributionSpan
        && Math.abs(contribution.value - expected.contributionValue) <= 1 / 64),
      `${scene.id}: nested Grid min-content parent contribution differs`);
    } else {
      assert.ok(nestedContributions.some(({ kind, span, value }) => kind === 'Minimum'
        && span === expected.contributionSpan
        && Math.abs(value - expected.minimumContributionValue) <= 1 / 64),
      `${scene.id}: nested Grid minimum parent contribution differs`);
      assert.ok(nestedContributions.every(({ value }) => Number.isFinite(value)
        && value >= 0 && value <= expected.maximumContributionValue),
        `${scene.id}: inner overflow enlarged the definite outer Grid`);
    }
  }
  if (scene.nestedGridTextImageIntrinsic) {
    const expected = scene.nestedGridTextImageIntrinsic;
    const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      source: input.source?.kind === 'text'
        ? {
          kind: 'text',
          minContent: input.source.minContent,
          maxContent: input.source.maxContent,
        }
        : {
          kind: input.source?.kind,
          width: input.source?.natural?.width,
          height: input.source?.natural?.height,
        },
    }));
    assert.deepEqual(actualInputs, [
      {
        grid: expected.nestedNode, node: expected.textNode, sourceOrder: 0,
        source: { kind: 'text', minContent: 70, maxContent: 100 },
      },
      {
        grid: expected.nestedNode, node: expected.imageNode, sourceOrder: 1,
        source: { kind: 'replaced', width: 24, height: 16 },
      },
    ], `${scene.id}: nested text/image intrinsic inputs differ`);
    assert.ok(actualInputs.every(({ node }) => node !== expected.nestedNode),
      `${scene.id}: nested Grid must remain the outer item, not a leaf intrinsic input`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: image intrinsic revision is missing from the layout stamp`);
    const nestedTrace = diagnostics.trackSizingTraces
      .find((trace) => trace.grid === expected.nestedNode);
    assert.ok(nestedTrace, `${scene.id}: nested Grid trace is missing`);
    const innerAxis = expected.innerAxis ?? 'Columns';
    const nestedPass = nestedTrace.passes.filter((pass) => pass.axis === innerAxis).at(-1);
    const nestedTracks = nestedPass.stages.at(-1).tracks
      .filter(({ gutter }) => !gutter).map(({ baseSize }) => baseSize);
    assert.deepEqual(nestedTracks, expected.innerTracks,
      `${scene.id}: nested Grid final ${innerAxis} differ`);
    const outerTrace = diagnostics.trackSizingTraces
      .find((trace) => trace.grid === expected.outerNode);
    assert.ok(outerTrace, `${scene.id}: outer Grid trace is missing`);
    const contributionAxis = expected.contributionAxis ?? 'Columns';
    const outerPass = outerTrace.passes.filter((pass) => pass.axis === contributionAxis).at(-1);
    const nestedContributions = outerPass.contributions
      .filter(({ item }) => item === expected.nestedNode);
    if (expected.contributionValue !== undefined) {
      assert.ok(nestedContributions.some((contribution) => contribution.kind === 'MinContent'
        && contribution.span === expected.contributionSpan
        && Math.abs(contribution.value - expected.contributionValue) <= 1 / 64),
      `${scene.id}: nested Grid min-content parent contribution differs`);
    } else {
      assert.ok(nestedContributions.some(({ kind, span, value }) => kind === 'Minimum'
        && span === expected.contributionSpan
        && Math.abs(value - expected.minimumContributionValue) <= 1 / 64),
      `${scene.id}: nested Grid minimum parent contribution differs`);
      assert.ok(nestedContributions.every(({ value }) => Number.isFinite(value)
        && value >= 0 && value <= expected.maximumContributionValue),
      `${scene.id}: nested content enlarged the definite outer Grid`);
    }
  }
  if (scene.nestedGridImplicitRows) {
    const expected = scene.nestedGridImplicitRows;
    const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      kind: input.source?.kind,
      width: input.source?.natural?.width,
      height: input.source?.natural?.height,
    }));
    assert.deepEqual(actualInputs, [{
      grid: expected.nestedNode, node: expected.imageNode, sourceOrder: 2,
      kind: 'replaced', width: 24, height: 16,
    }], `${scene.id}: nested Grid image intrinsic input differs`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: decoded image revision is missing from the layout stamp`);
    assert.equal(diagnostics.trackSizingTraces.length, 2,
      `${scene.id}: nested + outer Grid trace count differs`);
    const outerTrace = diagnostics.trackSizingTraces.find(
      (trace) => trace.grid === expected.outerNode);
    const nestedTrace = diagnostics.trackSizingTraces.find(
      (trace) => trace.grid === expected.nestedNode);
    assert.ok(outerTrace, `${scene.id}: outer Grid trace is missing`);
    assert.ok(nestedTrace, `${scene.id}: nested Grid trace is missing`);
    const tracksForAxis = (trace, axis) => {
      const matchingPasses = trace.passes.filter((pass) => pass.axis === axis);
      assert.equal(matchingPasses.length, 1,
        `${scene.id}: Grid ${trace.grid} must have one ${axis} pass`);
      const tracks = matchingPasses[0].stages.at(-1)?.tracks;
      assert.ok(tracks, `${scene.id}: Grid ${trace.grid} ${axis} final tracks are missing`);
      return tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
        .map(({ baseSize }) => baseSize);
    };
    assert.deepEqual(tracksForAxis(outerTrace, 'Columns'), [24],
      `${scene.id}: outer Grid columns differ`);
    assert.deepEqual(tracksForAxis(outerTrace, 'Rows'), expected.outerRows,
      `${scene.id}: outer Grid rows differ`);
    assert.deepEqual(tracksForAxis(nestedTrace, 'Columns'), [24],
      `${scene.id}: nested Grid columns differ`);
    assert.deepEqual(tracksForAxis(nestedTrace, 'Rows'), expected.innerRows,
      `${scene.id}: nested Grid implicit rows differ`);
    const outerRows = outerTrace.passes.find((pass) => pass.axis === 'Rows');
    const contribution = outerRows.contributions.find((item) => (
      item.item === expected.nestedNode
        && item.kind === 'MinContent'
        && item.span === expected.contributionSpan
        && Math.abs(item.value - expected.contributionValue) <= 1 / 64
    ));
    assert.ok(contribution,
      `${scene.id}: nested Grid min-content row contribution differs`);
  }
  if (scene.nestedGridTextMultiSpan) {
    const expected = scene.nestedGridTextMultiSpan;
    assert.equal(diagnostics.trackSizingTraces.length, expected.gridNodes.length,
      `${scene.id}: root-to-inner Grid trace count differs`);
    const traces = new Map(diagnostics.trackSizingTraces.map((trace) => [trace.grid, trace]));
    const tracksForAxis = (trace, axis) => {
      const passes = trace.passes.filter((pass) => pass.axis === axis);
      assert.equal(passes.length, 1,
        `${scene.id}: Grid ${trace.grid} must have one ${axis} pass`);
      const tracks = passes[0].stages.at(-1)?.tracks;
      assert.ok(tracks, `${scene.id}: Grid ${trace.grid} ${axis} final tracks are missing`);
      return tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
        .map(({ baseSize }) => baseSize);
    };
    const assertTracks = (trace, axis, expectedTracks) => {
      const actual = tracksForAxis(trace, axis);
      assert.equal(actual.length, expectedTracks.length,
        `${scene.id}: Grid ${trace.grid} ${axis} track count differs`);
      actual.forEach((size, index) => assert.ok(
        Math.abs(size - expectedTracks[index]) <= 1 / 64,
        `${scene.id}: Grid ${trace.grid} ${axis} track ${index + 1} expected ${expectedTracks[index]}, got ${size}`,
      ));
    };
    for (const [index, grid] of expected.gridNodes.entries()) {
      const trace = traces.get(grid);
      assert.ok(trace, `${scene.id}: Grid ${grid} trace is missing`);
      const inner = index === expected.gridNodes.length - 1;
      assertTracks(trace, 'Columns', inner ? expected.innerColumns : [expected.width]);
      assertTracks(trace, 'Rows', inner ? [20, 20, 20] : [60]);
      if (!inner) {
        const child = expected.gridNodes[index + 1];
        const pass = trace.passes.find((candidate) => candidate.axis === 'Columns');
        assert.ok(pass.contributions.some((contribution) => (
          contribution.item === child
            && contribution.kind === 'MinContent'
            && contribution.span === 1
            && Math.abs(contribution.value - expected.width) <= 1 / 64
        )), `${scene.id}: parent Grid ${grid} lost nested child ${child} contribution`);
      }
    }
    const innerTrace = traces.get(expected.gridNodes.at(-1));
    const innerColumns = innerTrace.passes.find((pass) => pass.axis === 'Columns');
    const expectedContributions = [];
    for (const span of [...new Set(expected.items.map((item) => item.span))].sort((a, b) => a - b)) {
      const spanItems = expected.items.filter((item) => item.span === span);
      for (const kind of ['Minimum', 'MinContent']) {
        const repetitions = kind === 'MinContent' ? 2 : 1;
        for (let repetition = 0; repetition < repetitions; repetition++) {
          for (const item of spanItems) {
            expectedContributions.push({
              item: item.node,
              kind,
              span,
              value: kind === 'Minimum' ? 0 : item.minContent,
            });
          }
        }
      }
    }
    assert.deepEqual(innerColumns.contributions.map(({ item, kind, span, value }) => ({
      item, kind, span, value,
    })), expectedContributions,
    `${scene.id}: inner text span contributions are duplicated, reordered, or lost`);
  }
  if (scene.nestedGridDepth) {
    const expected = scene.nestedGridDepth;
    assert.equal(diagnostics.trackSizingTraces.length, expected.gridNodes.length,
      `${scene.id}: root-to-leaf Grid trace count differs`);
    const traces = new Map(diagnostics.trackSizingTraces.map((trace) => [trace.grid, trace]));
    const tracksForAxis = (trace, axis) => {
      const passes = trace.passes.filter((pass) => pass.axis === axis);
      assert.equal(passes.length, 1,
        `${scene.id}: Grid ${trace.grid} must have one ${axis} pass`);
      const tracks = passes[0].stages.at(-1)?.tracks;
      assert.ok(tracks, `${scene.id}: Grid ${trace.grid} ${axis} final tracks are missing`);
      return tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
        .map(({ baseSize }) => baseSize);
    };
    for (const [index, grid] of expected.gridNodes.entries()) {
      const trace = traces.get(grid);
      assert.ok(trace, `${scene.id}: Grid ${grid} trace is missing`);
      const expectedColumns = index === expected.gridNodes.length - 1 && expected.innerColumns
        ? expected.innerColumns
        : [expected.width];
      assert.deepEqual(tracksForAxis(trace, 'Columns'), expectedColumns,
        `${scene.id}: Grid ${grid} final Columns differ`);
      assert.deepEqual(tracksForAxis(trace, 'Rows'), [expected.height],
        `${scene.id}: Grid ${grid} final Rows differ`);
      const child = expected.gridNodes[index + 1] ?? expected.leafNode;
      const contributionTargets = index === expected.gridNodes.length - 1
        && expected.directItemContributions
        ? expected.directItemContributions
        : [{ node: child, width: expected.width, height: expected.height }];
      for (const target of contributionTargets) {
        const expectedContributions = [['Columns', target.width]];
        if (expected.twoAxis) expectedContributions.push(['Rows', target.height]);
        for (const [axis, value] of expectedContributions) {
        const pass = trace.passes.find((candidate) => candidate.axis === axis);
        assert.ok(pass?.contributions.some((contribution) => (
          contribution.item === target.node
            && contribution.kind === expected.contributionKind
            && contribution.span === 1
            && Math.abs(contribution.value - value) <= 1 / 64
        )), `${scene.id}: Grid ${grid} ${axis} lacks ${expected.contributionKind} from child ${target.node}`);
        }
      }
    }
    if (scene.intrinsicInputs && !scene.mixedIntrinsicContributions
      && !scene.nestedGridAnonymousTextImage
      && !scene.replacedCyclicMinmaxImageSpan) {
      assert.deepEqual((diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        kind: input.source?.kind,
        fontRegistryRevision: input.source?.fontRegistryRevision,
        minContent: input.source?.minContent,
        minContentHeight: input.source?.minContentHeight,
        maxContent: input.source?.maxContent,
        maxContentHeight: input.source?.maxContentHeight,
      })), scene.intrinsicInputs,
      `${scene.id}: direct leaf intrinsic source or font revision differs`);
    }
    assert.ok(expected.leafNode !== expected.gridNodes.at(-1),
      `${scene.id}: text leaf must be separate from the innermost Grid`);
  }
  if (scene.anonymousTextItems) {
    const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      source: input.source && {
        kind: input.source.kind,
        fontRegistryRevision: input.source.fontRegistryRevision,
        minContent: input.source.minContent,
        minContentHeight: input.source.minContentHeight,
        maxContent: input.source.maxContent,
        maxContentHeight: input.source.maxContentHeight,
      },
      textSources: input.textSources,
    }));
    const expectedInputs = scene.anonymousTextItems.map((input) => ({
      grid: input.grid,
      node: input.node,
      sourceOrder: input.sourceOrder,
      source: {
        kind: 'text',
        fontRegistryRevision: 1,
        ...input.source,
      },
      textSources: input.textSources,
    }));
    assert.deepEqual(actualInputs, expectedInputs,
      `${scene.id}: anonymous item source grouping and intrinsic sizes differ`);
    assert.deepEqual(diagnostics.gridItemFrames ?? [],
      scene.anonymousTextItems.map(({ frame }) => frame),
      `${scene.id}: anonymous item frame evidence differs`);
  }
  if (scene.anonymousTextImageItems) {
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: text/image inputs do not share the stamped image revision`);
    const framesByNode = new Map((diagnostics.gridItemFrames ?? []).map((frame) => [frame.node, frame]));
    const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => {
      const source = input.source?.kind === 'text'
        ? {
          kind: 'text',
          fontRegistryRevision: input.source.fontRegistryRevision,
          minContent: input.source.minContent,
          minContentHeight: input.source.minContentHeight,
          maxContent: input.source.maxContent,
          maxContentHeight: input.source.maxContentHeight,
        }
        : {
          kind: 'replaced',
          aspectRatio: input.source?.natural?.aspectRatio ?? null,
          width: input.source?.natural?.width,
          height: input.source?.natural?.height,
        };
      return {
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        source,
        textSources: input.textSources,
        frame: framesByNode.get(input.node),
      };
    });
    assert.deepEqual(actualInputs, scene.anonymousTextImageItems,
      `${scene.id}: anonymous text grouping, image intrinsic input, and item frames differ`);
  }
  if (scene.intrinsicInputs && scene.replacedCyclicMinmaxImageSpan) {
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
      `${scene.id}: replaced image intrinsic input matrix differs from fixed PNG input`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: image intrinsic resource revision is not the fixed fixture revision`);
  } else if (scene.intrinsicInputs
    && (scene.mixedIntrinsicContributions || scene.nestedGridAnonymousTextImage)) {
    const intrinsicInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => (
      input.source?.kind === 'text' ? {
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        kind: input.source.kind,
        fontRegistryRevision: input.source.fontRegistryRevision,
        minContent: input.source.minContent,
        minContentHeight: input.source.minContentHeight,
        maxContent: input.source.maxContent,
        maxContentHeight: input.source.maxContentHeight,
      } : {
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        kind: input.source?.kind,
        aspectRatio: input.source?.natural?.aspectRatio ?? null,
        width: input.source?.natural?.width,
        height: input.source?.natural?.height,
      }
    ));
    assert.deepEqual(intrinsicInputs, scene.intrinsicInputs,
      `${scene.id}: 같은 Grid revision의 text/replaced intrinsic source mapping이 다릅니다.`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: mixed input의 image resource revision이 layout stamp에 없습니다.`);
  } else if (scene.intrinsicInputs && scene.mixedTextImageSpan) {
    const intrinsicInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => {
      const source = input.source ?? {};
      return source.kind === 'text' ? {
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        kind: source.kind,
        fontRegistryRevision: source.fontRegistryRevision,
        minContent: source.minContent,
        minContentHeight: source.minContentHeight,
        maxContent: source.maxContent,
        maxContentHeight: source.maxContentHeight,
      } : {
        grid: input.grid,
        node: input.node,
        sourceOrder: input.sourceOrder,
        kind: source.kind,
        natural: source.natural,
      };
    });
    assert.deepEqual(intrinsicInputs, scene.intrinsicInputs,
      `${scene.id}: fixed text/image intrinsic inputs differ`);
    assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
      `${scene.id}: mixed image resource revision is not in the layout stamp`);
  } else if (scene.intrinsicInputs && scene.imageIntrinsicContributions) {
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
  const axisTrace = scene.nestedGridIntrinsic || scene.nestedGridTextImageIntrinsic
    || scene.nestedGridImplicitRows || scene.nestedGridDepth
    || scene.nestedGridTextMultiSpan
    ? diagnostics.trackSizingTraces.find(
      (trace) => trace.grid === (scene.nestedGridIntrinsic
        ? scene.nestedGridIntrinsic.outerNode
        : scene.nestedGridTextImageIntrinsic
          ? scene.nestedGridTextImageIntrinsic.outerNode
          : scene.nestedGridImplicitRows
            ? scene.nestedGridImplicitRows.outerNode
            : scene.nestedGridDepth
              ? scene.nestedGridDepth.gridNodes[0]
              : scene.nestedGridTextMultiSpan.gridNodes[0]))
    : diagnostics.trackSizingTraces[0];
  assert.ok(axisTrace, `${scene.id}: expected track-sizing Grid trace가 없습니다.`);
  if (scene.replacedCyclicMinmaxImageSpan) {
    verifyCyclicMinmaxReplacedImageSpanTrace(scene, axisTrace);
  }
  for (const [axis, expected] of Object.entries(axisSpecs)) {
    const axisPass = axisTrace.passes
      .filter((pass) => pass.axis === axis).at(-1);
    assert.ok(axisPass, `${scene.id}: ${axis} track-sizing pass가 없습니다.`);
    const intrinsicSpans = axisPass.stages.flatMap(({ stage }) => {
      const match = stage.match(/^IntrinsicItemBatch\s*\{ span: (\d+),/);
      return match ? [Number(match[1])] : [];
    });
    const expectedSpans = scene.intrinsicSpansByAxis?.[axis] ?? scene.intrinsicSpans;
    if (expectedSpans) {
      assert.deepEqual(intrinsicSpans, expectedSpans,
        `${scene.id}: intrinsic span stage 순서가 다릅니다.`);
    }
    if (scene.twoAxisSpanningTextContributions) {
      const expectedContributions = scene.twoAxisSpanningTextContributions[axis];
      const input = scene.intrinsicInputs[0];
      const observed = axisPass.contributions.filter((contribution) => (
        contribution.item === input.node && contribution.span === 2
      ));
      for (const [kind, value] of [
        ['MinContent', expectedContributions.minContent],
        ['MaxContent', expectedContributions.maxContent],
      ]) {
        assert.ok(observed.some((contribution) => contribution.kind === kind
          && Math.abs(contribution.value - value) <= 1 / 64),
        `${scene.id}: ${axis} span-2 ${kind} contribution expected ${value}, got ${JSON.stringify(observed)}`);
      }
    }
    if (scene.crossAxisSpanGroups) {
      const expected = scene.crossAxisSpanGroups;
      const passes = axisTrace.passes.filter((pass) => pass.axis === axis);
      const kinds = [
        ['Minimum', expected.minimum[axis]],
        ['MinContent', expected.minContent[axis]],
        ['MaxContent', expected.maxContent[axis]],
      ];
      for (const [kind, itemValues] of kinds) {
        for (const item of expected.items) {
          const node = expected.itemNodes[item.key];
          const value = itemValues[item.key];
          assert.ok(passes.some((pass) => pass.contributions.some((contribution) => (
            contribution.item === node
              && contribution.kind === kind
              && contribution.span === 2
              && Math.abs(contribution.value - value) <= 1 / 64
          ))), `${scene.id}: ${axis} item ${item.key} ${kind} span-2 expected ${value}`);
        }
      }
      const revision = diagnostics.layoutInputRevision;
      assert.ok(revision && revision.source.includes('HostDocument'),
        `${scene.id}: HostDocument source revision is missing`);
      for (const key of ['style', 'environment', 'intrinsicSize']) {
        assert.ok(Number.isInteger(revision[key]), `${scene.id}: ${key} revision is missing`);
      }
      const inputByNode = new Map((diagnostics.gridItemIntrinsicInputs ?? [])
        .map((input) => [input.node, input]));
      for (const item of expected.items) {
        const input = inputByNode.get(expected.itemNodes[item.key]);
        assert.ok(input, `${scene.id}: item ${item.key} intrinsic input is missing`);
        assert.equal(input.source?.kind, 'text', `${scene.id}: item ${item.key} source kind`);
        assert.ok(Number.isInteger(input.source?.fontRegistryRevision)
          && input.source.fontRegistryRevision > 0,
        `${scene.id}: item ${item.key} font registry revision is missing`);
        assert.deepEqual(input.textSources, [],
          `${scene.id}: direct child text must not be reclassified as anonymous text input`);
      }
      if (axis === 'Columns' && expected.initialColumns) {
        const firstColumnPass = axisTrace.passes.find((pass) => pass.axis === 'Columns');
        assert.ok(firstColumnPass, `${scene.id}: initial Columns pass is missing`);
        const tracksInStage = (stage) => stage?.tracks
          .filter(({ gutter, collapsed }) => !gutter && !collapsed)
          .map(({ baseSize }) => baseSize);
        const initialized = tracksInStage(firstColumnPass.stages.find(({ stage }) => stage === 'Initialize'));
        const minContentBatch = tracksInStage(firstColumnPass.stages.find(({ stage }) => (
          stage === 'IntrinsicItemBatch { span: 2, crosses_flexible_track: false }'
        )));
        for (const [label, actual, values] of [
          ['Initialize', initialized, expected.initialColumns],
          ['min-content batch', minContentBatch, expected.minContentColumns],
        ]) {
          assert.ok(actual && actual.length === values.length,
            `${scene.id}: ${label} track state is missing`);
          actual.forEach((value, index) => assert.ok(
            Math.abs(value - values[index]) <= 1 / 64,
            `${scene.id}: ${label} column ${index + 1} expected ${values[index]}, got ${value}`,
          ));
        }
        const initialMinimum = Object.fromEntries(expected.items.map((item) => {
          const contribution = firstColumnPass.contributions.find((entry) => (
            entry.item === expected.itemNodes[item.key]
              && entry.kind === 'Minimum'
              && entry.span === 2
          ));
          assert.ok(contribution, `${scene.id}: item ${item.key} initial Minimum is missing`);
          return [item.key, contribution.value];
        }));
        for (const [itemKey, start, planned] of [
          ['a', [expected.initialColumns[0], expected.initialColumns[1]], expected.perItemPlannedIncreases.a],
          ['b', [expected.initialColumns[1], expected.initialColumns[2]], expected.perItemPlannedIncreases.b],
        ]) {
          const share = (initialMinimum[itemKey] - start[0] - start[1] - 2) / 2;
          assert.ok(planned.every((value) => Math.abs(value - share) <= 1 / 64),
            `${scene.id}: item ${itemKey} planned span increase differs`);
        }
        const aggregateIncrease = minContentBatch.map((value, index) => value - initialized[index]);
        aggregateIncrease.forEach((value, index) => assert.ok(
          Math.abs(value - expected.perTrackPlannedIncreases[index]) <= 1 / 64,
          `${scene.id}: track ${index + 1} planned increase expected ${expected.perTrackPlannedIncreases[index]}, got ${value}`,
        ));
      }
    }
    if (scene.mixedCrossAxisSpanGroups) {
      const expected = scene.mixedCrossAxisSpanGroups;
      const passes = axisTrace.passes.filter((pass) => pass.axis === axis);
      const kinds = [
        ['Minimum', expected.minimum[axis]],
        ['MinContent', expected.minContent[axis]],
        ['MaxContent', expected.maxContent[axis]],
      ];
      for (const [kind, itemValues] of kinds) {
        for (const item of expected.items) {
          const node = expected.itemNodes[item.key];
          const span = axis === 'Columns' ? item.columnSpan : item.rowSpan;
          const value = itemValues[item.key];
          assert.ok(passes.some((pass) => pass.contributions.some((contribution) => (
            contribution.item === node
              && contribution.kind === kind
              && contribution.span === span
              && Math.abs(contribution.value - value) <= 1 / 64
          ))), `${scene.id}: ${axis} item ${item.key} ${kind} span-${span} expected ${value}`);
        }
      }
      const revision = diagnostics.layoutInputRevision;
      assert.ok(revision && revision.source.includes('HostDocument'),
        `${scene.id}: HostDocument source revision is missing`);
      for (const key of ['style', 'environment', 'intrinsicSize']) {
        assert.ok(Number.isInteger(revision[key]), `${scene.id}: ${key} revision is missing`);
      }
      if (axis === 'Columns' && expected.initialColumns) {
        const firstColumnPass = axisTrace.passes.find((pass) => pass.axis === 'Columns');
        assert.ok(firstColumnPass, `${scene.id}: initial Columns pass is missing`);
        const tracksInStage = (stage) => stage?.tracks
          .filter(({ gutter, collapsed }) => !gutter && !collapsed)
          .map(({ baseSize }) => baseSize);
        const stagesForSpan = (span) => firstColumnPass.stages.filter(({ stage }) => (
          stage === `IntrinsicItemBatch { span: ${span}, crosses_flexible_track: false }`
        ));
        const initialized = tracksInStage(firstColumnPass.stages.find(({ stage }) => stage === 'Initialize'));
        const span2Stages = stagesForSpan(2);
        const span3Stages = stagesForSpan(3);
        assert.equal(span2Stages.length, 1, `${scene.id}: one span-2 sizing batch is required`);
        assert.equal(span3Stages.length, 1, `${scene.id}: one span-3 sizing batch is required`);
        const span2Columns = tracksInStage(span2Stages[0]);
        const span3Columns = tracksInStage(span3Stages[0]);
        for (const [label, actual, values] of [
          ['Initialize', initialized, expected.initialColumns],
          ['span-2 batch', span2Columns, expected.span2Columns],
          ['span-3 batch', span3Columns, expected.span3Columns],
        ]) {
          assert.ok(actual && actual.length === values.length,
            `${scene.id}: ${label} track state is missing`);
          actual.forEach((value, index) => assert.ok(
            Math.abs(value - values[index]) <= 1 / 64,
            `${scene.id}: ${label} column ${index + 1} expected ${values[index]}, got ${value}`,
          ));
        }
        const initialMinimum = Object.fromEntries(expected.items.map((item) => {
          const contribution = firstColumnPass.contributions.find((entry) => (
            entry.item === expected.itemNodes[item.key]
              && entry.kind === 'Minimum'
              && entry.span === item.columnSpan
          ));
          assert.ok(contribution, `${scene.id}: item ${item.key} initial Minimum is missing`);
          return [item.key, contribution.value];
        }));
        for (const [itemKey, start, planned] of [
          ['a', [expected.initialColumns[0], expected.initialColumns[1]], expected.perItemPlannedIncreases.a],
          ['b', [expected.initialColumns[1], expected.initialColumns[2]], expected.perItemPlannedIncreases.b],
        ]) {
          const share = (initialMinimum[itemKey] - start[0] - start[1] - 2) / 2;
          assert.ok(planned.every((value) => Math.abs(value - share) <= 1 / 64),
            `${scene.id}: item ${itemKey} planned span increase differs`);
        }
        const aggregateIncrease = span2Columns.map((value, index) => value - initialized[index]);
        aggregateIncrease.forEach((value, index) => assert.ok(
          Math.abs(value - expected.perTrackPlannedIncreases[index]) <= 1 / 64,
          `${scene.id}: track ${index + 1} planned increase expected ${expected.perTrackPlannedIncreases[index]}, got ${value}`,
        ));
        const span3Minimum = initialMinimum.c;
        assert.ok(span3Minimum <= span2Columns.reduce((sum, value) => sum + value, 0) + 4,
          `${scene.id}: C span-3 minimum exceeds the established track area`);
        assert.deepEqual(span3Columns, span2Columns,
          `${scene.id}: C span-3 contribution changed columns after the span-2 batch`);
      }
    }
    if (scene.mixedTextImageSpan) {
      const expected = scene.mixedTextImageSpan;
      const revision = diagnostics.layoutInputRevision;
      assert.ok(revision && revision.source.includes('HostDocument'),
        `${scene.id}: HostDocument source revision is missing`);
      for (const key of ['style', 'environment', 'intrinsicSize']) {
        assert.ok(Number.isInteger(revision[key]), `${scene.id}: ${key} revision is missing`);
      }
      const actualInputs = (diagnostics.gridItemIntrinsicInputs ?? []).map((input) => {
        const source = input.source ?? {};
        const normalized = {
          grid: input.grid,
          node: input.node,
          sourceOrder: input.sourceOrder,
          kind: source.kind,
        };
        if (source.kind === 'text') {
          Object.assign(normalized, {
            fontRegistryRevision: source.fontRegistryRevision,
            minContent: source.minContent,
            minContentHeight: source.minContentHeight,
            maxContent: source.maxContent,
            maxContentHeight: source.maxContentHeight,
          });
          assert.deepEqual(input.textSources, [],
            `${scene.id}: direct element text must not become anonymous text input`);
        } else if (source.kind === 'replaced') {
          normalized.natural = source.natural;
        }
        return normalized;
      });
      const expectedInputs = scene.intrinsicInputs.map((input) => input.kind === 'text'
        ? (({ grid, node, sourceOrder, kind, fontRegistryRevision,
          minContent, minContentHeight, maxContent, maxContentHeight }) => ({
          grid, node, sourceOrder, kind, fontRegistryRevision,
          minContent, minContentHeight, maxContent, maxContentHeight,
        }))(input)
        : (({ grid, node, sourceOrder, kind, natural }) => ({
          grid, node, sourceOrder, kind, natural,
        }))(input));
      assert.deepEqual(actualInputs, expectedInputs,
        `${scene.id}: ordered text/replaced intrinsic source inputs differ`);

      const expectedAxes = expected.intrinsic
        ? ['Columns', 'Rows', 'Columns', 'Rows'] : ['Columns', 'Rows', 'Rows'];
      assert.deepEqual(axisTrace.passes.map((pass) => pass.axis), expectedAxes,
        `${scene.id}: cyclic-auto and resolved axis order differs`);
      const spanForStage = (stage) => {
        const match = stage.match(/^IntrinsicItemBatch\s*\{ span: (\d+),/);
        return match ? Number(match[1]) : null;
      };
      const spanBases = (pass, span) => {
        const stage = pass.stages.find((candidate) => spanForStage(candidate.stage) === span);
        return stage?.tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
          .map(({ baseSize }) => baseSize);
      };
      const stageBases = (pass, name) => {
        const stage = pass.stages.find((candidate) => candidate.stage === name);
        return stage?.tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
          .map(({ baseSize }) => baseSize);
      };
      const closeArray = (actual, values, label) => {
        assert.ok(actual && actual.length === values.length,
          `${scene.id}: ${label} track state is missing`);
        actual.forEach((value, index) => assert.ok(
          Math.abs(value - values[index]) <= 1 / 64,
          `${scene.id}: ${label} track ${index + 1} expected ${values[index]}, got ${value}`,
        ));
      };
      for (const [index, pass] of axisTrace.passes.entries()) {
        const axis = expectedAxes[index];
        const spans = pass.stages.flatMap(({ stage }) => {
          const span = spanForStage(stage);
          return span === null ? [] : [span];
        });
        assert.deepEqual(spans, axis === 'Columns' ? [2, 3] : [2],
          `${scene.id}: ${axis} span group order differs`);
        const expectedFinal = expected.maxContentWidth && index === 0
          ? [24, 34, 48]
          : expected.intrinsic && index === 0 ? [9, 19, 19]
          : axis === 'Columns' ? scene.axes.Columns.tracks : scene.axes.Rows.tracks;
        closeArray(pass.stages.at(-1)?.tracks
          .filter(({ gutter, collapsed }) => !gutter && !collapsed)
          .map(({ baseSize }) => baseSize), expectedFinal, `${axis} pass ${index + 1} final`);
        const axisPasses = axisTrace.passes.filter((candidate) => candidate.axis === axis);
        for (const [kind, values] of [
          ['Minimum', expected.minimum[axis]],
          ['MinContent', expected.minContent[axis]],
          ['MaxContent', expected.maxContent[axis]],
        ]) {
          for (const item of expected.items) {
            const span = axis === 'Columns' ? item.columnSpan : item.rowSpan;
            assert.ok(axisPasses.some((candidate) => candidate.contributions.some((entry) => (
              entry.item === item.node && entry.kind === kind && entry.span === span
                && Math.abs(entry.value - values[item.key]) <= 1 / 64
            ))), `${scene.id}: ${axis} ${item.key} ${kind} span-${span} is missing`);
          }
        }
      }
      if (expected.intrinsic) {
        closeArray(stageBases(axisTrace.passes[0], 'Initialize'), [2, 2, 2], 'cyclic Columns Initialize');
        closeArray(spanBases(axisTrace.passes[0], 2), [9, 19, 19], 'span-2 planned columns');
        closeArray(spanBases(axisTrace.passes[0], 3), [9, 19, 19], 'span-3 columns');
        closeArray(stageBases(axisTrace.passes[1], 'Initialize'), [2, 2], 'cyclic Rows Initialize');
        closeArray(spanBases(axisTrace.passes[1], 2), [37 / 3, 37 / 3], 'cyclic row contributions');
        closeArray(stageBases(axisTrace.passes[2], 'Initialize'),
          expected.maxContentWidth ? [13, 13, 13] : [7.1, 7.1, 7.1], 'resolved Columns Initialize');
        closeArray(stageBases(axisTrace.passes[3], 'Initialize'), [14 / 3, 14 / 3], 'resolved Rows Initialize');
      } else {
        closeArray(stageBases(axisTrace.passes[0], 'Initialize'), [6, 6, 6], 'definite Columns Initialize');
        closeArray(stageBases(axisTrace.passes[1], 'Initialize'), [2, 2], 'cyclic Rows Initialize');
        closeArray(stageBases(axisTrace.passes[2], 'Initialize'), [6, 6], 'resolved Rows Initialize');
      }
    }
    if (scene.mixedIntrinsicContributions && axis === 'Columns') {
      const expectedContributions = [];
      for (const span of scene.intrinsicSpans) {
        const spanItems = scene.intrinsicInputs.flatMap((input, itemIndex) => (
          scene.itemSpans[itemIndex] === span ? [{ input, itemIndex }] : []
        ));
        if (scene.anonymousTextMultiImageContributions) {
          for (const { input } of spanItems) {
            if (input.kind === 'replaced') {
              expectedContributions.push({
                item: input.node,
                kind: 'Minimum',
                span,
                value: 0,
              });
            }
          }
          if (scene.anonymousGridBorderOverflowContributions && span === 1) {
            for (const { input } of spanItems) {
              for (let repetition = 0; repetition < 2; repetition++) {
                expectedContributions.push({
                  item: input.node,
                  kind: 'MinContent',
                  span,
                  value: input.minContent,
                });
              }
            }
          } else {
            for (let repetition = 0; repetition < 2; repetition++) {
              for (const { input, itemIndex } of spanItems) {
                expectedContributions.push({
                  item: input.node,
                  kind: 'MinContent',
                  span,
                  value: input.kind === 'text'
                    ? input.minContent : scene.imageMinContentWidths[itemIndex],
                });
              }
            }
          }
        } else if (scene.anonymousTextImageContributions) {
          for (const { input, itemIndex } of spanItems) {
            if (input.kind === 'text') {
              for (let repetition = 0; repetition < 2; repetition++) {
                expectedContributions.push({
                  item: input.node,
                  kind: 'MinContent',
                  span,
                  value: input.minContent,
                });
              }
            } else {
              expectedContributions.push({
                item: input.node,
                kind: 'Minimum',
                span,
                value: 0,
              });
              for (let repetition = 0; repetition < 2; repetition++) {
                expectedContributions.push({
                  item: input.node,
                  kind: 'MinContent',
                  span,
                  value: scene.imageMinContentWidths[itemIndex],
                });
              }
            }
          }
        } else {
          for (const kind of ['Minimum', 'MinContent']) {
            const repetitions = kind === 'MinContent' ? 2 : 1;
            for (let repetition = 0; repetition < repetitions; repetition++) {
              for (const { input, itemIndex } of spanItems) {
                expectedContributions.push({
                  item: input.node,
                  kind,
                  span,
                  value: kind === 'Minimum' ? 0
                    : input.kind === 'text' ? input.minContent : scene.imageMinContentWidths[itemIndex],
                });
              }
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
        `${scene.id}: 혼합 source의 span별 Minimum/MinContent trace가 다릅니다.`);
    }
    if (scene.overlappingSpanTextContributions && axis === scene.axis) {
      const expectedContributions = [];
      for (const span of scene.intrinsicSpans) {
        const spanItems = scene.intrinsicInputs.filter((_, itemIndex) => {
          const itemSpan = scene.id.endsWith('-reverse')
            ? [3, 2, 2][itemIndex] : [2, 2, 3][itemIndex];
          return itemSpan === span;
        });
        for (const kind of ['Minimum', 'MinContent', 'MaxContent']) {
          for (const input of spanItems) {
            expectedContributions.push({
              item: input.node,
              kind,
              span,
              value: kind === 'Minimum' ? 0
                : scene.axis === 'Rows' ? input.minContentHeight : input.minContent,
            });
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
        `${scene.id}: overlapping span Minimum/MinContent/MaxContent contribution mapping differs`);
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
  if (scene.columnSizingFeedsRows) {
    const columnPassIndex = axisTrace.passes.findLastIndex((pass) => pass.axis === 'Columns');
    const rowPassIndex = axisTrace.passes.findLastIndex((pass) => pass.axis === 'Rows');
    assert.ok(columnPassIndex >= 0 && rowPassIndex > columnPassIndex,
      `${scene.id}: Columns must size before Rows use the item's available inline size`);
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
    trackTraceChunks: traceState.parts.length,
    drawPresented: true,
  };
}

function verifyCyclicMinmaxReplacedImageSpanTrace(scene, trace) {
  const expected = scene.replacedCyclicMinmaxImageSpan;
  const { intrinsic, order, nodeByKey, itemSpecs, itemSizes } = expected;
  const expectedPassAxes = intrinsic
    ? ['Columns', 'Rows', 'Columns', 'Rows']
    : ['Columns', 'Rows'];
  assert.deepEqual(trace.passes.map((pass) => pass.axis), expectedPassAxes,
    `${scene.id}: cyclic-auto passes must precede resolved Columns and Rows`);

  const passesFor = (axis) => trace.passes.filter((pass) => pass.axis === axis);
  const columns = passesFor('Columns');
  const rows = passesFor('Rows');
  assert.equal(columns.length, intrinsic ? 2 : 1, `${scene.id}: Columns pass count`);
  assert.equal(rows.length, intrinsic ? 2 : 1, `${scene.id}: Rows pass count`);

  const batchSpans = (pass) => pass.stages.flatMap(({ stage }) => {
    const match = stage.match(/^IntrinsicItemBatch \{ span: (\d+),/);
    return match ? [Number(match[1])] : [];
  });
  for (const pass of trace.passes) {
    assert.deepEqual(batchSpans(pass), pass.axis === 'Columns' ? [2, 3] : [2],
      `${scene.id}: ${pass.axis} span-group order`);
  }
  const stageBases = (pass, name) => {
    const stage = pass.stages.find((candidate) => candidate.stage === name);
    assert.ok(stage, `${scene.id}: ${pass.axis} ${name} stage missing`);
    return stage.tracks.filter(({ gutter, collapsed }) => !gutter && !collapsed)
      .map(({ baseSize }) => baseSize);
  };
  const spanBases = (pass, span) => {
    const name = `IntrinsicItemBatch { span: ${span}, crosses_flexible_track: false }`;
    return stageBases(pass, name);
  };
  const closeArray = (actual, expectedValues, label) => {
    assert.equal(actual.length, expectedValues.length, `${scene.id}: ${label} track count`);
    actual.forEach((value, index) => assert.ok(
      Math.abs(value - expectedValues[index]) <= 1 / 64,
      `${scene.id}: ${label} track ${index + 1} expected ${expectedValues[index]}, got ${value}`,
    ));
  };

  for (const [axis, axisPasses] of [['Columns', columns], ['Rows', rows]]) {
    for (const [passIndex, pass] of axisPasses.entries()) {
      for (const key of order) {
        const node = nodeByKey[key];
        const span = axis === 'Columns' ? itemSpecs[key].columnSpan : 2;
        const value = axis === 'Columns' ? itemSizes[key][0] : itemSizes[key][1];
        const minimum = axis === 'Columns'
          ? (intrinsic && passIndex === 0 ? value : 0)
          : value;
        for (const [kind, expectedValue] of [
          ['Minimum', minimum],
          ['MinContent', value],
          ['MaxContent', value],
        ]) {
          assert.ok(pass.contributions.some((contribution) => (
            contribution.item === node
              && contribution.kind === kind
              && contribution.span === span
              && Math.abs(contribution.value - expectedValue) <= 1 / 64
          )), `${scene.id}: ${axis} item ${key} ${kind} span-${span} expected ${expectedValue}`);
        }
      }
    }
  }

  if (intrinsic) {
    const initialColumns = stageBases(columns[0], 'Initialize');
    closeArray(initialColumns, [2, 2, 2], 'cyclic-auto Columns Initialize');
    closeArray(spanBases(columns[0], 2), [5, 8, 8], 'span-2 Columns');
    closeArray(spanBases(columns[0], 3), [5, 8, 8], 'span-3 Columns');
    const span2Columns = spanBases(columns[0], 2);
    closeArray(
      span2Columns.map((value, index) => value - initialColumns[index]),
      [3, 6, 6],
      'per-track maximum planned increase',
    );
    for (const [key, expectedShare] of [['a', 3], ['b', 6]]) {
      const contribution = columns[0].contributions.find((entry) => (
        entry.item === nodeByKey[key] && entry.kind === 'Minimum' && entry.span === 2
      ));
      assert.ok(contribution, `${scene.id}: item ${key} Minimum contribution missing`);
      const share = (contribution.value - 2 - 2 - 2) / 2;
      assert.ok(Math.abs(share - expectedShare) <= 1 / 64,
        `${scene.id}: item ${key} planned increase per track expected ${expectedShare}, got ${share}`);
    }
    const cMinimum = columns[0].contributions.find((entry) => (
      entry.item === nodeByKey.c && entry.kind === 'Minimum' && entry.span === 3
    ));
    assert.equal(cMinimum?.value, 24, `${scene.id}: span-3 C natural contribution`);
    assert.ok(24 <= span2Columns.reduce((sum, value) => sum + value, 0) + 4,
      `${scene.id}: C span-3 contribution exceeds the established tracks and gutters`);

    const initialRows = stageBases(rows[0], 'Initialize');
    closeArray(initialRows, [2, 2], 'cyclic-auto Rows Initialize');
    const span2Rows = spanBases(rows[0], 2);
    closeArray(span2Rows, [7, 7], 'span-2 Rows');
    closeArray(
      span2Rows.map((value, index) => value - initialRows[index]),
      [5, 5],
      'row per-track maximum planned increase',
    );
    for (const [key, expectedShare] of [['a', 1], ['b', 3], ['c', 5]]) {
      const contribution = rows[0].contributions.find((entry) => (
        entry.item === nodeByKey[key] && entry.kind === 'Minimum' && entry.span === 2
      ));
      assert.ok(contribution, `${scene.id}: item ${key} row Minimum contribution missing`);
      const share = (contribution.value - 2 - 2 - 2) / 2;
      assert.ok(Math.abs(share - expectedShare) <= 1 / 64,
        `${scene.id}: item ${key} row planned increase expected ${expectedShare}, got ${share}`);
    }
    closeArray(stageBases(columns[1], 'Initialize'), [4.5, 4.5, 4.5],
      'resolved Columns minimum basis');
    closeArray(stageBases(rows[1], 'Initialize'), [3.6, 3.6],
      'resolved Rows minimum basis');
  } else {
    closeArray(stageBases(columns[0], 'Initialize'), [10, 10, 10],
      'definite Columns minimum');
    closeArray(stageBases(rows[0], 'Initialize'), [10, 10],
      'definite Rows minimum');
    closeArray(spanBases(columns[0], 2), [10, 10, 10], 'definite span-2 Columns');
    closeArray(spanBases(columns[0], 3), [10, 10, 10], 'definite span-3 Columns');
    closeArray(spanBases(rows[0], 2), [10, 10], 'definite span-2 Rows');
  }
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
  console.log(JSON.stringify({
    sceneCount: scenes.length,
    sceneIDs: scenes.map(({ id }) => id),
    selectedSceneCount: runScenes.length,
    selectedSceneIDs: runScenes.map(({ id }) => id),
  }, null, 2));
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
  assert.ok(['iPhone 17', 'iPhone 17 Pro'].includes(simulator[0].name),
    `지원하지 않는 iOS 시뮬레이터 모델입니다: ${simulator[0].name}`);
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulatorUDID, appPath]));
  const appSha256 = sha256(await readFile(resolve(appPath, 'SpinonBootstrap')));
  for (const [index, scene] of runScenes.entries()) {
    let runID = `${runIDPrefix}-${index}`;
    let log = '';
    let supersededAttempts = 0;
    for (let attempt = 0; attempt <= 3; attempt++) {
      runID = attempt === 0 ? `${runIDPrefix}-${index}` : `${runIDPrefix}-${index}-retry-${attempt}`;
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
    sourceRevision: sourceRevision(),
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
  metadata.sourceRevision = sourceRevision();
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
    let runID = `${runIDPrefix}-${index}`;
    let log = '';
    let supersededAttempts = 0;
    for (let attempt = 0; attempt <= 3; attempt++) {
      runID = attempt === 0 ? `${runIDPrefix}-${index}` : `${runIDPrefix}-${index}-retry-${attempt}`;
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
        const traceComplete = c114TraceChunkState(log, runID).complete;
        const draw = log.includes('SPINON_C0410_DRAW')
          && log.includes(`presented boxes=${scene.boxCount}`);
        const pixelSummary = !scene.pixelSamples
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`);
        const pixelError = log.includes(`SPINON_C114_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`);
        if ((summary && traceComplete && draw && pixelSummary)
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
    adb(['shell', 'cmd', 'statusbar', 'collapse']);
    sleep(300);
    const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
  }
}

await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, sceneCount: runScenes.length, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
