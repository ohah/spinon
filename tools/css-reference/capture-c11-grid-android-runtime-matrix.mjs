import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const [serial = ''] = process.argv.slice(2);
assert.match(serial, /^[A-Za-z0-9._:-]+$/, '사용법: node capture-c11-grid-android-runtime-matrix.mjs <실기기 serial>');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const matrixPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/runtime-grid-matrix.json');
const referencePath = resolve(repositoryRoot, 'tests/fixtures/css/references/c11-grid-precomparison-v2.json');
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const runID = `c11-grid-android-matrix-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-grid-android-physical-device-${date}/${runID}`);
const pixelColors = new Map([
  ['grid-background', [237, 241, 247, 255]], ['item-blue', [40, 102, 246, 255]],
  ['item-red', [229, 72, 77, 255]], ['item-green', [25, 169, 116, 255]],
  ['item-yellow', [255, 200, 87, 255]], ['column-gap', [237, 241, 247, 255]],
  ['row-gap', [237, 241, 247, 255]],
]);

function command(binary, args, { timeout = 30_000, encoding = 'utf8', allowFailure = false } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot, encoding, timeout, maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

const adb = (args, options) => command('adb', ['-s', serial, ...args], options);
const sha256 = (value) => createHash('sha256').update(value).digest('hex');
const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

function parseFrames(log) {
  return [...log.matchAll(/SPINON_C111_GRID_NODE_FRAME \d+:node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g)]
    .map((match) => ({
      nodeId: Number(match[1]),
      rect: { x: Number(match[2]), y: Number(match[3]), width: Number(match[4]), height: Number(match[5]) },
    }));
}

function parsePixels(log) {
  return [...log.matchAll(/SPINON_C111_GRID_SURFACE_PIXEL run_id=\S+ sample=(\S+) x=(\d+) y=(\d+) rgba=(\d+),(\d+),(\d+),(\d+) expected=(\d+),(\d+),(\d+),(\d+) exact=(true|false)/g)]
    .map((match) => ({
      id: match[1], x: Number(match[2]), y: Number(match[3]),
      rgba: match.slice(4, 8).map(Number), expected: match.slice(8, 12).map(Number),
      exact: match[12] === 'true',
    }));
}

function logForProcess(log, processId) {
  return log.split(/\r?\n/).filter((line) => {
    const fields = line.match(/^\d{2}-\d{2}\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:/);
    return fields && Number(fields[1]) === processId;
  }).join('\n');
}

function processId() {
  return adb(['shell', 'pidof', packageName], { timeout: 10_000, allowFailure: true }).stdout.trim()
    .split(/\s+/).filter(Boolean).map(Number)[0] ?? null;
}

function waitFor(predicate, description, timeout = 120_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const value = predicate();
    if (value) return value;
    sleep(250);
  }
  throw new Error(`${description} 대기 시간 초과`);
}

