import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');
const target = process.argv[2];
const serialOrUDID = process.argv[3];
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  '사용법: node capture-generic-row-axis.mjs android-avd <ADB serial> [scene IDs] | ios-simulator <UDID> [scene IDs]');
assert.ok(serialOrUDID, 'Android AVD serial 또는 iOS Simulator UDID가 필요합니다.');

const sceneFilter = process.argv[4]?.split(',').filter(Boolean);
const sceneDefinitions = [
  ...['forward', 'reversed'].flatMap((order) => [
    { id: `direct-min-content-${order}`, axis: 'column', maxContent: false, reversed: order === 'reversed' },
    { id: `direct-max-content-${order}`, axis: 'column', maxContent: true, reversed: order === 'reversed' },
    { id: `direct-row-min-content-${order}`, axis: 'row', maxContent: false, reversed: order === 'reversed' },
    { id: `direct-row-max-content-${order}`, axis: 'row', maxContent: true, reversed: order === 'reversed' },
    { id: `direct-min-content-generic-${order}`, axis: 'column', maxContent: false, reversed: order === 'reversed' },
    { id: `direct-max-content-generic-${order}`, axis: 'column', maxContent: true, reversed: order === 'reversed' },
    { id: `direct-row-min-content-generic-${order}`, axis: 'row', maxContent: false, reversed: order === 'reversed' },
    { id: `direct-row-max-content-generic-${order}`, axis: 'row', maxContent: true, reversed: order === 'reversed' },
  ]),
];
const allScenes = sceneDefinitions.map((scene) => {
  const rowAxis = scene.axis === 'row';
  const column = scene.maxContent ? 80 : 50;
  const textWidth = rowAxis || !scene.maxContent ? 50 : 80;
  const textHeight = rowAxis || !scene.maxContent ? 20 : 10;
  const rootWidth = rowAxis ? 74 : 120;
  const rootHeight = rowAxis ? 20 : 48;
  const columns = rowAxis ? [50, 24] : [column];
  const rows = rowAxis ? [20] : [24, 24];
  const textNode = scene.reversed ? 3 : 2;
  const imageNode = scene.reversed ? 2 : 4;
  const textFrame = `x=0,y=0,width=${textWidth},height=${textHeight}`;
  const imageFrame = rowAxis
    ? 'x=50,y=0,width=24,height=16'
    : 'x=0,y=24,width=24,height=16';
  return {
    ...scene,
    column,
    columns,
    rows,
    rootWidth,
    rootHeight,
    item: `width=${textWidth},height=${textHeight}`,
    textNode,
    imageNode,
    frames: scene.reversed
      ? [
        `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
        `1:node=2,${imageFrame}`,
        `2:node=3,${textFrame}`,
      ]
      : [
        `0:node=1,x=0,y=0,width=${rootWidth},height=${rootHeight}`,
        `1:node=2,${textFrame}`,
        `2:node=4,${imageFrame}`,
      ],
  };
});
assert.ok(!sceneFilter || sceneFilter.every((id) => allScenes.some((scene) => scene.id === id)),
  `알 수 없는 C11.4 mixed scene: ${sceneFilter?.join(',')}`);
const scenes = sceneFilter ? allScenes.filter(({ id }) => sceneFilter.includes(id)) : allScenes;
const screenshotFilter = process.env.SPINON_C114_SCREENSHOT_SCENES?.split(',').filter(Boolean);
assert.ok(!screenshotFilter || screenshotFilter.every((id) => scenes.some((scene) => scene.id === id)),
  `스크린샷 대상에 선택되지 않은 scene이 있습니다: ${screenshotFilter?.join(',')}`);
const shouldCaptureScreenshot = (scene) => !screenshotFilter || screenshotFilter.includes(scene.id);

const packageName = 'dev.spinon.bootstrap';
const bundlePath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const apkPath = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const captureScriptPath = fileURLToPath(import.meta.url);
const timestamp = new Date().toISOString().replaceAll(/[^0-9TZ]/g, '');
const runToken = randomUUID().slice(0, 8);
const evidenceRoot = process.env.SPINON_C114_EVIDENCE_ROOT
  ? resolve(repositoryRoot, process.env.SPINON_C114_EVIDENCE_ROOT)
  : dirname(captureScriptPath);
const iosScreenshotVerifierPath = resolve(dirname(captureScriptPath), 'verify-ios-screenshot.swift');
const runPrefix = `c114-generic-row-axis-${target}-${timestamp}-${runToken}`;
const evidenceDirectory = resolve(evidenceRoot, 'runs', runPrefix);
const sourceInputPaths = [
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic_rows.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_mixed_direct_text_image_intrinsic.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_mixed_direct_text_image_row_axis.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image_row_axis.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image_row_axis/fixture.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image_row_axis/tests.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image/tests.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/tests.rs',
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
];

function command(program, args, options = {}) {
  const result = spawnSync(program, args, {
    cwd: repositoryRoot,
    env: { ...process.env, ...options.env },
    encoding: Object.hasOwn(options, 'encoding') ? options.encoding : 'utf8',
    timeout: options.timeout ?? 30_000,
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (!options.allowFailure && result.status !== 0) {
    throw new Error(`${program} ${args.join(' ')} failed (${result.status}):\n${result.stderr}`);
  }
  return result;
}

const sha256 = (value) => createHash('sha256').update(value).digest('hex');
const sleep = (milliseconds) => new Promise((done) => setTimeout(done, milliseconds));
const pngSignature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

function redactLocalPaths(value) {
  let redacted = value.replaceAll(repositoryRoot, '<REPO>');
  if (process.env.HOME) redacted = redacted.replaceAll(process.env.HOME, '<HOME>');
  return redacted;
}

function normalizeEvidenceText(value) {
  const normalized = redactLocalPaths(value)
    .replaceAll('/Users/node_modules', '<USERS>/node_modules')
    .replaceAll('\r\n', '\n')
    .split('\n')
    .map((line) => line.replace(/[ \t]+$/g, ''))
    .join('\n');
  return normalized.length === 0 ? '' : `${normalized.replace(/\n+$/g, '')}\n`;
}

function assertPngSignature(bytes, description) {
  assert.ok(Buffer.isBuffer(bytes), `${description} must remain binary.`);
  assert.ok(bytes.subarray(0, pngSignature.length).equals(pngSignature),
    `${description} has no PNG signature.`);
}

function assertImageDecodes(path, expected) {
  const dimensions = command('sips', ['-g', 'pixelWidth', '-g', 'pixelHeight', path]).stdout;
  const width = Number(dimensions.match(/pixelWidth:\s*(\d+)/)?.[1]);
  const height = Number(dimensions.match(/pixelHeight:\s*(\d+)/)?.[1]);
  assert.deepEqual({ width, height }, expected,
    `Image dimensions do not match the selected simulator for ${path}`);
  return { width, height };
}

function sourceRevision() {
  const commit = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const tree = command('git', ['rev-parse', 'HEAD^{tree}']).stdout.trim();
  const trackedDiff = command('git', ['diff', '--binary', 'HEAD']).stdout;
  const dirtyPaths = command('git', ['diff', '--name-only', 'HEAD']).stdout
    .split(/\r?\n/).filter(Boolean);
  const sourceFileSha256 = Object.fromEntries(sourceInputPaths.map((path) => [
    path,
    sha256(readFileSync(resolve(repositoryRoot, path))),
  ]));
  return {
    commit,
    tree,
    trackedDiffSha256: sha256(trackedDiff),
    trackedDirtyPaths: dirtyPaths,
    sourceFileSha256,
    sourceFilesSha256: sha256(JSON.stringify(sourceFileSha256)),
  };
}

async function buildApplication() {
  const buildSource = sourceRevision();
  const captureScriptSha256 = sha256(readFileSync(captureScriptPath));
  const buildLogPath = resolve(evidenceDirectory,
    target === 'android-avd' ? 'android-build.log' : 'ios-build.log');
  const args = ['exec', '--', 'bun', 'run',
    target === 'android-avd' ? 'build:android' : 'build:ios-sim'];
  const environment = {
    SPINON_ENABLE_C04_RUNTIME_GPU: '1',
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST: '1',
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
  };
  const result = command('mise', args, {
    allowFailure: true,
    timeout: 900_000,
    env: environment,
  });
  const buildLog = normalizeEvidenceText(`${result.stdout}${result.stderr}`);
  await writeFile(buildLogPath, buildLog);
  assert.equal(result.status, 0,
    `${target}: current-source app build failed; see ${buildLogPath}`);
  const sourceAfterBuild = sourceRevision();
  assert.equal(sourceAfterBuild.sourceFilesSha256, buildSource.sourceFilesSha256,
    `${target}: source inputs changed during app build`);
  assert.equal(sha256(readFileSync(captureScriptPath)), captureScriptSha256,
    `${target}: capture script changed during app build`);
  const artifactPath = target === 'android-avd'
    ? apkPath
    : resolve(bundlePath, 'SpinonBootstrap');
  const artifactSha256 = sha256(await readFile(artifactPath));
  return {
    command: ['mise', ...args],
    environment,
    source: buildSource,
    captureScript: {
      path: captureScriptPath.slice(repositoryRoot.length + 1),
      sha256: captureScriptSha256,
    },
    artifactPath: artifactPath.slice(repositoryRoot.length + 1),
    artifactSha256,
    buildLogPath: buildLogPath.slice(repositoryRoot.length + 1),
    buildLogSha256: sha256(buildLog),
  };
}

function traceParts(log, runID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  return log.split(/\r?\n/).flatMap((line) => {
    const offset = line.indexOf(marker);
    if (offset < 0) return [];
    const match = line.slice(offset + marker.length).match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
}

function hasSuccessfulDraw(log) {
  if (target === 'android-avd') {
    return log.includes('SPINON_C0410_DRAW status=0 presented boxes=3');
  }
  return /SPINON_C0410_DRAW generation=\d+ status=0 presented boxes=3/.test(log);
}

function sceneCaptureComplete(log, runID, scene) {
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  const summaryOffset = log.indexOf(summary);
  return summaryOffset >= 0 && hasSuccessfulDraw(log.slice(summaryOffset));
}

function verifyScene(log, scene, runID) {
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`),
    `${scene.id}: 앱 실행기 프레임/trace 검증 실패`);
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'), `${scene.id}: scene 인자 거부`);
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  assert.ok(log.includes(summary), `${scene.id}: runtime summary 누락`);
  assert.ok(log.includes('status=0 layout=ready boxes=3'), `${scene.id}: layout/box 상태 실패`);
  assert.ok(log.includes(`expected_root_frame=width=${scene.rootWidth},height=${scene.rootHeight}`),
    `${scene.id}: root frame 불일치`);
  assert.ok(log.includes(`expected_item_frame=${scene.item}`), `${scene.id}: text item frame 불일치`);

  const frameLines = log.split(/\r?\n/).flatMap((line) => {
    const androidMarker = `SPINON_C114_NODE_FRAME state=${scene.id} `;
    const iosMarker = 'SPINON_C114_GRID_NODE_FRAME ';
    const marker = target === 'android-avd'
      ? line.includes(androidMarker) ? androidMarker : null
      : line.includes(iosMarker) ? iosMarker : null;
    const offset = marker ? line.indexOf(marker) : -1;
    return offset < 0 ? [] : [line.slice(offset + marker.length).trim()];
  });
  assert.deepEqual(frameLines, scene.frames, `${scene.id}: node 순서/frame 불일치`);

  const parts = traceParts(log, runID);
  assert.ok(parts.length > 0, `${scene.id}: trace chunk 누락`);
  const total = parts[0].total;
  assert.ok(Number.isInteger(total) && total > 0, `${scene.id}: trace 전체 chunk 수가 잘못됨`);
  assert.equal(parts.length, total, `${scene.id}: trace chunk 중복 또는 누락`);
  assert.ok(parts.every((part) => part.total === total && part.index >= 1 && part.index <= total),
    `${scene.id}: trace part 수가 일치하지 않음`);
  const byIndex = new Map(parts.map((part) => [part.index, part.payload]));
  assert.equal(byIndex.size, total, `${scene.id}: trace chunk 일부 누락`);
  const tracePayload = [...Array(total)].map((_, index) => byIndex.get(index + 1)).join('');
  const diagnostics = JSON.parse(tracePayload.replace(/^grid_track_sizing_diagnostics=/, ''));

  assert.equal(diagnostics.layoutInputRevision?.intrinsicSize, 1,
    `${scene.id}: image intrinsic revision 불일치`);
  const inputs = diagnostics.gridItemIntrinsicInputs ?? [];
  assert.equal(inputs.length, 2, `${scene.id}: mixed intrinsic input 수 불일치`);
  const ordered = scene.reversed
    ? [[scene.imageNode, 'replaced'], [scene.textNode, 'text']]
    : [[scene.textNode, 'text'], [scene.imageNode, 'replaced']];
  for (const [index, [node, kind]] of ordered.entries()) {
    const input = inputs[index];
    assert.equal(input.grid, 1, `${scene.id}: intrinsic grid ID`);
    assert.equal(input.node, node, `${scene.id}: intrinsic node ID`);
    assert.equal(input.sourceOrder, index, `${scene.id}: intrinsic source order`);
    assert.equal(input.source?.kind, kind, `${scene.id}: intrinsic source kind`);
    if (kind === 'text') {
      assert.equal(input.source.fontRegistryRevision, 1);
      assert.deepEqual([
        input.source.minContent,
        input.source.minContentHeight,
        input.source.maxContent,
        input.source.maxContentHeight,
      ], [50, 20, 80, 10]);
    } else {
      assert.equal(input.source.natural?.width, 24);
      assert.equal(input.source.natural?.height, 16);
    }
  }

  const traces = diagnostics.trackSizingTraces ?? [];
  assert.equal(traces.length, 1, `${scene.id}: Grid trace 수`);
  const passes = traces[0].passes;
  assert.deepEqual(passes.map((pass) => pass.axis), ['Columns', 'Rows']);
  const finalTracks = (pass) => pass.stages.at(-1).tracks
    .filter(({ gutter, collapsed }) => !gutter && !collapsed).map(({ baseSize }) => baseSize);
  assert.deepEqual(finalTracks(passes[0]), scene.columns);
  assert.deepEqual(finalTracks(passes[1]), scene.rows);
  const contributionKind = scene.maxContent ? 'MaxContent' : 'MinContent';
  const contributionPass = scene.axis === 'row' ? passes[1] : passes[0];
  const expectedContributions = scene.axis === 'row'
    ? [[scene.textNode, 20], [scene.imageNode, 16]]
    : [[scene.textNode, scene.column], [scene.imageNode, 24]];
  for (const [node, value] of expectedContributions) {
    assert.ok(contributionPass.contributions.some((contribution) => (
      contribution.item === node && contribution.span === 1
        && contribution.kind === contributionKind
        && Math.abs(contribution.value - value) <= 1 / 64
    )), `${scene.id}: ${node} ${contributionKind} contribution 누락`);
  }
  assert.ok(hasSuccessfulDraw(log), `${scene.id}: draw/present 로그 누락`);
  return {
    sceneId: scene.id,
    runID,
    columns: scene.columns,
    rows: scene.rows,
    intrinsicSizingAxis: scene.axis,
    nodeFrames: frameLines,
    intrinsicRevision: diagnostics.layoutInputRevision.intrinsicSize,
    traceChunks: total,
    drawPresented: true,
  };
}

async function runAndroid() {
  const adb = (args, options = {}) => command('adb', ['-s', serialOrUDID, ...args], options);
  try {
    const buildProvenance = await buildApplication();
    const metadata = {
      serial: serialOrUDID,
      target,
      avdName: adb(['shell', 'getprop', 'ro.boot.qemu.avd_name']).stdout.trim(),
      apiLevel: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
      abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
      model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
      pageSizeBytes: Number(adb(['shell', 'getconf', 'PAGE_SIZE']).stdout.trim()),
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
      qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(),
      buildProvenance,
    };
    assert.equal(metadata.apiLevel, 37);
    assert.equal(metadata.abi, 'arm64-v8a');
    assert.equal(metadata.model, 'sdk_gphone16k_arm64');
    assert.equal(metadata.pageSizeBytes, 16_384);
    assert.equal(metadata.qemu, '1');
    const apkBytes = await readFile(apkPath);
    const localApkSha256 = sha256(apkBytes);
    assert.equal(localApkSha256, buildProvenance.artifactSha256,
      'Android APK가 capture용 current-source build와 다릅니다.');
    await writeFile(resolve(evidenceDirectory, 'android-install.log'),
      normalizeEvidenceText(adb(['install', '-r', apkPath]).stdout));
    const installedBaseApk = adb(['shell', 'pm', 'path', packageName]).stdout
      .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim())
      .find((path) => path.endsWith('/base.apk'));
    assert.ok(installedBaseApk, '설치된 base.apk 경로 없음');
    const installedApkSha256 = adb(['shell', 'sha256sum', installedBaseApk]).stdout.trim().split(/\s+/)[0];
    assert.equal(installedApkSha256, localApkSha256);
    const observations = [];
    for (const [index, scene] of scenes.entries()) {
      let log = '';
      let runID = '';
      for (let attempt = 0; attempt < 2; attempt++) {
        runID = `${runPrefix}-${index}${attempt === 0 ? '' : `-retry${attempt}`}`;
        const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n',
          `${packageName}/.MainActivity`, '--ez', 'spinon_c114_grid_track_sizing', 'true',
          '--es', 'spinon_c114_grid_scene', scene.id,
          '--es', 'spinon_c114_grid_run_id', runID]);
        const launchLog = normalizeEvidenceText(`${launch.stdout}${launch.stderr}`);
        await writeFile(resolve(evidenceDirectory,
          `android-launch-${scene.id}${attempt === 0 ? '' : `-attempt${attempt}`}.log`), launchLog);
        const attemptStarted = Date.now();
        const deadline = attemptStarted + 30_000;
        while (Date.now() < deadline) {
          const pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
            .stdout.trim().split(/\s+/)[0]) || null;
          if (pid) {
            log = adb(['logcat', '-d', '-T', '1000', '-v', 'threadtime', `--pid=${pid}`,
              'SpinonBootstrap:V', '*:S'], { timeout: 30_000 }).stdout;
          }
          if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
              || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
          if (sceneCaptureComplete(log, runID, scene)) break;
          const emptyStartupDraw = log.includes('SPINON_C114_GRID_HOST=session=ready')
            && log.includes('SPINON_C0410_DRAW status=0 presented boxes=0')
            && !log.includes(`SPINON_C114_GRID_EVAL_RUN run_id=${runID}`);
          if (emptyStartupDraw && Date.now() - attemptStarted >= 3_000) break;
          await sleep(250);
        }
        if (sceneCaptureComplete(log, runID, scene)) break;
        await writeFile(resolve(evidenceDirectory,
          `android-runtime-${scene.id}-attempt${attempt}.log`), normalizeEvidenceText(log));
        const hardFailure = log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
          || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR');
        if (attempt > 0 || hardFailure) break;
        adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
        await sleep(500);
      }
      const runtimeLog = normalizeEvidenceText(log);
      await writeFile(resolve(evidenceDirectory, `android-runtime-${scene.id}.log`), runtimeLog);
      const observation = verifyScene(runtimeLog, scene, runID);
      const activityState = adb(['shell', 'dumpsys', 'activity', 'activities']).stdout;
      const topResumedActivity = activityState.split(/\r?\n/)
        .find((line) => line.includes('topResumedActivity='));
      assert.ok(topResumedActivity?.includes(`${packageName}/`),
        `${scene.id}: Spinon is not the top-resumed activity before screenshot capture.`);
      observation.foregroundActivity = topResumedActivity.trim();
      const runtimeLogPath = resolve(evidenceDirectory, `android-runtime-${scene.id}.log`);
      observation.runtimeLogSha256 = sha256(await readFile(runtimeLogPath));
      if (shouldCaptureScreenshot(scene)) {
        const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
        assertPngSignature(screenshot, `Android screenshot ${scene.id}`);
        const screenshotPath = resolve(evidenceDirectory, `android-${scene.id}.png`);
        await writeFile(screenshotPath, screenshot);
        observation.screenshotPixels = assertImageDecodes(screenshotPath, {
          width: 1080,
          height: 2400,
        });
        observation.screenshotSha256 = sha256(screenshot);
      }
      observations.push(observation);
      console.log(`Android AVD ${scene.id}: PASS`);
    }
    const finalSource = sourceRevision();
    assert.equal(finalSource.sourceFilesSha256, buildProvenance.source.sourceFilesSha256,
      'Android capture source differs from the source used for its app build.');
    assert.equal(sha256(readFileSync(captureScriptPath)), buildProvenance.captureScript.sha256,
      'Android capture script changed during the run.');
    metadata.apkSha256 = localApkSha256;
    metadata.installedApkSha256 = installedApkSha256;
    metadata.screenshotScenes = screenshotFilter ?? scenes.map(({ id }) => id);
    metadata.sourceRevision = finalSource;
    await writeFile(resolve(evidenceDirectory, 'android-simulator.json'),
      `${JSON.stringify({ ...metadata, observations }, null, 2)}\n`);
  } finally {
    adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
  }
}

