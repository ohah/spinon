import assert from 'node:assert/strict';
import fs from 'node:fs';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';

const evidence = 'spec/internal/evidence/c15-3-flex-intrinsic-layout-2026-11-15';
const c11Pass07 = 'spec/internal/evidence/c11-4-sibling-layout-regressions-current-source-2026-11-15/runtime-matrices-current-source-pass07';
const candidatePaths = ['crates/spinon-layout/src/c15_text_layout_tests.rs', 'crates/spinon-layout/src/c15_text_layout_tests/flex_intrinsic.rs'];
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
function parseShaFile(filename) {
  return new Map(fs.readFileSync(filename, 'utf8').trim().split(/\r?\n/).map(line => {
    const [hash, ...parts] = line.trim().split(/\s+/);
    return [parts.join(' '), hash];
  }));
}
function findJsonHash(filename, sourcePath) {
  const data = JSON.parse(fs.readFileSync(filename, 'utf8'));
  const entry = data.files.find(item => item.path === sourcePath);
  assert.ok(entry, filename + ': missing ' + sourcePath);
  return { sha256: entry.sha256, bytes: entry.bytes };
}
const c15Before = parseShaFile(evidence + '/source-hashes-pass02-before.sha256');
const c15After = parseShaFile(evidence + '/source-hashes-pass02-after.sha256');
const c11Before = c11Pass07 + '/source-hashes-before.json';
const c11After = c11Pass07 + '/source-hashes-after.json';
const c11Manifest = JSON.parse(fs.readFileSync(c11Pass07 + '/run-manifest.json', 'utf8'));
const currentHead = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
assert.equal(currentHead, c11Manifest.head, 'checkout HEAD equals pass07 source HEAD');
const files = candidatePaths.map(sourcePath => {
  const currentBytes = fs.readFileSync(sourcePath);
  const currentSha256 = sha256(currentBytes);
  const c11b = findJsonHash(c11Before, sourcePath);
  const c11a = findJsonHash(c11After, sourcePath);
  const result = {
    path: sourcePath,
    bytes: currentBytes.length,
    currentSha256,
    c15Pass02BeforeSha256: c15Before.get(sourcePath),
    c15Pass02AfterSha256: c15After.get(sourcePath),
    c11Pass07BeforeSha256: c11b.sha256,
    c11Pass07AfterSha256: c11a.sha256,
    c11Pass07BeforeBytes: c11b.bytes,
    c11Pass07AfterBytes: c11a.bytes,
  };
  for (const key of ['c15Pass02BeforeSha256','c15Pass02AfterSha256','c11Pass07BeforeSha256','c11Pass07AfterSha256']) {
    assert.equal(result[key], currentSha256, sourcePath + ': ' + key + ' differs from current');
  }
  assert.equal(c11b.bytes, currentBytes.length, sourcePath + ': pass07 before size differs');
  assert.equal(c11a.bytes, currentBytes.length, sourcePath + ': pass07 after size differs');
  return result;
});
const counts = Object.fromEntries(['passed','failed'].map(status => [status, c11Manifest.selected_tests.filter(test => test.status === status).length]));
assert.equal(c11Manifest.selected_tests.length, 43);
assert.equal(counts.passed, 36);
assert.equal(counts.failed, 7);
assert.equal(c11Manifest.input_hashes.file_count, 1920);
assert.equal(c11Manifest.input_hashes.compared_file_count, 1920);
assert.equal(c11Manifest.input_hashes.mismatch_count, 0);
const report = {
  audit: 'C15.3 Flex intrinsic test source hash closure after C11.4 sibling pass07',
  head: currentHead,
  c11Pass07: { selectedTestCount: 43, counts, inputCount: 1920, comparedInputCount: 1920, hashMismatchCount: 0, matrixPassed: false, interpretation: '36 passed and 7 failed; hash closure is not a green matrix result' },
  candidateSourceCount: files.length,
  allCandidateSourcesMatchC15FocusedRunAndC11Pass07BeforeAfterAndCurrent: true,
  files,
};
const output = evidence + '/source-hash-closure-pass01.json';
assert.ok(!fs.existsSync(output), 'preserve existing closure evidence');
fs.writeFileSync(output, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify(report, null, 2) + '\n');
