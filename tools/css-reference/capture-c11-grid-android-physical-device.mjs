import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const [serial = ''] = process.argv.slice(2);
assert.match(serial, /^[A-Za-z0-9._:-]+$/, '사용법: node capture-c11-grid-android-physical-device.mjs <실기기 serial>');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const referencePath = resolve(repositoryRoot, 'tests/fixtures/css/references/c11-grid-precomparison-v2.json');
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const evidenceDate = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());
const runID = `c11-grid-android-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(repositoryRoot,
  `spec/internal/evidence/c11-grid-android-physical-device-${evidenceDate}/${runID}`);
const expectedSamples = new Map([
  ['grid-background', [237, 241, 247, 255]],
  ['item-blue', [40, 102, 246, 255]],
  ['item-red', [229, 72, 77, 255]],
  ['item-green', [25, 169, 116, 255]],
  ['item-yellow', [255, 200, 87, 255]],
  ['column-gap', [237, 241, 247, 255]],
  ['row-gap', [237, 241, 247, 255]],
]);

function command(binary, args, { timeout = 30_000, encoding = 'utf8', allowFailure = false } = {}) {
  const result = spawnSync(binary, args, {
    cwd: repositoryRoot,
    encoding,
    timeout,
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

function adb(args, options) {
  return command('adb', ['-s', serial, ...args], options);
}

function sleep(milliseconds) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
}

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

async function main() {
  await mkdir(evidenceDirectory, { recursive: true });
  const deviceList = command('adb', ['devices', '-l']).stdout.split(/\r?\n/)
    .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
  assert.equal(deviceList.length, 1, `Android 기기가 연결되지 않았습니다: ${serial}`);
  assert.equal(adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(), '0', '실기기 전용 수집기에 에뮬레이터가 연결됐습니다.');

  const build = command('mise', ['exec', '--', 'env', 'SPINON_ENABLE_C04_RUNTIME_GPU=1',
    'bun', 'run', 'build:android'], { timeout: 900_000 });
  await writeFile(resolve(evidenceDirectory, 'build.log'), `${build.stdout}${build.stderr}`);
  assert.match(`${build.stdout}${build.stderr}`, /BUILD SUCCESSFUL/i, 'Android 빌드가 성공하지 않았습니다.');

  const apk = await readFile(apkPath);
  const apkSha256 = createHash('sha256').update(apk).digest('hex');
  const model = adb(['shell', 'getprop', 'ro.product.model']).stdout.trim();
  const release = adb(['shell', 'getprop', 'ro.build.version.release']).stdout.trim();
  const apiLevel = Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim());
  const renderer = adb(['shell', 'dumpsys', 'SurfaceFlinger']).stdout.split(/\r?\n/)
    .find((line) => /^GLES:/i.test(line)) ?? 'SurfaceFlinger renderer를 읽지 못했습니다.';
  await writeFile(resolve(evidenceDirectory, 'device.txt'), [
    `serial=${serial}`, `model=${model}`, `android_release=${release}`,
    `api_level=${apiLevel}`, `qemu=0`, `surfaceflinger_renderer=${renderer}`,
    `apk_sha256=${apkSha256}`, '',
  ].join('\n'));

  await writeFile(resolve(evidenceDirectory, 'install.log'),
    command('adb', ['-s', serial, 'install', '-r', apkPath], { timeout: 180_000 }).stdout);
  adb(['shell', 'am', 'force-stop', packageName]);
  adb(['logcat', '-c']);
  const launch = adb(['shell', 'am', 'start', '-W', '-n', component,
    '--ez', 'spinon_c11_grid_fixture', 'true']);
  await writeFile(resolve(evidenceDirectory, 'launch.log'), `${launch.stdout}${launch.stderr}`);
  assert.match(launch.stdout, /Status:\s*ok/, 'C11.1 Android scene을 시작하지 못했습니다.');

  let log = '';
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    log = adb(['logcat', '-d', '-s', 'SpinonBootstrap:I']).stdout;
    if (log.includes('SPINON_C111_GRID_SURFACE_SUMMARY status=matched')
        || log.includes('SPINON_C111_GRID_SURFACE_SUMMARY status=mismatch')
        || log.includes('SPINON_C111_GRID_SURFACE_CAPTURE status=error')) break;
    sleep(250);
  }
  await writeFile(resolve(evidenceDirectory, 'runtime.log'), log);

  assert.match(log, /SPINON_C111_GRID_HOST=session=ready/, 'V8 host 준비 로그가 없습니다.');
  assert.match(log, /SPINON_C111_GRID_EVAL status=0 .*layout=ready boxes=6/, 'V8→layout Grid scene과 runtime wrapper가 준비되지 않았습니다.');
  assert.match(log, /SPINON_C0410_DRAW .*status=0 presented boxes=6/, 'WGPU surface에 wrapper와 Grid scene을 제출하지 못했습니다.');
  assert.match(log, /SPINON_C111_GRID_SURFACE_SUMMARY status=matched .*expected_samples=7 observed_samples=7 matched_samples=7/,
    'Android PixelCopy 표본의 요약 결과가 일치하지 않습니다.');

  const reference = JSON.parse(await readFile(referencePath, 'utf8'));
  const referenceCase = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1)
    ?.cases.find(({ id }) => id === 'fixed-tracks-gap');
  assert.ok(referenceCase, '고정 Chromium fixed-tracks-gap scene이 없습니다.');
  const expectedFrames = referenceCase.nodes.map(({ rect }) => rect);
  const renderFrames = parseFrames(log);
  assert.equal(renderFrames.length, expectedFrames.length + 1, 'scene wrapper를 포함한 frame 수가 다릅니다.');
  const wrapperFrame = renderFrames[0];
  assert.equal(wrapperFrame.nodeId, 1, 'runtime wrapper의 preorder NodeId가 다릅니다.');
  for (const [field, expected] of Object.entries({ x: 0, y: 0, width: 360, height: 800 })) {
    assert.ok(Math.abs(wrapperFrame.rect[field] - expected) <= 0.5,
      `runtime wrapper ${field}: actual=${wrapperFrame.rect[field]} expected=${expected}`);
  }
  const actualFrames = renderFrames.slice(1);
  assert.equal(actualFrames.length, expectedFrames.length, 'Grid data node frame 수가 기준과 다릅니다.');
  for (const [index, expected] of expectedFrames.entries()) {
    const actual = actualFrames[index];
    assert.equal(actual.nodeId, index + 2, `frame ${index}: source-order NodeId가 다릅니다.`);
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Math.abs(actual.rect[field] - expected[field]) <= 0.5,
        `frame ${index} ${field}: actual=${actual.rect[field]} expected=${expected[field]}`);
    }
  }

  const pixels = parsePixels(log);
  assert.equal(pixels.length, expectedSamples.size, 'PixelCopy 표본 수가 다릅니다.');
  assert.deepEqual(new Set(pixels.map(({ id }) => id)), new Set(expectedSamples.keys()), 'PixelCopy 표본 ID가 다릅니다.');
  for (const pixel of pixels) {
    assert.deepEqual(pixel.expected, expectedSamples.get(pixel.id), `${pixel.id}: 고정 expected 색이 바뀌었습니다.`);
    assert.deepEqual(pixel.rgba, pixel.expected, `${pixel.id}: 실제 surface pixel이 다릅니다.`);
    assert.equal(pixel.exact, true, `${pixel.id}: runtime summary가 표본을 불일치로 판정했습니다.`);
  }

  const pngPath = log.match(/SPINON_C111_GRID_SURFACE_CAPTURE status=ok .* png=([^\s]+) sha256=([a-f0-9]{64})/)?.[1];
  assert.ok(pngPath, 'PixelCopy PNG 경로 또는 해시가 없습니다.');
  const cacheRelativePath = `cache/c11-grid-surface-captures/${pngPath.split('/').at(-1)}`;
  const pngBytes = adb(['exec-out', 'run-as', packageName, 'cat', cacheRelativePath], { encoding: null }).stdout;
  assert.equal(pngBytes.toString('hex', 0, 8), '89504e470d0a1a0a', 'PixelCopy 결과가 PNG가 아닙니다.');
  const pngSha256 = createHash('sha256').update(pngBytes).digest('hex');
  const loggedPngSha256 = log.match(/SPINON_C111_GRID_SURFACE_CAPTURE status=ok .* sha256=([a-f0-9]{64})/)?.[1];
  assert.equal(pngSha256, loggedPngSha256, 'PixelCopy PNG 해시가 runtime 로그와 다릅니다.');
  await writeFile(resolve(evidenceDirectory, 'surface-pixelcopy.png'), pngBytes);
  const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
  await writeFile(resolve(evidenceDirectory, 'device-screen.png'), screenshot);

  const comparison = {
    schema: 'spinon-c11-grid-android-physical/v1',
    status: 'matched', runID, serial, model, release, apiLevel, apkSha256,
    renderer: 'WGPU Vulkan; Samsung Xclipse 940; SurfaceView PixelCopy',
    oracle: { referenceId: reference.referenceId, chrome: reference.oracle.product,
      referenceSha256: createHash('sha256').update(await readFile(referencePath)).digest('hex'),
      scene: referenceCase.id },
    geometry: { expectedDataNodeCount: expectedFrames.length, runtimeWrapperFrame: wrapperFrame,
      actualDataFrames: actualFrames, expectedDataFrames: expectedFrames, toleranceCssPx: 0.5 },
    surfacePixels: { expectedCount: expectedSamples.size, actualCount: pixels.length,
      maximumChannelError: 0, samples: pixels, png: 'surface-pixelcopy.png' },
    limitations: [
      '고정 C11.1 fixed-tracks-gap scene 1개·5 data node와 1 runtime wrapper에 한정한다.',
      'Android SM-S731N 한 대의 검증이며 다른 기기군 성능·호환성을 뜻하지 않는다.',
      'C11.1 전체 완료·전체 CSS 지원·Spinon WPT 적합성을 뜻하지 않는다.',
    ],
  };
  await writeFile(resolve(evidenceDirectory, 'comparison.json'), `${JSON.stringify(comparison, null, 2)}\n`);
  await writeFile(resolve(evidenceDirectory, 'README.md'), [
    '# C11.1 Android 실기기 V8→WGPU 검증', '',
    `- 실행 ID: ${runID}`, `- 기기: ${model} · Android ${release}/API ${apiLevel} · serial ${serial}`,
    `- APK SHA-256: \`${apkSha256}\``, '- backend: Samsung Xclipse 940 / Vulkan',
    '- V8 runtime host와 C11.1 scene 평가가 성공했다. runtime wrapper 1개를 별도 확인하고, 5개 data node frame을 pinned Chrome 154 기준과 비교했다.',
    '- `SurfaceView`를 `PixelCopy`로 읽은 7/7 표본이 CSS 기대 RGBA와 완전히 일치했다.',
    '- [기기 화면](./device-screen.png) · [surface PixelCopy](./surface-pixelcopy.png) · [원본 로그](./runtime.log) · [비교 JSON](./comparison.json)',
    '- C11.1 fixed-track 한 장면만 검증했다. iOS Metal 직접 readback, 나머지 Chromium scene, 공식 WPT 적합성은 별도 관문이다.', '',
  ].join('\n'));
  process.stdout.write(`${JSON.stringify({ status: 'matched', runID, model, release, apiLevel,
    apkSha256, frames: actualFrames.length, exactPixels: pixels.length, evidenceDirectory }, null, 2)}\n`);
}

main().catch(async (error) => {
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(resolve(evidenceDirectory, 'failure.txt'), `${error.stack ?? error}\n`);
  process.stderr.write(`${error.stack ?? error}\n`);
  process.exitCode = 1;
});
