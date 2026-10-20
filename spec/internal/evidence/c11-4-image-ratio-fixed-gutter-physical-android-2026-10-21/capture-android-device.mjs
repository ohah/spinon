import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const evidenceDirectory = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDirectory, '../../../../');
const serial = process.argv[2];
assert.match(serial ?? '', /^[A-Za-z0-9._:-]+$/, '사용법: node capture-android-device.mjs <ADB serial>');

const packageName = 'dev.spinon.bootstrap';
const component = `${packageName}/.MainActivity`;
const apkPath = resolve(repositoryRoot, 'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const timestamp = new Date().toISOString().replaceAll(/[-:.]/g, '');
const runID = `c114-fg-device-${timestamp}`;
const runDirectory = resolve(evidenceDirectory, runID);
const scenes = [
  {
    id: 'fixed-gutter-column-suggestion-clamp',
    samples: [
      { id: 'grid-border', nodeId: 'c114-fixed-gutter-column-grid', xCssPx: 0, yCssPx: 2, rgba: [34, 197, 94, 255] },
      { id: 'inside-item', nodeId: 'c114-fixed-gutter-column-item', xCssPx: 2, yCssPx: 2, rgba: [229, 72, 77, 255] },
      { id: 'right-of-grid', nodeId: 'c114-fixed-gutter-column-item', xCssPx: 12, yCssPx: 2, rgba: [229, 72, 77, 255] },
      { id: 'below-grid', nodeId: 'c114-fixed-gutter-column-item', xCssPx: 2, yCssPx: 6, rgba: [229, 72, 77, 255] },
    ],
  },
  {
    id: 'fixed-gutter-row-suggestion-clamp',
    samples: [
      { id: 'grid-border', nodeId: 'c114-fixed-gutter-row-grid', xCssPx: 0, yCssPx: 2, rgba: [34, 197, 94, 255] },
      { id: 'inside-item', nodeId: 'c114-fixed-gutter-row-item', xCssPx: 2, yCssPx: 2, rgba: [229, 72, 77, 255] },
      { id: 'right-of-grid', nodeId: 'c114-fixed-gutter-row-item', xCssPx: 6, yCssPx: 2, rgba: [229, 72, 77, 255] },
      { id: 'below-grid', nodeId: 'c114-fixed-gutter-row-item', xCssPx: 2, yCssPx: 12, rgba: [229, 72, 77, 255] },
    ],
  },
];

function command(program, args, encoding = 'utf8') {
  const result = spawnSync(program, args, { encoding, maxBuffer: 64 * 1024 * 1024 });
  if (result.error) throw result.error;
  assert.equal(result.status, 0, `${program} ${args.join(' ')} failed: ${result.stderr ?? ''}`);
  return result.stdout;
}

const adb = (args, encoding = 'utf8') => command('adb', ['-s', serial, ...args], encoding);
const getprop = (name) => adb(['shell', 'getprop', name]).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => new Promise((resolveSleep) => setTimeout(resolveSleep, milliseconds));
const saveText = (name, value) => writeFile(resolve(runDirectory, name), `${value.trimEnd()}\n`);

await mkdir(runDirectory, { recursive: true });
const deviceList = command('adb', ['devices', '-l']);
const matchingDevices = deviceList.split(/\r?\n/)
  .filter((line) => line.startsWith(`${serial} `) && line.includes(' device '));
assert.equal(matchingDevices.length, 1, `ADB 실기기 연결을 확인할 수 없습니다: ${serial}`);

const device = {
  serial,
  manufacturer: getprop('ro.product.manufacturer'),
  model: getprop('ro.product.model'),
  release: getprop('ro.build.version.release'),
  apiLevel: Number(getprop('ro.build.version.sdk')),
  abi: getprop('ro.product.cpu.abi'),
  qemu: getprop('ro.kernel.qemu') || '0',
  eglDriver: getprop('ro.hardware.egl'),
  vulkanDriver: getprop('ro.hardware.vulkan'),
  displaySize: adb(['shell', 'wm', 'size']).trim(),
  displayDensity: adb(['shell', 'wm', 'density']).trim(),
  gpuRenderer: adb(['shell', 'dumpsys', 'SurfaceFlinger'])
    .split(/\r?\n/).find((line) => line.includes('GLES:'))?.trim().replace(/^GLES:\s*/, ''),
  sourceHead: command('git', ['-C', repositoryRoot, 'rev-parse', 'HEAD']).trim(),
};
assert.equal(device.model, 'SM-S731N', `예상한 Samsung 실기기가 아닙니다: ${device.model}`);
assert.equal(device.apiLevel, 36, `Android 16/API 36 기기가 아닙니다: ${device.apiLevel}`);
assert.match(device.abi, /arm64-v8a/, `ARM64 기기가 아닙니다: ${device.abi}`);
assert.equal(device.qemu, '0', `실기기가 아닌 에뮬레이터입니다: qemu=${device.qemu}`);
assert.match(device.gpuRenderer ?? '', /Xclipse 940.*Vulkan/i,
  `예상한 물리 GPU renderer를 확인하지 못했습니다: ${device.gpuRenderer}`);
await writeFile(resolve(runDirectory, 'device.json'), `${JSON.stringify(device, null, 2)}\n`);
await saveText('adb-device-line.txt', matchingDevices[0]);
await saveText('gpu-renderer.txt', device.gpuRenderer);

const apkBytes = await readFile(apkPath);
const localApkSha256 = sha256(apkBytes);
const packagePaths = adb(['shell', 'pm', 'path', packageName]).split(/\r?\n/)
  .map((line) => line.trim().replace(/^package:/, ''));
const installedBaseApk = packagePaths.find((path) => path.endsWith('/base.apk'));
assert.ok(installedBaseApk, `설치된 ${packageName}의 base.apk를 찾지 못했습니다.`);
let installedApkSha256 = adb(['shell', 'sha256sum', installedBaseApk]).trim().split(/\s+/)[0];
if (installedApkSha256 !== localApkSha256) {
  await saveText('install.log', adb(['install', '-r', apkPath]));
  installedApkSha256 = adb(['shell', 'sha256sum', installedBaseApk]).trim().split(/\s+/)[0];
} else {
  await saveText('install.log', '설치 APK와 현재 debug APK SHA-256이 이미 일치해 재설치를 생략했습니다.');
}
assert.equal(installedApkSha256, localApkSha256, '설치 APK와 현재 debug APK가 다릅니다.');
await writeFile(resolve(runDirectory, 'installed-apk-verification.json'), `${JSON.stringify({
  apkPath: 'platforms/android/app/build/outputs/apk/debug/app-debug.apk',
  localApkSha256,
  installedBaseApk,
  installedApkSha256,
  matched: installedApkSha256 === localApkSha256,
}, null, 2)}\n`);

adb(['shell', 'am', 'force-stop', packageName]);
const sceneResults = [];
for (const scene of scenes) {
  const sceneRunID = `${runID}-${scene.id}`;
  const encodedSamples = Buffer.from(JSON.stringify(scene.samples)).toString('base64url');
  const launchLog = adb(['shell', 'am', 'start', '-n', component,
    '--ez', 'spinon_c114_grid_track_sizing', 'true',
    '--es', 'spinon_c114_grid_scene', scene.id,
    '--es', 'spinon_c114_grid_run_id', sceneRunID,
    '--es', 'spinon_c11_grid_pixel_matrix_run_id', sceneRunID,
    '--es', 'spinon_c11_grid_pixel_matrix_samples', encodedSamples]);
  await saveText(`launch-${scene.id}.log`, launchLog);

  let runtimeLog = '';
  let complete = false;
  for (let attempt = 0; attempt < 45; attempt += 1) {
    runtimeLog = adb(['logcat', '-d', '-v', 'time', '-s', 'SpinonBootstrap'])
      .split(/\r?\n/).filter((line) => line.includes(sceneRunID)).join('\n');
    const scenePassed = runtimeLog.includes(
      `SPINON_C114_GRID_SUMMARY run_id=${sceneRunID} scene_id=${scene.id} `)
      && /SPINON_C114_GRID_SUMMARY .*status=0/.test(runtimeLog);
    const pixelsPassed = runtimeLog.includes(
      `SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${sceneRunID} `)
      && /expected_samples=4 observed_samples=4 matched_samples=4/.test(runtimeLog);
    if (scenePassed && pixelsPassed) {
      complete = true;
      break;
    }
    await sleep(1000);
  }
  await saveText(`runtime-${scene.id}.log`, runtimeLog);
  assert.ok(complete, `${scene.id}: frame/trace 또는 PixelCopy 검증 제한 시간 초과.\n${runtimeLog}`);
  assert.ok(!/SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_ARGUMENT_ERROR|SPINON_C114_GRID_PIXEL_MATRIX status=error/.test(runtimeLog),
    `${scene.id}: Android runtime 오류가 있습니다.`);
  assert.ok(runtimeLog.includes('fixed_track_sum: Some(7.0)')
    && runtimeLog.includes('fixed_gutter_sum: Some(3.0)')
    && runtimeLog.includes('fixed_track_limit: Some(10.0)'),
  `${scene.id}: fixed track/gutter clamp trace가 없습니다.`);

  const screen = adb(['exec-out', 'screencap', '-p'], null);
  assert.ok(screen.length > 0, `${scene.id}: 기기 화면 캡처가 비었습니다.`);
  await writeFile(resolve(runDirectory, `screen-${scene.id}.png`), screen);
  const surfacePng = adb(['exec-out', 'run-as', packageName, 'cat',
    `cache/c114-grid-surface-pixel-matrix/${scene.id}-${sceneRunID}.png`], null);
  assert.ok(surfacePng.length > 0, `${scene.id}: SurfaceView PixelCopy PNG가 비었습니다.`);
  await writeFile(resolve(runDirectory, `surface-${scene.id}.png`), surfacePng);
  sceneResults.push({
    sceneID: scene.id,
    runID: sceneRunID,
    status: 'passed',
    summary: runtimeLog.split(/\r?\n/).find((line) => line.includes('SPINON_C114_GRID_SUMMARY')),
    pixelSummary: runtimeLog.split(/\r?\n/).find((line) => line.includes('SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY')),
    runtimeLogSha256: sha256(Buffer.from(runtimeLog)),
    screenSha256: sha256(screen),
    surfacePixelCopySha256: sha256(surfacePng),
    expectedSamples: 4,
    exactSamples: 4,
  });
  if (scene !== scenes.at(-1)) adb(['shell', 'am', 'force-stop', packageName]);
  process.stdout.write(`${scene.id}: Android physical frame/trace + 4/4 exact PixelCopy passed\n`);
}

await writeFile(resolve(runDirectory, 'result.json'), `${JSON.stringify({
  schema: 'spinon-c11-4-image-ratio-fixed-gutter-android-device/v1',
  runID,
  status: 'passed',
  scope: 'Samsung SM-S731N physical Android device; fixed-gutter image ratio contribution scenes and four native surface samples per scene.',
  device,
  apk: { localApkSha256, installedApkSha256, matched: installedApkSha256 === localApkSha256 },
  scenes: sceneResults,
  limits: ['One Android device and one build only.', 'PixelCopy samples validate selected surface colors, not PNG texels or post-compositor output.'],
}, null, 2)}\n`);
process.stdout.write(`${JSON.stringify({ runDirectory, runID, device, scenes: sceneResults }, null, 2)}\n`);
