import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr517-tailnet-root-pass52-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-replaced-image-plan-root-pass08');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(previousRoot).isDirectory(), 'prior selected documentation root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace the selected root');

function hardlinkTree(sourceRoot, targetRoot) {
  fs.mkdirSync(targetRoot, { recursive: true });
  for (const entry of fs.readdirSync(sourceRoot, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const source = path.join(sourceRoot, entry.name);
    const target = path.join(targetRoot, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in prior selected root: ${source}`);
    if (entry.isDirectory()) hardlinkTree(source, target);
    else if (entry.isFile()) fs.linkSync(source, target);
    else throw new Error(`unsupported prior selected root entry: ${source}`);
  }
}

hardlinkTree(previousRoot, selectedRoot);
const copied = [];
function overlay(sourceRelative, targetRelative) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source must be a regular file: ${sourceRelative}`);
  if (fs.existsSync(target)) {
    const targetStat = fs.lstatSync(target);
    assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `overlay target must be a regular file: ${targetRelative}`);
    fs.unlinkSync(target);
  }
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target, fs.constants.COPYFILE_EXCL);
  const bytes = fs.readFileSync(target);
  copied.push({ source: sourceRelative, target: targetRelative, bytes: bytes.byteLength, sha256: hash(bytes) });
}

for (const file of [
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-cyclic-minmax-replaced-image-span.md',
  'plan/c11-4-cyclic-minmax-replaced-image-span-plan-review-20.md',
]) overlay(file, file);

overlay('spec/public/roadmap.html', 'roadmap.html');
overlay('spec/public/roadmap.html', 'public/roadmap.html');

const evidenceRelative = 'spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23';
const evidenceNames = ['README.md', 'spec-derived-expectations.json', 'chrome156-observation.json', 'chrome-agent-browser-precomparison.png', 'plan-pr-diff-review-20.md', 'roadmap-browser-verification.json'];
for (const alias of ['internal', 'spec/internal', 'public/internal', 'public/spec/internal']) {
  for (const name of evidenceNames) {
    overlay(`${evidenceRelative}/${name}`, `${alias}/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/${name}`);
  }
}

const sourceRevision = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceMain = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const manifest = {
  pass: 'pass08',
  sourceRevision,
  sourceMain,
  previousRoot: path.relative(repositoryRoot, previousRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  method: 'hard-linked prior selected tree with copy-on-write file overlays',
  copied,
  unrelatedUntrackedWorkspaceFilesTouched: false,
  existingPreviewServerTouched: false,
  existingTmuxSessionsTouched: false,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass08.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ pass: manifest.pass, sourceRevision, sourceMain, copiedCount: copied.length, selectedRoot: manifest.selectedRoot }, null, 2)}\n`);
