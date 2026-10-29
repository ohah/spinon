import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { relative, resolve } from 'node:path';
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
  'cyclic-max-content-auto-fill-rows', 'definite-40px-auto-fill-control-rows',
  'cyclic-max-content-auto-fill-prefix-suffix',
  'cyclic-max-content-auto-fit-prefix-suffix',
  'definite-44px-auto-fill-prefix-suffix-control',
  'definite-44px-auto-fit-prefix-suffix-control',
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
const generalizedCyclicRowSceneIDs = new Set([
  'cyclic-max-content-auto-fill-prefix-suffix',
  'cyclic-max-content-auto-fit-prefix-suffix',
  'definite-44px-auto-fill-prefix-suffix-control',
  'definite-44px-auto-fit-prefix-suffix-control',
]);
const imageAutoMaxBorderOverflowSceneIDs = new Set([
  'image-auto-max-mixed-tracks', 'image-auto-max-mixed-tracks-border-overflow',
]);
const requestedRunKind = process.argv.find((argument) => argument.startsWith('--run-kind='))
  ?.slice('--run-kind='.length);
const evidenceGroup = process.argv.find((argument) => argument.startsWith('--evidence-group='))
  ?.slice('--evidence-group='.length);
const multiSpanSceneIDs = new Set([
  'competing-span2-forward', 'competing-span2-reversed',
  'span2-before-span3', 'span3-before-span2',
]);
const fixedAutoAreaSceneIDs = new Set([
  'fixed-auto-area-columns-fill-collision', 'fixed-auto-area-columns-fit-empty',
  'fixed-auto-area-columns-fit-occupied', 'fixed-auto-area-columns-area-larger',
  'fixed-auto-area-columns-area-shorter', 'fixed-auto-area-rows-fill-collision',
  'fixed-auto-area-rows-fit-empty', 'fixed-auto-area-rows-fit-occupied',
  'fixed-auto-area-rows-area-larger',
]);
const fitContentAutoNeighborSceneIDs = new Set(['fit-content-auto-neighbor-wpt']);
const fitContentMinHeightSceneIDs = new Set(['fit-content-min-height-wpt']);
const fitContentZoomedGridSceneIDs = new Set(['fit-content-zoomed-grid-wpt']);
const fitContentPercentageArgumentSceneIDs = new Set([
  'fit-content-percentage-row-wpt', 'fit-content-calc-percentage-row-wpt',
]);
const fitContentSpanningSceneIDs = new Set(['fit-content-spanning-wpt']);
const fitContentLimitBoundarySceneIDs = new Set([
  'fit-content-limit-40-wpt', 'fit-content-limit-30-wpt', 'fit-content-limit-200-wpt',
]);
const fitContentLimitColumnsSceneIDs = new Set([
  'column-fit-content-40', 'column-fit-content-30', 'column-fit-content-200',
  'column-fit-content-auto-minimum-control',
]);
const fitContentAutoMinFloorSceneIDs = new Set([
  'fit-content-auto-min-floor-wpt', 'fit-content-auto-min-floor-zero-control',
]);
const zeroFixedMaximumSceneIDs = new Set([
  'columns-auto-fill-auto-max0-gap0', 'columns-auto-fill-auto-max0-gap4',
  'columns-auto-fit-auto-max0-gap0', 'columns-auto-fit-auto-max0-gap4',
  'columns-auto-fill-min-content-max0-gap0', 'columns-auto-fill-min-content-max0-gap4',
  'columns-auto-fit-min-content-max0-gap0', 'columns-auto-fit-min-content-max0-gap4',
  'columns-auto-fill-max-content-max0-gap0', 'columns-auto-fill-max-content-max0-gap4',
  'columns-auto-fit-max-content-max0-gap0', 'columns-auto-fit-max-content-max0-gap4',
  'rows-auto-fill-auto-max0-gap0', 'rows-auto-fill-auto-max0-gap4',
  'rows-auto-fit-auto-max0-gap0', 'rows-auto-fit-auto-max0-gap4',
  'rows-auto-fill-min-content-max0-gap0', 'rows-auto-fill-min-content-max0-gap4',
  'rows-auto-fit-min-content-max0-gap0', 'rows-auto-fit-min-content-max0-gap4',
  'rows-auto-fill-max-content-max0-gap0', 'rows-auto-fill-max-content-max0-gap4',
  'rows-auto-fit-max-content-max0-gap0', 'rows-auto-fit-max-content-max0-gap4',
]);
const overflowAutoMinimumSceneIDs = new Set([
  'columns-span2-visible-content-minimum', 'columns-span2-clip-content-minimum',
  'columns-span2-hidden-zero-minimum', 'columns-span2-auto-zero-minimum',
  'columns-span2-scroll-zero-minimum',
  'column-visible-content-minimum', 'column-clip-content-minimum',
  'column-hidden-zero-minimum', 'column-auto-zero-minimum',
  'column-scroll-zero-minimum', 'column-specified-minimum-control',
  'column-zero-track-control', 'row-visible-content-minimum',
  'row-clip-content-minimum', 'row-hidden-zero-minimum',
  'row-auto-zero-minimum', 'row-scroll-zero-minimum',
  'row-specified-minimum-control', 'row-zero-track-control',
]);
const equalAutoRowsSceneIDs = new Set(['align-content-normal-equal-auto-rows']);
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
const isGeneralizedCyclicRowRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => generalizedCyclicRowSceneIDs.has(sceneID));
const isImageAutoMaxBorderOverflowRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => imageAutoMaxBorderOverflowSceneIDs.has(sceneID));
const isFixedAutoAreaRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fixedAutoAreaSceneIDs.has(sceneID));
const isFitContentAutoNeighborRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentAutoNeighborSceneIDs.has(sceneID));
const isFitContentMinHeightRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentMinHeightSceneIDs.has(sceneID));
const isFitContentZoomedGridRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentZoomedGridSceneIDs.has(sceneID));
const isFitContentPercentageArgumentRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentPercentageArgumentSceneIDs.has(sceneID));
const isFitContentSpanningRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentSpanningSceneIDs.has(sceneID));
const isFitContentLimitBoundaryRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentLimitBoundarySceneIDs.has(sceneID));
const isFitContentLimitColumnsRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentLimitColumnsSceneIDs.has(sceneID));
const isFitContentAutoMinFloorRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => fitContentAutoMinFloorSceneIDs.has(sceneID));
const isOverflowAutoMinimumRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => overflowAutoMinimumSceneIDs.has(sceneID));
const isZeroFixedMaximumRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => zeroFixedMaximumSceneIDs.has(sceneID));
const equalAutoRowsSceneID = 'align-content-normal-equal-auto-rows';
if (requestedSceneIDs?.includes(equalAutoRowsSceneID)
    || requestedRunKind === equalAutoRowsSceneID) {
  assert.deepEqual(requestedSceneIDs, [equalAutoRowsSceneID],
    'align-content-normal-equal-auto-rows는 해당 scene 하나만 선택해야 합니다.');
  assert.ok(!requestedRunKind || requestedRunKind === equalAutoRowsSceneID,
    'align-content-normal-equal-auto-rows에 다른 run-kind를 지정할 수 없습니다.');
}
const isEqualAutoRowsRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => equalAutoRowsSceneIDs.has(sceneID));
const isCrossAxisTemplateAreaRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => (
    sceneID.startsWith('template-area-repeat-cross-axis-')
  ));
const isMultiSpanRun = requestedSceneIDs?.length > 0
  && requestedSceneIDs.every((sceneID) => multiSpanSceneIDs.has(sceneID)
    || textMultiSpanSceneIDs.has(sceneID) || mixedMultiSpanSceneIDs.has(sceneID));
