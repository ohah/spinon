import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = fileURLToPath(new URL('../..', import.meta.url));
const classifier = path.join(
  repositoryRoot,
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/WheelSourceClassifier.java',
);
const harness = path.join(
  repositoryRoot,
  'tools/css-reference/fixtures/WheelSourceClassifierJvmTest.java',
);

test('Android wheel source classification rejects device-union misclassification', () => {
  const outputDirectory = mkdtempSync(path.join(os.tmpdir(), 'spinon-c133-wheel-classifier-'));
  try {
    execFileSync('javac', ['-d', outputDirectory, classifier, harness], { stdio: 'pipe' });
    const output = execFileSync('java', [
      '-cp', outputDirectory,
      'dev.spinon.bootstrap.WheelSourceClassifierJvmTest',
    ], { encoding: 'utf8', stdio: 'pipe' });
    assert.match(output, /assertions=8 status=passed/);
    assert.match(output, /case=composite-device-mouse-negative-control result=0/);
    assert.match(output, /case=legacy-device-source-union result=1 expected_mismatch=true/);
    assert.equal(readFileSync(classifier, 'utf8').includes('getSources'), false);
  } finally {
    rmSync(outputDirectory, { recursive: true, force: true });
  }
});
