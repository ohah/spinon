import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-plan-status-sync-20261024-root-pass03-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-code-roadmap-sync-20261024-root-pass01-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-code-roadmap-sync-20261024-site-pass01-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const originMain = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();

assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'previous curated root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to reuse selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to reuse selected output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected-root manifest');

const sourceFiles = [
  'plan/c11-4-grid-track-sizing.md',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/README.md',
  'spec/internal/evidence/c11-4-code-roadmap-sync-2026-10-24/pr-diff-review-20.md',
];

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in curated base: ' + sourcePath);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error('unsupported curated-base entry: ' + sourcePath);
  }
}

function aliases(relative) {
  if (relative.startsWith('plan/')) return [relative, 'spec/' + relative];
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    return [...new Set([relative, withoutSpec, 'public/' + withoutSpec, 'public/' + relative])];
  }
  return [relative];
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of sourceFiles) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe overlay source: ' + relative);
  const bytes = fs.readFileSync(source);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const destinationStat = fs.lstatSync(destination);
      assert.ok(destinationStat.isFile() && !destinationStat.isSymbolicLink(), 'unsafe overlay destination: ' + alias);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    overlays.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const manifest = {
  pass: 'pass01',
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  head,
  originMain,
  sourceFiles,
  overlayCount: overlays.length,
  overlays,
  storageMethod: 'hardlink previous curated root, unlink overlay target, write current source bytes',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ pass: manifest.pass, head, originMain, selectedRoot: manifest.selectedRoot, overlayCount: overlays.length, noDeletes: true }, null, 2) + '\n');