async function runIOS() {
  const initialDevices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
  const initial = Object.values(initialDevices).flat().find(({ udid }) => udid === serialOrUDID);
  assert.ok(initial, `Simulator UDID를 찾지 못했습니다: ${serialOrUDID}`);
  let bootedByThisRun = false;
  try {
    if (initial.state !== 'Booted') {
      command('xcrun', ['simctl', 'boot', serialOrUDID]);
      bootedByThisRun = true;
    }
    command('xcrun', ['simctl', 'bootstatus', serialOrUDID, '-b'], { timeout: 180_000 });
    const devices = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout).devices;
    const simulator = Object.entries(devices).flatMap(([runtime, values]) => values
      .filter(({ udid, state }) => udid === serialOrUDID && state === 'Booted')
      .map((device) => ({ ...device, runtime })));
    assert.equal(simulator.length, 1);
    assert.equal(simulator[0].deviceTypeIdentifier,
      'com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro');
    assert.match(simulator[0].runtime, /iOS-26-2$/);
    const buildProvenance = await buildApplication();
    const screenshotVerifierSha256 = sha256(readFileSync(iosScreenshotVerifierPath));
    const appPath = bundlePath;
    await writeFile(resolve(evidenceDirectory, 'ios-install.log'),
      normalizeEvidenceText(command('xcrun', ['simctl', 'install', serialOrUDID, appPath]).stdout));
    const appSha256 = sha256(await readFile(resolve(appPath, 'SpinonBootstrap')));
    assert.equal(appSha256, buildProvenance.artifactSha256,
      'iOS app가 capture용 current-source build와 다릅니다.');
    const observations = [];
    for (const [index, scene] of scenes.entries()) {
      const runID = `${runPrefix}-${index}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        serialOrUDID, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-run-id=${runID}`, `--spinon-c114-grid-scene=${scene.id}`]);
      await writeFile(resolve(evidenceDirectory, `ios-launch-${scene.id}.log`),
        normalizeEvidenceText(`${launch.stdout}${launch.stderr}`));
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: iOS launch PID 없음`);
      const predicate = 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C114_GRID" '
        + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW" '
        + 'OR eventMessage CONTAINS[c] "SPINON_C114_GRID_NODE_FRAME")';
      let log = '';
      const deadline = Date.now() + 150_000;
      while (Date.now() < deadline) {
        log = command('xcrun', ['simctl', 'spawn', serialOrUDID, 'log', 'show', '--last', '90s',
          '--style', 'syslog', '--predicate', predicate], { timeout: 45_000 }).stdout;
        const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
        log = log.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
        if (log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || log.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        if (sceneCaptureComplete(log, runID, scene)) break;
        await sleep(500);
      }
      const runtimeLogPath = resolve(evidenceDirectory, `ios-runtime-${scene.id}.log`);
      const runtimeLog = normalizeEvidenceText(log);
      await writeFile(runtimeLogPath, runtimeLog);
      const observation = verifyScene(runtimeLog, scene, runID);
      observation.runtimeLogSha256 = sha256(await readFile(runtimeLogPath));
      if (shouldCaptureScreenshot(scene)) {
        const screenshotPath = resolve(evidenceDirectory, `ios-${scene.id}.png`);
        command('xcrun', ['simctl', 'io', serialOrUDID, 'screenshot', screenshotPath]);
        const screenshot = await readFile(screenshotPath);
        assertPngSignature(screenshot, `iOS screenshot ${scene.id}`);
        observation.screenshotPixels = assertImageDecodes(screenshotPath, {
          width: 1206,
          height: 2622,
        });
        observation.screenshotOcrText = command('mise', ['exec', '--', 'swift',
          iosScreenshotVerifierPath, screenshotPath], { timeout: 120_000 }).stdout.trim();
        observation.screenshotSha256 = sha256(screenshot);
      }
      observations.push(observation);
      console.log(`iOS Simulator ${scene.id}: PASS`);
    }
    const finalSource = sourceRevision();
    assert.equal(finalSource.sourceFilesSha256, buildProvenance.source.sourceFilesSha256,
      'iOS capture source differs from the source used for its app build.');
    assert.equal(sha256(readFileSync(captureScriptPath)), buildProvenance.captureScript.sha256,
      'iOS capture script changed during the run.');
    assert.equal(sha256(readFileSync(iosScreenshotVerifierPath)), screenshotVerifierSha256,
      'iOS screenshot verifier changed during the run.');
    await writeFile(resolve(evidenceDirectory, 'ios-simulator.json'),
      `${JSON.stringify({
        target, udid: serialOrUDID, name: simulator[0].name, runtime: simulator[0].runtime,
        deviceTypeIdentifier: simulator[0].deviceTypeIdentifier,
        screenshotScenes: screenshotFilter ?? scenes.map(({ id }) => id),
        appSha256, buildProvenance,
        screenshotVerifier: {
          path: iosScreenshotVerifierPath.slice(repositoryRoot.length + 1),
          sha256: screenshotVerifierSha256,
        },
        sourceRevision: finalSource, observations,
      }, null, 2)}\n`);
  } finally {
    command('xcrun', ['simctl', 'terminate', serialOrUDID, packageName], { allowFailure: true });
    if (bootedByThisRun) {
      command('xcrun', ['simctl', 'shutdown', serialOrUDID], { allowFailure: true });
    }
  }
}

await mkdir(evidenceDirectory, { recursive: true });
if (target === 'android-avd') await runAndroid();
else await runIOS();
console.log(`Capture evidence: ${evidenceDirectory.replaceAll(repositoryRoot, '<REPO>')}`);
