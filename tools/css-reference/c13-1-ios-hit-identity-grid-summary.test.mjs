import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const helper = path.join(repositoryRoot, 'platforms/ios/Sources/C131HitIdentityGridSummary.swift');
const harness = path.join(repositoryRoot, 'tools/css-reference/c13-1-ios-hit-identity-grid-summary-test.swift');

test('C13.1 iOS app summary parser rejects malformed and mutated hit-grid reports', {
  skip: process.platform !== 'darwin',
}, () => {
  const temporaryDirectory = mkdtempSync(path.join(tmpdir(), 'spinon-c131-ios-grid-summary-'));
  const executable = path.join(temporaryDirectory, 'summary-tests');
  try {
    const compile = spawnSync('xcrun', ['swiftc', helper, harness, '-o', executable], {
      encoding: 'utf8',
      timeout: 120_000,
      maxBuffer: 4 * 1024 * 1024,
    });
    assert.equal(compile.status, 0, `${compile.stdout}\n${compile.stderr}`);

    const run = spawnSync(executable, [], {
      encoding: 'utf8',
      timeout: 10_000,
      maxBuffer: 4 * 1024 * 1024,
    });
    assert.equal(run.status, 0, `${run.stdout}\n${run.stderr}`);
    assert.match(run.stdout, /C13\.1 iOS hit identity grid summary parser: \d+ positive assertions and \d+ negative controls passed/);
  } finally {
    rmSync(temporaryDirectory, { recursive: true, force: true });
  }
});
