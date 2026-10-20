import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-image-ratio-fixed-gutter-physical-android-root-pass31-selected');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-image-ratio-fixed-gutter-physical-android-root-pass29-selected');
const addedEvidence = 'c11-4-image-ratio-fixed-gutter-physical-android-2026-10-21';
const sourceEvidence = path.join(repositoryRoot, 'spec/internal/evidence', addedEvidence);
assert.ok(fs.statSync(previousRoot).isDirectory(), `previous selected root is missing: ${previousRoot}`);
assert.ok(!fs.existsSync(selectedRoot), `refusing to replace existing selected root: ${selectedRoot}`);

fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true });
for (const [relative, source] of [
  ['STATUS.md', 'spec/STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['public/roadmap.html', 'spec/public/roadmap.html'],
  ['plan/c11-4-image-ratio-fixed-gutter.md', 'plan/c11-4-image-ratio-fixed-gutter.md'],
]) {
  const target = path.join(selectedRoot, relative);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(repositoryRoot, source), target);
}
const runIDs = fs.readdirSync(sourceEvidence, { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && entry.name.startsWith('c114-fg-device-'))
  .map((entry) => entry.name).sort();
assert.ok(runIDs.length > 0, 'no Android physical run evidence was found');
for (const runID of runIDs) {
  const result = JSON.parse(fs.readFileSync(path.join(sourceEvidence, runID, 'result.json'), 'utf8'));
  assert.equal(result.runID, runID, `run directory/result ID mismatch: ${runID}`);
  assert.equal(result.status, 'passed', `Android device run did not pass: ${runID}`);
}
for (const relative of [`internal/evidence/${addedEvidence}`, `spec/internal/evidence/${addedEvidence}`]) {
  const target = path.join(selectedRoot, relative);
  fs.rmSync(target, { recursive: true, force: true });
  fs.mkdirSync(target, { recursive: true });
  for (const name of ['README.md', 'physical-validation-review-20.md', 'capture-android-device.mjs']) {
    fs.copyFileSync(path.join(sourceEvidence, name), path.join(target, name));
  }
  for (const runID of runIDs) {
    fs.cpSync(path.join(sourceEvidence, runID), path.join(target, runID), { recursive: true });
  }
}

const publicEvidence = path.join(selectedRoot, 'public/internal/evidence', addedEvidence);
fs.rmSync(publicEvidence, { recursive: true, force: true });
fs.mkdirSync(publicEvidence, { recursive: true });
fs.copyFileSync(path.join(sourceEvidence, 'capture-android-device.mjs'), path.join(publicEvidence, 'capture-android-device.mjs'));
for (const runID of runIDs) {
  fs.cpSync(path.join(sourceEvidence, runID), path.join(publicEvidence, runID), { recursive: true });
}

const evidenceFiles = [];
for (const relative of [`internal/evidence/${addedEvidence}`, `spec/internal/evidence/${addedEvidence}`]) {
  const directory = path.join(selectedRoot, relative);
  for (const entry of fs.readdirSync(directory)) evidenceFiles.push(`${relative}/${entry}`);
}
assert.equal(evidenceFiles.filter((entry) => entry.endsWith('/README.md')).length, 2);
process.stdout.write(`${JSON.stringify({
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  previousRoot: path.relative(repositoryRoot, previousRoot),
  overwrittenFiles: ['STATUS.md', 'spec/STATUS.md', 'public/roadmap.html', 'plan/c11-4-image-ratio-fixed-gutter.md'],
  includedRunIDs: runIDs,
  addedEvidenceCopies: evidenceFiles.length,
  publicRawEvidenceFiles: fs.readdirSync(publicEvidence).length,
}, null, 2)}\n`);
