import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { createWriteStream } from 'node:fs';
import { readFile, writeFile } from 'node:fs/promises';
import { once } from 'node:events';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../../../../', import.meta.url)));
const serial = process.argv[2];
assert.ok(serial, '사용법: node run-physical-matrix.mjs <ADB serial>');

const baselinePath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-auto-fit-min-size-axis-2026-10-20/'
    + 'android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/result.json');
const baseline = JSON.parse(await readFile(baselinePath, 'utf8'));
const sceneIDs = baseline.observations.map(({ sceneId }) => sceneId);
assert.equal(sceneIDs.length, 53, '기준 Android AVD 행렬은 53 scene이어야 합니다.');
assert.equal(new Set(sceneIDs).size, sceneIDs.length, '기준 scene ID가 중복됐습니다.');

const evidenceDirectory = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21');
const outputLogPath = resolve(evidenceDirectory, 'run-physical-matrix.stdout.log');
const outputLog = createWriteStream(outputLogPath, { flags: 'w' });
const capturedStdout = [];
const args = [
  'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs',
  'android-device',
  serial,
  `--scene-id=${sceneIDs.join(',')}`,
  '--run-kind=auto-fit-min-size-axis',
  '--evidence-group=c11-4-auto-fit-min-size-axis-physical-android',
];
const child = spawn('node', args, { cwd: repositoryRoot, stdio: ['ignore', 'pipe', 'pipe'] });

child.stdout.on('data', (chunk) => {
  capturedStdout.push(chunk);
  outputLog.write(chunk);
  process.stdout.write(chunk);
});
child.stderr.on('data', (chunk) => {
  outputLog.write(chunk);
  process.stderr.write(chunk);
});

const [exitCode] = await once(child, 'close');
await new Promise((resolveEnd) => outputLog.end(resolveEnd));
assert.equal(exitCode, 0, `실기기 capture harness가 exit ${exitCode}로 끝났습니다.`);

const stdout = Buffer.concat(capturedStdout).toString('utf8');
const resultStart = stdout.lastIndexOf('{\n  "evidenceDirectory"');
assert.notEqual(resultStart, -1, 'capture harness의 evidence 요약 JSON을 찾지 못했습니다.');
const captureSummary = JSON.parse(stdout.slice(resultStart));
const runContext = {
  schemaVersion: 1,
  serial,
  baselinePath,
  baselineSceneCount: sceneIDs.length,
  sceneIDs,
  captureSummary: {
    evidenceDirectory: captureSummary.evidenceDirectory,
    target: captureSummary.target,
    observationCount: captureSummary.observations.length,
  },
};
assert.equal(runContext.captureSummary.target, 'android-device');
assert.equal(runContext.captureSummary.observationCount, sceneIDs.length);
await writeFile(resolve(captureSummary.evidenceDirectory, 'run-context.json'),
  `${JSON.stringify(runContext, null, 2)}\n`);
console.log(`run context saved: ${resolve(captureSummary.evidenceDirectory, 'run-context.json')}`);