function assertGeometry(actual, expected, sceneId, nodeIdOffset = 0) {
  assert.equal(actual.length, expected.length, `${sceneId}: frame 수`);
  for (const [index, expectedNode] of expected.entries()) {
    const frame = actual[index];
    assert.equal(frame.nodeId, index + 1 + nodeIdOffset, `${sceneId}/${expectedNode.id}: preorder NodeId`);
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Math.abs(frame.rect[field] - expectedNode.rect[field]) <= 0.5,
        `${sceneId}/${expectedNode.id}/${field}: actual=${frame.rect[field]} expected=${expectedNode.rect[field]}`);
    }
  }
}

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const connected = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(connected.length, 1, `Android 기기가 연결되지 않았습니다: ${serial}`);
  assert.equal(adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(), '0', '에뮬레이터는 이 실기기 수집기에 허용되지 않습니다.');

  const matrixBytes = await readFile(matrixPath);
  const matrix = JSON.parse(matrixBytes.toString('utf8'));
  const referenceBytes = await readFile(referencePath);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const expectedCases = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1).cases;
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.nodeCount, 40);
  assert.deepEqual(matrix.cases.map(({ id }) => id), expectedCases.map(({ id }) => id));
  assert.equal(matrix.inputs.chromiumReference.sha256, sha256(referenceBytes), 'runtime matrix가 현재 Chrome reference에 고정되지 않았습니다.');
  command('mise', ['exec', '--', 'node', 'tools/css-reference/build-c11-grid-runtime-matrix.mjs', '--check']);

  const build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1', 'SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1', 'bun', 'run', 'build:android'],
    { timeout: 900_000 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.match(`${build.stdout}${build.stderr}`, /BUILD SUCCESSFUL/i, 'Android 빌드가 성공하지 않았습니다.');
  const apkBytes = await readFile(apkPath);
  const apkSha256 = sha256(apkBytes);
  await writeFile(resolve(evidenceDirectory, 'install.log'),
    command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 180_000 }).stdout);

  const model = adb(['shell', 'getprop', 'ro.product.model']).stdout.trim();
  const manufacturer = adb(['shell', 'getprop', 'ro.product.manufacturer']).stdout.trim();
  const release = adb(['shell', 'getprop', 'ro.build.version.release']).stdout.trim();
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger renderer를 읽지 못했습니다.';
  const device = { serial, manufacturer, model, release, apiLevel, qemu: false, renderer, apkSha256 };
  await writeFile(resolve(evidenceDirectory, 'device.txt'), `${Object.entries(device)
    .map(([key, value]) => `${key}=${value}`).join('\n')}\n`);

  const observations = [];
  for (const [index, fixture] of matrix.cases.entries()) {
    const expectedCase = expectedCases[index];
    assert.equal(expectedCase.id, fixture.id);
    adb(['shell', 'am', 'force-stop', packageName]);
    waitFor(() => processId() === null, `${fixture.id}: 이전 앱 프로세스 종료`);
    adb(['logcat', '-c']);
    const launch = adb(['shell', 'am', 'start', '-W', '-n', component,
      '--es', 'spinon_c11_grid_case', fixture.id]);
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-launch.log`), `${launch.stdout}${launch.stderr}`);
    assert.match(launch.stdout, /Status:\s*ok/, `${fixture.id}: 앱 시작`);
    const pid = waitFor(processId, `${fixture.id}: 새 앱 프로세스 시작`, 30_000);

    let completeLog = '';
    let log = '';
    const runtimeDeadline = Date.now() + 120_000;
    while (Date.now() < runtimeDeadline) {
      completeLog = logForProcess(adb(['logcat', '-d', '-v', 'threadtime', '-s', 'SpinonBootstrap:I']).stdout, pid);
      const failedEval = completeLog.match(/SPINON_C111_GRID_EVAL status=(-\d+) scene_id=([^\s]+)/);
      if (failedEval) {
        await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), completeLog);
        throw new Error(`${fixture.id}: runtime eval 실패 ${failedEval[0]}`);
      }
      const evaluated = completeLog.includes(`SPINON_C111_GRID_EVAL status=0 scene_id=${fixture.id}`);
      const expectedRenderBoxes = fixture.nodeIds.length + 1;
      const drawn = completeLog.includes(`SPINON_C0410_DRAW `)
        && completeLog.includes(`presented boxes=${expectedRenderBoxes}`);
      const pixelsReady = index !== 0 || completeLog.includes('SPINON_C111_GRID_SURFACE_SUMMARY status=matched');
      if (evaluated && drawn && pixelsReady) {
        log = completeLog;
        break;
      }
      sleep(250);
    }
    if (!log) {
      await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), completeLog);
      throw new Error(`${fixture.id}: V8, layout, WGPU draw${index === 0 ? ', PixelCopy' : ''} 대기 시간 초과`);
    }
    await writeFile(resolve(evidenceDirectory, `${fixture.id}-runtime.log`), log);
    assert.match(log, new RegExp(`SPINON_C111_GRID_HOST=session=ready`), `${fixture.id}: V8 host 준비`);
    const expectedRenderBoxes = fixture.nodeIds.length + 1;
    assert.match(log, new RegExp(`SPINON_C111_GRID_EVAL status=0 scene_id=${fixture.id} .*layout=ready boxes=${expectedRenderBoxes}`),
      `${fixture.id}: V8→Stylo→Taffy 평가`);
    assert.match(log, new RegExp(`SPINON_C0410_DRAW .*status=0 presented boxes=${expectedRenderBoxes}`),
      `${fixture.id}: WGPU surface 제출`);

    const renderFrames = parseFrames(log);
    assert.equal(renderFrames.length, expectedRenderBoxes, `${fixture.id}: scene wrapper 포함 box 수`);
    const wrapperFrame = renderFrames[0];
    assert.equal(wrapperFrame.nodeId, 1, `${fixture.id}: scene wrapper preorder NodeId`);
    for (const [field, expected] of Object.entries({ x: 0, y: 0, width: 360, height: 800 })) {
      assert.ok(Math.abs(wrapperFrame.rect[field] - expected) <= 0.5,
        `${fixture.id}/scene-root/${field}: actual=${wrapperFrame.rect[field]} expected=${expected}`);
    }
    const actualFrames = renderFrames.slice(1);
    assertGeometry(actualFrames, expectedCase.nodes, fixture.id, 1);
    sleep(250);
    const screen = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
    assert.equal(screen.toString('hex', 0, 8), '89504e470d0a1a0a', `${fixture.id}: 화면 캡처 PNG`);
    const screenshotPath = `${fixture.id}-device-screen.png`;
    await writeFile(resolve(evidenceDirectory, screenshotPath), screen);

    let surfacePixelEvidence = null;
    if (index === 0) {
      assert.match(log, /SPINON_C111_GRID_SURFACE_SUMMARY status=matched .*expected_samples=7 observed_samples=7 matched_samples=7/,
        'fixed-tracks-gap PixelCopy 표본 요약');
      const pixels = parsePixels(log);
      assert.equal(pixels.length, pixelColors.size);
      assert.deepEqual(new Set(pixels.map(({ id }) => id)), new Set(pixelColors.keys()));
      for (const pixel of pixels) {
        assert.deepEqual(pixel.expected, pixelColors.get(pixel.id), `${pixel.id}: 고정 색 기준`);
        assert.deepEqual(pixel.rgba, pixel.expected, `${pixel.id}: PixelCopy 실제 색`);
        assert.equal(pixel.exact, true, `${pixel.id}: exact 표본`);
      }
      const pngPath = log.match(/SPINON_C111_GRID_SURFACE_CAPTURE status=ok .* png=([^\s]+) sha256=([a-f0-9]{64})/)?.[1];
      assert.ok(pngPath, 'PixelCopy PNG 경로가 없습니다.');
      const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat',
        `cache/c11-grid-surface-captures/${pngPath.split('/').at(-1)}`], { encoding: null }).stdout;
      assert.equal(pngBytes.toString('hex', 0, 8), '89504e470d0a1a0a');
      const loggedHash = log.match(/SPINON_C111_GRID_SURFACE_CAPTURE status=ok .* sha256=([a-f0-9]{64})/)?.[1];
      assert.equal(sha256(pngBytes), loggedHash, 'PixelCopy PNG hash');
      await writeFile(resolve(evidenceDirectory, 'fixed-tracks-gap-surface-pixelcopy.png'), pngBytes);
      surfacePixelEvidence = { count: pixels.length, maximumChannelError: 0, samples: pixels,
        png: 'fixed-tracks-gap-surface-pixelcopy.png' };
    }

    observations.push({
      sceneId: fixture.id, processId: pid, nodeCount: fixture.nodeIds.length,
      wrapperFrame, frames: actualFrames, screenshot: screenshotPath,
      surfacePixelEvidence,
    });
    process.stdout.write(`${fixture.id}: ${actualFrames.length}/${expectedCase.nodes.length} frame · pid=${pid} · matched\n`);
  }

  const comparison = {
    schema: 'spinon-c11-grid-android-runtime-matrix/v1',
    status: 'matched', runID, device, matrixSha256: sha256(matrixBytes),
    oracle: { referenceId: reference.referenceId, chrome: reference.oracle.product,
      referenceSha256: sha256(referenceBytes), toleranceCssPx: 0.5 },
    summary: { expectedScenes: 10, matchedScenes: observations.length,
      expectedNodes: 40, matchedNodes: observations.reduce((sum, scene) => sum + scene.nodeCount, 0),
      freshProcessPerScene: true, exactPixelCanaryScene: 'fixed-tracks-gap' },
    observations,
    limitations: [
      '단일 Android 실기기의 고정 C11.1 fixture geometry 비교다.',
      'PixelCopy exact 7/7은 fixed-tracks-gap 한 장면에만 적용된다. 다른 scene의 픽셀 전체를 비교하지 않았다.',
      '이 결과는 iOS 실행, WPT 적합성, 다른 기기군, 공개 CSS 전체 지원을 입증하지 않는다.',
    ],
  };
  await writeFile(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(comparison, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 Android 실기기 10-scene runtime 비교', '',
    `- 실행 ID: ${runID}`, `- 기기: ${manufacturer} ${model} · Android ${release}/API ${apiLevel}`,
    `- 그래픽: ${renderer}`, `- APK SHA-256: \`${apkSha256}\``,
    '- 고정 Chrome 154 reference의 10개 scene·40개 node를 scene별 새 앱 프로세스에서 실행했다.',
    '- 모든 node의 x/y/width/height를 CSS px 기준 ±0.5 이내로 비교했다.',
    '- `fixed-tracks-gap`의 Android `SurfaceView` PixelCopy는 7/7 표본 RGBA가 exact였다.',
    '- 다른 9개 scene은 frame geometry와 device screenshot만 수집했으며, 전체 scene pixel equality를 주장하지 않는다.',
    '- [상세 비교](./comparison.json) · 실행별 `*-runtime.log`, 화면 PNG, build/install 로그를 같은 디렉터리에 보존했다.',
    '- iOS Simulator, WPT, hardware GPU 외 기기군 및 전체 CSS 지원은 별도 관문이다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: 'matched', runID, device,
    scenes: observations.length, nodes: comparison.summary.matchedNodes,
    evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