const runKind = requestedRunKind ?? (isFixedAutoAreaRun ? 'fixed-auto-area'
  : isOverflowAutoMinimumRun ? 'overflow-auto-minimum'
  : isZeroFixedMaximumRun ? 'zero-fixed-maximum'
  : isFitContentAutoNeighborRun ? 'fit-content-auto-neighbor'
  : isFitContentMinHeightRun ? 'fit-content-min-height'
  : isFitContentZoomedGridRun ? 'fit-content-zoomed-grid'
  : isFitContentPercentageArgumentRun ? 'fit-content-percentage-argument'
  : isFitContentSpanningRun ? 'fit-content-spanning'
  : isFitContentLimitColumnsRun ? 'fit-content-limit-columns'
  : isFitContentLimitBoundaryRun ? 'fit-content-limit-boundaries'
  : isFitContentAutoMinFloorRun ? 'fit-content-auto-min-floor'
  : isEqualAutoRowsRun ? 'align-content-normal-equal-auto-rows'
  : isImageAutoMaxBorderOverflowRun ? 'image-auto-max-border-overflow'
  : isCrossAxisTemplateAreaRun ? 'cross-axis-fixed-auto-area'
  : isCyclicPercentRun ? 'cyclic-percent-tracks'
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
  : isCrossAxisTemplateAreaRun
    ? `spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-${date}/${target}-${runPrefix}`
  : isFitContentAutoNeighborRun
    ? `spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-${date}/${target}-${runPrefix}`
  : isFitContentMinHeightRun
    ? `spec/internal/evidence/c11-4-fit-content-min-height-runtime-${date}/${target}-${runPrefix}`
  : isFitContentZoomedGridRun
    ? `spec/internal/evidence/c11-4-fit-content-zoom-runtime-${date}/${target}-${runPrefix}`
  : isFitContentPercentageArgumentRun
    ? `spec/internal/evidence/c11-4-fit-content-percentage-argument-runtime-${date}/${target}-${runPrefix}`
  : isFitContentSpanningRun
    ? `spec/internal/evidence/c11-4-fit-content-spanning-simulators-${date}/${target}-${runPrefix}`
  : isFitContentLimitBoundaryRun
    ? `spec/internal/evidence/c11-4-fit-content-limit-boundaries-simulators-${date}/${target}-${runPrefix}`
  : isFitContentLimitColumnsRun
    ? `spec/internal/evidence/c11-4-fit-content-limit-columns-simulators-${date}/${target}-${runPrefix}`
  : isFitContentAutoMinFloorRun
    ? `spec/internal/evidence/c11-4-fit-content-auto-min-floor-runtime-${date}/${target}-${runPrefix}`
  : isOverflowAutoMinimumRun || runKind === 'overflow-auto-minimum'
    ? `spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-${date}/${target}-${runPrefix}`
  : isZeroFixedMaximumRun || runKind === 'zero-fixed-maximum'
    ? `spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-runtime-${date}/${target}-${runPrefix}`
  : isImageAutoMaxBorderOverflowRun
    ? `spec/internal/evidence/c11-4-image-auto-max-border-overflow-runtime-${date}/${target}-${runPrefix}`
  : runKind === 'fixed-auto-area'
    ? `spec/internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-${date}/${target}-${runPrefix}`
  : isGeneralizedCyclicRowRun
    ? `spec/internal/evidence/c11-4-cyclic-row-auto-repeat-generalization-runtime-${date}/${target}-${runPrefix}`
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
  const trackedDirtyPaths = command('git', ['diff', '--name-only', 'HEAD']).stdout
    .split(/\r?\n/).filter(Boolean);
  const untrackedDirtyPaths = command('git', [
    'ls-files', '--others', '--exclude-standard',
  ]).stdout.split(/\r?\n/).filter((path) => path
    && !path.startsWith('spec/internal/evidence/')
    && !path.startsWith('build/')
    && !/(^|\/)target\//.test(path));
  const untrackedFiles = untrackedDirtyPaths.map((path) => ({
    path,
    sha256: sha256(readFileSync(resolve(repositoryRoot, path))),
  }));
  const trackedDiffSha256 = sha256(trackedDiff);
  const sourceSnapshotSha256 = sha256(JSON.stringify({
    commit,
    tree,
    trackedDiffSha256,
    untrackedFiles,
  }));
  return {
    commit,
    tree,
    trackedDiffSha256,
    trackedDirtyPaths,
    untrackedFiles,
    sourceSnapshotSha256,
  };
}

const fixedRepeatMinmaxFlexRunInputPaths = [
  'crates/spinon-style/src/stylo_dom/cascade/incremental/grid/auto_repeat.rs',
  'crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs',
  'crates/spinon-style/src/stylo_dom/cascade/snapshot.rs',
  'crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs',
  'crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_line_names.rs',
  'crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/template_repeat.rs',
  'crates/spinon-layout/src/grid.rs',
  'crates/spinon-layout/src/grid_track_sizing_tests.rs',
  'crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs',
  'crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs',
  'crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_repeat_neighbor_tests.rs',
  'crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_track_tests.rs',
  'crates/spinon-style-to-layout/src/projection/style_values/grid_values/combined_metadata_tests.rs',
  'crates/spinon-style-to-layout/src/projection/style_values/grid_values/tests.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_minmax_flex.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_minmax_flex_scenes.json',
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/chrome156-observation.json',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/chrome156-agent-browser-precomparison.png',
  'tests/fixtures/css/c11/grid-fixed-repeat-minmax-flex-c114-precomparison.html',
  'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs',
];

async function fixedRepeatMinmaxFlexInputHashes() {
  const uniquePaths = new Set(fixedRepeatMinmaxFlexRunInputPaths);
  assert.equal(uniquePaths.size, fixedRepeatMinmaxFlexRunInputPaths.length,
    'fixed-repeat minmax flex 입력 경로 목록에 중복이 있습니다.');
  return Promise.all(fixedRepeatMinmaxFlexRunInputPaths.map(async (path) => {
    let bytes;
    try {
      bytes = await readFile(resolve(repositoryRoot, path));
    } catch (error) {
      throw new Error(`fixed-repeat minmax flex 입력 파일을 읽지 못했습니다 (${path}): ${error.message}`, {
        cause: error,
      });
    }
    assert.ok(bytes.length > 0, `fixed-repeat minmax flex 입력 파일이 비어 있습니다: ${path}`);
    return { path, sha256: sha256(bytes) };
  }));
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
  assert.equal(fixtures.length, 31, 'named auto-repeat 고정 fixture는 31 scene이어야 합니다.');
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
    if (id === 'auto-repeat-named-row'
      || (expect.tracks.Columns.length === 1 && expect.tracks.Rows.length > 1)) {
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
  assert.equal(fixtures.length, 40, 'template-area auto-repeat fixture는 40 scene이어야 합니다.');
  assert.equal(fixtures.filter(({ id }) => id.startsWith('template-area-repeat-cross-axis-')).length, 11,
    'cross-axis fixed-repeat + auto-repeat fixture는 11 scene이어야 합니다.');
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
  const iosCrossAxisScenes = allowlist(iosSource,
    /c114CrossAxisFixedAutoRepeatAreaSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS cross-axis fixed-repeat + auto-repeat');
  assert.match(iosSource,
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1\s*private let c114CrossAxisFixedAutoRepeatAreaSceneIDs/,
    'iOS cross-axis scene IDs must remain behind the fixture-test build flag.');
  assert.match(androidSource,
    /if \(sceneId != null && sceneId\.startsWith\(C114_CROSS_AXIS_TEMPLATE_AREA_SCENE_PREFIX\)\)\s*\{\s*return BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&& c114CrossAxisTemplateAreaCanonicalSceneId\(sceneId\) != null;/,
    'Android cross-axis scene IDs must remain behind the fixture-test build flag.');

  return fixtures.map(({ id, expect, rootColor, items: fixtureItems }) => {
    const isCrossAxis = id.startsWith('template-area-repeat-cross-axis-');
    if (isCrossAxis) {
      const canonicalID = id.slice('template-area-repeat-cross-axis-'.length);
      assert.ok(androidSource.includes(`case "${canonicalID}":`),
        `Android cross-axis scene route is missing ${canonicalID}.`);
      assert.ok(iosCrossAxisScenes.has(id), `iOS gated allowlist에 ${id}가 없습니다.`);
      assert.ok(!androidScenes.has(id), `Android base allowlist must not expose ${id}.`);
      assert.ok(!iosScenes.has(id), `iOS base allowlist must not expose ${id}.`);
    } else {
      assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
      assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    }
    const [rootWidth, rootHeight] = expect.root;
    const items = expect.items;
    if (id.startsWith('template-area-repeat-two-axis-') || isCrossAxis) {
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
        xCssPx: rootWidth - 2,
        yCssPx: rootHeight - 2,
        rgba: expandedSpan ? [229, 72, 77, 255] : rgba(rootColor ?? '#101820'),
      }];
      if (isCrossAxis) {
        assert.ok(rootWidth > 2 && rootHeight > 2,
          `${id} root background sample must be inside the root.`);
        assert.ok(items.every(({ frame: [x, y, width, height] }) => (
          rootWidth - 2 < x || rootWidth - 2 >= x + width
            || rootHeight - 2 < y || rootHeight - 2 >= y + height)),
        `${id} root background sample must not overlap an item.`);
        const overflowAxis = id.endsWith('-fixed-cols-fill-rows-area-adds-two-rows')
          || id.endsWith('-fixed-cols-fill-rows-area-adds-one-row') ? 'Rows'
          : id.endsWith('-fixed-rows-fill-cols-area-adds-one-column') ? 'Columns' : null;
        if (overflowAxis) {
          const overflowItem = items.find(({ name }) => name === 'hero');
          assert.ok(overflowItem, `${id} overflow samples require a hero item.`);
          const [x, y, width, height] = overflowItem.frame;
          const itemSample = [x + Math.floor(width / 2), y + Math.floor(height / 2)];
          const beforeItemSample = overflowAxis === 'Rows'
            ? [itemSample[0], rootHeight]
            : [rootWidth, itemSample[1]];
          const afterItemSample = overflowAxis === 'Rows'
            ? [itemSample[0], y + height]
            : [x + width, itemSample[1]];
          assert.ok(
            itemSample[0] >= rootWidth || itemSample[1] >= rootHeight,
            `${id} overflow sample must be outside the root.`
          );
          assert.ok(
            itemSample[0] >= x && itemSample[0] < x + width
              && itemSample[1] >= y && itemSample[1] < y + height,
            `${id} overflow sample must be inside the hero frame.`
          );
          assert.ok(
            beforeItemSample[0] >= rootWidth || beforeItemSample[1] >= rootHeight,
            `${id} pre-item sample must be outside the root.`
          );
          assert.equal(
            overflowAxis === 'Rows' ? beforeItemSample[1] : beforeItemSample[0],
            overflowAxis === 'Rows' ? rootHeight : rootWidth,
            `${id} pre-item sample must be the first pixel outside the root.`
          );
          assert.ok(
            beforeItemSample[0] < x || beforeItemSample[0] >= x + width
              || beforeItemSample[1] < y || beforeItemSample[1] >= y + height,
            `${id} pre-item sample must be outside the hero frame.`
          );
          assert.ok(
            afterItemSample[0] >= x + width || afterItemSample[1] >= y + height,
            `${id} after-item sample must be outside the hero frame.`
          );
          assert.equal(
            overflowAxis === 'Rows' ? afterItemSample[1] : afterItemSample[0],
            overflowAxis === 'Rows' ? y + height : x + width,
            `${id} after-item sample must be the first pixel after the hero frame.`
          );
          assert.ok(
            afterItemSample[0] >= rootWidth || afterItemSample[1] >= rootHeight,
            `${id} after-item sample must be outside the root.`
          );
          const item = fixtureItems.find(({ name }) => name === 'hero');
          assert.ok(item, `${id} overflow sample requires a hero fixture item.`);
          pixelSamples.push(
            {
              id: `${id}-before-overflow-item`, nodeId: rootNodeId,
              xCssPx: beforeItemSample[0], yCssPx: beforeItemSample[1], rgba: [0, 0, 0, 255],
            },
            {
              id: `${id}-hero-overflow-outside-root`,
              nodeId: `c114-${id}-hero`,
              xCssPx: itemSample[0], yCssPx: itemSample[1], rgba: rgba(item.color),
            },
            {
              id: `${id}-after-overflow-item`, nodeId: rootNodeId,
              xCssPx: afterItemSample[0], yCssPx: afterItemSample[1], rgba: [0, 0, 0, 255],
            },
          );
        } else {
          for (const { name, frame } of items) {
            const [x, y, width, height] = frame;
            const item = fixtureItems.find(({ name: itemName }) => itemName === name);
            assert.ok(item, `${id}/${name} fixture item is missing.`);
            pixelSamples.push({
              id: `${id}-${name}-interior`, nodeId: `c114-${id}-${name}`,
              xCssPx: x + Math.floor(width / 2), yCssPx: y + Math.floor(height / 2),
              rgba: rgba(item.color),
            });
          }
          const furthestX = Math.max(rootWidth,
            ...items.map(({ frame: [x, , width] }) => x + width));
          const furthestY = Math.max(rootHeight,
            ...items.map(({ frame: [, y, , height] }) => y + height));
          pixelSamples.push({
            id: `${id}-after-root`, nodeId: rootNodeId,
            xCssPx: id.includes('-fixed-cols-') ? 10 : furthestX,
            yCssPx: id.includes('-fixed-cols-') ? furthestY : 10,
            rgba: [0, 0, 0, 255],
          });
        }
      } else if (id.endsWith('-area-added-both')) {
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

async function autoRepeatFixedMaximumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_maximum_scenes.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 49,
    'intrinsic minimum fixed-maximum auto-repeat fixture는 49 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_FIXED_MAXIMUM_PIXEL_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android fixed-maximum pixel');
  const iosScenes = allowlist(iosSource,
    /private let c114FixedMaximumPixelSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS fixed-maximum pixel');
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
    const axis = id.startsWith('columns-') ? 'Columns' : 'Rows';
    const rootNodeId = `c114-${id}-grid`;
    const itemNodeId = `c114-${id}-${items[0].name}`;
    const [itemX, itemY, itemWidth, itemHeight] = expect.items[0].frame;
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight};`
      + `1:node=2,x=${itemX},y=${itemY},width=${itemWidth},height=${itemHeight}]`;
    const overflow = id.includes('-overflow-');
    const pixelSamples = overflow
      ? axis === 'Columns'
        ? [
          { id: `${id}-root-interior`, nodeId: rootNodeId,
            xCssPx: 5, yCssPx: 20, rgba: colorRgba('#e2e8f0') },
          { id: `${id}-item-inside-root`, nodeId: itemNodeId,
            xCssPx: 5, yCssPx: 10, rgba: colorRgba(items[0].color) },
          { id: `${id}-item-outside-root`, nodeId: itemNodeId,
            xCssPx: 15, yCssPx: 10, rgba: colorRgba(items[0].color) },
        ]
        : [
          { id: `${id}-root-interior`, nodeId: rootNodeId,
            xCssPx: 30, yCssPx: 5, rgba: colorRgba('#e2e8f0') },
          { id: `${id}-item-inside-root`, nodeId: itemNodeId,
            xCssPx: 10, yCssPx: 5, rgba: colorRgba(items[0].color) },
          { id: `${id}-item-outside-root`, nodeId: itemNodeId,
            xCssPx: 10, yCssPx: 15, rgba: colorRgba(items[0].color) },
        ]
      : [
        { id: `${id}-root-interior`, nodeId: rootNodeId,
          xCssPx: Math.floor(rootWidth / 2), yCssPx: Math.floor(rootHeight / 2),
          rgba: colorRgba('#e2e8f0') },
        { id: `${id}-item-1-interior`, nodeId: itemNodeId,
          xCssPx: Math.floor(itemX + itemWidth / 2),
          yCssPx: Math.floor(itemY + itemHeight / 2), rgba: colorRgba(items[0].color) },
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

async function autoRepeatZeroFixedMaximumScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_fixed_maximum_scenes.json');
  const chromePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/chrome-156-observations.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, chrome, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 24,
    'zero fixed-maximum auto-repeat fixture는 24 scene이어야 합니다.');
  assert.ok(chrome.userAgent.includes('HeadlessChrome/156.0.0.0'),
    'zero fixed-maximum oracle는 Chrome 156이어야 합니다.');
  assert.equal(chrome.scenes.length, 24,
    'zero fixed-maximum Chrome 관측은 24 scene이어야 합니다.');
  const allowlist = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} scene allowlist를 읽을 수 없습니다.`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidScenes = allowlist(androidSource,
    /C114_ZERO_FIXED_MAXIMUM_PIXEL_SCENE_IDS\s*=\s*BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*\?\s*new String\[\]\s*\{([\s\S]*?)\}\s*:\s*new String\[0\];/,
    'Android zero fixed-maximum pixel');
  const iosScenes = allowlist(iosSource,
    /private let c114ZeroFixedMaximumPixelSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS zero fixed-maximum pixel');
  const chromeById = new Map(chrome.scenes.map((scene) => [scene.id, scene]));
  assert.deepEqual(fixtures.map(({ id }) => id), chrome.scenes.map(({ id }) => id),
    'runtime fixture와 Chrome scene ID 순서가 달라졌습니다.');
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];
  const close = (actual, expected, label) => {
    assert.ok(Math.abs(actual - expected) <= 1 / 64,
      `${label}: ${actual} != ${expected}`);
  };

  return fixtures.map((scene) => {
    const { id, expect } = scene;
    const oracle = chromeById.get(id);
    assert.ok(oracle, `Chrome oracle에 ${id}가 없습니다.`);
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    for (const key of ['axis', 'mode', 'minimum', 'maximum']) {
      assert.equal(scene[key], oracle[key], `${id} ${key}`);
    }
    for (const key of ['gap', 'basis']) {
      close(Number(scene[key]), Number(oracle[key]), `${id} ${key}`);
    }
    const axisKey = scene.axis;
    const repeatedValues = (axisKey === 'columns'
      ? oracle.gridTemplateColumns : oracle.gridTemplateRows)
      .split(/\s+/).map((value) => Number.parseFloat(value));
    const expectedCount = Math.floor((scene.basis + scene.gap) / (1 + scene.gap));
    assert.equal(repeatedValues.length, expectedCount, `${id} independent repeat count`);
    assert.equal(oracle.repeatedTrackCount, expectedCount, `${id} Chrome repeat count`);
    assert.deepEqual(expect.tracks[axisKey], repeatedValues, `${id} used track list`);
    assert.equal(expect.tracks[axisKey][2], 12, `${id} occupied track minimum`);
    assert.ok(expect.tracks[axisKey].every((value, index) => index === 2 || value === 0),
      `${id} empty track sizes`);
    const collapsed = repeatedValues.map((_, index) => scene.mode === 'auto-fit' && index !== 2);
    assert.deepEqual(expect.collapsed[axisKey], collapsed, `${id} auto-fit collapse`);
    assert.deepEqual(expect.items[0].frame, [
      oracle.itemOffset.x, oracle.itemOffset.y, oracle.item.width, oracle.item.height,
    ], `${id} item frame`);

    const [rootWidth, rootHeight] = expect.root;
    const [itemX, itemY, itemWidth, itemHeight] = expect.items[0].frame;
    const rootNodeId = `c114-${id}-grid`;
    const itemNodeId = `c114-${id}-item-1`;
    const axis = axisKey === 'columns' ? 'Columns' : 'Rows';
    const pixelSamples = [
      {
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: axisKey === 'columns' ? 2 : 30,
        yCssPx: axisKey === 'columns' ? 30 : 2,
        rgba: colorRgba('#e2e8f0'),
      },
      {
        id: `${id}-item-interior`, nodeId: itemNodeId,
        xCssPx: Math.floor(itemX + itemWidth / 2),
        yCssPx: Math.floor(itemY + itemHeight / 2),
        rgba: colorRgba('#e5484d'),
      },
    ];
    const itemOverflowsRoot = scene.mode === 'auto-fill' && scene.gap === 4;
    if (itemOverflowsRoot) {
      pixelSamples.push({
        id: `${id}-item-outside-root`, nodeId: itemNodeId,
        xCssPx: axisKey === 'columns' ? 18 : 6,
        yCssPx: axisKey === 'columns' ? 6 : 18,
        rgba: colorRgba('#e5484d'),
      });
    }
    const nodeFrames = `node_frames_css_px=[0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight};`
      + `1:node=2,x=${itemX},y=${itemY},width=${itemWidth},height=${itemHeight}]`;
    return {
      id,
      axis,
      boxCount: 2,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: {
        Columns: Number.parseFloat(scene.columnGap),
        Rows: Number.parseFloat(scene.rowGap),
      },
      pixelSamples,
    };
  });
}

async function autoRepeatPercentageNeighborScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_percentage_neighbor_scenes.json');
  const expectationPath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/spec-derived-expectations.json');
  const chromePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, oracle, chrome, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(expectationPath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 13,
    'percentage auto-repeat neighbor fixture는 13 scene이어야 합니다.');
  assert.equal(oracle.cases.length, 13,
    'percentage auto-repeat neighbor 독립 기대값은 13 case여야 합니다.');
  assert.equal(chrome.scenes.length, 13,
    'Chrome 156 percentage auto-repeat neighbor 관측은 13 scene이어야 합니다.');
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
  const oracleById = new Map(oracle.cases.map((scene) => [scene.id, scene]));
  const chromeById = new Map(chrome.scenes.map((scene) => [scene.id, scene]));
  assert.deepEqual(fixtures.map(({ id }) => id), oracle.cases.map(({ id }) => id),
    'runtime fixture and independent normative matrix scene order differs.');
  assert.deepEqual(fixtures.map(({ id }) => id), chrome.scenes.map(({ id }) => id),
    'runtime fixture and Chrome CSSOM observation scene order differs.');
  const close = (actual, expected, label) => {
    assert.ok(Math.abs(actual - expected) <= 1 / 64,
      `${label}: ${actual} != ${expected}`);
  };
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];

  return fixtures.map((fixture) => {
    const { id, expect, columnGap, rowGap, items } = fixture;
    const oracleScene = oracleById.get(id);
    const chromeScene = chromeById.get(id);
    assert.ok(oracleScene, `${id}: independent normative case is missing.`);
    assert.ok(chromeScene, `${id}: Chrome CSSOM observation is missing.`);
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    assert.equal(fixture.boxSizing, 'border-box', `${id}: content basis must be border-box adjusted.`);
    assert.equal(fixture.padding, '8px', `${id}: fixture padding differs from the oracle.`);
    assert.equal(fixture.border, '2px solid #111111', `${id}: fixture border differs from the oracle.`);
    const axis = oracleScene.axis === 'columns' ? 'Columns' : 'Rows';
    const axisKey = axis.toLowerCase();
    const otherAxis = axis === 'Columns' ? 'Rows' : 'Columns';
    const [rootWidth, rootHeight] = expect.root;
    const rootAxisSize = axis === 'Columns' ? rootWidth : rootHeight;
    const borderBox = id.endsWith('border-box');
    const expectedRootSize = oracleScene.axis === 'columns'
      ? [borderBox ? 136 : 260, 48]
      : [48, borderBox ? 136 : 260];
    assert.deepEqual(expect.root, expectedRootSize, `${id}: outer root frame`);
    assert.equal(rootAxisSize - 20, borderBox ? 116 : 240,
      `${id}: padding/border adjusted percentage basis`);
    assert.equal(items.length, oracleScene.itemFrameOuterPx ? 1 : 0, `${id}: item count`);
    assert.equal(expect.items.length, items.length, `${id}: expected item count`);
    if (items.length) {
      assert.deepEqual(expect.items[0].frame, oracleScene.itemFrameOuterPx,
        `${id}: independently expected item frame`);
      assert.equal(items[0].name, expect.items[0].name, `${id}: item identity`);
    }

    const repeatTemplate = `repeat(${oracleScene.mode},minmax(40px,1fr))`;
    const prefix = oracleScene.inputs.prefixPercent == null
      ? null : `${oracleScene.inputs.prefixPercent}%`;
    const suffix = oracleScene.inputs.suffixPercent == null
      ? null : `${oracleScene.inputs.suffixPercent}%`;
    const expectedTemplate = [prefix, repeatTemplate, suffix].filter(Boolean).join(' ');
    assert.equal(fixture[axisKey], expectedTemplate, `${id}: explicit track source`);
    assert.equal(fixture[otherAxis.toLowerCase()], '12px', `${id}: cross-axis fixed track`);
    const expectedActiveCollapsed = oracleScene.usedTracksPx.map((_, index) => {
      const repeatStart = oracleScene.prefixTrackCount;
      const inRepeat = index >= repeatStart && index < repeatStart + oracleScene.repeatCount;
      return inRepeat && oracleScene.mode === 'auto-fit'
        && !oracleScene.inputs.occupiedRepeatIndexes.includes(index - repeatStart);
    });
    assert.deepEqual(expect.tracks[axisKey], oracleScene.usedTracksPx,
      `${id}: normative used-track list`);
    assert.deepEqual(expect.collapsed[axisKey], expectedActiveCollapsed,
      `${id}: normative auto-fit collapse list`);
    assert.equal(expect.tracks[otherAxis.toLowerCase()].length, 1,
      `${id}: cross-axis track count`);
    assert.deepEqual(expect.collapsed[otherAxis.toLowerCase()], [false],
      `${id}: cross-axis collapse list`);

    const chromeTracks = chromeScene[axisKey].map((track) => Number.parseFloat(track));
    const browserExpected = oracleScene.chromeObservedTracksPx ?? oracleScene.usedTracksPx;
    assert.equal(chromeTracks.length, browserExpected.length, `${id}: Chrome track count`);
    for (const [index, actual] of chromeTracks.entries()) {
      close(actual, browserExpected[index], `${id}: Chrome CSSOM track ${index}`);
    }
    const expectedGap = Number.parseFloat(axis === 'Columns' ? columnGap : rowGap);
    const chromeGapText = axis === 'Columns' ? chromeScene.columnGap : chromeScene.rowGap;
    const chromeGap = chromeGapText === 'normal' ? 0 : Number.parseFloat(chromeGapText);
    assert.ok(Number.isFinite(expectedGap) && Number.isFinite(chromeGap), `${id}: gap value`);
    close(chromeGap, expectedGap, `${id}: Chrome gap observation`);

    const rootNodeId = `c114-${id}-grid`;
    const nodeFrames = [
      `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
      ...expect.items.map(({ frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      }),
    ].join(';');
    const pixelSamples = [
      {
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: Math.floor(rootWidth / 2), yCssPx: Math.floor(rootHeight / 2),
        rgba: colorRgba(fixture.rootColor),
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
      itemFrame: items.length ? 'width=2,height=2' : null,
      nodeFrames: `node_frames_css_px=[${nodeFrames}]`,
      axes: {
        Columns: {
          tracks: expect.tracks.columns,
          collapsed: expect.collapsed.columns,
        },
        Rows: {
          tracks: expect.tracks.rows,
          collapsed: expect.collapsed.rows,
        },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      pixelSamples,
    };
  });
}

async function autoRepeatCalcNeighborScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_neighbor_scenes.json');
  const expectationPath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/spec-derived-expectations.json');
  const chromePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/chrome156-observation.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, oracle, chrome, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(expectationPath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 13, '기존 calc 이웃 runtime fixture는 13 scene이어야 합니다.');
  assert.equal(Object.keys(oracle.cases).length, 14, '독립 calc 이웃 판정은 14 case여야 합니다.');
  assert.equal(chrome.scenes.length, 14, 'Chrome calc 이웃 관측은 14 scene이어야 합니다.');
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
  const chromeById = new Map(chrome.scenes.map((scene) => [scene.id, scene]));
  const expectedIDs = Object.keys(oracle.cases)
    .filter((id) => id !== 'col-fill-negative-percentage-term');
  const currentChromeIDs = chrome.scenes
    .filter((scene) => scene.id !== 'col-fill-negative-percentage-term')
    .map(({ id }) => id);
  assert.deepEqual(fixtures.map(({ oracleId }) => oracleId), expectedIDs,
    'runtime fixture and independent calc expectation order differs.');
  assert.deepEqual(fixtures.map(({ oracleId }) => oracleId), currentChromeIDs,
    'runtime fixture and Chrome calc observation order differs.');
  const close = (actual, expected, label) => {
    assert.ok(Math.abs(actual - expected) <= 1 / 64,
      `${label}: ${actual} != ${expected}`);
  };
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];
  const lineNumber = (placement) => Number.parseInt(placement ?? '1', 10);
  const positionAtLine = (tracks, gap, line) => tracks.slice(0, line - 1)
    .reduce((sum, size) => sum + size, 0) + Math.max(0, line - 1) * gap;

  return fixtures.map((fixture) => {
    const { id, expect, columnGap, rowGap, items } = fixture;
    const oracleID = fixture.oracleId;
    const normative = oracle.cases[oracleID];
    const chromeScene = chromeById.get(oracleID);
    assert.ok(normative && chromeScene, `${id}: independent or Chrome observation is missing.`);
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    const rejection = fixture.expectedRejection;
    if (normative.fixtureScope.startsWith('out-of-scope')) {
      assert.ok(rejection, `${id}: out-of-scope control must have a rejection contract.`);
      assert.equal(rejection.status, -11, `${id}: rejection status`);
      assert.equal(rejection.property, 'grid-template-columns', `${id}: rejected property`);
      assert.equal(rejection.nodeId, 1, `${id}: rejected node`);
      assert.equal(items.length, 1, `${id}: negative controls keep a visible source item.`);
      return {
        id,
        expectedRejection: rejection,
        axis: normative.axis === 'columns' ? 'Columns' : 'Rows',
        boxCount: null,
        rootFrame: null,
        itemFrame: null,
        nodeFrames: null,
        pixelSamples: null,
      };
    }

    assert.equal(rejection, undefined, `${id}: in-scope scene must reach layout and paint.`);
    const axis = normative.axis === 'columns' ? 'Columns' : 'Rows';
    const axisKey = axis.toLowerCase();
    const otherAxis = axis === 'Columns' ? 'Rows' : 'Columns';
    const [rootWidth, rootHeight] = expect.root;
    const axisTracks = expect.tracks[axisKey];
    const expectedTracks = normative.usedTracksCssPx;
    assert.equal(axisTracks.length, expectedTracks.length, `${id}: independent track count`);
    for (const [index, track] of expectedTracks.entries()) {
      close(axisTracks[index], track, `${id}: normative track ${index}`);
    }
    assert.deepEqual(expect.collapsed[axisKey], axisTracks.map((_, index) => (
      id === 'calc-neighbor-col-fit-first' ? index >= 2 && index <= 4
        : id === 'calc-neighbor-col-fit-empty' ? index >= 1 && index <= 4
          : false
    )), `${id}: auto-fit collapse state`);
    assert.deepEqual(expect.tracks[otherAxis.toLowerCase()], [10], `${id}: cross-axis track`);
    assert.deepEqual(expect.collapsed[otherAxis.toLowerCase()], [false], `${id}: cross-axis collapse`);
    assert.equal(items.length, expect.items.length, `${id}: item count`);
    assert.ok(expect.items.length <= 1, `${id}: this matrix has at most one item.`);
    assert.equal(fixture.rootColor, '#eef2ff', `${id}: root sample color`);

    const gap = Number.parseFloat(axis === 'Columns' ? columnGap : rowGap);
    const chromeTracks = chromeScene[axisKey].trim().split(/\s+/)
      .map((value) => Number.parseFloat(value));
    assert.equal(chromeTracks.length, expectedTracks.length, `${id}: Chrome track count`);
    for (const [index, actual] of chromeTracks.entries()) {
      close(actual, expectedTracks[index], `${id}: Chrome CSSOM track ${index}`);
    }
    const chromeGap = Number.parseFloat(axis === 'Columns'
      ? chromeScene.columnGap : chromeScene.rowGap);
    close(chromeGap, gap, `${id}: Chrome gap`);
    close(chromeScene.grid.width, rootWidth, `${id}: Chrome root width`);
    close(chromeScene.grid.height, rootHeight, `${id}: Chrome root height`);
    const expectedBasis = normative.definiteContentSize;
    assert.ok(expectedBasis != null, `${id}: definite used-size basis is missing.`);
    close(axis === 'Columns' ? rootWidth - (fixture.boxSizing === 'border-box' ? 20 : 0)
      : rootHeight - (fixture.boxSizing === 'border-box' ? 20 : 0),
    expectedBasis, `${id}: content-box basis`);

    if (items.length) {
      const item = items[0];
      const itemExpected = expect.items[0];
      const itemColumn = lineNumber(item.placement.gridColumn);
      const itemRow = lineNumber(item.placement.gridRow);
      const [borderWidth] = fixture.border.match(/^[\d.]+/)?.map(Number) ?? [0];
      const padding = Number.parseFloat(fixture.padding);
      const contentOrigin = fixture.boxSizing === 'border-box'
        ? borderWidth + padding : 0;
      const x = contentOrigin + (axis === 'Columns'
        ? positionAtLine(expect.tracks.columns, Number.parseFloat(columnGap), itemColumn)
        : (itemColumn - 1) * Number.parseFloat(columnGap));
      const y = contentOrigin + (axis === 'Rows'
        ? positionAtLine(expect.tracks.rows, Number.parseFloat(rowGap), itemRow)
        : (itemRow - 1) * Number.parseFloat(rowGap));
      close(itemExpected.frame[0], x, `${id}: independently positioned item x`);
      close(itemExpected.frame[1], y, `${id}: independently positioned item y`);
      const chromeItem = chromeScene.item;
      assert.ok(chromeItem, `${id}: Chrome item geometry is missing.`);
      close(chromeItem.x - chromeScene.grid.x, x, `${id}: Chrome local item x`);
      close(chromeItem.y - chromeScene.grid.y, y, `${id}: Chrome local item y`);
      close(chromeItem.width, itemExpected.frame[2], `${id}: Chrome item width`);
      close(chromeItem.height, itemExpected.frame[3], `${id}: Chrome item height`);
    } else {
      assert.equal(chromeScene.item, null, `${id}: Chrome auto-fit empty scene item`);
    }

    const rootNodeId = `c114-${id}-grid`;
    const nodeFrames = [
      `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
      ...expect.items.map(({ frame }, index) => {
        const [x, y, width, height] = frame;
        return `${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`;
      }),
    ].join(';');
    const pixelSamples = [
      {
        id: `${id}-root-interior`, nodeId: rootNodeId,
        xCssPx: Math.floor(rootWidth / 2), yCssPx: Math.floor(rootHeight / 2),
        rgba: colorRgba(fixture.rootColor),
      },
      ...expect.items.map(({ name, frame }) => {
        const [x, y, width, height] = frame;
        return {
          id: `${id}-${name}-interior`, nodeId: `c114-${id}-${name}`,
          xCssPx: Math.floor(x + width / 2), yCssPx: Math.floor(y + height / 2),
          rgba: colorRgba(items[0].color),
        };
      }),
    ];
    if (id === 'calc-neighbor-col-fill-overflow') {
      pixelSamples.push(
        { id: `${id}-root-right-interior`, nodeId: rootNodeId,
          xCssPx: rootWidth - 1, yCssPx: 5, rgba: colorRgba(fixture.rootColor) },
        { id: `${id}-root-exterior`, nodeId: rootNodeId,
          xCssPx: rootWidth, yCssPx: 5, rgba: [0, 0, 0, 255] },
        { id: `${id}-item-overflow`, nodeId: `c114-${id}-item-1`,
          xCssPx: 244, yCssPx: 1, rgba: colorRgba(items[0].color) },
      );
    }
    return {
      id,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: items.length ? 'width=2,height=2' : null,
      nodeFrames: `node_frames_css_px=[${nodeFrames}]`,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      pixelSamples,
    };
  });
}

async function autoRepeatNegativePercentageNeighborScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_percentage_neighbor_scenes.json');
  const expectationPath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/spec-derived-expectations.json');
  const chromePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, oracle, chrome, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(expectationPath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 9, '음수 percentage calc runtime fixture는 9 scene이어야 합니다.');
  assert.equal(oracle.sceneCount, 9, '독립 음수 percentage calc 판정은 9 case여야 합니다.');
  assert.equal(chrome.scenes.length, 9, 'Chrome 음수 percentage calc 관측은 9 scene이어야 합니다.');
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
  const oracleById = new Map(oracle.cases.map((scene) => [scene.id, scene]));
  const chromeById = new Map(chrome.scenes.map((scene) => [scene.id, scene]));
  assert.deepEqual(fixtures.map(({ oracleId }) => oracleId), oracle.cases.map(({ id }) => id),
    'runtime fixture and independent negative percentage expectations differ.');
  assert.deepEqual(fixtures.map(({ oracleId }) => oracleId), chrome.scenes.map(({ id }) => id),
    'runtime fixture and Chrome negative percentage observations differ.');
  const close = (actual, expected, label) => {
    assert.ok(Math.abs(actual - expected) <= 1 / 64,
      `${label}: ${actual} != ${expected}`);
  };
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];
  const positionAtLine = (tracks, gap, line) => tracks.slice(0, line - 1)
    .reduce((sum, size) => sum + size, 0) + Math.max(0, line - 1) * gap;

  return fixtures.map((fixture) => {
    const { id, expect, columnGap, rowGap, items } = fixture;
    const normative = oracleById.get(fixture.oracleId);
    const chromeScene = chromeById.get(fixture.oracleId);
    assert.ok(normative && chromeScene, `${id}: independent or Chrome observation is missing.`);
    assert.ok(androidScenes.has(id), `Android Grid allowlist에 ${id}가 없습니다.`);
    assert.ok(iosScenes.has(id), `iOS Grid allowlist에 ${id}가 없습니다.`);
    assert.equal(fixture.expectedRejection, undefined, `${id}: supported scene must reach layout.`);
    assert.equal(items.length, 1, `${id}: the fixed matrix has one painted item.`);
    assert.equal(expect.items.length, 1, `${id}: expected item geometry is missing.`);
    assert.equal(fixture.rootColor, '#eef2ff', `${id}: root sample color`);
    assert.equal(items[0].color, '#e5484d', `${id}: item sample color`);
    const axis = normative.axis === 'columns' ? 'Columns' : 'Rows';
    const axisKey = axis.toLowerCase();
    const otherAxis = axis === 'Columns' ? 'Rows' : 'Columns';
    const [rootWidth, rootHeight] = expect.root;
    const axisTracks = expect.tracks[axisKey];
    assert.equal(axisTracks.length, normative.usedTracksCssPx.length, `${id}: track count`);
    for (const [index, track] of normative.usedTracksCssPx.entries()) {
      close(axisTracks[index], track, `${id}: independent track ${index}`);
    }
    assert.deepEqual(expect.collapsed[axisKey], axisTracks.map((_, index) => (
      normative.collapsedTrackIndices.includes(index)
    )), `${id}: auto-fit collapse state`);
    assert.deepEqual(expect.tracks[otherAxis.toLowerCase()], [10], `${id}: cross-axis track`);
    assert.deepEqual(expect.collapsed[otherAxis.toLowerCase()], [false], `${id}: cross-axis collapse`);
    const chromeTracks = chromeScene[axisKey].trim().split(/\s+/)
      .map((value) => Number.parseFloat(value));
    assert.equal(chromeTracks.length, axisTracks.length, `${id}: Chrome track count`);
    for (const [index, actual] of chromeTracks.entries()) {
      close(actual, axisTracks[index], `${id}: Chrome CSSOM track ${index}`);
    }
    const gap = Number.parseFloat(axis === 'Columns' ? columnGap : rowGap);
    close(Number.parseFloat(axis === 'Columns' ? chromeScene.columnGap : chromeScene.rowGap),
      gap, `${id}: Chrome gap`);
    close(chromeScene.grid.width, rootWidth, `${id}: Chrome root width`);
    close(chromeScene.grid.height, rootHeight, `${id}: Chrome root height`);
    close(axis === 'Columns' ? rootWidth - (fixture.boxSizing === 'border-box' ? 20 : 0)
      : rootHeight - (fixture.boxSizing === 'border-box' ? 20 : 0),
    normative.definiteContentSize, `${id}: content-box basis`);

    const item = items[0];
    const itemExpected = expect.items[0];
    const itemColumn = Number.parseInt(item.placement.gridColumn ?? '1', 10);
    const itemRow = Number.parseInt(item.placement.gridRow ?? '1', 10);
    const [borderWidth] = fixture.border.match(/^[\d.]+/)?.map(Number) ?? [0];
    const padding = Number.parseFloat(fixture.padding);
    const contentOrigin = fixture.boxSizing === 'border-box' ? borderWidth + padding : 0;
    const x = contentOrigin + (axis === 'Columns'
      ? positionAtLine(expect.tracks.columns, Number.parseFloat(columnGap), itemColumn)
      : (itemColumn - 1) * Number.parseFloat(columnGap));
    const y = contentOrigin + (axis === 'Rows'
      ? positionAtLine(expect.tracks.rows, Number.parseFloat(rowGap), itemRow)
      : (itemRow - 1) * Number.parseFloat(rowGap));
    close(itemExpected.frame[0], x, `${id}: independent item x`);
    close(itemExpected.frame[1], y, `${id}: independent item y`);
    close(chromeScene.item.x - chromeScene.grid.x, x, `${id}: Chrome local item x`);
    close(chromeScene.item.y - chromeScene.grid.y, y, `${id}: Chrome local item y`);
    close(chromeScene.item.width, itemExpected.frame[2], `${id}: Chrome item width`);
    close(chromeScene.item.height, itemExpected.frame[3], `${id}: Chrome item height`);

    const rootNodeId = `c114-${id}-grid`;
    const [itemX, itemY, itemWidth, itemHeight] = itemExpected.frame;
    return {
      id,
      axis,
      boxCount: 2,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: `width=${itemWidth},height=${itemHeight}`,
      nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight};1:node=2,x=${itemX},y=${itemY},width=${itemWidth},height=${itemHeight}]`,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      pixelSamples: [
        {
          id: `${id}-root-interior`, nodeId: rootNodeId,
          xCssPx: Math.floor(rootWidth / 2), yCssPx: Math.floor(rootHeight / 2),
          rgba: colorRgba(fixture.rootColor),
        },
        {
          id: `${id}-item-1-interior`, nodeId: `c114-${id}-item-1`,
          xCssPx: Math.floor(itemX + itemWidth / 2), yCssPx: Math.floor(itemY + itemHeight / 2),
          rgba: colorRgba(item.color),
        },
      ],
    };
  });
}

async function fixedRepeatAutoRepeatScenes({
  fixtureRelativePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_repeat_scenes.json',
  chromeRelativePath = 'spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-observation.json',
  additionalChromeRelativePaths = [],
  chromeInputRelativePath = 'tests/fixtures/css/c11/grid-auto-repeat-fixed-repeat-c114-precomparison.html',
  expectedCount = 9,
} = {}) {
  const fixturePath = resolve(repositoryRoot, fixtureRelativePath);
  const chromePath = resolve(repositoryRoot, chromeRelativePath);
  const chromeInputPath = resolve(repositoryRoot, chromeInputRelativePath);
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, chrome, chromeInput, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(chromeInputPath, 'utf8'),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  const additionalChrome = await Promise.all(additionalChromeRelativePaths.map(async (relativePath) => {
    const observation = JSON.parse(await readFile(resolve(repositoryRoot, relativePath), 'utf8'));
    return observation.observations.map((scene) => ({
      id: scene.id,
      trackAxis: scene.trackAxis,
      ...(scene.trackAxis === 'columns'
        ? { templateColumns: scene.tracks }
        : { templateRows: scene.tracks }),
      grid: scene.grid,
      items: scene.items,
    }));
  }));
  const fixedRepeatFixtures = fixtures.filter(({ id }) => id.startsWith('fixed-repeat-')
    && !id.startsWith('fixed-repeat-minmax-flex-'));
  assert.equal(fixedRepeatFixtures.length, expectedCount,
    `fixed-repeat + auto-repeat runtime matrix는 ${expectedCount} scene이어야 합니다.`);
  const chromeCases = [...(chrome.cases ?? chrome.scenes), ...additionalChrome.flat()];
  assert.equal(chromeCases.length, expectedCount,
    `Chrome 156 사전 비교는 ${expectedCount} case여야 합니다.`);
  assert.ok(chromeInput.includes('gap: 8px;'), 'Chrome oracle input must keep its fixed 8px gutter.');
  assert.match(androidSource,
    /BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&&\s*sceneId != null\s*&& sceneId\.startsWith\("fixed-repeat-"\)/,
    'Android는 opt-in fixture build에서만 fixed-repeat scene ID를 받아야 합니다.');
  const iosFixtureBlock = iosSource.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock,
    'iOS fixed-repeat scene ID는 opt-in fixture build에서만 컴파일되어야 합니다.');
  const chromeById = new Map(chromeCases.map((scene) => [scene.id, scene]));
  const close = (actual, expected, label) => {
    assert.ok(Math.abs(actual - expected) <= 1 / 64,
      `${label}: ${actual} != ${expected}`);
  };
  const colorRgba = (color) => [
    Number.parseInt(color.slice(1, 3), 16),
    Number.parseInt(color.slice(3, 5), 16),
    Number.parseInt(color.slice(5, 7), 16),
    255,
  ];

  return fixedRepeatFixtures.map((fixture) => {
    const { id, oracleId, repeatAxis, expect, items, rootColor, columnGap, rowGap } = fixture;
    const chromeScene = chromeById.get(oracleId);
    assert.ok(id.startsWith('fixed-repeat-'), `${id}: opt-in scene prefix`);
    assert.ok(chromeScene, `${id}: Chrome oracle scene ${oracleId} is missing.`);
    assert.ok(iosFixtureBlock.includes(`"${id}"`),
      `iOS opt-in scene allowlist is missing ${id}.`);
    assert.equal(chromeScene.trackAxis, repeatAxis, `${id}: repeat axis`);
    assert.equal(items.length, expect.items.length, `${id}: expected item count`);
    assert.equal(chromeScene.items.length, expect.items.length, `${id}: Chrome item count`);
    const [rootWidth, rootHeight] = expect.root;
    close(chromeScene.grid.width, rootWidth, `${id}: Chrome root width`);
    close(chromeScene.grid.height, rootHeight, `${id}: Chrome root height`);
    const axis = repeatAxis === 'columns' ? 'Columns' : 'Rows';
    const axisKey = repeatAxis;
    const chromeTemplate = axis === 'Columns'
      ? chromeScene.templateColumns : chromeScene.templateRows;
    const chromeTracks = chromeTemplate.trim().split(/\s+/).map(Number.parseFloat);
    const expectedTracks = expect.tracks[axisKey];
    assert.equal(chromeTracks.length, expectedTracks.length, `${id}: Chrome track count`);
    for (const [index, track] of chromeTracks.entries()) {
      close(track, expectedTracks[index], `${id}: Chrome track ${index}`);
    }
    assert.equal(expect.collapsed.columns.length, expect.tracks.columns.length,
      `${id}: column collapse count`);
    assert.equal(expect.collapsed.rows.length, expect.tracks.rows.length,
      `${id}: row collapse count`);
    assert.equal(Number.parseFloat(columnGap), 8, `${id}: column gutter`);
    assert.equal(Number.parseFloat(rowGap), 8, `${id}: row gutter`);

    const rootNodeId = `c114-${id}-grid`;
    const frames = [`0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`];
    for (const [index, item] of expect.items.entries()) {
      const [x, y, width, height] = item.frame;
      const chromeItem = chromeScene.items[index];
      close(chromeItem.x, x, `${id}: Chrome item ${index} x`);
      close(chromeItem.y, y, `${id}: Chrome item ${index} y`);
      close(chromeItem.width, width, `${id}: Chrome item ${index} width`);
      close(chromeItem.height, height, `${id}: Chrome item ${index} height`);
      frames.push(`${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`);
    }
    const pixelSamples = [{
      id: `${id}-root-interior`, nodeId: rootNodeId,
      xCssPx: Math.floor(rootWidth / 2), yCssPx: Math.floor(rootHeight / 2),
      rgba: colorRgba(rootColor),
    }];
    for (const [index, item] of items.entries()) {
      const [x, y, width, height] = expect.items[index].frame;
      pixelSamples.push({
        id: `${id}-${item.name}-interior`, nodeId: `c114-${id}-${item.name}`,
        xCssPx: Math.floor(x + width / 2), yCssPx: Math.floor(y + height / 2),
        rgba: colorRgba(item.color),
      });
    }
    return {
      id,
      oracleId,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: expect.items.length === 0
        ? null
        : `width=${expect.items[0].frame[2]},height=${expect.items[0].frame[3]}`,
      nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      pixelSamples,
    };
  });
}

async function fixedRepeatMinmaxFlexScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_minmax_flex_scenes.json');
  const expectationsPath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtures, expectations, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(expectationsPath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  assert.equal(fixtures.length, 6, 'fixed-repeat minmax flex matrix는 여섯 scene이어야 합니다.');
  assert.equal(expectations.scenes.length, fixtures.length,
    'fixed-repeat minmax flex의 독립 기대값 scene 수가 다릅니다.');
  const oracleById = new Map(expectations.scenes.map((scene) => [scene.id, scene]));
  assert.match(androidSource,
    /BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&&\s*sceneId != null\s*&& sceneId\.startsWith\("fixed-repeat-"\)/,
    'Android fixed-repeat minmax flex scene은 opt-in fixture build에만 허용되어야 합니다.');
  const iosFixtureBlock = iosSource.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock,
    'iOS fixed-repeat minmax flex scene은 opt-in fixture build에만 컴파일되어야 합니다.');

  return fixtures.map((fixture) => {
    const {
      id, oracleId, repeatAxis, expect, items, columnGap, rowGap,
    } = fixture;
    assert.ok(id.startsWith('fixed-repeat-minmax-flex-'),
      `${id}: runtime scene id must keep the fixed-repeat prefix`);
    assert.ok(iosFixtureBlock.includes(`"${id}"`),
      `iOS opt-in scene allowlist is missing ${id}.`);
    const oracle = oracleById.get(oracleId);
    assert.ok(oracle, `${id}: independent expectation ${oracleId} is missing.`);
    assert.equal(oracle.axis, repeatAxis, `${id}: repeat axis differs from its oracle.`);
    assert.equal(Number.parseFloat(columnGap), 4, `${id}: column gutter must remain 4px.`);
    assert.equal(Number.parseFloat(rowGap), 4, `${id}: row gutter must remain 4px.`);
    const axis = repeatAxis === 'columns' ? 'Columns' : 'Rows';
    const offsetIndex = repeatAxis === 'columns' ? 0 : 1;
    const expectedTracks = expect.tracks[repeatAxis];
    assert.deepEqual(expectedTracks, oracle.expectedTracks,
      `${id}: expanded track sizes differ from independent expectations.`);
    for (const axisKey of ['columns', 'rows']) {
      assert.equal(expect.collapsed[axisKey].length, expect.tracks[axisKey].length,
        `${id}: ${axisKey} collapsed track count differs.`);
    }
    assert.equal(items.length, expect.items.length, `${id}: fixture item count differs.`);
    const expectedStarts = oracle.expectedItemStarts;
    assert.equal(expect.items.length, expectedStarts.length,
      `${id}: item offset count differs from independent expectations.`);
    for (const [index, item] of expect.items.entries()) {
      assert.equal(item.name, items[index].name, `${id}: item ${index} identity differs.`);
      assert.equal(item.frame[offsetIndex], expectedStarts[index],
        `${id}: item ${index} offset differs from independent expectations.`);
    }

    const [rootWidth, rootHeight] = expect.root;
    assert.equal(expect.root[offsetIndex], oracle.container,
      `${id}: auto-repeat axis dimension differs from the independent expectation.`);
    const frames = [`0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`];
    for (const [index, item] of expect.items.entries()) {
      const [x, y, width, height] = item.frame;
      frames.push(`${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`);
    }
    return {
      id,
      oracleId,
      axis,
      boxCount: items.length + 1,
      rootFrame: `width=${rootWidth},height=${rootHeight}`,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      geometryTolerance: 0.02,
    };
  });
}

async function fixedAutoRepeatAreaScenes() {
  const fixturePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_repeat_scenes.json');
  const chromePath = resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/chrome156-observation.json');
  const androidPath = resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  const iosPath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  const [fixtureJSON, chromeJSON, androidSource, iosSource] = await Promise.all([
    readFile(fixturePath, 'utf8').then(JSON.parse),
    readFile(chromePath, 'utf8').then(JSON.parse),
    readFile(androidPath, 'utf8'),
    readFile(iosPath, 'utf8'),
  ]);
  const fixtures = fixtureJSON.filter(({ id }) => id.startsWith('fixed-auto-area-'));
  assert.equal(fixtures.length, 9, 'fixed + auto-repeat + areas runtime matrix must have 9 scenes.');
  assert.equal(chromeJSON.length, fixtures.length, 'Chrome 156 scene count differs.');
  assert.match(androidSource, /C114_FIXED_AUTO_AREA_SCENE_IDS/,
    'Android must gate fixed-auto-area scene IDs in the fixture adapter.');
  assert.match(androidSource, /c114IsFixedAutoAreaScene\(sceneId\)/,
    'Android must route the new scene family through its fixture-only adapter.');
  const iosFixtureBlock = iosSource.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock, 'iOS C11.4 scenes must remain inside the opt-in fixture build.');
  const chromeById = new Map(chromeJSON.map((scene) => [scene.id, scene]));
  const close = (actual, expected, label) => assert.ok(Math.abs(actual - expected) <= 1 / 64,
    `${label}: ${actual} != ${expected}`);
  const tracksFromCssom = (value) => [...value.matchAll(/([0-9]+(?:\.[0-9]+)?)px/g)]
    .map((match) => Number(match[1]));
  const colorRgba = (hex) => [
    Number.parseInt(hex.slice(1, 3), 16),
    Number.parseInt(hex.slice(3, 5), 16),
    Number.parseInt(hex.slice(5, 7), 16),
    255,
  ];

  return fixtures.map((fixture) => {
    const { id, oracleId, repeatAxis, expect, items, areas, rootColor, columnGap, rowGap } = fixture;
    const chrome = chromeById.get(oracleId);
    assert.ok(chrome, `${id}: Chrome oracle ${oracleId} is missing.`);
    assert.ok(iosFixtureBlock.includes(`"${id}"`), `${id}: iOS opt-in allowlist is missing.`);
    const [rootX, rootY, rootWidth, rootHeight] = chrome.root;
    close(rootWidth, expect.root[0], `${id}: Chrome root width`);
    close(rootHeight, expect.root[1], `${id}: Chrome root height`);
    assert.equal(chrome.areas, areas, `${id}: Chrome grid-template-areas differs.`);
    for (const axis of ['columns', 'rows']) {
      const cssTracks = tracksFromCssom(chrome[axis]);
      const expectedTracks = expect.tracks[axis];
      assert.equal(cssTracks.length, expectedTracks.length, `${id}: Chrome ${axis} track count`);
      for (const [index, size] of cssTracks.entries()) {
        close(size, expectedTracks[index], `${id}: Chrome ${axis} track ${index}`);
      }
    }
    assert.equal(chrome.children.length, expect.items.length, `${id}: Chrome item count`);
    const frames = [`0:node=1,x=0,y=0,width=${expect.root[0]},height=${expect.root[1]}`];
    for (const [index, expectedItem] of expect.items.entries()) {
      const chromeItem = chrome.children[index];
      assert.ok(chromeItem.className.split(/\s+/).includes(expectedItem.name),
        `${id}: Chrome item ${index} is not ${expectedItem.name}.`);
      const [x, y, width, height] = expectedItem.frame;
      const [actualX, actualY, actualWidth, actualHeight] = chromeItem.rect;
      for (const [label, actual, expected] of [
        ['x', actualX - rootX, x], ['y', actualY - rootY, y],
        ['width', actualWidth, width], ['height', actualHeight, height],
      ]) close(actual, expected, `${id}: Chrome item ${expectedItem.name} ${label}`);
      frames.push(`${index + 1}:node=${index + 2},x=${x},y=${y},width=${width},height=${height}`);
    }
    const axis = repeatAxis === 'columns' ? 'Columns' : 'Rows';
    let pixelSamples;
    if (id.endsWith('-area-larger')) {
      const rowAxis = repeatAxis === 'rows';
      const points = [
        ['root-interior-gap', 118, 10, rootColor],
        ['item-crosses-root-edge', 147, 10, '#2563eb'],
        ['overflow-item-outside-root', 160, 10, '#2563eb'],
        ['clear-outside-item', 171, 10, '#000000'],
      ];
      pixelSamples = points.map(([sampleID, x, y, color]) => ({
        id: sampleID,
        nodeId: sampleID === 'root-interior-gap' || sampleID === 'clear-outside-item'
          ? `c114-${id}-grid` : `c114-${id}-right`,
        xCssPx: rowAxis ? y : x,
        yCssPx: rowAxis ? x : y,
        rgba: colorRgba(color),
      }));
    }
    return {
      id,
      oracleId,
      axis,
      boxCount: expect.items.length + 1,
      rootFrame: `width=${expect.root[0]},height=${expect.root[1]}`,
      itemFrame: null,
      nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
      axes: {
        Columns: { tracks: expect.tracks.columns, collapsed: expect.collapsed.columns },
        Rows: { tracks: expect.tracks.rows, collapsed: expect.collapsed.rows },
      },
      gaps: { Columns: Number.parseFloat(columnGap), Rows: Number.parseFloat(rowGap) },
      ...(pixelSamples ? { pixelSamples } : {}),
    };
  });
}

scenes.push(...await templateAreaRepeatScenes());
scenes.push(...await autoRepeatNamedLineScenes());
scenes.push(...await autoRepeatCalcMinimumScenes());
scenes.push(...await autoRepeatPositivePercentMinimumScenes());
scenes.push(...await autoRepeatZeroPercentMinimumScenes());
scenes.push(...await autoRepeatPercentageMaximumScenes());
scenes.push(...await autoRepeatFixedMaximumScenes());
scenes.push(...await autoRepeatZeroFixedMaximumScenes());
scenes.push(...await autoRepeatPercentageNeighborScenes());
scenes.push(...await autoRepeatCalcNeighborScenes());
scenes.push(...await autoRepeatNegativePercentageNeighborScenes());
scenes.push(...await fixedRepeatAutoRepeatScenes());
scenes.push(...await fixedRepeatAutoRepeatScenes({
  fixtureRelativePath:
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/multiple_fixed_repeat_groups_scenes.json',
  chromeRelativePath:
    'spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-precomparison-2026-10-24/chrome156-observation.json',
  additionalChromeRelativePaths: [
    'spec/internal/evidence/c11-4-unbounded-fixed-repeat-groups-precomparison-2026-10-24/chrome156-observation.json',
  ],
  chromeInputRelativePath:
    'tests/fixtures/css/c11/grid-unbounded-fixed-repeat-auto-repeat-c114-precomparison.html',
  expectedCount: 12,
}));
scenes.push(...await fixedRepeatMinmaxFlexScenes());
scenes.push(...await fixedAutoRepeatAreaScenes());
const generalizedCyclicRowSamples = (sceneId) => [
  { id: 'generalized-prefix', nodeId: `c114-${sceneId}-prefix`,
    xCssPx: 12, yCssPx: 4, rgba: [47, 158, 68, 255] },
  { id: 'generalized-secondary-overlap', nodeId: `c114-${sceneId}-secondary`,
    xCssPx: 8, yCssPx: 15, rgba: [240, 140, 0, 255] },
  { id: 'generalized-primary-overlap', nodeId: `c114-${sceneId}-primary`,
    xCssPx: 20, yCssPx: 15, rgba: [229, 72, 77, 255] },
  { id: 'generalized-primary-overflow', nodeId: `c114-${sceneId}-primary`,
    xCssPx: 20, yCssPx: 33, rgba: [229, 72, 77, 255] },
  { id: 'generalized-root-background', nodeId: `c114-${sceneId}-grid`,
    xCssPx: 20, yCssPx: 40, rgba: [16, 24, 32, 255] },
];
const generalizedCyclicRowScene = (id, rows, collapsed) => ({
  id,
  axis: 'Rows',
  boxCount: 4,
  rootFrame: 'width=24,height=44',
  itemFrame: 'width=24,height=24',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=44;'
    + '1:node=2,x=0,y=0,width=24,height=8;'
    + '2:node=3,x=0,y=10,width=24,height=24;'
    + '3:node=4,x=0,y=10,width=16,height=16]',
  tracks: rows,
  collapsed,
  axes: {
    Columns: { tracks: [24], collapsed: [false] },
    Rows: { tracks: rows, collapsed },
  },
  gaps: { Columns: 0, Rows: 2 },
  pixelSamples: generalizedCyclicRowSamples(id),
});
scenes.push(
  {
    id: 'cyclic-max-content-auto-fill-rows',
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
    gaps: { Columns: 0, Rows: 0 },
  },
  {
    id: 'definite-40px-auto-fill-control-rows',
    axis: 'Rows',
    boxCount: 2,
    rootFrame: 'width=24,height=40',
    itemFrame: 'width=24,height=16',
    nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=40;'
      + '1:node=2,x=0,y=0,width=24,height=16]',
    tracks: [20, 20],
    collapsed: [false, false],
    axes: {
      Columns: { tracks: [24], collapsed: [false] },
      Rows: { tracks: [20, 20], collapsed: [false, false] },
    },
    gaps: { Columns: 0, Rows: 0 },
  },
  generalizedCyclicRowScene(
    'cyclic-max-content-auto-fill-prefix-suffix', [8, 11, 8], [false, false, false],
  ),
  generalizedCyclicRowScene(
    'cyclic-max-content-auto-fit-prefix-suffix', [8, 11, 8], [false, false, false],
  ),
  generalizedCyclicRowScene(
    'definite-44px-auto-fill-prefix-suffix-control',
    [8, 11, 11, 8], [false, false, false, false],
  ),
  generalizedCyclicRowScene(
    'definite-44px-auto-fit-prefix-suffix-control',
    [8, 11, 0, 8], [false, false, true, false],
  ),
);
scenes.push({
  id: 'image-auto-max-mixed-tracks',
  axis: 'Both',
  boxCount: 2,
  rootFrame: 'width=100,height=40',
  itemFrame: 'width=24,height=16',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=40;'
    + '1:node=2,x=0,y=0,width=24,height=16]',
  axes: {
    Columns: { tracks: [24, 17], collapsed: [false, false] },
    Rows: { tracks: [16], collapsed: [false] },
  },
  gaps: { Columns: 5, Rows: 0 },
});
scenes.push({
  id: 'image-auto-max-mixed-tracks-border-overflow',
  axis: 'Both',
  boxCount: 2,
  rootFrame: 'width=12,height=40',
  itemFrame: 'width=24,height=16',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=12,height=40;'
    + '1:node=2,x=1,y=1,width=24,height=16]',
  axes: {
    Columns: { tracks: [24, 17], collapsed: [false, false] },
    Rows: { tracks: [16], collapsed: [false] },
  },
  gaps: { Columns: 5, Rows: 0 },
  pixelSamples: [
    { id: 'image-auto-max-overflow-item-box',
      nodeId: 'c114-image-auto-max-border-overflow-item',
      xCssPx: 12, yCssPx: 8, rgba: [229, 72, 77, 255] },
    { id: 'image-auto-max-outside-root',
      nodeId: 'c114-image-auto-max-border-overflow-grid',
      xCssPx: 26, yCssPx: 8, rgba: [0, 0, 0, 255] },
    { id: 'image-auto-max-left-border',
      nodeId: 'c114-image-auto-max-border-overflow-grid',
      xCssPx: 0, yCssPx: 30, rgba: [34, 197, 94, 255] },
    { id: 'image-auto-max-right-border',
      nodeId: 'c114-image-auto-max-border-overflow-grid',
      xCssPx: 11, yCssPx: 30, rgba: [34, 197, 94, 255] },
  ],
});
scenes.push({
  id: 'fit-content-auto-neighbor-wpt',
  axis: 'Rows',
  boxCount: 5,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=10;'
    + '2:node=3,x=0,y=0,width=60,height=10;'
    + '3:node=4,x=0,y=10,width=60,height=290;'
    + '4:node=5,x=0,y=10,width=60,height=10]',
  tracks: [10, 290],
  collapsed: [false, false],
  requiresStrictPresentationIdentity: true,
  pixelSamples: [
    { id: 'fit-content-first-row-child',
      nodeId: 'c114-fit-content-auto-neighbor-fit-child',
      xCssPx: 30, yCssPx: 5, rgba: [255, 209, 102, 255] },
    { id: 'auto-neighbor-row-child',
      nodeId: 'c114-fit-content-auto-neighbor-auto-child',
      xCssPx: 30, yCssPx: 15, rgba: [6, 214, 160, 255] },
    { id: 'auto-neighbor-row-fill-top',
      nodeId: 'c114-fit-content-auto-neighbor-auto-item',
      xCssPx: 30, yCssPx: 20, rgba: [58, 134, 255, 255] },
    { id: 'auto-neighbor-row-fill-bottom',
      nodeId: 'c114-fit-content-auto-neighbor-auto-item',
      xCssPx: 30, yCssPx: 230, rgba: [58, 134, 255, 255] },
    { id: 'grid-root-right-edge-outside',
      nodeId: 'c114-fit-content-auto-neighbor-grid',
      xCssPx: 60, yCssPx: 20, rgba: [0, 0, 0, 255] },
  ],
});
scenes.push({
  id: 'fit-content-min-height-wpt',
  axis: 'Rows',
  boxCount: 3,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=10;'
    + '2:node=3,x=0,y=0,width=60,height=10]',
  tracks: [10],
  collapsed: [false],
  requiresStrictPresentationIdentity: true,
  fitContentMinHeight: {
    rootNode: 1,
    itemNode: 2,
    childNode: 3,
    rowBaseSize: 10,
  },
  pixelSamples: [
    { id: 'fit-content-min-height-child',
      nodeId: 'c114-fit-content-min-height-child',
      xCssPx: 30, yCssPx: 5, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-min-height-root-background',
      nodeId: 'c114-fit-content-min-height-grid',
      xCssPx: 30, yCssPx: 15, rgba: [23, 33, 43, 255] },
    { id: 'fit-content-min-height-outside-root',
      nodeId: 'c114-fit-content-min-height-grid',
      xCssPx: 60, yCssPx: 15, rgba: [0, 0, 0, 255] },
  ],
});
scenes.push({
  id: 'fit-content-zoomed-grid-wpt',
  axis: 'Rows',
  boxCount: 3,
  rootFrame: 'width=120,height=600',
  itemFrame: 'width=120,height=80',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=600;'
    + '1:node=2,x=0,y=0,width=120,height=80;'
    + '2:node=3,x=0,y=0,width=120,height=800]',
  tracks: [80],
  collapsed: [false],
  requiresStrictPresentationIdentity: true,
  fitContentZoomedGrid: {
    rootNode: 1,
    itemNode: 2,
    childNode: 3,
  },
  pixelSamples: [
    { id: 'fit-content-zoom-child-in-row',
      nodeId: 'c114-fit-content-zoomed-grid-child',
      xCssPx: 60, yCssPx: 40, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-zoom-child-past-item',
      nodeId: 'c114-fit-content-zoomed-grid-child',
      xCssPx: 60, yCssPx: 100, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-zoom-child-past-root',
      nodeId: 'c114-fit-content-zoomed-grid-child',
      xCssPx: 60, yCssPx: 610, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-zoom-outside-root',
      nodeId: 'c114-fit-content-zoomed-grid',
      xCssPx: 125, yCssPx: 610, rgba: [0, 0, 0, 255] },
  ],
});
scenes.push({
  id: 'fit-content-percentage-row-wpt',
  axis: 'Rows',
  boxCount: 3,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=150',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=150;'
    + '2:node=3,x=0,y=0,width=30,height=400]',
  tracks: [150],
  collapsed: [false],
  requiresStrictPresentationIdentity: true,
  fitContentPercentageArgument: {
    rootNode: 1,
    itemNode: 2,
    childNode: 3,
    rowSize: 150,
    gridAutoRows: 'fit-content(50%)',
    layoutAutoRows: 'TrackList([FitContent(Percentage(0.5))])',
  },
  pixelSamples: [
    { id: 'fit-content-percentage-child-in-row',
      nodeId: 'c114-fit-content-percentage-child',
      xCssPx: 15, yCssPx: 149, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-percentage-item-in-row',
      nodeId: 'c114-fit-content-percentage-item',
      xCssPx: 45, yCssPx: 149, rgba: [229, 72, 77, 255] },
    { id: 'fit-content-percentage-root-below-row',
      nodeId: 'c114-fit-content-percentage-grid',
      xCssPx: 45, yCssPx: 200, rgba: [23, 33, 43, 255] },
    { id: 'fit-content-percentage-child-outside-root',
      nodeId: 'c114-fit-content-percentage-child',
      xCssPx: 15, yCssPx: 350, rgba: [255, 209, 102, 255] },
  ],
});
scenes.push({
  id: 'fit-content-calc-percentage-row-wpt',
  axis: 'Rows',
  boxCount: 3,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=35',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=35;'
    + '2:node=3,x=0,y=0,width=30,height=400]',
  tracks: [35],
  collapsed: [false],
  requiresStrictPresentationIdentity: true,
  fitContentPercentageArgument: {
    rootNode: 1,
    itemNode: 2,
    childNode: 3,
    rowSize: 35,
    gridAutoRows: 'fit-content(calc(10% + 5px))',
    layoutAutoRowsIncludes: 'FitContent(Calc(',
    layoutCssMathExpression: 'Sum([Percentage(0.1), LengthPx(5.0)])',
  },
  pixelSamples: [
    { id: 'fit-content-calc-percentage-child-in-row',
      nodeId: 'c114-fit-content-calc-percentage-child',
      xCssPx: 15, yCssPx: 34, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-calc-percentage-item-in-row',
      nodeId: 'c114-fit-content-calc-percentage-item',
      xCssPx: 45, yCssPx: 34, rgba: [229, 72, 77, 255] },
    { id: 'fit-content-calc-percentage-root-below-row',
      nodeId: 'c114-fit-content-calc-percentage-grid',
      xCssPx: 45, yCssPx: 200, rgba: [23, 33, 43, 255] },
    { id: 'fit-content-calc-percentage-child-outside-root',
      nodeId: 'c114-fit-content-calc-percentage-child',
      xCssPx: 15, yCssPx: 350, rgba: [255, 209, 102, 255] },
  ],
});
scenes.push({
  id: 'fit-content-spanning-wpt',
  axis: 'Rows',
  boxCount: 3,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=80',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=80;'
    + '2:node=3,x=0,y=0,width=60,height=200]',
  tracks: [40, 40],
  collapsed: [false, false],
  requiresStrictPresentationIdentity: true,
  fitContentSpanning: {
    rootNode: 1,
    itemNode: 2,
    childNode: 3,
    expectedRows: [40, 40],
    expectedSpanMaxContent: 200,
  },
  pixelSamples: [
    { id: 'spanning-child-inside',
      nodeId: 'c114-fit-content-spanning-child',
      xCssPx: 30, yCssPx: 30, rgba: [255, 209, 102, 255] },
    { id: 'spanning-child-over-item',
      nodeId: 'c114-fit-content-spanning-child',
      xCssPx: 30, yCssPx: 80, rgba: [255, 209, 102, 255] },
    { id: 'spanning-root-background-after-child',
      nodeId: 'c114-fit-content-spanning-grid',
      xCssPx: 30, yCssPx: 200, rgba: [23, 33, 43, 255] },
    { id: 'spanning-outside-root',
      nodeId: 'c114-fit-content-spanning-grid',
      xCssPx: 60, yCssPx: 120, rgba: [0, 0, 0, 255] },
  ],
});
for (const boundary of [
  { id: 'fit-content-limit-40-wpt', limit: 40, row: 40, overflowSampleY: 60 },
  { id: 'fit-content-limit-30-wpt', limit: 30, row: 30, overflowSampleY: 50 },
  { id: 'fit-content-limit-200-wpt', limit: 200, row: 100, overflowSampleY: null },
]) {
  const suffix = boundary.id.replace('fit-content-limit-', '').replace('-wpt', '');
  const rootId = `c114-fit-content-limit-${suffix}-grid`;
  const itemId = `c114-fit-content-limit-${suffix}-item`;
  const childId = `c114-fit-content-limit-${suffix}-child`;
  scenes.push({
    id: boundary.id,
    axis: 'Rows',
    boxCount: 3,
    rootFrame: 'width=60,height=300',
    itemFrame: `width=60,height=${boundary.row}`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;`
      + `1:node=2,x=0,y=0,width=60,height=${boundary.row};`
      + '2:node=3,x=0,y=0,width=60,height=100]',
    tracks: [boundary.row],
    collapsed: [false],
    requiresStrictPresentationIdentity: true,
    fitContentLimitBoundaries: {
      rootNode: 1,
      itemNode: 2,
      childNode: 3,
      limit: boundary.limit,
      expectedRow: boundary.row,
      expectedMaxContent: 100,
    },
    pixelSamples: [
      { id: `${boundary.id}-child-in-row`, nodeId: childId,
        xCssPx: 30, yCssPx: 10, rgba: [255, 209, 102, 255] },
      ...(boundary.overflowSampleY === null ? [] : [{
        id: `${boundary.id}-child-past-item`, nodeId: childId,
        xCssPx: 30, yCssPx: boundary.overflowSampleY, rgba: [255, 209, 102, 255],
      }]),
      { id: `${boundary.id}-root-after-child`, nodeId: rootId,
        xCssPx: 30, yCssPx: 120, rgba: [23, 33, 43, 255] },
      { id: `${boundary.id}-outside-root`, nodeId: rootId,
        xCssPx: 60, yCssPx: 120, rgba: [0, 0, 0, 255] },
    ],
  });
}
for (const boundary of [
  { id: 'column-fit-content-40', limit: 40, rootWidth: 60, column: 40, minimum: 0 },
  { id: 'column-fit-content-30', limit: 30, rootWidth: 60, column: 30, minimum: 0 },
  { id: 'column-fit-content-200', limit: 200, rootWidth: 300, column: 100, minimum: 0 },
  { id: 'column-fit-content-auto-minimum-control', limit: 40, rootWidth: 60,
    column: 100, minimum: 100 },
]) {
  const suffix = boundary.id.replace('column-fit-content-', '')
    .replace('auto-minimum-control', 'auto-minimum');
  const rootId = `c114-fit-content-column-limit-${suffix}-grid`;
  const childId = `c114-fit-content-column-limit-${suffix}-child`;
  const rootBackgroundX = boundary.rootWidth > 100 ? 120 : 30;
  scenes.push({
    id: boundary.id,
    axis: 'Columns',
    boxCount: 3,
    rootFrame: `width=${boundary.rootWidth},height=60`,
    itemFrame: `width=${boundary.column},height=10`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${boundary.rootWidth},height=60;`
      + `1:node=2,x=0,y=0,width=${boundary.column},height=10;`
      + '2:node=3,x=0,y=0,width=100,height=10]',
    tracks: [boundary.column],
    collapsed: [false],
    requiresStrictPresentationIdentity: true,
    fitContentLimitColumns: {
      rootNode: 1,
      itemNode: 2,
      childNode: 3,
      limit: boundary.limit,
      expectedMinimum: boundary.minimum,
      expectedMaxContent: 100,
      expectedColumn: boundary.column,
      automaticMinimum: boundary.minimum > 0,
      rootWidth: boundary.rootWidth,
    },
    pixelSamples: [
      { id: `${boundary.id}-child-inside`, nodeId: childId,
        xCssPx: 10, yCssPx: 5, rgba: [255, 209, 102, 255] },
      ...(boundary.column < 100 ? [{
        id: `${boundary.id}-child-overflows-item`, nodeId: childId,
        xCssPx: 80, yCssPx: 5, rgba: [255, 209, 102, 255],
      }] : []),
      { id: `${boundary.id}-root-background`, nodeId: rootId,
        xCssPx: rootBackgroundX, yCssPx: 30, rgba: [23, 33, 43, 255] },
      { id: `${boundary.id}-outside-root`, nodeId: rootId,
        xCssPx: boundary.rootWidth, yCssPx: 30, rgba: [0, 0, 0, 255] },
    ],
  });
}
scenes.push({
  id: 'fit-content-auto-min-floor-wpt',
  axis: 'Rows',
  boxCount: 5,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=10;'
    + '2:node=3,x=0,y=0,width=60,height=10;'
    + '3:node=4,x=0,y=10,width=60,height=100;'
    + '4:node=5,x=0,y=10,width=60,height=100]',
  tracks: [10, 100],
  collapsed: [false, false],
  requiresStrictPresentationIdentity: true,
  fitContentAutoMinFloor: {
    expectedRows: [10, 100],
    intrinsicRows: [10, 100],
    secondItemMinHeight: 'auto',
    rootNode: 1,
    nodeIDs: [1, 2, 3, 4, 5],
  },
  pixelSamples: [
    { id: 'fit-content-auto-min-floor-first-child',
      nodeId: 'c114-fit-content-auto-min-floor-wpt-first-child',
      xCssPx: 30, yCssPx: 5, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-auto-min-floor-second-child',
      nodeId: 'c114-fit-content-auto-min-floor-wpt-second-child',
      xCssPx: 30, yCssPx: 60, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-auto-min-floor-root-background',
      nodeId: 'c114-fit-content-auto-min-floor-wpt-grid',
      xCssPx: 30, yCssPx: 120, rgba: [23, 33, 43, 255] },
    { id: 'fit-content-auto-min-floor-outside-root',
      nodeId: 'c114-fit-content-auto-min-floor-wpt-grid',
      xCssPx: 60, yCssPx: 20, rgba: [0, 0, 0, 255] },
  ],
});
scenes.push({
  id: 'fit-content-auto-min-floor-zero-control',
  axis: 'Rows',
  boxCount: 5,
  rootFrame: 'width=60,height=300',
  itemFrame: 'width=60,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=60,height=300;'
    + '1:node=2,x=0,y=0,width=60,height=10;'
    + '2:node=3,x=0,y=0,width=60,height=10;'
    + '3:node=4,x=0,y=10,width=60,height=40;'
    + '4:node=5,x=0,y=10,width=60,height=100]',
  tracks: [10, 40],
  collapsed: [false, false],
  requiresStrictPresentationIdentity: true,
  fitContentAutoMinFloor: {
    expectedRows: [10, 40],
    intrinsicRows: [10, 0],
    secondItemMinHeight: '0px',
    rootNode: 1,
    nodeIDs: [1, 2, 3, 4, 5],
  },
  pixelSamples: [
    { id: 'fit-content-auto-min-floor-control-first-child',
      nodeId: 'c114-fit-content-auto-min-floor-zero-first-child',
      xCssPx: 30, yCssPx: 5, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-auto-min-floor-control-second-row',
      nodeId: 'c114-fit-content-auto-min-floor-zero-second-child',
      xCssPx: 30, yCssPx: 25, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-auto-min-floor-control-overflow',
      nodeId: 'c114-fit-content-auto-min-floor-zero-second-child',
      xCssPx: 30, yCssPx: 75, rgba: [255, 209, 102, 255] },
    { id: 'fit-content-auto-min-floor-control-root-background',
      nodeId: 'c114-fit-content-auto-min-floor-zero-grid',
      xCssPx: 30, yCssPx: 120, rgba: [23, 33, 43, 255] },
    { id: 'fit-content-auto-min-floor-control-outside-root',
      nodeId: 'c114-fit-content-auto-min-floor-zero-grid',
      xCssPx: 60, yCssPx: 20, rgba: [0, 0, 0, 255] },
  ],
});
scenes.push({
  id: 'align-content-normal-equal-auto-rows',
  axis: 'Rows',
  axes: {
    Rows: { tracks: [32.5, 42.5, 5], collapsed: [false, false, false] },
  },
  gaps: { Rows: 10 },
  boxCount: 4,
  rootFrame: 'width=160,height=100',
  itemFrame: 'width=160,height=10',
  nodeFrames: 'node_frames_css_px=[0:node=1,x=0,y=0,width=160,height=100;'
    + '1:node=2,x=0,y=0,width=160,height=10;'
    + '2:node=3,x=0,y=42.5,width=160,height=20;'
    + '3:node=4,x=0,y=95,width=160,height=5]',
  geometryTolerance: 1e-4,
  requiresStrictPresentationIdentity: true,
  equalAutoRowsStretch: true,
  pixelSamples: [
    { id: 'equal-auto-rows-first-row', nodeId: 'c114-align-content-normal-item-first',
      xCssPx: 80, yCssPx: 5, rgba: [229, 72, 77, 255] },
    { id: 'equal-auto-rows-first-gutter', nodeId: 'c114-align-content-normal-grid',
      xCssPx: 80, yCssPx: 37, rgba: [16, 24, 32, 255] },
    { id: 'equal-auto-rows-second-row', nodeId: 'c114-align-content-normal-item-second',
      xCssPx: 80, yCssPx: 52, rgba: [58, 134, 255, 255] },
    { id: 'equal-auto-rows-fixed-row', nodeId: 'c114-align-content-normal-item-fixed',
      xCssPx: 80, yCssPx: 97, rgba: [6, 214, 160, 255] },
    { id: 'equal-auto-rows-outside-root', nodeId: 'c114-align-content-normal-grid',
      xCssPx: 160, yCssPx: 5, rgba: [0, 0, 0, 255] },
  ],
});
function intrinsicAutoRepeatRecomputeScenes() {
  const expected = JSON.parse(readFileSync(resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-grid-change-intrinsic-auto-repeat-precomparison-2026-10-27/expected.json'), 'utf8'));
  assert.equal(expected.states.length, 8, 'C11.4 intrinsic auto-repeat oracle must have eight states.');
  return expected.states.map((state) => {
    const frames = [
      `0:node=1,x=0,y=0,width=${state.rootBorderBox[0]},height=${state.rootBorderBox[1]}`,
      `1:node=2,x=${state.item[0] + 2},y=${state.item[1] + 2},width=${state.item[2]},height=${state.item[3]}`,
      ...['a1', 'a2', 'a3', 'a4'].map((probe, index) => {
        const [x, y, width, height] = state.probes[probe];
        return `${index + 2}:node=${index + 3},x=${x + 2},y=${y + 2},width=${width},height=${height}`;
      }),
    ];
    const tracks = state.columns;
    return {
      id: `intrinsic-auto-repeat-${state.id}`,
      axis: 'Columns',
      boxCount: 2 + ['a1', 'a2', 'a3', 'a4']
        .filter((probe) => state.probes[probe][2] > 0).length,
      rootFrame: `width=${state.rootBorderBox[0]},height=${state.rootBorderBox[1]}`,
      itemFrame: `width=${state.item[2]},height=${state.item[3]}`,
      nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
      axes: {
        Columns: { tracks, collapsed: tracks.map(() => false) },
      },
      gaps: { Columns: 0 },
      geometryTolerance: 1 / 64,
      intrinsicAutoRepeatRecompute: {
        repeatCount: state.repeatCount,
        suffixTrackCount: 1,
        usedColumns: tracks,
      },
    };
  });
}

function gridAutoRepeatIntrinsicWptScenes() {
  const expected = JSON.parse(readFileSync(resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/expected.json'), 'utf8'));
  assert.equal(expected.cases.length, 8, 'grid-auto-repeat-intrinsic WPT oracle must have eight cases.');
  return expected.cases.map((state) => {
    const probeFrames = state.probeFramesPaddingEdgePx;
    const frames = [
      `0:node=1,x=0,y=0,width=${state.rootBorderBoxPx[0]},height=${state.rootBorderBoxPx[1]}`,
      `1:node=2,x=${state.itemFramePaddingEdgePx[0] + 2},y=${state.itemFramePaddingEdgePx[1] + 2},width=${state.itemFramePaddingEdgePx[2]},height=${state.itemFramePaddingEdgePx[3]}`,
      ...probeFrames.map(([x, y, width, height], index) => (
        `${index + 2}:node=${index + 3},x=${x + 2},y=${y + 2},width=${width},height=${height}`
      )),
    ];
    const overflowing = Boolean(state.surfaceChecks);
    const rootNodeId = `c114-${state.sceneId}-grid`;
    const pixelSamples = overflowing ? [
      { id: `${state.sceneId}-right-border`, nodeId: rootNodeId,
        xCssPx: state.surfaceChecks.borderRightPixelIndex,
        yCssPx: state.surfaceChecks.sampleYRelativeToRootBorderBox, rgba: [0, 255, 255, 255] },
      { id: `${state.sceneId}-overflow-item`, nodeId: rootNodeId,
        xCssPx: state.surfaceChecks.overflowItemPixelIndex,
        yCssPx: state.surfaceChecks.sampleYRelativeToRootBorderBox, rgba: [0, 255, 255, 255] },
      { id: `${state.sceneId}-past-item`, nodeId: rootNodeId,
        xCssPx: state.surfaceChecks.pastItemPixelIndex,
        yCssPx: state.surfaceChecks.sampleYRelativeToRootBorderBox, rgba: [0, 0, 0, 255] },
    ] : undefined;
    return {
      id: state.sceneId,
      axis: 'Columns',
      boxCount: 2 + probeFrames.filter(([, , width]) => width > 0).length,
      rootFrame: `width=${state.rootBorderBoxPx[0]},height=${state.rootBorderBoxPx[1]}`,
      itemFrame: `width=${state.itemFramePaddingEdgePx[2]},height=${state.itemFramePaddingEdgePx[3]}`,
      nodeFrames: `node_frames_css_px=[${frames.join(';')}]`,
      axes: {
        Columns: { tracks: state.usedColumnsPx, collapsed: state.usedColumnsPx.map(() => false) },
      },
      gaps: { Columns: 0 },
      geometryTolerance: 1 / 64,
      gridAutoRepeatIntrinsicWpt: {
        repeatCount: state.repeatCount,
        suffixTrackCount: 1,
        usedColumns: state.usedColumnsPx,
        columnContributionSpan: state.usedColumnsPx.length,
        columnContributionKinds: state.trailingTrack === 'minmax(60px,auto)'
          ? ['Minimum', 'MinContent', 'MaxContent']
          : ['Minimum', 'MinContent'],
      },
      ...(pixelSamples ? { pixelSamples } : {}),
    };
  });
}

for (const overflow of [
  { id: 'visible-content-minimum', x: 'visible', y: 'visible', minimum: 100,
    contentBased: true, tracks: [50, 50], itemWidth: 100 },
  { id: 'clip-content-minimum', x: 'clip', y: 'visible', minimum: 100,
    contentBased: true, tracks: [50, 50], itemWidth: 100 },
  { id: 'hidden-zero-minimum', x: 'hidden', y: 'auto', minimum: 0,
    contentBased: false, tracks: [40, 40], itemWidth: 80 },
  { id: 'auto-zero-minimum', x: 'auto', y: 'auto', minimum: 0,
    contentBased: false, tracks: [40, 40], itemWidth: 80 },
  { id: 'scroll-zero-minimum', x: 'scroll', y: 'auto', minimum: 0,
    contentBased: false, tracks: [40, 40], itemWidth: 80 },
]) {
  const contentBasedColor = [229, 72, 77, 255];
  scenes.push({
    id: `columns-span2-${overflow.id}`,
    axis: 'Both',
    boxCount: 2,
    rootFrame: 'width=80,height=20',
    itemFrame: `width=${overflow.itemWidth},height=20`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=80,height=20;`
      + `1:node=2,x=0,y=0,width=${overflow.itemWidth},height=20]`,
    axes: {
      Columns: { tracks: overflow.tracks, collapsed: [false, false] },
      Rows: { tracks: [20], collapsed: [false] },
    },
    gaps: { Columns: 0, Rows: 0 },
    overflowAutoMinimum: overflow,
    pixelSamples: [
      { id: `${overflow.id}-inside-root-edge`,
        nodeId: `c114-auto-minmax-span-overflow-${overflow.x}`,
        xCssPx: 79, yCssPx: 18, rgba: contentBasedColor },
      { id: `${overflow.id}-outside-root-edge`,
        nodeId: `c114-auto-minmax-span-overflow-${overflow.x}`,
        xCssPx: 90, yCssPx: 18, rgba: overflow.contentBased ? contentBasedColor : [0, 0, 0, 255] },
    ],
  });
}

for (const spec of [
  { id: 'column-visible-content-minimum', axis: 'Columns', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 100, minimum: 100, minimumSource: 'ContentBased' },
  { id: 'column-clip-content-minimum', axis: 'Columns', authoredOverflowX: 'clip', authoredOverflowY: 'visible', computedOverflowX: 'clip', computedOverflowY: 'visible',
    track: 100, minimum: 100, minimumSource: 'ContentBased' },
  { id: 'column-hidden-zero-minimum', axis: 'Columns', authoredOverflowX: 'hidden', authoredOverflowY: 'visible', computedOverflowX: 'hidden', computedOverflowY: 'auto',
    track: 60, minimum: 0, minimumSource: 'Overflow' },
  { id: 'column-auto-zero-minimum', axis: 'Columns', authoredOverflowX: 'auto', authoredOverflowY: 'visible', computedOverflowX: 'auto', computedOverflowY: 'auto',
    track: 60, minimum: 0, minimumSource: 'Overflow' },
  { id: 'column-scroll-zero-minimum', axis: 'Columns', authoredOverflowX: 'scroll', authoredOverflowY: 'visible', computedOverflowX: 'scroll', computedOverflowY: 'auto',
    track: 60, minimum: 0, minimumSource: 'Overflow' },
  { id: 'column-specified-minimum-control', axis: 'Columns', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 60, minimum: 0, minimumSource: 'Specified' },
  { id: 'column-zero-track-control', axis: 'Columns', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 60, minimum: null, minimumSource: 'TrackMinimum' },
  { id: 'row-visible-content-minimum', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 10, minimum: 10, minimumSource: 'ContentBased', image: true },
  { id: 'row-clip-content-minimum', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'clip', computedOverflowX: 'visible', computedOverflowY: 'clip',
    track: 10, minimum: 10, minimumSource: 'ContentBased', image: true },
  { id: 'row-hidden-zero-minimum', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'hidden', computedOverflowX: 'auto', computedOverflowY: 'hidden',
    track: 5, minimum: 0, minimumSource: 'Overflow', image: true },
  { id: 'row-auto-zero-minimum', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'auto', computedOverflowX: 'auto', computedOverflowY: 'auto',
    track: 5, minimum: 0, minimumSource: 'Overflow', image: true },
  { id: 'row-scroll-zero-minimum', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'scroll', computedOverflowX: 'auto', computedOverflowY: 'scroll',
    track: 5, minimum: 0, minimumSource: 'Overflow', image: true },
  { id: 'row-specified-minimum-control', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 5, minimum: 0, minimumSource: 'Specified', image: true },
  { id: 'row-zero-track-control', axis: 'Rows', authoredOverflowX: 'visible', authoredOverflowY: 'visible', computedOverflowX: 'visible', computedOverflowY: 'visible',
    track: 5, minimum: null, minimumSource: 'TrackMinimum', image: true },
]) {
  const isColumns = spec.axis === 'Columns';
  const rootWidth = isColumns ? 60 : 30;
  const rootHeight = isColumns ? 20 : 5;
  const itemWidth = isColumns ? spec.track : 10;
  const itemHeight = isColumns ? 20 : spec.track;
  const rootId = `c114-auto-min-scrollable-overflow-${spec.id}`;
  const pixelSamples = spec.id === 'column-visible-content-minimum'
    ? [
      { id: 'column-visible-outside-root', nodeId: rootId, xCssPx: 70, yCssPx: 10, rgba: [229, 72, 77, 255] },
      { id: 'column-visible-after-item', nodeId: rootId, xCssPx: 101, yCssPx: 10, rgba: [0, 0, 0, 255] },
    ]
    : spec.id === 'row-visible-content-minimum'
      ? [
        { id: 'row-visible-outside-root', nodeId: rootId, xCssPx: 5, yCssPx: 7, rgba: [229, 72, 77, 255] },
        { id: 'row-visible-after-item', nodeId: rootId, xCssPx: 5, yCssPx: 11, rgba: [0, 0, 0, 255] },
      ]
      : undefined;
  scenes.push({
    id: spec.id,
    axis: spec.axis,
    boxCount: 2,
    rootFrame: `width=${rootWidth},height=${rootHeight}`,
    itemFrame: `width=${itemWidth},height=${itemHeight}`,
    nodeFrames: `node_frames_css_px=[0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight};`
      + `1:node=2,x=0,y=0,width=${itemWidth},height=${itemHeight}]`,
    axes: isColumns
      ? { Columns: { tracks: [spec.track], collapsed: [false] }, Rows: { tracks: [20], collapsed: [false] } }
      : { Columns: { tracks: [10], collapsed: [false] }, Rows: { tracks: [spec.track], collapsed: [false] } },
    gaps: { Columns: 0, Rows: 0 },
    geometryTolerance: 1 / 64,
    requiresStrictPresentationIdentity: true,
    ...(spec.image ? { runtimeImageCount: 1 } : {}),
    ...(pixelSamples ? { pixelSamples } : {}),
    singleTrackOverflowAutoMinimum: {
      axis: spec.axis,
      authoredOverflowX: spec.authoredOverflowX,
      authoredOverflowY: spec.authoredOverflowY,
      computedOverflowX: spec.computedOverflowX,
      computedOverflowY: spec.computedOverflowY,
      track: spec.track,
      minimum: spec.minimum,
      minimumSource: spec.minimumSource,
      itemWidth,
      itemHeight,
      rootWidth,
      rootHeight,
    },
  });
}

scenes.push(...intrinsicAutoRepeatRecomputeScenes());
scenes.push(...gridAutoRepeatIntrinsicWptScenes());
assert.equal(scenes.filter(({ intrinsicAutoRepeatRecompute }) => intrinsicAutoRepeatRecompute).length,
  8, 'C11.4 intrinsic auto-repeat matrix must add eight scenes.');
assert.equal(scenes.filter(({ gridAutoRepeatIntrinsicWpt }) => gridAutoRepeatIntrinsicWpt).length,
  8, 'grid-auto-repeat-intrinsic WPT matrix must add eight scenes.');
assert.equal(scenes.length, 622, 'C11.4 fixture scene count differs.');
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
if (runKind === 'fixed-repeat-minmax-flex') {
  assert.ok(runScenes.every(({ id }) => id.startsWith('fixed-repeat-minmax-flex-')),
    'fixed-repeat-minmax-flex run에는 해당 scene만 선택해야 합니다.');
}
if (requestedSceneIDs?.some((sceneID) => overflowAutoMinimumSceneIDs.has(sceneID))) {
  assert.equal(runKind, 'overflow-auto-minimum',
    'overflow automatic-minimum scenes need their dedicated evidence run-kind.');
  assert.ok(runScenes.every(({ id }) => overflowAutoMinimumSceneIDs.has(id)),
    'overflow automatic-minimum run에는 고정 span/single-track scene만 선택해야 합니다.');
}
if (requestedRunKind === 'zero-fixed-maximum'
    || requestedSceneIDs?.some((sceneID) => zeroFixedMaximumSceneIDs.has(sceneID))) {
  assert.ok(requestedSceneIDs?.length > 0,
    'zero fixed-maximum run requires an explicit scene selection.');
  assert.equal(runKind, 'zero-fixed-maximum',
    'zero fixed-maximum scenes need their dedicated evidence run-kind.');
  assert.ok(runScenes.every(({ id }) => zeroFixedMaximumSceneIDs.has(id)),
    'zero fixed-maximum run에는 해당 auto-repeat scene만 선택해야 합니다.');
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
    const rejectionSummary = latest.includes(
      `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `);
    const traceComplete = c114TraceChunkState(latest, runID).complete;
    const draw = latest.includes('SPINON_C0410_DRAW')
      && latest.includes(`presented boxes=${scene.boxCount}`);
    const pixelSummary = !scene.pixelSamples
      || (target === 'ios-simulator'
        ? latest.includes(`SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`)
        : latest.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`));
    if ((summary && traceComplete && draw && pixelSummary)
        || (scene.expectedRejection && rejectionSummary)
        || latest.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
        || latest.includes(`SPINON_C114_GRID_ENVIRONMENT_RESULT run_id=${runID} scene_id=${scene.id} strict_matches=false`)
        || latest.includes(`SPINON_C114_GRID_PRESENTATION_MATCH run_id=${runID} matched=false`)
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
  if (scene.expectedRejection) {
    const { status, code, property, nodeId } = scene.expectedRejection;
    const expectedSummary = `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `
      + `expected=true status=${status} code=${code} property=${property} node_id=${nodeId} frames=absent`;
    assert.ok(log.includes(expectedSummary),
      `${scene.id}: expected rejection summary is missing or differs.\n${log}`);
    assert.ok(!log.includes(`SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `),
      `${scene.id}: rejected scene unexpectedly reached a layout summary.`);
    assert.ok(!log.includes(`SPINON_C114_GRID_NODE_FRAME run_id=${runID}`),
      `${scene.id}: rejected scene emitted node frames.`);
    return { id: scene.id, status: 'expected-rejection', code, property, nodeId };
  }
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  const assertionError = log.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID} `));
  assert.ok(!assertionError,
    `${scene.id}: native runtime assertion failed: ${assertionError ?? ''}`);
  assert.ok(log.includes(summary), `${scene.id}: runtime summary가 없습니다.`);
  assert.ok(log.includes(`layout=ready boxes=${scene.boxCount}`),
    `${scene.id}: visible box 수가 예상과 다릅니다.`);
  const geometryTolerance = scene.geometryTolerance ?? 1 / 64;
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
      assert.ok(Math.abs(actual[property] - expected[property]) <= geometryTolerance,
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
  if (scene.requiresStrictPresentationIdentity) {
    assert.ok(log.includes(`SPINON_C114_GRID_ENVIRONMENT_RESULT run_id=${runID} `)
      && log.includes('strict_matches=true'),
    `${scene.id}: surface 환경 revision이 strict validation을 통과하지 않았습니다.`);
    assert.ok(log.includes(`SPINON_C114_GRID_PRESENTATION_MATCH run_id=${runID} matched=true `),
      `${scene.id}: draw revision이 surface 환경 revision과 일치하지 않습니다.`);
  }
  if (scene.runtimeImageCount !== undefined) {
    assert.ok(log.split(/\r?\n/).some((line) => line.includes('SPINON_C0410_DRAW')
      && line.includes(`runtime_image_count=${scene.runtimeImageCount} image_draw_count=${scene.runtimeImageCount}`)),
    `${scene.id}: 고정 PNG 전체가 runtime scene과 실제 draw에 포함되지 않았습니다.`);
  }
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: 실행 인자가 거부됐습니다.`);
  const tracePayload = traceState.parts.map(({ payload }) => payload).join('')
    .replace(/^grid_track_sizing_diagnostics=/, '');
  const diagnostics = JSON.parse(tracePayload);
  if (scene.overflowAutoMinimum) {
    const expected = scene.overflowAutoMinimum;
    const observation = diagnostics.overflowAutomaticMinimum;
    assert.ok(observation, `${scene.id}: overflow automatic-minimum observation is missing`);
    assert.deepEqual({
      sceneId: observation.sceneId,
      authoredOverflowX: observation.authoredOverflowX,
      computedOverflowX: observation.computedOverflowX,
      computedOverflowY: observation.computedOverflowY,
      actualComputedOverflowX: observation.actualComputedOverflowX,
      actualComputedOverflowY: observation.actualComputedOverflowY,
      gridRoot: observation.gridRoot,
      item: observation.item,
      placementMode: observation.placementMode,
      expectedMinimumValue: observation.expectedMinimumValue,
      expectsContentBasedMinimum: observation.expectsContentBasedMinimum,
      expectedItemWidth: observation.expectedItemWidth,
    }, {
      sceneId: scene.id,
      authoredOverflowX: expected.x,
      computedOverflowX: expected.x,
      computedOverflowY: expected.y,
      actualComputedOverflowX: expected.x,
      actualComputedOverflowY: expected.y,
      gridRoot: 1,
      item: 2,
      placementMode: 'TrackSizing',
      expectedMinimumValue: expected.minimum,
      expectsContentBasedMinimum: expected.contentBased,
      expectedItemWidth: expected.itemWidth,
    }, `${scene.id}: overflow style/minimum metadata differs`);
    assert.match(observation.layoutInputColumns, /MinMax/,
      `${scene.id}: fixed MinMax LayoutInput is missing`);
    assert.equal(observation.layoutInputRows, '[LengthPx(20.0)]',
      `${scene.id}: fixed Rows LayoutInput differs`);
    assert.deepEqual(observation.expectedTrackBaseSizes, expected.tracks,
      `${scene.id}: expected track base sizes differ`);
    assert.deepEqual(observation.actualTrackBaseSizes, expected.tracks,
      `${scene.id}: actual track base sizes differ`);
    assert.equal(observation.actualItemWidth, expected.itemWidth,
      `${scene.id}: actual item width differs`);
    assert.deepEqual(observation.rootFrame, { x: 0, y: 0, width: 80, height: 20 },
      `${scene.id}: Grid root frame differs`);
    assert.deepEqual(observation.itemFrame, {
      x: 0, y: 0, width: expected.itemWidth, height: 20,
    }, `${scene.id}: Grid item frame differs`);
    const [trace] = diagnostics.trackSizingTraces;
    assert.equal(diagnostics.trackSizingTraces.length, 1,
      `${scene.id}: expected one fixed Grid trace`);
    assert.equal(trace.grid, 1, `${scene.id}: Grid trace must belong to root node`);
    assert.deepEqual(trace.passes.map(({ axis }) => axis), ['Columns', 'Rows'],
      `${scene.id}: expected Columns and Rows passes`);
    const contributions = trace.passes[0].contributions.filter(({ item, span }) => (
      item === 2 && span === 2
    ));
    const [minimum] = contributions.filter(({ kind }) => kind === 'Minimum');
    assert.ok(minimum, `${scene.id}: span-2 Minimum contribution is missing`);
    assert.equal(minimum.value, expected.minimum,
      `${scene.id}: Minimum contribution value differs`);
    const maxContent = contributions.filter(({ kind }) => kind === 'MaxContent');
    assert.equal(maxContent.length, 1, `${scene.id}: expected one MaxContent contribution`);
    assert.equal(maxContent[0].value, 100,
      `${scene.id}: content MaxContent contribution must remain 100px`);
    if (expected.contentBased) {
      assert.match(minimum.minimumSource, /AutomaticMinimum/);
      assert.match(minimum.minimumSource, /spans_auto_min_track: true/);
      assert.match(minimum.minimumSource, /spans_multiple_tracks: true/);
      assert.match(minimum.minimumSource, /spans_flexible_max_track: false/);
      assert.match(minimum.minimumSource, /content_based: true/);
    } else {
      assert.equal(minimum.minimumSource, 'OverflowAutomaticMinimum');
    }
  }
  if (scene.singleTrackOverflowAutoMinimum) {
    const expected = scene.singleTrackOverflowAutoMinimum;
    const observation = diagnostics.singleTrackOverflowAutoMinimum;
    assert.ok(observation, `${scene.id}: single-track overflow automatic-minimum observation is missing`);
    assert.deepEqual({
      sceneId: observation.sceneId,
      axis: observation.axis,
      authoredOverflowX: observation.authoredOverflowX,
      authoredOverflowY: observation.authoredOverflowY,
      computedOverflowX: observation.computedOverflowX,
      computedOverflowY: observation.computedOverflowY,
      actualComputedOverflowX: observation.actualComputedOverflowX,
      actualComputedOverflowY: observation.actualComputedOverflowY,
      gridRoot: observation.gridRoot,
      item: observation.item,
      placementMode: observation.placementMode,
      expectedMinimumValue: observation.expectedMinimumValue,
      expectedMinimumSource: observation.expectedMinimumSource,
      expectedTrackBaseSize: observation.expectedTrackBaseSize,
      actualTrackBaseSize: observation.actualTrackBaseSize,
      expectedItemWidth: observation.expectedItemWidth,
      actualItemWidth: observation.actualItemWidth,
      expectedItemHeight: observation.expectedItemHeight,
      actualItemHeight: observation.actualItemHeight,
    }, {
      sceneId: scene.id,
      axis: expected.axis,
      authoredOverflowX: expected.authoredOverflowX,
      authoredOverflowY: expected.authoredOverflowY,
      computedOverflowX: expected.computedOverflowX,
      computedOverflowY: expected.computedOverflowY,
      actualComputedOverflowX: expected.computedOverflowX,
      actualComputedOverflowY: expected.computedOverflowY,
      gridRoot: 1,
      item: 2,
      placementMode: 'TrackSizing',
      expectedMinimumValue: expected.minimum,
      expectedMinimumSource: expected.minimumSource,
      expectedTrackBaseSize: expected.track,
      actualTrackBaseSize: expected.track,
      expectedItemWidth: expected.itemWidth,
      actualItemWidth: expected.itemWidth,
      expectedItemHeight: expected.itemHeight,
      actualItemHeight: expected.itemHeight,
    }, `${scene.id}: single-track overflow style/geometry metadata differs`);
    assert.equal(observation.layoutInputColumns.startsWith('['), true,
      `${scene.id}: typed Columns input is missing`);
    assert.equal(observation.layoutInputRows.startsWith('['), true,
      `${scene.id}: typed Rows input is missing`);
    if (expected.axis === 'Columns') {
      assert.match(observation.layoutInputColumns, /MinMax/);
      assert.equal(observation.layoutInputRows, '[LengthPx(20.0)]');
    } else {
      assert.equal(observation.layoutInputColumns, '[LengthPx(10.0)]');
      assert.match(observation.layoutInputRows, /MinMax/);
    }
    assert.deepEqual(observation.rootFrame,
      { x: 0, y: 0, width: expected.rootWidth, height: expected.rootHeight },
      `${scene.id}: Grid root frame differs`);
    assert.deepEqual(observation.itemFrame,
      { x: 0, y: 0, width: expected.itemWidth, height: expected.itemHeight },
      `${scene.id}: Grid item frame differs`);
    const [trace] = diagnostics.trackSizingTraces;
    assert.equal(diagnostics.trackSizingTraces.length, 1,
      `${scene.id}: expected one fixed Grid trace`);
    assert.ok(trace.passes.length >= 2 && trace.passes[0].axis === 'Columns'
      && trace.passes.some(({ axis }) => axis === 'Columns')
      && trace.passes.some(({ axis }) => axis === 'Rows'),
    `${scene.id}: expected an ordered trace with Columns and Rows passes`);
    for (const [passIndex, pass] of trace.passes.entries()) {
      assert.equal(pass.sequence, passIndex, `${scene.id}: trace pass sequence differs`);
      assert.ok(['Columns', 'Rows'].includes(pass.axis),
        `${scene.id}: unexpected track-sizing axis ${pass.axis}`);
      const targetPass = pass.axis === expected.axis;
      const expectedTrack = targetPass ? expected.track
        : expected.axis === 'Columns' ? 20 : 10;
      const contributions = pass.contributions;
      assert.ok(Array.isArray(contributions), `${scene.id}: pass contributions are missing`);
      if (!targetPass) {
        assert.deepEqual(contributions, [],
          `${scene.id}: fixed-axis pass must not query item contributions`);
      } else {
        assert.ok(contributions.every(({ item, span, kind, crossesFlexibleTrack }) =>
          item === 2 && span === 1 && kind === 'Minimum' && crossesFlexibleTrack === true),
        `${scene.id}: target pass contains an unexpected contribution`);
        const minimums = contributions;
        if (expected.minimumSource === 'TrackMinimum') {
          assert.deepEqual(minimums, [], `${scene.id}: zero track minimum must not query Minimum`);
        } else {
          assert.equal(minimums.length, 1, `${scene.id}: expected one Minimum contribution per target pass`);
          assert.equal(minimums[0].value, expected.minimum,
            `${scene.id}: Minimum contribution value differs`);
          if (expected.minimumSource === 'ContentBased') {
            assert.match(minimums[0].minimumSource, /AutomaticMinimum/);
            assert.match(minimums[0].minimumSource, /spans_auto_min_track: true/);
            assert.match(minimums[0].minimumSource, /spans_multiple_tracks: false/);
            assert.match(minimums[0].minimumSource, /spans_flexible_max_track: true/);
            assert.match(minimums[0].minimumSource, /content_based: true/);
          } else if (expected.minimumSource === 'Overflow') {
            assert.equal(minimums[0].minimumSource, 'OverflowAutomaticMinimum');
          } else {
            assert.equal(minimums[0].minimumSource, 'SpecifiedMinimum');
          }
        }
      }
      const finalTracks = pass.stages.at(-1)?.tracks;
      assert.ok(Array.isArray(finalTracks), `${scene.id}: final pass track snapshot is missing`);
      const usedTracks = finalTracks.filter(({ gutter, collapsed }) => !gutter && !collapsed);
      assert.equal(usedTracks.length, 1, `${scene.id}: expected one used track per pass`);
      assert.equal(usedTracks[0].baseSize, expectedTrack,
        `${scene.id}: final used track size differs for ${pass.axis} pass ${pass.sequence}`);
    }
  }
  if (scene.fitContentMinHeight) {
    const expected = scene.fitContentMinHeight;
    const observation = diagnostics.fitContentMinHeight;
    assert.ok(observation, `${scene.id}: min-height fixture observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      display: 'grid',
      width: 'LengthPx(60.0)',
      height: 'Auto',
      minHeight: 'LengthPx(300.0)',
      gridTemplateColumns: '60px',
      gridTemplateRows: 'none',
      gridAutoRows: 'fit-content(40px)',
      alignContent: 'normal',
    });
    assert.ok(typeof style.typedGridStyle === 'string' && style.typedGridStyle.length > 0,
      `${scene.id}: typed computed Grid style is missing`);
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(60.0)]',
      explicitRows: '[]',
      autoRows: 'TrackList([FitContent(LengthPx(40.0))])',
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 60, height: 300 },
      item: { x: 0, y: 0, width: 60, height: 10 },
      child: { x: 0, y: 0, width: 60, height: 10 },
    });
    const expectedStages = [
      'Initialize',
      'IntrinsicItemBatch { span: 1, crosses_flexible_track: false }',
      'IntrinsicSizingComplete', 'MaximizeTracks', 'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ];
    assert.deepEqual(observation.rowSizingStages.map(({ stage }) => stage), expectedStages,
      `${scene.id}: row sizing stages are missing, duplicated, or out of order`);
    for (const stage of observation.rowSizingStages) {
      const expectedBase = stage.stage === 'Initialize' ? 0 : expected.rowBaseSize;
      assert.deepEqual(stage.baseSizesCssPx, [expectedBase],
        `${scene.id}: fit-content row size changed at ${stage.stage}`);
    }
  }
  if (scene.fitContentZoomedGrid) {
    const expected = scene.fitContentZoomedGrid;
    const observation = diagnostics.fitContentZoomedGrid;
    assert.ok(observation, `${scene.id}: zoomed-grid fixture observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.deepEqual(style.zoom, {
      root: { specified: 2, effective: 2 },
      item: { specified: 1, effective: 2 },
      child: { specified: 1, effective: 2 },
    });
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.equal(style.computedPropertyGridTemplateRows, 'none');
    assert.deepEqual(style.typedDimensions, {
      rootWidth: 'LengthPx(120.0)',
      rootHeight: 'LengthPx(600.0)',
      itemMinHeight: 'LengthPx(0.0)',
      childHeight: 'LengthPx(800.0)',
    });
    assert.ok(style.typedGridStyle.includes(
      'auto_rows: TrackList([FitContent(LengthPx(80.0))])'),
    `${scene.id}: zoom-scaled typed fit-content limit is missing`);
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(120.0)]',
      explicitRows: '[]',
      autoColumns: 'Auto',
      autoRows: 'TrackList([FitContent(LengthPx(80.0))])',
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 120, height: 600 },
      item: { x: 0, y: 0, width: 120, height: 80 },
      child: { x: 0, y: 0, width: 120, height: 800 },
    });
    const expectedStages = [
      'Initialize',
      'IntrinsicItemBatch { span: 1, crosses_flexible_track: false }',
      'IntrinsicSizingComplete', 'MaximizeTracks', 'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ];
    assert.deepEqual(observation.rowSizingStages.map(({ stage }) => stage), expectedStages,
      `${scene.id}: row sizing stages are missing, duplicated, or out of order`);
    for (const [index, stage] of observation.rowSizingStages.entries()) {
      assert.deepEqual(stage.baseSizesCssPx, [index < 3 ? 0 : 80],
        `${scene.id}: fit-content zoomed row differs at ${stage.stage}`);
    }
    assert.deepEqual(observation.rowContributions.map(({ kind, valueCssPx }) => ({
      kind, valueCssPx,
    })), [
      { kind: 'Minimum', valueCssPx: 0 },
      { kind: 'MinContent', valueCssPx: 800 },
      { kind: 'MaxContent', valueCssPx: 800 },
    ], `${scene.id}: zoom-scaled row contributions differ`);
  }
  if (scene.fitContentPercentageArgument) {
    const expected = scene.fitContentPercentageArgument;
    const observation = diagnostics.fitContentPercentageArgument;
    assert.ok(observation, `${scene.id}: percentage-argument fixture observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      display: 'grid',
      width: 'LengthPx(60.0)',
      height: 'LengthPx(300.0)',
      gridTemplateColumns: '60px',
      gridTemplateRows: 'none',
      gridAutoRows: expected.gridAutoRows,
      alignContent: 'normal',
    });
    assert.ok(typeof style.typedGridStyle === 'string' && style.typedGridStyle.length > 0,
      `${scene.id}: typed computed Grid style is missing`);
    const input = observation.layoutInputGridContainer;
    assert.deepEqual({ ...input, autoRows: undefined }, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(60.0)]',
      explicitRows: '[]',
      autoRows: undefined,
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    if (expected.layoutAutoRows !== undefined) {
      assert.equal(input.autoRows, expected.layoutAutoRows);
    } else {
      assert.ok(input.autoRows.includes(expected.layoutAutoRowsIncludes),
        `${scene.id}: projected auto-row calc is missing`);
    }
    const mathRegistry = observation.layoutCssMathRegistry;
    assert.ok(Array.isArray(mathRegistry),
      `${scene.id}: projected CSS math registry is missing`);
    if (expected.layoutCssMathExpression === undefined) {
      assert.deepEqual(mathRegistry, [], `${scene.id}: unexpected CSS math registry entry`);
    } else {
      const calcID = input.autoRows.match(/LayoutCalcId\((\d+)\)/)?.[1];
      assert.ok(calcID !== undefined, `${scene.id}: projected calc handle is missing`);
      assert.deepEqual(mathRegistry, [{
        id: Number(calcID),
        node: expected.rootNode,
        property: 'GridTemplateRows',
        expression: expected.layoutCssMathExpression,
      }], `${scene.id}: calc registry must bind the handle to its row owner and expression`);
    }
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 60, height: 300 },
      item: { x: 0, y: 0, width: 60, height: expected.rowSize },
      child: { x: 0, y: 0, width: 30, height: 400 },
    });
    const stages = observation.rowSizingStages;
    const expectedStages = [
      'Initialize',
      'IntrinsicItemBatch { span: 1, crosses_flexible_track: false }',
      'IntrinsicSizingComplete',
      'MaximizeTracks',
      'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ];
    assert.ok(Array.isArray(stages), `${scene.id}: row sizing trace is missing`);
    assert.deepEqual(stages.map(({ stage }) => stage.replace(
      'crosses_flexible_track: false}', 'crosses_flexible_track: false }')),
    expectedStages,
      `${scene.id}: row sizing stages are missing, duplicated, or out of order`);
    for (const [index, stage] of stages.entries()) {
      assert.deepEqual(stage.baseSizesCssPx, [index < 3 ? 0 : expected.rowSize],
        `${scene.id}: fit-content row base differs at ${stage.stage}`);
    }
  }
  if (scene.fitContentSpanning) {
    const expected = scene.fitContentSpanning;
    const observation = diagnostics.fitContentSpanning;
    assert.ok(observation, `${scene.id}: fit-content spanning observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    assert.deepEqual(observation.expectedRowsCssPx, expected.expectedRows);
    assert.equal(observation.expectedSpanMaxContentCssPx, expected.expectedSpanMaxContent);
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      alignContent: 'normal',
      display: 'grid',
      gridAutoRows: 'fit-content(40px)',
      gridTemplateColumns: '60px',
      gridTemplateRows: 'none',
      height: 'LengthPx(300.0)',
      itemMinHeight: 'LengthPx(0.0)',
      itemOverflowY: 'visible',
      itemRowEnd: 'Some(Auto)',
      itemRowSpanTwo: true,
      itemRowStart: 'Some(Span(2))',
      width: 'LengthPx(60.0)',
    });
    assert.ok(style.typedGridStyle.includes('auto_rows: TrackList([FitContent(LengthPx(40.0))])'));
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(60.0)]',
      explicitRows: '[]',
      autoRows: 'TrackList([FitContent(LengthPx(40.0))])',
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 60, height: 300 },
      item: { x: 0, y: 0, width: 60, height: 80 },
      child: { x: 0, y: 0, width: 60, height: 200 },
    });
    assert.deepEqual(observation.rowSizingStages.map(({ stage }) => stage), [
      'Initialize',
      'IntrinsicItemBatch { span: 2, crosses_flexible_track: false }',
      'IntrinsicSizingComplete', 'MaximizeTracks', 'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ], `${scene.id}: row sizing stages are missing, duplicated, or out of order`);
    const expectedBases = [[0, 0], [0, 0], [0, 0], [40, 40], [40, 40], [40, 40]];
    for (const [index, stage] of observation.rowSizingStages.entries()) {
      assert.deepEqual(stage.tracks.map(({ baseSizeCssPx }) => baseSizeCssPx),
        [0, expectedBases[index][0], 0, expectedBases[index][1], 0],
        `${scene.id}: row sizing values differ at ${stage.stage}`);
      assert.deepEqual(stage.tracks.map(({ gutter }) => gutter),
        [true, false, true, false, true],
        `${scene.id}: row gutters differ at ${stage.stage}`);
      assert.deepEqual(stage.tracks.map(({ collapsed }) => collapsed),
        [true, false, false, false, true],
        `${scene.id}: collapsed-track state differs at ${stage.stage}`);
      assert.deepEqual(stage.tracks.map(({ growthLimitCssPx }) => growthLimitCssPx),
        index === 0 ? [0, null, 0, null, 0] : [0, 40, 0, 40, 0],
        `${scene.id}: fit-content growth limits differ at ${stage.stage}`);
      assert.deepEqual(stage.tracks.map(({ infinitelyGrowable }) => infinitelyGrowable),
        [false, false, false, false, false],
        `${scene.id}: track growability differs at ${stage.stage}`);
    }
    assert.deepEqual(observation.rowContributions, [
      { item: expected.itemNode, kind: 'Minimum', span: 2, crossesFlexibleTrack: false,
        valueCssPx: 0, minimumSource: 'SpecifiedMinimum' },
      { item: expected.itemNode, kind: 'MinContent', span: 2, crossesFlexibleTrack: false,
        valueCssPx: expected.expectedSpanMaxContent, minimumSource: null },
      { item: expected.itemNode, kind: 'MaxContent', span: 2, crossesFlexibleTrack: false,
        valueCssPx: expected.expectedSpanMaxContent, minimumSource: null },
    ], `${scene.id}: span-2 intrinsic contribution trace differs`);
  }
  if (scene.fitContentLimitBoundaries) {
    const expected = scene.fitContentLimitBoundaries;
    const observation = diagnostics.fitContentLimitBoundaries;
    assert.ok(observation, `${scene.id}: fit-content limit observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    assert.equal(observation.expectedLimitCssPx, expected.limit);
    assert.equal(observation.expectedRowCssPx, expected.expectedRow);
    assert.equal(observation.expectedMaxContentCssPx, expected.expectedMaxContent);
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      alignContent: 'normal',
      display: 'grid',
      gridAutoRows: `fit-content(${expected.limit}px)`,
      gridTemplateColumns: '60px',
      gridTemplateRows: 'none',
      height: 'LengthPx(300.0)',
      itemMinHeight: 'LengthPx(0.0)',
      itemOverflowY: 'visible',
      width: 'LengthPx(60.0)',
    });
    assert.ok(style.typedGridStyle.includes(
      `auto_rows: TrackList([FitContent(LengthPx(${expected.limit}.0))])`));
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(60.0)]',
      explicitRows: '[]',
      autoRows: `TrackList([FitContent(LengthPx(${expected.limit}.0))])`,
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 60, height: 300 },
      item: { x: 0, y: 0, width: 60, height: expected.expectedRow },
      child: { x: 0, y: 0, width: 60, height: 100 },
    });
    const maximize = observation.rowSizingStages.find(({ stage }) => stage === 'MaximizeTracks');
    const maximizedTrack = maximize?.tracks?.filter(({ gutter, collapsed }) => !gutter && !collapsed);
    assert.equal(maximizedTrack?.length, 1, `${scene.id}: MaximizeTracks row is missing`);
    assert.equal(maximizedTrack[0].baseSizeCssPx, expected.expectedRow);
    assert.equal(maximizedTrack[0].growthLimitCssPx, expected.expectedMaxContent);
    assert.equal(maximizedTrack[0].infinitelyGrowable, false);
    const maxContent = observation.rowContributions.filter(({ kind }) => kind === 'MaxContent');
    assert.equal(maxContent.length, 1, `${scene.id}: max-content row contribution count differs`);
    assert.deepEqual(maxContent[0], {
      item: expected.itemNode,
      kind: 'MaxContent',
      span: 1,
      crossesFlexibleTrack: false,
      valueCssPx: expected.expectedMaxContent,
      minimumSource: null,
    });
  }
  if (scene.fitContentLimitColumns) {
    const expected = scene.fitContentLimitColumns;
    const observation = diagnostics.fitContentLimitColumns;
    assert.ok(observation, `${scene.id}: fit-content column observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    assert.equal(observation.expectedLimitCssPx, expected.limit);
    assert.equal(observation.expectedMinimumCssPx, expected.expectedMinimum);
    assert.equal(observation.expectedMaxContentCssPx, expected.expectedMaxContent);
    assert.equal(observation.expectedColumnCssPx, expected.expectedColumn);
    assert.equal(observation.automaticMinimumControl, expected.automaticMinimum);
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      alignContent: 'normal',
      display: 'grid',
      gridAutoColumns: `fit-content(${expected.limit}px)`,
      gridAutoFlow: 'column',
      gridAutoRows: 'auto',
      gridTemplateColumns: 'none',
      gridTemplateRows: '60px',
      height: 'LengthPx(60.0)',
      itemMinWidth: expected.automaticMinimum ? 'Auto' : 'LengthPx(0.0)',
      itemOverflowX: 'visible',
      width: `LengthPx(${expected.rootWidth}.0)`,
    });
    assert.ok(style.typedGridStyle.includes(
      `auto_columns: TrackList([FitContent(LengthPx(${expected.limit}.0))])`));
    assert.ok(style.typedGridStyle.includes('auto_flow: Column'));
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[]',
      explicitRows: '[LengthPx(60.0)]',
      autoColumns: `TrackList([FitContent(LengthPx(${expected.limit}.0))])`,
      autoRows: 'Auto',
      autoFlow: 'Column',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.rootNode,
      item: expected.itemNode,
      child: expected.childNode,
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: expected.rootWidth, height: 60 },
      item: { x: 0, y: 0, width: expected.expectedColumn, height: 10 },
      child: { x: 0, y: 0, width: 100, height: 10 },
    });
    const maximize = observation.columnSizingStages.find(({ stage }) => stage === 'MaximizeTracks');
    const maximizedTrack = maximize?.tracks?.filter(({ gutter, collapsed }) => !gutter && !collapsed);
    assert.equal(maximizedTrack?.length, 1, `${scene.id}: MaximizeTracks column is missing`);
    assert.equal(maximizedTrack[0].baseSizeCssPx, expected.expectedColumn);
    assert.equal(maximizedTrack[0].growthLimitCssPx, expected.expectedMaxContent);
    assert.equal(maximizedTrack[0].infinitelyGrowable, false);
    assert.deepEqual(observation.columnContributions.map(({ kind, valueCssPx }) => [kind, valueCssPx]), [
      ['Minimum', expected.expectedMinimum],
      ['MinContent', expected.expectedMaxContent],
      ['MaxContent', expected.expectedMaxContent],
    ]);
    assert.ok(observation.columnContributions.every((contribution) =>
      contribution.item === expected.itemNode && contribution.span === 1
        && contribution.crossesFlexibleTrack === false));
    if (expected.automaticMinimum) {
      assert.match(observation.columnContributions[0].minimumSource, /^AutomaticMinimum/);
    } else {
      assert.equal(observation.columnContributions[0].minimumSource, 'SpecifiedMinimum');
    }
    assert.equal(observation.columnContributions[1].minimumSource, null);
    assert.equal(observation.columnContributions[2].minimumSource, null);
    assert.deepEqual(observation.rowSizingStages.map(({ stage }) => stage), [
      'Initialize', 'FixedTrackEarlyReturn',
    ], `${scene.id}: explicit row must take the fixed-track early return`);
    const rootTrace = diagnostics.trackSizingTraces.find(({ grid }) => grid === expected.rootNode);
    assert.ok(rootTrace, `${scene.id}: root Grid sizing trace is missing`);
    assert.deepEqual(rootTrace.passes.map(({ axis, sequence }) => [axis, sequence]), [
      ['Columns', 0], ['Rows', 1],
    ], `${scene.id}: both sizing axes and their order must be present`);
    assert.deepEqual(rootTrace.passes[1].contributions, [],
      `${scene.id}: fixed 60px row must not collect intrinsic contributions`);
  }
  if (scene.fitContentAutoMinFloor) {
    const expected = scene.fitContentAutoMinFloor;
    const observation = diagnostics.fitContentAutoMinFloor;
    assert.ok(observation, `${scene.id}: automatic-minimum observation is missing`);
    assert.equal(observation.sceneId, scene.id);
    assert.equal(observation.frameSource, 'pre-render LayoutOutput CSS px');
    const style = observation.styleSnapshot;
    assert.equal(style.profile, 'RuntimeBlockFlexGridTrackSizingV1');
    assert.equal(style.rootNode, expected.rootNode);
    assert.ok(Number.isInteger(style.generation) && style.generation > 0);
    assert.ok(Number.isInteger(style.documentRevision) && style.documentRevision > 0);
    assert.ok(Number.isInteger(style.renderTreeRevision) && style.renderTreeRevision > 0);
    assert.ok(Number.isInteger(style.styleRevision) && style.styleRevision >= 0);
    assert.ok(Number.isInteger(style.environmentRevision) && style.environmentRevision >= 0);
    assert.equal(style.styleRevision, diagnostics.layoutInputRevision.style);
    assert.equal(style.environmentRevision, diagnostics.layoutInputRevision.environment);
    assert.deepEqual(style.computedProperties, {
      display: 'grid',
      width: 'LengthPx(60.0)',
      height: 'LengthPx(300.0)',
      minHeight: 'Auto',
      gridTemplateColumns: '60px',
      gridTemplateRows: 'none',
      gridAutoRows: 'fit-content(40px)',
      alignContent: 'normal',
    });
    assert.ok(typeof style.typedGridStyle === 'string' && style.typedGridStyle.length > 0,
      `${scene.id}: typed computed Grid style is missing`);
    assert.deepEqual(observation.layoutInputGridContainer, {
      rootNode: expected.rootNode,
      columns: '[LengthPx(60.0)]',
      explicitRows: '[]',
      autoRows: 'TrackList([FitContent(LengthPx(40.0))])',
      autoFlow: 'Row',
      placementMode: 'TrackSizing',
    });
    assert.deepEqual(observation.nodeIds, {
      root: expected.nodeIDs[0],
      firstItem: expected.nodeIDs[1],
      firstChild: expected.nodeIDs[2],
      secondItem: expected.nodeIDs[3],
      secondChild: expected.nodeIDs[4],
    });
    assert.deepEqual(observation.framesCssPx, {
      root: { x: 0, y: 0, width: 60, height: 300 },
      firstItem: { x: 0, y: 0, width: 60, height: 10 },
      firstChild: { x: 0, y: 0, width: 60, height: 10 },
      secondItem: { x: 0, y: 10, width: 60, height: expected.expectedRows[1] },
      secondChild: { x: 0, y: 10, width: 60, height: 100 },
    });
    assert.deepEqual(observation.itemMinHeights,
      scene.id === 'fit-content-auto-min-floor-wpt' ? ['Auto', 'Auto'] : ['Auto', 'LengthPx(0.0)'],
      `${scene.id}: the automatic minimum or zero control changed`);
    assert.deepEqual(observation.expectedRowBaseSizesCssPx, expected.expectedRows);
    const expectedStages = [
      'Initialize',
      'IntrinsicItemBatch { span: 1, crosses_flexible_track: false }',
      'IntrinsicSizingComplete', 'MaximizeTracks', 'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ];
    assert.deepEqual(observation.rowSizingStages.map(({ stage }) => stage), expectedStages,
      `${scene.id}: row sizing stages are missing, duplicated, or out of order`);
    const stageRows = [
      [0, 0], expected.intrinsicRows, expected.intrinsicRows,
      expected.expectedRows, expected.expectedRows, expected.expectedRows,
    ];
    for (const [index, stage] of observation.rowSizingStages.entries()) {
      assert.deepEqual(stage.baseSizesCssPx, stageRows[index],
        `${scene.id}: row sizing values differ at ${stage.stage}`);
    }
    assert.equal(expected.secondItemMinHeight,
      scene.id === 'fit-content-auto-min-floor-wpt' ? 'auto' : '0px');
  }
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
  if (scene.equalAutoRowsStretch) {
    assert.equal(diagnostics.trackSizingTraces.length, 1,
      `${scene.id}: expected exactly one Grid trace`);
    assert.equal(axisTrace.grid, 1, `${scene.id}: Grid trace must belong to root node 1`);
    assert.equal(axisTrace.passes.length, 2, `${scene.id}: expected Columns and Rows passes`);
    assert.equal(axisTrace.passes[0].axis, 'Columns', `${scene.id}: first pass must be Columns`);
    assert.equal(axisTrace.passes[1].axis, 'Rows', `${scene.id}: second pass must be Rows`);
    const rowPasses = axisTrace.passes.filter((pass) => pass.axis === 'Rows');
    assert.equal(rowPasses.length, 1, `${scene.id}: expected exactly one Rows pass`);
    const rowPass = rowPasses[0];
    const expectedStageNames = [
      'Initialize',
      'IntrinsicItemBatch { span: 1, crosses_flexible_track: false }',
      'IntrinsicSizingComplete',
      'MaximizeTracks',
      'ExpandFlexibleTracks',
      'StretchAutoTracks { applied: true }',
    ];
    assert.equal(rowPass.stages.length, expectedStageNames.length,
      `${scene.id}: unexpected Rows sizing stage count`);
    const stageIndexes = expectedStageNames.map((name) => {
      const matches = rowPass.stages.flatMap((entry, index) => (
        entry.stage === name ? [index] : []
      ));
      assert.equal(matches.length, 1,
        `${scene.id}: Rows ${name} stage count expected 1, got ${matches.length}`);
      return matches[0];
    });
    assert.ok(stageIndexes.every((index, position) => (
      position === 0 || index > stageIndexes[position - 1]
    )), `${scene.id}: Rows sizing stages are out of order`);
    const rowBases = (stage, expected, stageName) => {
      assert.equal(stage.tracks.length, 7,
        `${scene.id}: ${stageName} expected 7 row/gutter entries`);
      const expectedSizes = [0, expected[0], 10, expected[1], 10, expected[2], 0];
      stage.tracks.forEach((track, index) => {
        assert.equal(track.internalIndex, index,
          `${scene.id}: ${stageName} track ${index} has a mismatched internalIndex`);
        assert.equal(track.gutter, index % 2 === 0,
          `${scene.id}: ${stageName} track ${index} gutter identity`);
        assert.equal(track.collapsed, index === 0 || index === 6,
          `${scene.id}: ${stageName} track ${index} collapsed edge identity`);
        assert.ok(Math.abs(track.baseSize - expectedSizes[index]) <= 1e-4,
          `${scene.id}: ${stageName} track ${index} expected ${expectedSizes[index]}, got ${track.baseSize}`);
      });
      return stage.tracks.filter(({ gutter }) => !gutter).map(({ baseSize }) => baseSize);
    };
    for (const [stageIndex, stageName] of expectedStageNames.entries()) {
      if (stageIndex === expectedStageNames.length - 1) break;
      const stage = rowPass.stages[stageIndexes[stageIndex]];
      const expected = stageIndex === 0 ? [0, 0, 5] : [10, 20, 5];
      const actual = rowBases(stage, expected, stageName);
      assert.equal(actual.length, 3, `${scene.id}: ${stageName} row count`);
      actual.forEach((size, index) => {
        assert.ok(Math.abs(size - expected[index]) <= 1e-4,
          `${scene.id}: ${stageName} row ${index + 1} base expected ${expected[index]}, got ${size}`);
      });
    }
    const stretchStage = rowPass.stages[stageIndexes.at(-1)];
    assert.equal(rowPass.stages.at(-1), stretchStage,
      `${scene.id}: StretchAutoTracks must be the final Rows stage`);
    const finalTracks = stretchStage.tracks;
    const finalBases = rowBases(stretchStage, [32.5, 42.5, 5], 'StretchAutoTracks');
    assert.equal(finalBases.length, 3, `${scene.id}: final row count`);
    finalBases.forEach((size, index) => {
      assert.ok(Math.abs(size - [32.5, 42.5, 5][index]) <= 1e-4,
        `${scene.id}: final row ${index + 1} expected ${[32.5, 42.5, 5][index]}, got ${size}`);
    });
    const allGutters = finalTracks.filter(({ gutter }) => gutter);
    const internalGutters = allGutters.slice(1, -1);
    assert.equal(internalGutters.length, 2, `${scene.id}: expected two internal row gutters`);
    internalGutters.forEach(({ baseSize }, index) => {
      assert.ok(Math.abs(baseSize - 10) <= 1e-4,
        `${scene.id}: internal row gutter ${index + 1} expected 10, got ${baseSize}`);
    });
  }
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
    finalTracks.forEach(({ baseSize }, index) => {
      if (scene.geometryTolerance !== undefined && expected.collapsed[index]) {
        assert.equal(baseSize, 0,
          `${scene.id}: ${axis} collapsed track ${index + 1} must be exactly zero.`);
      } else {
        assert.ok(Math.abs(baseSize - expected.tracks[index]) <= geometryTolerance,
          `${scene.id}: ${axis} track ${index + 1} 크기 예상 ${expected.tracks[index]}, 실제 ${baseSize}`);
      }
    });
    if (scene.axes || scene.id.startsWith('auto-fill-two-track-')
        || scene.id.startsWith('auto-fit-min-') || boundaryGutters) {
      const allGutters = finalStageTracks.filter(({ gutter }) => gutter);
      const internalGutters = allGutters.slice(1, -1);
      assert.equal(internalGutters.length, finalTracks.length - 1,
        `${scene.id}: ${axis} 내부 gap 수가 track 사이 개수와 다릅니다.`);
      const sharedCollapsedRunGutters = new Set();
      for (let start = 0; start < expected.collapsed.length;) {
        if (!expected.collapsed[start]) {
          start++;
          continue;
        }
        let end = start;
        while (end + 1 < expected.collapsed.length && expected.collapsed[end + 1]) end++;
        if (start > 0 && end + 1 < expected.collapsed.length) {
          sharedCollapsedRunGutters.add(start - 1);
        }
        start = end + 1;
      }
      internalGutters.forEach(({ baseSize }, index) => {
        const preservedBoundary = (index === 0 && boundaryGutters?.leading)
          || (index === internalGutters.length - 1 && boundaryGutters?.trailing);
        const sharedRunBoundary = sharedCollapsedRunGutters.has(index);
        const expectedSize = preservedBoundary || sharedRunBoundary
          ? (scene.gaps?.[axis] ?? 8)
          : expected.collapsed[index] || expected.collapsed[index + 1]
            ? 0 : (scene.gaps?.[axis] ?? 8);
        assert.ok(Math.abs(baseSize - expectedSize) <= 1 / 64,
          `${scene.id}: ${axis} gap ${index + 1} 크기 예상 ${expectedSize}, 실제 ${baseSize}`);
      });
    }
    tracksByAxis[axis] = finalTracks.map(({ baseSize, collapsed }) => ({ baseSize, collapsed }));
  }
  if (scene.intrinsicAutoRepeatRecompute) {
    const expected = scene.intrinsicAutoRepeatRecompute;
    const columns = tracksByAxis.Columns.map(({ baseSize }) => baseSize);
    assert.equal(columns.length, expected.repeatCount + expected.suffixTrackCount,
      `${scene.id}: auto-repeat count must be derived after excluding the single suffix track`);
    assert.deepEqual(columns.map((size) => Math.round(size * 64) / 64),
      expected.usedColumns.map((size) => Math.round(size * 64) / 64),
      `${scene.id}: final used columns differ from the WPT expectations`);
    const contributionItems = new Set(axisTrace.passes.flatMap((pass) => pass.contributions)
      .map(({ item }) => item));
    assert.ok(contributionItems.has(2),
      `${scene.id}: the in-flow spanning item must be present in track-sizing contributions`);
    assert.ok([...contributionItems].every((item) => item === 2),
      `${scene.id}: absolute probes must not contribute to track sizing`);
  }
  if (scene.gridAutoRepeatIntrinsicWpt) {
    const expected = scene.gridAutoRepeatIntrinsicWpt;
    assert.equal(diagnostics.trackSizingTraces.length, 1,
      `${scene.id}: exactly one root Grid trace is required`);
    assert.equal(axisTrace.grid, 1, `${scene.id}: track trace must belong to root node 1`);
    assert.equal(axisTrace.passes.length, 2,
      `${scene.id}: Columns and Rows must each have one sizing pass`);
    const [columns, rows] = axisTrace.passes;
    assert.deepEqual(axisTrace.passes.map(({ axis }) => axis), ['Columns', 'Rows'],
      `${scene.id}: track sizing axis order differs`);
    for (const [pass, sequence, axis, tracks, span, kinds, value] of [
      [columns, 0, 'Columns', expected.usedColumns, expected.columnContributionSpan,
        expected.columnContributionKinds, 100],
      [rows, 1, 'Rows', [20], 1,
        ['Minimum', 'MinContent', 'MinContent', 'MaxContent'], 20],
    ]) {
      assert.equal(pass.sequence, sequence, `${scene.id}: ${axis} pass sequence differs`);
      assert.equal(pass.phase, null, `${scene.id}: ${axis} pass must not carry a cyclic phase`);
      assert.equal(pass.autoRepeatCount, null,
        `${scene.id}: ${axis} must not expose cyclic auto-repeat metadata`);
      assert.equal(pass.percentageBasis, null,
        `${scene.id}: ${axis} must not carry percentage-cycle metadata`);
      const expectedStages = [
        'Initialize',
        `IntrinsicItemBatch { span: ${span}, crosses_flexible_track: false }`,
        'IntrinsicSizingComplete',
        'MaximizeTracks',
        'ExpandFlexibleTracks',
      ];
      assert.deepEqual(pass.stages.map(({ stage }) => stage
        .replace(/^IntrinsicItemBatch \{\s*/, 'IntrinsicItemBatch { '))
        .slice(0, expectedStages.length),
        expectedStages, `${scene.id}: ${axis} sizing stage sequence differs`);
      assert.equal(pass.stages.length, 6, `${scene.id}: ${axis} sizing stage count differs`);
      assert.match(pass.stages[5].stage, /^StretchAutoTracks \{ applied: (?:true|false) \}$/,
        `${scene.id}: ${axis} auto-track stretch stage is malformed`);
      const finalTracks = pass.stages.at(-1).tracks.filter(({ gutter }) => !gutter);
      assert.deepEqual(finalTracks.map(({ baseSize, collapsed }) => ({ baseSize, collapsed })),
        tracks.map((baseSize) => ({ baseSize, collapsed: false })),
        `${scene.id}: ${axis} final trace tracks differ`);
      const contributions = pass.contributions.map(({ item, kind, span: contributionSpan, value: contributionValue }) => ({
        item, kind, span: contributionSpan, value: contributionValue,
      }));
      assert.equal(contributions.length, kinds.length,
        `${scene.id}: ${axis} contribution count differs`);
      const remainingKinds = [...kinds];
      for (const contribution of contributions) {
        assert.equal(contribution.item, 2,
          `${scene.id}: absolute probes must not contribute to ${axis} sizing`);
        assert.equal(contribution.span, span,
          `${scene.id}: ${axis} contribution span differs`);
        assert.equal(contribution.value, value,
          `${scene.id}: ${axis} contribution value differs`);
        const kindIndex = remainingKinds.indexOf(contribution.kind);
        assert.notEqual(kindIndex, -1,
          `${scene.id}: unexpected ${axis} contribution kind ${contribution.kind}`);
        remainingKinds.splice(kindIndex, 1);
      }
      assert.deepEqual(remainingKinds, [], `${scene.id}: ${axis} contribution kinds are incomplete`);
    }
    assert.equal(tracksByAxis.Columns.length, expected.repeatCount + expected.suffixTrackCount,
      `${scene.id}: repeat count must be derived after excluding its one suffix track`);
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
    evidenceDirectory: relative(repositoryRoot, evidenceDirectory),
  }, null, 2));
  process.exit(0);
}

const fixedRepeatMinmaxFlexSourceRevision = runKind === 'fixed-repeat-minmax-flex'
  ? {
    ...sourceRevision(),
    fixedRepeatMinmaxFlexInputFiles: await fixedRepeatMinmaxFlexInputHashes(),
  }
  : null;
await mkdir(evidenceDirectory, { recursive: true });
const observations = [];

if (target === 'ios-simulator') {
  const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
    .filter(({ udid, state }) => udid === simulatorUDID && state === 'Booted')
    .map((device) => ({ ...device, runtime })));
  assert.equal(simulator.length, 1, `Booted iOS Simulator를 찾지 못했습니다: ${simulatorUDID}`);
  assert.match(simulator[0].runtime, /iOS-26-2$/);
  const simulatorModelIdentifier = command('xcrun', [
    'simctl', 'getenv', simulatorUDID, 'SIMULATOR_MODEL_IDENTIFIER',
  ]).stdout.trim();
  if (isImageAutoMaxBorderOverflowRun || isOverflowAutoMinimumRun || isZeroFixedMaximumRun) {
    assert.equal(simulatorModelIdentifier, 'iPhone18,1',
      `iPhone 17 Pro simulator가 아닙니다: ${simulator[0].name}/${simulatorModelIdentifier}`);
  } else if (isEqualAutoRowsRun) {
    assert.equal(simulatorModelIdentifier, 'iPhone18,1',
      `iPhone 17 Pro simulator가 아닙니다: ${simulator[0].name}/${simulatorModelIdentifier}`);
  } else {
    assert.ok(['iPhone 17', 'iPhone 17 Pro'].includes(simulator[0].name),
      `지원하지 않는 iOS 시뮬레이터 모델입니다: ${simulator[0].name}`);
  }
  await saveLog('install.log', command('xcrun', ['simctl', 'install', simulatorUDID, appPath]));
  const appSha256 = sha256(await readFile(resolve(appPath, 'SpinonBootstrap')));
  const installedAppPath = command('xcrun', [
    'simctl', 'get_app_container', simulatorUDID, packageName, 'app',
  ]).stdout.trim();
  const installedAppSha256 = sha256(await readFile(resolve(installedAppPath, 'SpinonBootstrap')));
  assert.equal(installedAppSha256, appSha256,
    '시뮬레이터에 설치된 app executable과 빌드 executable의 SHA-256이 다릅니다.');
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
    if (!scene.expectedRejection) {
      await saveLog(`screenshot-${scene.id}.log`, command('xcrun', ['simctl', 'io', simulatorUDID,
        'screenshot', resolve(evidenceDirectory, `${scene.id}.png`)]));
    }
  }
  await writeFile(resolve(evidenceDirectory, 'simulator.json'), `${JSON.stringify({
    target, udid: simulatorUDID, name: simulator[0].name,
    modelIdentifier: simulatorModelIdentifier, runtime: simulator[0].runtime,
    appSha256, installedAppSha256,
    sourceRevision: fixedRepeatMinmaxFlexSourceRevision ?? sourceRevision(),
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
  metadata.sourceRevision = fixedRepeatMinmaxFlexSourceRevision ?? sourceRevision();
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
        const rejectionSummary = log.includes(
          `SPINON_C114_GRID_REJECTION_SUMMARY run_id=${runID} scene_id=${scene.id} `);
        const traceComplete = c114TraceChunkState(log, runID).complete;
        const draw = log.includes('SPINON_C0410_DRAW')
          && log.includes(`presented boxes=${scene.boxCount}`);
        const pixelSummary = !scene.pixelSamples
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`);
        const pixelError = log.includes(`SPINON_C114_GRID_PIXEL_MATRIX status=error run_id=${runID}`)
          || log.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=mismatch run_id=${runID}`);
        if ((summary && traceComplete && draw && pixelSummary)
            || (scene.expectedRejection && rejectionSummary)
            || pixelError
            || log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes(`SPINON_C114_GRID_ENVIRONMENT_RESULT run_id=${runID} scene_id=${scene.id} strict_matches=false`)
            || log.includes(`SPINON_C114_GRID_PRESENTATION_MATCH run_id=${runID} matched=false`)
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
    if (!scene.expectedRejection) {
      adb(['shell', 'cmd', 'statusbar', 'collapse']);
      sleep(300);
      const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
      await writeFile(resolve(evidenceDirectory, `${scene.id}.png`), screenshot);
    }
  }
}

await writeFile(resolve(evidenceDirectory, 'result.json'), `${JSON.stringify({
  schemaVersion: 1, target, runPrefix, sceneCount: runScenes.length, observations,
}, null, 2)}\n`);
console.log(JSON.stringify({ evidenceDirectory, target, observations }, null, 2));
