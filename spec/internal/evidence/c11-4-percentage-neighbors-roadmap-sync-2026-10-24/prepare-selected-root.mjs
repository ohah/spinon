import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-root-pass01-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-percentage-neighbors-roadmap-sync-20261024-root-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-percentage-neighbors-roadmap-sync-20261024-site-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-manifest.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const headTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(headTree, mainTree, 'base checkout differs from origin/main');
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'existing selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to reuse selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to reuse build output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite manifest');

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base root: ${sourcePath}`);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error(`unsupported base entry: ${sourcePath}`);
  }
}

function aliases(relative) {
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative === 'spec/public/roadmap.html') {
    targets.add('public/roadmap.html');
    targets.add('public/spec/public/roadmap.html');
  }
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    targets.add(withoutSpec);
    targets.add(`public/${withoutSpec}`);
    targets.add(`public/${relative}`);
  }
  return [...targets];
}

function overlay(relative, bytes, records) {
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe overlay target: ${alias}`);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    records.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const files = [
  'plan/c11-4-auto-repeat-percentage-neighbors.md',
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/README.md',
  'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/col-fill-both.png',
  'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T203501591Z-007b4f1b/col-fill-both.png',
  'spec/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.md',
  'spec/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/tailnet-roadmap-before-sync.png',
];

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of files) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${relative}`);
  overlay(relative, fs.readFileSync(source), overlays);
}

const manifest = {
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  headTree,
  originMainTree: mainTree,
  overlaySourceFiles: files,
  overlays,
  storageMethod: 'hardlink existing selected root; unlink each overlay target before writing selected copies',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selectedRoot: manifest.selectedRoot, outputRoot: manifest.outputRoot, overlaySourceFiles: files.length, overlayCopies: overlays.length, headTree, noDeletes: true }, null, 2)}\n`);
